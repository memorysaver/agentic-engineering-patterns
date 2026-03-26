---
name: evaluator-setup
description: Set up a separate evaluator agent for a workspace session. Use when starting a complex feature, UI-heavy work, or any task at the edge of model capability. Guides spawning a dedicated evaluator agent via cmux that reviews the generator's work with calibrated scoring criteria. Inspired by Anthropic's GAN-based generator-evaluator pattern.
---

# Evaluator Setup

Set up a dedicated evaluator agent that reviews the generator agent's work from a separate context window. This is the single most impactful improvement for agent output quality.

## Why Separate Evaluation

When asked to evaluate their own work, agents consistently rate it positively — even when quality is mediocre. Anthropic's research found that separating generation from evaluation (inspired by GANs) dramatically improves output quality:

- The **generator** focuses on building features
- The **evaluator** focuses on finding problems
- Separation makes it easy to calibrate the evaluator toward **skepticism**
- Self-evaluation cannot achieve the same level of honesty

> "When asked to evaluate work they've produced, agents tend to respond by confidently praising the work — even when, to a human observer, the quality is obviously mediocre."

**Source:** [Harness Design for Long-Running Application Development](https://www.anthropic.com/engineering/harness-design-long-running-apps) — Anthropic Engineering

---

## When to Use

| Use evaluator | Skip evaluator |
|--------------|---------------|
| Complex features with multiple tasks | Single-file config changes |
| UI-heavy work (forms, dashboards, layouts) | Simple CRUD endpoints |
| Auth, payments, or security-sensitive work | Documentation updates |
| Features at the edge of model capability | Bug fixes with clear repro steps |
| Multi-component integrations | Dependency upgrades |

**Rule of thumb:** If the feature has 3+ tasks in `tasks.md` or touches UI, use an evaluator.

---

## Prerequisites

- A workspace session is already running (Part D of the main workflow)
- The generator agent has completed (or is about to start) Phase 4
- `tmux` and `cmux` are available
- The same tmux session as the workspace is accessible

---

## Setup Steps

### 0. Brainstorm Evaluation Criteria

Before spawning the evaluator, brainstorm **project-specific** scoring criteria with the user. Generic criteria miss what matters; task-specific criteria catch the right problems.

#### a. Read the OpenSpec change

```bash
cat openspec/changes/<change-name>/proposal.md
cat openspec/changes/<change-name>/design.md
ls openspec/changes/<change-name>/specs/
cat openspec/changes/<change-name>/tasks.md
```

#### b. Identify the feature type

Based on the change artifacts, classify the feature:

| Feature type | Signals |
|-------------|---------|
| **UI-heavy** | Forms, dashboards, layouts, user-facing pages |
| **API-only** | Endpoints, services, integrations, no frontend |
| **Security-sensitive** | Auth, payments, data handling, permissions |
| **Data pipeline** | ETL, migrations, batch processing, data transforms |
| **Mixed** | Full-stack features spanning multiple categories |

#### c. Propose dimensions

Read the dimension presets in `references/evaluator-criteria.md` (bottom section). Based on the feature type, propose:

- Which default dimensions to **keep** (Completeness, Correctness, UX, Security, Code Quality)
- Which to **drop** or de-weight
- Which to **add** (Originality, Accessibility, API Design, Performance, Data Integrity, etc.)
- Which to **weight heavily** — these are where the model tends to fall short

#### d. Ask the user

Present the proposed dimensions and ask:

1. **Which dimensions matter most** for this specific feature?
2. **What does "good" look like** — any concrete quality bars? (e.g., "must pass WCAG AA", "P95 latency < 200ms", "no N+1 queries")
3. **Where have you seen mediocre output** from the model before on similar work?
4. **Any hard failure conditions** beyond the defaults? (e.g., "Security below 4 is unacceptable for this auth feature")

#### e. Generate project-specific criteria

Write `.dev-workflow/evaluator-criteria.md` (per-workspace, not the default reference) with:

- The agreed-upon dimensions with weights
- Scale definitions tailored to this feature (what 1/3/5 look like for *this* project)
- Hard failure thresholds reflecting what the user cares about
- Few-shot examples adapted from the defaults in `references/evaluator-criteria.md`

```
.dev-workflow/evaluator-criteria.md   ← brainstormed, per-workspace
references/evaluator-criteria.md       ← defaults + presets (read-only reference)
```

> **Skip brainstorming?** If the user wants to move fast, fall back to the default criteria at `references/evaluator-criteria.md`. But note that Anthropic's research found task-specific calibration significantly improves evaluator judgment.

---

### 1. Identify the workspace session

```bash
# Find the tmux session name
tmux list-sessions

# Verify the workspace path
ls .claude/workspaces/<name>/
```

### 2. Spawn the evaluator agent

Create a second cmux tab in the same tmux session, pointed at the same workspace directory:

```bash
# Create a new cmux tab for the evaluator
EVAL_SURFACE=$(cmux new-surface --type terminal | grep -o 'surface:[0-9]*')
cmux send --surface "$EVAL_SURFACE" "tmux new-window -t <session-name> -n evaluator -c .claude/workspaces/<name>\n"

# Start Claude Code in the evaluator tab
cmux send --surface "$EVAL_SURFACE" "claude --dangerously-skip-permissions --rc\n"

# Rename the tab
cmux rename-tab --surface "$EVAL_SURFACE" "evaluator-<name>"
```

### 3. Send the evaluator bootstrap prompt

Wait for Claude Code to be ready, then send the bootstrap prompt. Keep it concise — the evaluator will read the referenced files itself:

```bash
cmux send --surface "$EVAL_SURFACE" "You are an EVALUATOR agent. Your job is to critically evaluate the generator's work, not to write code.

Start by reading these files:
1. .dev-workflow/evaluator-criteria.md (your scoring calibration — brainstormed for this feature)
   If this file does not exist, fall back to: skills/development-workflow/agentic-development-workflow/references/evaluator-criteria.md
2. All files in openspec/changes/<change-name>/ (proposal, design, specs, tasks)
3. .dev-workflow/contracts.md (success criteria per task)
4. .dev-workflow/feature-verification.json (verification checklist)

After reading, check .dev-workflow/signals/eval-request.md — if it exists, begin your evaluation. If it does not exist yet, the generator is still implementing. Check again at phase boundaries or when prompted.

Follow the evaluation protocol in your criteria file. Write your response to .dev-workflow/signals/eval-response-1.md.

CRITICAL: Score honestly. Do not rationalize problems away. Apply hard failure thresholds strictly. Never modify verification_steps in feature-verification.json.
"
```

### 4. Notify the generator

After the evaluator is set up, tell the generator agent that evaluation is active:

```bash
# In the generator's cmux tab:
cmux send --surface "$GEN_SURFACE" "An evaluator agent is now running in a separate tab. After you complete Phase 4 implementation:
1. Write .dev-workflow/signals/eval-request.md describing what to evaluate
2. Wait for .dev-workflow/signals/eval-response-1.md
3. Read the eval response and fix any FAIL items
4. Write a new eval-request for round 2
5. Repeat until the evaluator passes all thresholds
"
```

---

## Generator-Evaluator Loop

The loop works through files in `.dev-workflow/signals/`:

```
Generator                           Evaluator
    │                                   │
    ├── Completes Phase 4               │
    │                                   │
    ├── Writes eval-request.md ────────►│
    │                                   ├── Reads request
    │                                   ├── Tests app
    │                                   ├── Scores dimensions
    │                                   ├── Writes eval-response-1.md
    │◄──────────────────────────────────┤
    │                                   │
    ├── Reads response                  │
    ├── Fixes FAIL items                │
    ├── Writes eval-request.md (round 2)│
    │──────────────────────────────────►│
    │                                   ├── Re-evaluates
    │                                   ├── Writes eval-response-2.md
    │◄──────────────────────────────────┤
    │                                   │
    ├── All PASS → continues to Phase 9 │
    │                                   │
```

### File conventions

- **eval-request.md** — Overwritten each round by the generator
- **eval-response-\<N\>.md** — One per round, numbered sequentially (preserved for history)
- **feature-verification.json** — Updated by evaluator with pass/fail results per round

### Max iterations

If the loop hasn't converged after **5 rounds**, stop and escalate to the human. This usually indicates a fundamental misunderstanding between spec and implementation.

---

## Evaluator Calibration

The evaluator's effectiveness depends on calibration. The scoring criteria in `references/evaluator-criteria.md` provide:

- **Scale definitions** — What each score means concretely
- **Hard failure thresholds** — Automatic fail conditions
- **Few-shot examples** — Show the difference between lenient (bad) and calibrated (good) evaluations
- **Anti-patterns** — Common evaluator failure modes to avoid

If you find the evaluator is too lenient or too strict, adjust the criteria document and re-send it to the evaluator agent.

---

## Integration with Main Workflow

The evaluator replaces the self-review portions of Phases 5-6:

| Without evaluator | With evaluator |
|------------------|---------------|
| Phase 5: Generator reviews own code | Phase 5: Evaluator reviews code and running app |
| Phase 6: Generator runs own dogfood | Phase 6: Evaluator runs dogfood with calibrated scoring |
| Self-evaluation bias | Independent critical assessment |

The generator still does Phase 5 completeness check (ensuring all tasks were implemented), but the **quality evaluation** shifts to the evaluator.

---

## Cleanup

After the evaluation loop completes and all phases pass:

```bash
# The evaluator session can be closed
tmux kill-window -t <session-name>:evaluator
```

The evaluation artifacts (`.dev-workflow/signals/eval-response-*.md`) are preserved in the workspace for reference during PR review.
