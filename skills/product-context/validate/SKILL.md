---
name: aep-validate
description: |-
  Generator/evaluator validation for any AEP artifact — product context, architecture, specs, code, or documents. Use after a generation phase (/aep-envision, /aep-map, /aep-design) or on "validate", "verify", "check the design", "dry-run", "evaluate", "gen/eval". Spawns parallel Generator, Evaluator, and (product context) Protocol Checker agents; fixes only the validated artifact.
---

# Validate

Run a generator/evaluator pattern against any artifact produced by the AEP workflow. The generator attempts to use the artifact (dry-run), the evaluator checks it against reality (codebase, constraints, downstream protocols), and the results are consolidated into fixes applied to the artifact itself.

**Core principle:** the agent that produced an artifact cannot honestly evaluate it — agents praise their own work. The generator/evaluator separation fixes this and is the single most impactful quality lever in agentic workflows. The role-separation contract, scoring framework, agent prompt templates, eval protocol, and findings format are canonical in `/aep-gen-eval` references — read them for the underlying mechanics; this skill applies the pattern to product artifacts.

**Where this fits:**

```
/aep-envision → /aep-map → /aep-model (UI-facing) → /aep-validate → /aep-dispatch → /aep-design → /aep-launch → /aep-build → /aep-wrap
                    ▲ you are here

Also usable after any phase:
  /aep-envision → /aep-validate   (validate product context)
  /aep-map      → /aep-validate   (validate architecture + stories)
  /aep-design   → /aep-validate   (validate specs before launch)
  /aep-build    → /aep-validate   (already built into Phase 5 — use that instead)
```

**Session:** Main, can be autonomous or interactive
**Input:** Any AEP artifact (`product/index.yaml` + `product-context.yaml` in split mode, or `product-context.yaml` alone in v1 mode, or OpenSpec change, design doc, code)
**Output:** The same artifact, with issues fixed. A validation report appended to changelog.

---

## Before Starting

Identify what is being validated:

```bash
ls product-context.yaml 2>/dev/null   # product context
ls openspec/changes/ 2>/dev/null      # OpenSpec changes
ls .dev-workflow/ 2>/dev/null          # design artifacts
```

When the user names the target, validate it. When they don't: if exactly one of the probes above returns a hit, validate that artifact; if more than one is present, ask which to validate rather than guessing. **Postcondition:** the artifact (a concrete path) and its mode (A/B/C/D) are fixed before spawning agents.

For product context, resolve split vs v1 mode with the probe in [references/file-resolution.md](references/file-resolution.md) (canonical for mode semantics). Split mode: validate both files and check cross-file consistency. V1 mode: validate `product-context.yaml` only.

---

## Step 1: Determine Validation Mode

The skill operates in one of four modes based on the artifact type. Each mode configures which agents to spawn and what they check. **Mode A (Product Context) is detailed below.** For Mode B (Design), Mode C (Code), or Mode D (Document) — agent roles, the Phase-5 code branch, and agent-count customization — read [references/modes.md](references/modes.md).

### Mode A: Product Context Validation

**When:** After `/aep-envision` or `/aep-map` — validating `product-context.yaml` (and `product/index.yaml` in split mode)

**Split-mode cross-file checks:**

- `stories[].layer` values must exist in `product/index.yaml` `product.layers[]`
- `stories[].activity` values must exist in `product/index.yaml` `product.activities[]`
- `calibration.plan[].dimensions[]` must reference `product/index.yaml` `product.quality_dimensions[]`
- No `opportunity` or `product` section should exist in `product-context.yaml` when split mode is active
- `product/index.yaml` must have `personas`, `capabilities`, and `product` sections
- If `object-model` is a declared quality dimension: every UI-facing capability has a `product/maps/<capability>/object-map.yaml`; each `stories[].object_model_refs` entry points to an existing object-map path + object id; object names are consistent with `architecture.domain_model` and `docs/glossary.md`

Mode A runs **two passes** — product design quality first, then technical correctness.

#### Pass 1: Product Design Evaluation ("Are we building the right thing?")

**Agents:** Product Design Evaluator + Vision Alignment Checker

| Agent                    | Role                                         | What it checks                                                                                                                 |
| ------------------------ | -------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------ |
| Product Design Evaluator | Review against user story mapping principles | Walking skeleton validity, layer ordering, INVEST compliance, dependency graph quality, activity coverage, narrative coherence |
| Vision Alignment Checker | Trace stories to opportunity brief           | Every story maps to a stated user need, no scope creep, JTBD coverage                                                          |

Use the Product Design Evaluator prompt from `/aep-gen-eval` `agent-contracts.md`, and the story-mapping scoring dimensions (Walking Skeleton Validity, Layer Ordering, Vision Alignment, INVEST Compliance) from `/aep-gen-eval` `scoring-framework.md`.

**Pass 1 hard failures:**

- Walking Skeleton Validity < 3 — Layer 0 is not minimal enough
- Vision Alignment < 3 — Stories have drifted from the product vision
- INVEST Compliance < 3 — Stories are not actionable by an autonomous agent

**Pass 1 activity checks** (if `product.activities` exists):

- **Activity Coverage:** Every activity with `layer_introduced: 0` has at least one Layer 0 story. An activity with no stories is a gap in the walking skeleton.
- **Activity Consistency:** Every story with a non-null `activity` references a valid activity id from `product.activities`.
- **Narrative Coherence:** Read activities left-to-right by `order` — they should form a coherent user narrative: "User [activity 1], then [activity 2], then..." If it doesn't flow, the backbone needs restructuring.
- **Infrastructure Ratio:** If more than 60% of Layer 0 stories have null activity, the decomposition may be too technical — consider reframing stories around user capabilities.

**Pass 1 Object Map checks** (if any `product/maps/*/object-map.yaml` exists):

- **CTA Coverage:** Every UI story's verbs map to a CTA on some object in its capability's object-map `coverage`. An uncovered UI story means a missing object or a hidden task-flow.
- **Object Home:** Every object in `primary_objects` has at least one `screens[]` entry (`collection` or `detail`). A primary object with no home screen is unreachable.
- **Anchor Present:** Each capability object-map declares a `navigation.anchor_object`.
- **Task-Flow Justified:** Every `interaction_modes` entry with `mode: task_oriented` has a non-empty `reason` (object-first is the default; deviations must be justified).
- **Approval State:** No UI-facing story is dispatch-ready while its capability object-map is `status: draft` or `stale` — it must be `approved` first (run `/aep-model`).
- **Ref Resolution:** Each `stories[].object_model_refs` entry (`<path>#<object-id>`) resolves — the path exists and `<object-id>` appears in that map's `primary_objects`/`supporting_objects` and in `object-model.yaml` `objects[]`.
- **Noun Coverage:** Every noun foraged from `product.activities` maps to an object or is justified as implementation-only.

#### Pass 2: Technical Validation ("Can we build it correctly?")

**Agents:** Generator + Evaluator + Protocol Checker

| Agent            | Role                            | What it checks                                                                         |
| ---------------- | ------------------------------- | -------------------------------------------------------------------------------------- |
| Generator        | Dry-run each story/layer        | Can each story be implemented? Missing details, ambiguous criteria, dependency gaps    |
| Evaluator        | Compare design vs codebase      | Package versions, import paths, existing patterns, file existence, API compatibility   |
| Protocol Checker | Verify downstream compatibility | Dispatch-required fields, DAG validity, scoring compatibility, file conflict detection |

**Why two passes:** Pass 1 catches product design problems (wrong stories, bad layering, vision drift); Pass 2 catches technical problems (missing fields, broken references, codebase mismatches). Both are required before dispatching to autonomous agents — the agents will faithfully build whatever you give them, right or wrong.

---

## Step 2: Assemble Validation Context

For each agent, prepare a focused context package — its scope determines evaluation quality. The generic Generator and Evaluator context recipes (what to include, what to exclude) are canonical in `/aep-gen-eval` `agent-contracts.md` ("Context Assembly Rules").

**Protocol Checker context (Mode A only):** give it the stories section of the artifact, the downstream protocol specification it must satisfy, and the topology + layer-gate definitions. The downstream protocol requirements (dispatch story fields, scoring formula, DAG rules, design/build handoff contracts) are specified in [references/protocol-specs.md](references/protocol-specs.md).

---

## Step 3: Spawn Agents

Launch all agents in parallel. Each works independently — they do not see each other's output. Use the prompt templates from `/aep-gen-eval` `agent-contracts.md`: Generator (Artifact Validation), Evaluator (Codebase Verification), Protocol Checker, and — for Mode A Pass 1 — Product Design Evaluator. **Postcondition:** one findings report returned per spawned agent.

---

## Step 4: Consolidate Findings

After all agents return, consolidate their findings into a single action list.

### Categorize by severity

| Category      | Description                                           | Action                 |
| ------------- | ----------------------------------------------------- | ---------------------- |
| **Blocking**  | Would stop downstream consumers from working          | Fix immediately        |
| **Important** | Would cause friction, confusion, or rework            | Fix before proceeding  |
| **Minor**     | Cosmetic, missing optional fields, documentation gaps | Record; fix if trivial |

### Deduplicate

Multiple agents may find the same issue from different angles. Merge these into a single finding with the combined evidence.

### Present to user

Show the consolidated findings with counts **before applying any fix** — the user may reprioritize or reject findings:

```
Validation complete: {N} blocking, {M} important, {K} minor issues found.

Blocking:
  1. [issue] — found by Generator + Evaluator
  2. [issue] — found by Protocol Checker

Important:
  3. [issue] — found by Generator
  ...
```

**Postcondition:** the counted, deduplicated findings list has been shown to the user.

---

## Step 5: Apply Fixes

Read the current on-disk state of the artifact (not a cached copy), then fix every **blocking** and **important** finding; record **minor** findings in the changelog and fix them if trivial. The artifact is not validated while any blocking finding remains.

**Rules for fixes:**

- Modify only the artifact being validated — never create new files, edit other artifacts, or implement code. (This is the one hard guardrail: fixes land in the validated artifact and nowhere else.)
- Preserve the artifact's existing structure and conventions.
- If a fix requires a decision the agent can't make (architectural choice, business priority), mark it as an `open_question` with a default assumption rather than guessing silently.
- Append a changelog entry (`date`, `author: aep-validate`, `summary`) per the product-context schema, recording what was validated and the blocking/important/minor counts fixed.

When the artifact is `product-context.yaml`, confirm it still parses before committing — run the validation command in [references/yaml-guardrails.md](references/yaml-guardrails.md). **Postcondition:** `npx js-yaml product-context.yaml` exits 0.

---

## Step 6: Commit

```bash
# Resolve $BASE (integration branch) per /aep-git-ref "Integration Branch".
git pull --ff-only origin "$BASE"
git add <validated-files>
git commit -m "fix: validate {artifact-name} — {N} issues found and fixed"
git push origin "$BASE"
```

**Postcondition:** the fixes are committed and pushed to `$BASE`.

---

## Validation Dimensions

When the agents evaluate, they weigh mode-specific dimensions (Completeness, Consistency, Security, Feasibility, Accuracy, and more). The per-mode dimension tables — and the pointer to the full 5-dimension code-scoring framework — are in [references/validation-dimensions.md](references/validation-dimensions.md).

---

## When NOT to Use This Skill

- **During `/aep-build` Phase 5** — use the built-in evaluator loop instead (it has `executor.spawn_evaluator`, verification JSON, scoring framework).
- **For subjective quality** — this skill validates factual correctness and completeness, not aesthetic judgment. For usability / design-quality review, use `/aep-design-lens` (a heuristic health-check grounded in HCI theory) — its design-quality pass complements validate's factual pass.
- **For tiny changes** — single-file edits or typo fixes don't need a 3-agent validation.

---

## Next Step

After validation, proceed to the appropriate downstream skill:

```
Product context validated → /aep-dispatch
Design validated          → /aep-launch
Code validated            → create PR (or /aep-build Phase 9)
Document validated        → publish/share
```
