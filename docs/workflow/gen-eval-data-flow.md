# Generator/Evaluator Data Flow

How the three tracking systems in `.dev-workflow/` work together during the gen/eval loop. This document covers the build workflow (Phase 0 init → Phase 5 evaluation). For the general pattern, see `skills/patterns/gen-eval/`.

---

## The Three Systems

The gen/eval pattern uses three complementary tracking systems. Each answers a different question:

| System                   | File                                                           | Question it answers                        | Granularity        |
| ------------------------ | -------------------------------------------------------------- | ------------------------------------------ | ------------------ |
| **Feature Verification** | `.dev-workflow/feature-verification.json`                      | "Did each task get done correctly?"        | Per task           |
| **Eval Signals**         | `.dev-workflow/signals/eval-request.md` + `eval-response-N.md` | "How good is the overall work?"            | Per round          |
| **Evaluator Criteria**   | `.dev-workflow/evaluator-criteria.md`                          | "What dimensions matter for this feature?" | Per feature (once) |

---

## System 1: Feature Verification JSON

**Purpose:** Task-level pass/fail checklist. Tracks whether each individual task was implemented correctly.

**File:** `.dev-workflow/feature-verification.json`

```json
[
  {
    "task": "feat: add creator_profile table with cascade deletion",
    "change_id": "motvylon",
    "verification_steps": [
      "creator_profile table exists in Drizzle schema",
      "userId has unique constraint and cascade delete",
      "migration generates and applies cleanly"
    ],
    "passes": false,
    "evaluated_by": null,
    "round": null,
    "notes": null
  }
]
```

### Field Ownership

| Field                | Who creates            | Who can modify     | When                                       |
| -------------------- | ---------------------- | ------------------ | ------------------------------------------ |
| `task`               | Generator (Phase 0)    | Nobody             | Set once during init                       |
| `change_id`          | Generator (Phase 0)    | Nobody             | Set once when creating jj change stack     |
| `verification_steps` | Generator (Phase 0)    | **Nobody**         | Extracted from contracts/specs — immutable |
| `passes`             | Generator sets `false` | **Evaluator only** | Updated after running each step            |
| `evaluated_by`       | Generator sets `null`  | **Evaluator only** | Agent identifier                           |
| `round`              | Generator sets `null`  | **Evaluator only** | Which eval round                           |
| `notes`              | Generator sets `null`  | **Evaluator only** | Detailed findings per task                 |

**Critical rule:** The generator MUST NOT modify `verification_steps`, `passes`, `evaluated_by`, `round`, or `notes`. This ensures the generator cannot mark its own work as passing.

**Why JSON:** Models tamper with JSON less than Markdown. The structured format makes field ownership enforceable.

---

## System 2: Eval Request/Response Signals

**Purpose:** Round-level quality assessment. The generator requests evaluation, the evaluator responds with dimension scores and a PASS/FAIL verdict.

**Files:**

- `.dev-workflow/signals/eval-request.md` — generator writes before each round
- `.dev-workflow/signals/eval-response-N.md` — evaluator writes after each round

### eval-request.md (generator writes)

```markdown
# Evaluation Request — Round 1

## What to evaluate

- All 5 OpenSpec tasks implemented across jj change stack
- Dev server running on port 3000

## Changes since last round

- First evaluation

## Known issues

- Asset cleanup on failed upload not implemented (deferred)

## Files changed

packages/db/src/schema/domain.ts | 85 ++++++
packages/api/src/routers/creator.ts | 42 +++
packages/api/src/routers/storage.ts | 68 +++++
5 files changed, 312 insertions(+)
```

### eval-response-N.md (evaluator writes)

```markdown
# Evaluation Round 1

## Findings

### FAIL: Missing ownership check on storage.createDownloadUrl (Security: 2)

- Steps: Call storage.createDownloadUrl with another user's assetId
- Expected: 403 Forbidden
- Actual: Returns presigned URL for any asset regardless of ownership
- Impact: Any authenticated user can download any other user's files
- Fix: Verify asset.creatorId matches session user's creator profile

### PASS: Creator upsert works correctly (Completeness: 4)

- Tested create and update flows, both work as specified

## Scores

- Completeness: 4 — All tasks implemented
- Correctness: 3 — Flows work but edge case in download auth
- UX Quality: N/A — API only, no frontend in this slice
- Security: 2 — Missing ownership check on download endpoint
- Code Quality: 4 — Clean, consistent, follows project patterns

## Result: FAIL

Security (2) is below threshold (minimum 3).
Generator must add ownership verification before re-evaluation.

## Verification Updates

- Task 1 (creator_profile table): passes: true
- Task 2 (asset table): passes: true
- Task 3 (storage router): passes: false — ownership check missing
- Task 4 (creator router): passes: true
- Task 5 (migration): passes: true
```

### Field Ownership

| Field                | Who writes | When                         |
| -------------------- | ---------- | ---------------------------- |
| `eval-request.md`    | Generator  | Before each evaluation round |
| `eval-response-N.md` | Evaluator  | After evaluating each round  |

The generator never writes eval-response files. The evaluator never writes eval-request files.

---

## System 3: Evaluator Criteria

**Purpose:** Calibration document that tells the evaluator which dimensions to score, how to weight them, and where the hard failure thresholds are. Written once during `/launch`, read-only during the eval loop.

**File:** `.dev-workflow/evaluator-criteria.md`

**Source:** Generated by `/launch` during evaluator brainstorming, based on the presets in `.claude/skills/aep-gen-eval/references/scoring-framework.md`.

### Example (API-only feature)

```markdown
# Evaluator Criteria — Storage API

## Dimensions

- Completeness (1-5) — All endpoints implemented per spec
- Correctness (1-5) — Endpoints return correct responses for all inputs
- API Design (1-5) — Consistent naming, proper status codes, error format
- Security (1-5) — Input validation, auth checks, ownership verification
- Performance (1-5) — No N+1 queries, proper indexing

## Hard Failure Thresholds

- Completeness < 4 → FAIL
- Correctness < 3 → FAIL
- Security < 3 → FAIL
- Any dimension < 2 → FAIL

## Feature-Specific Notes

- This feature handles biometric data (selfie images) — security is weighted high
- Presigned URLs must have TTL ≤ 1 hour
- All asset access must verify ownership via creatorId → userId chain
```

### Field Ownership

| Who                             | Action                    | When                                  |
| ------------------------------- | ------------------------- | ------------------------------------- |
| `/launch` (main session + user) | Creates the file          | During evaluator brainstorming        |
| Evaluator                       | Reads the file            | At the start of each evaluation round |
| Nobody                          | Modifies during eval loop | Criteria are fixed once set           |

---

## How They Connect: The Eval Loop

```
┌─────────────────────────────────────────────────────────────────────┐
│ Phase 0: Initialize Tracking                                        │
│                                                                     │
│  Generator reads:                                                   │
│    openspec/changes/<name>/specs/*.md                               │
│    openspec/changes/<name>/tasks.md                                 │
│                                                                     │
│  Generator creates:                                                 │
│    .dev-workflow/contracts.md          ← success criteria per task   │
│    .dev-workflow/feature-verification.json  ← task checklist        │
│                                             (all passes: false)     │
│                                                                     │
│  /launch already created:                                           │
│    .dev-workflow/evaluator-criteria.md ← scoring calibration        │
└─────────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────────┐
│ Phase 4: Implementation                                             │
│                                                                     │
│  Generator implements each task in jj change stack                  │
│  (one change per task from tasks.md)                                │
└─────────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────────┐
│ Phase 5: Eval Loop — Round N                                        │
│                                                                     │
│  ┌──────────────────────┐                                           │
│  │     GENERATOR        │                                           │
│  │                      │                                           │
│  │  1. Self-check:      │                                           │
│  │     jj diff -r each  │                                           │
│  │     change vs spec   │                                           │
│  │                      │                                           │
│  │  2. Write:           │                                           │
│  │     signals/         │                                           │
│  │     eval-request.md  │─────────────────────┐                     │
│  │                      │                     │                     │
│  │  3. Spawn evaluator  │                     │                     │
│  │     (tmux bottom)    │                     ▼                     │
│  │                      │          ┌──────────────────────┐         │
│  │  4. Poll for         │          │     EVALUATOR        │         │
│  │     response...      │          │                      │         │
│  │                      │          │  Reads:              │         │
│  │                      │          │  ① evaluator-        │         │
│  │                      │          │    criteria.md       │         │
│  │                      │          │    (HOW to score)    │         │
│  │                      │          │                      │         │
│  │                      │          │  ② eval-request.md   │         │
│  │                      │          │    (WHAT to evaluate)│         │
│  │                      │          │                      │         │
│  │                      │          │  ③ openspec/changes/ │         │
│  │                      │          │    (WHAT was spec'd) │         │
│  │                      │          │                      │         │
│  │                      │          │  ④ feature-          │         │
│  │                      │          │    verification.json │         │
│  │                      │          │    (WHICH tasks)     │         │
│  │                      │          │                      │         │
│  │                      │          │  Actions:            │         │
│  │                      │          │  - Review code       │         │
│  │                      │          │  - Test running app  │         │
│  │                      │          │  - Score dimensions  │         │
│  │                      │          │  - Apply thresholds  │         │
│  │                      │          │                      │         │
│  │                      │          │  Writes:             │         │
│  │                      │     ┌────│  ⑤ eval-response-N   │         │
│  │                      │     │    │    .md (scores)      │         │
│  │                      │     │    │                      │         │
│  │                      │     │    │  ⑥ Updates           │         │
│  │                      │     │    │    feature-           │         │
│  │                      │     │    │    verification.json  │         │
│  │                      │     │    │    (passes: t/f)      │         │
│  │  5. Read response  ◄─┘    │    └──────────────────────┘         │
│  │                      │                                           │
│  │  6. If FAIL:         │                                           │
│  │     Fix issues       │                                           │
│  │     Loop → Round N+1 │                                           │
│  │                      │                                           │
│  │  7. If PASS:         │                                           │
│  │     → Phase 6        │                                           │
│  │                      │                                           │
│  │  8. Max 5 rounds:    │                                           │
│  │     → Escalate       │                                           │
│  └──────────────────────┘                                           │
└─────────────────────────────────────────────────────────────────────┘
```

---

## Lifecycle Summary

| Phase            | What happens to each system                                                                                                             |
| ---------------- | --------------------------------------------------------------------------------------------------------------------------------------- |
| `/launch`        | `evaluator-criteria.md` created (brainstormed with user)                                                                                |
| Phase 0          | `feature-verification.json` created (all `passes: false`), `contracts.md` created                                                       |
| Phase 4          | No tracking changes (generator implements code)                                                                                         |
| Phase 5 Round 1  | Generator writes `eval-request.md`. Evaluator reads all three systems, writes `eval-response-1.md`, updates `feature-verification.json` |
| Phase 5 Round 2+ | Generator writes updated request (notes fixes). Evaluator re-evaluates, updates response + verification.                                |
| Phase 5 PASS     | All three systems finalized. Verification JSON shows which tasks pass/fail. Response shows overall scores.                              |
| Phase 6+         | Systems are read-only. Results referenced in dogfood/testing/PR phases.                                                                 |

---

## Relationship to /validate

The `/validate` skill uses the same gen/eval pattern but with a different execution model:

| Aspect    | `/build` Phase 5                           | `/validate`                                        |
| --------- | ------------------------------------------ | -------------------------------------------------- |
| Execution | Sequential (tmux split panes)              | Parallel (Agent tool calls)                        |
| Tracking  | `feature-verification.json` + signal files | Findings consolidated in memory                    |
| Rounds    | Multi-round (1-5) with fixes between       | Single-pass (no fix loop)                          |
| Criteria  | `.dev-workflow/evaluator-criteria.md`      | Mode-specific dimensions from scoring-framework.md |
| Artifact  | Code in a workspace                        | Any artifact (product context, design, docs)       |

Both consume the shared framework from `skills/patterns/gen-eval/references/`.

---

## Key Design Decisions

**Why two separate pass/fail systems (verification JSON + eval response)?**

They measure different things. The verification JSON asks "did you do what you said you'd do?" (task completeness). The eval response asks "is the work good?" (quality). A task can pass verification (code exists, tests run) but fail quality review (security hole, poor UX). Both signals are needed for honest assessment.

**Why is the verification JSON immutable for the generator?**

If the generator could modify `verification_steps` or `passes`, it would self-certify its own work. The entire point of gen/eval separation is that the producer cannot judge its own output. The JSON format (not Markdown) makes field boundaries machine-parseable and harder to accidentally corrupt.

**Why is `evaluator-criteria.md` written at launch, not at eval time?**

Criteria should be agreed with the user before implementation begins, not improvised during evaluation. Writing criteria after seeing the code would anchor the evaluator to what was built rather than what was specified. The criteria represent the user's quality bar, not the model's.
