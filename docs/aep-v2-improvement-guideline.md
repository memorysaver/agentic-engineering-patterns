# AEP v2 Improvement Roadmap

**Capability Maps, Technical Specifications, and Dispatch Enhancements**

> This document is the forward-looking improvement roadmap for AEP. It captures proposals not yet implemented. For current documentation, see the [README](../README.md), [Glossary](glossary.md), and [Skills Quick Reference](skills-quick-reference.md).

> **What's already landed:** `.5` alignment layers via `/calibrate` (7 quality dimensions, heavy/light classes), process learning capture via `/workflow-feedback`, "wave" terminology for parallel story batches, technical spec template (Symphony SPEC.md pattern), and documentation reorganization. This roadmap covers what remains.

> **Design principle: no new commands.** Every proposal below enhances existing skills. AEP already has 17 commands — adding more creates cognitive overhead that outweighs the benefit.

---

## 1. Multi-Map Capability Structure

### 1.1 The Problem

`product-context.yaml` works well for products with a single user journey. But complex products with multiple distinct user journeys (e.g., "buyer flow" and "seller flow") need multiple story maps — each telling one complete left-to-right narrative per Jeff Patton's guidance.

Today, all activities and stories live in one flat list inside `product-context.yaml`. There's no structural way to say "these activities belong to the buyer journey" vs "these belong to the seller journey."

### 1.2 Capability Maps

Add per-capability map files for human narrative, while `product-context.yaml` remains the single operational file.

```
product/
  index.yaml                           # Program frame (opportunity, personas, capabilities)
  maps/
    <capability-a>/
      frame.yaml                       # Scope + boundary + outcome contract
      map.yaml                         # Backbone + layers + story stubs
    <capability-b>/
      frame.yaml
      map.yaml
product-context.yaml                   # Operational file (unchanged — stories, dispatch, topology, cost)
```

**How it works:**

- `/envision` creates `product/index.yaml` + first capability's `frame.yaml`
- `/map` writes per-capability `map.yaml` (backbone, layers, story stubs, architecture hints) AND updates `product-context.yaml` with full stories
- `/dispatch`, `/autopilot`, `/wrap` continue reading `product-context.yaml` — no change
- `/reflect` updates both: capability maps (product learnings) and `product-context.yaml` (story state)
- `/calibrate scope-direction` replaces the proposed `/slice` command — release line cutting is a human alignment decision about scope

**When to use:** Products with 2+ distinct user journeys. Simple single-journey products can skip capability maps entirely and continue with `product-context.yaml` alone.

### 1.3 Index File Schema

```yaml
# product/index.yaml
schema: v2
project: <project-name>
version: "0.1.0"

opportunity:
  bet: "Core hypothesis"
  why_now: "Market timing"
  kill_criteria: []
  decision: proceed # proceed | kill | defer

personas:
  - id: <persona-id>
    description: "Concrete user with context"
    jtbd: "When [situation], I want to [motivation], so I can [outcome]"

capabilities:
  - id: <capability-id>
    name: "e.g., Order to Cash"
    description: "One-line description of the user journey"
    map_path: "maps/<capability-id>/"
    status: active # active | planned | completed | deprecated
    depends_on: [] # cross-capability dependencies

constraints:
  technical: []
  business: []
```

### 1.4 Capability Frame (`frame.yaml`)

```yaml
capability: <capability-id>
scope: "What this map covers"
primary_user: <persona-id>

boundary:
  start_trigger: "What initiates this journey"
  end_success_state: "What 'done' looks like"

outcome_contract:
  hypothesis: "If users can do X, then Y business outcome follows"
  success_metric:
    type: "e.g., task_completion_rate"
    target: ">= 80%"
  decision_rule:
    keep_if: "metric >= target"
    otherwise: "reflect_and_reslice"

out_of_scope:
  - "Explicitly excluded concern"
```

### 1.5 Capability Map (`map.yaml`)

```yaml
capability: <capability-id>

activities:
  - id: <activity-id>
    verb_phrase: "e.g., Authenticate"
    description: "What the user does and why"
    layer_introduced: 0

layers:
  - layer: 0
    name: "Walking Skeleton"
    outcome_hypothesis: "Users can complete crudest end-to-end journey"
    success_metric:
      type: task_completion_rate
      target: ">= 60%"

stories:
  - id: <CAPABILITY>-001
    title: "Short, descriptive"
    layer: 0
    activity: <activity-id>
    description: "What changes when complete"
    acceptance_criteria_sketch:
      - "High-level criterion (refined by /map when writing to product-context.yaml)"
    complexity_estimate: S # S | M | L
    depends_on_stubs: []
```

### 1.6 How Existing Commands Adapt

| Command      | Change                                                                                           |
| ------------ | ------------------------------------------------------------------------------------------------ |
| `/envision`  | Also writes `product/index.yaml` + first capability `frame.yaml`                                 |
| `/map`       | Writes per-capability `map.yaml`, then expands stubs into full stories in `product-context.yaml` |
| `/calibrate` | `scope-direction` dimension covers release line cutting (replaces proposed `/slice`)             |
| `/dispatch`  | No change — reads `product-context.yaml`                                                         |
| `/build`     | No change — reads OpenSpec change                                                                |
| `/wrap`      | No change — updates `product-context.yaml`                                                       |
| `/reflect`   | Updates capability maps (product learnings) AND `product-context.yaml` (story state)             |

### 1.7 Migration

- v1 continues working. Capability maps are additive — you can adopt them for new capabilities without migrating existing stories.
- For existing products: extract activities and story stubs from `product-context.yaml` into capability maps. The full stories remain in `product-context.yaml`.

---

## 2. Technical Spec as Agent Specification

### 2.1 The Insight

The original v2 proposal introduced a "Machine Graph" — a compiled YAML file optimized for agent consumption, produced by a `/compile` command. After analysis, this concept is better served by the **technical specification** pattern already in AEP.

OpenAI's Symphony SPEC.md demonstrated that a well-written specification document is precise enough for any coding agent to implement without clarifying questions. The tech spec template (`skills/product-context/templates/technical-spec.md`) already captures this pattern with: service identity, typed entities, state machines, protocol specs with JSON transcripts, enumerated failure classes, and agent-friendly redundancy.

### 2.2 Artifact Location

The technical spec path is set in `architecture.technical_spec` (e.g., `docs/technical-spec.md`). For multi-capability products, each capability may have its own spec (e.g., `docs/<capability>-technical-spec.md`). `/dispatch` includes the relevant spec in agent context packages alongside story-specific OpenSpec artifacts.

### 2.3 When to Produce a Technical Spec

During `/map`, if the System Map reveals:

- 3+ interface contracts requiring multi-step protocol sequences
- 2+ distinct state machines
- Explicit failure classes with different recovery behaviors
- Trust boundaries crossing module lines

This is already documented in the tech spec template. The improvement: make `/calibrate` dimensions (`api-surface`, `data-model`) the pathway for refining the spec with human alignment before agents build from it.

### 2.4 Flow

```
/envision → product-context.yaml (product narrative)
     ↓
/map → capability maps (if multi-journey) + product-context.yaml (stories)
     ↓ (if complex system detected)
/map triggers → technical-spec.md (Symphony-style agent specification)
     ↓
/calibrate api-surface, data-model → refine spec with human alignment
     ↓
/dispatch → assembles context from product-context.yaml + tech spec
```

Simple products skip the tech spec entirely. Complex products get a precise, agent-readable specification — without new commands or new YAML formats.

---

## 3. Enhanced Dispatch Formula

### 3.1 Current Formula

```
dispatch_score = (critical_path_urgency + business_value + unblock_potential) / complexity_cost
```

### 3.2 Proposed Enhancement

```
dispatch_score = (business_value + unblock_potential + critical_path_urgency + reuse_leverage) / (complexity_cost + ambiguity_penalty + interface_risk)
```

**New numerator term:**

- **`reuse_leverage`** (0-10): Stories producing shared enablers (auth middleware, base components) score higher. `min(10, count_of_modules_depending_on_output * 3)`.

**New denominator terms:**

- **`ambiguity_penalty`** (0-5): Fewer than 3 acceptance criteria (+2), missing interface obligations (+1), unresolved open questions (+1), no `files_affected` (+1).
- **`interface_risk`** (0-3): +1 per interface contract touched, max 3.

### 3.3 Readiness Score

Per-story spec completeness score (0.0-1.0), computed in `/dispatch` (Step 3):

```
readiness_score = (
  min(3, acceptance_criteria_count)
  + (interface_obligations_defined ? 2 : 0)
  + (files_affected_identified ? 1 : 0)
  + (verification_defined ? 2 : 0)
  + (no_relevant_open_questions ? 2 : 0)
) / 10
```

Routing: `< 0.5` → `/design`; `>= 0.7` → `/launch`; between → user decision.

### 3.4 Story-to-Change Mapping

Three modes beyond the current 1:1 assumption:

| Mode             | When                                              | Behavior                                               |
| ---------------- | ------------------------------------------------- | ------------------------------------------------------ |
| `single_change`  | Default. Vertical slice, independently mergeable. | One story = one OpenSpec change                        |
| `grouped_change` | Tightly coupled stories, atomically verified.     | Group dispatched as one unit, single PR, max 3 stories |
| `shared_enabler` | Infrastructure consumed by multiple stories.      | Prioritized via `reuse_leverage`                       |

---

## 4. Outcome Contracts

### 4.1 Why

Jeff Patton emphasizes that layers should be anchored in outcomes, not feature completeness. v1 has `layer.verification` (tests pass) but nothing about whether the layer matters to users.

### 4.2 Schema Addition

Add to `product-context.yaml` layer definitions:

```yaml
layers:
  - layer: 0
    name: "Walking Skeleton"
    user_can: "Complete crudest end-to-end journey"
    verification: "E2E test definition"
    # NEW: outcome contract
    outcome_contract:
      hypothesis: "If users can complete basic flow, architecture is validated"
      success_metric:
        type: task_completion_rate
        target: ">= 60%"
      decision_rule:
        keep_if: "metric >= target"
        otherwise: "reflect_and_reslice"
```

### 4.3 Flow

1. `/envision` defines program-level success criteria
2. `/map` defines per-layer outcome hypotheses
3. `/reflect` evaluates outcomes after layer completion — if `keep_if` met → advance; if `otherwise` → re-slice

Outcome evaluation is not test automation. It may require user testing, analytics, or qualitative assessment. `/reflect` prompts the human to evaluate against the contract.

---

## 5. `/design` Reclassification

`/design` currently lives in Execution Plane (`skills/agentic-development-workflow/design/SKILL.md`) but runs interactively on the main session, refines specifications (not code), and involves human judgment. It's more like `/dispatch` than `/build`.

**Proposed:** Move to `skills/product-context/design/SKILL.md` (Control Plane).

```
CONTROL PLANE: /envision → /map → /validate → /dispatch → /design
EXECUTION PLANE: /launch → /build → /wrap
```

**Priority:** Low. This is a file move + docs update. The skill works the same regardless. **Revisit trigger:** If `auto_design: true` becomes common, `/design` effectively belongs in Execution Plane.

---

## 6. VCS Abstraction (resolved 2026-04 — migrated to pure git)

The original v2 proposal here was to introduce a backend-agnostic VCS abstraction (jj-backend.md + git-backend.md), keeping jj as the recommended path while making git a compatible fallback.

**Decision:** Superseded by a single-shot migration to pure git + git worktree. AEP no longer uses or supports jj. The migration was driven by repeated agent friction in Claude Code and Codex sessions — see [docs/decisions/migrate-from-jj-to-git.md](decisions/migrate-from-jj-to-git.md) for the full rationale, what we lost, and the workaround for each lost feature.

| Former jj Operation       | Replacement                                                                            |
| ------------------------- | -------------------------------------------------------------------------------------- |
| `jj workspace add`        | `git worktree add -b feat/<name>`                                                      |
| `jj edit <change>`        | (removed — implement linearly, one commit per task)                                    |
| `jj squash`               | (removed — squash-merge at PR-merge time)                                              |
| `jj describe`             | `git commit -m "..."`                                                                  |
| `jj git push --change @-` | `git push origin main` (control plane) or `git push -u origin feat/<name>` (workspace) |
| auto-rebase               | `git rebase origin/main` + `git push --force-with-lease`                               |
| `jj op log` / `jj undo`   | `git reflog` + `git reset` / `git restore --source`                                    |

This item is closed; do not relitigate without a new lessons-driven case.

---

## 7. Schema Bugfixes

Bugs in `product-context-schema.yaml` that create confusion between skills:

| Issue                     | Description                                                                           | Fix                                                                 |
| ------------------------- | ------------------------------------------------------------------------------------- | ------------------------------------------------------------------- |
| `business_value` missing  | Dispatch formula references it but schema only has `priority` enum                    | Add explicit numeric field (1-10), default from priority if not set |
| Waves undocumented        | `/map` computes wave assignments (per-story `slice` field) but no section groups them | Add a `waves` section grouping stories by layer + wave              |
| Autonomous routing fields | `autonomous`, `auto_design`, `skip_human_eval` used but not in schema                 | Add under `topology.routing`                                        |

**Priority:** P0 — these should just be fixed, not planned.

---

## 8. Implementation Roadmap

### Priority Order

```
Phase 1: Schema bugfixes                    ─── P0, low effort, independent
Phase 2: Outcome contracts in product-context ─── P1, low effort, independent
Phase 3: Enhanced dispatch formula           ─── P1, medium effort, independent
Phase 4: Multi-map capability structure      ─── P1, medium effort, independent
Phase 5: /design reclassification            ─── P2, low effort, independent
Phase 6: VCS abstraction                     ─── P2, medium effort, independent
```

All phases are independent — no hard dependencies between them.

### Validation Gate (for Phase 4)

Before implementing multi-map:

1. Run AEP v1 on a product with 2+ distinct user journeys. Document where the single-file structure breaks down.
2. If the pain is real, implement capability maps. If not, defer.

---

## 9. Decision Log

### D1: No new commands

**Decision:** All v2 improvements enhance existing commands. No `/slice`, `/compile`, or other new commands.

**Why:** AEP has 17 commands. Each new command is a new mental model, a new decision point ("when do I use this?"), and a new transition to remember. The original `/slice` is just `/calibrate scope-direction`. The original `/compile` is just `/map` expanding stubs into full stories.

### D2: Technical spec replaces Machine Graph YAML

**Decision:** The Machine Graph concept (precise agent-readable specification) is implemented as a Symphony-style technical specification in markdown, not as a new YAML format.

**Why:** The tech spec template already exists. It follows proven patterns (typed entities, state machines, protocol specs, failure classes). A markdown spec is readable by both humans and agents. A new YAML format would require new tooling, new validation, and new mental models for something that already works.

**Alternative rejected:** Machine Graph as compiled YAML (`machine-graph.yaml`) produced by `/compile`. This required a new command, write coordination logic, staleness detection, and "where do I edit?" policies — all complexity for a cached version of what `/dispatch` already computes on-the-fly.

### D3: Capability maps are additive, not mandatory

**Decision:** Multi-map is opt-in for complex products with 2+ user journeys. Simple products continue with `product-context.yaml` alone.

**Why:** Forcing multi-map on simple products adds overhead (extra files, extra directories) without benefit. The structure should match the product's actual complexity.

### D4: `/design` belongs in Control Plane (tentative)

**Decision:** Move when convenient, but low priority.

**Why:** Conceptually correct (it runs interactively, refines specs). But practically low impact — the skill works identically regardless of directory. Revisit if `auto_design` becomes the common case.

---

## 10. Open Design Questions

### Q1: Cross-Story Context Assembly

When Story B depends on Story A, how does B's agent get accurate context about A's actual implementation? Pre-implementation contracts may diverge from reality.

**Proposed:** `/wrap` extracts a dependency manifest from completed stories (actual exports, endpoint signatures, deviation notes). `/dispatch` includes this in downstream context packages.

### Q2: `files_affected` Accuracy

Estimated during `/map` when the project may not exist. Two-phase approach: `files_affected_planned` (from `/map`) and `files_affected_actual` (from `/wrap`, populated from merged PR diff).

### Q3: Learning Policy Expressiveness

Is the rule-based pattern system (`when: "error_class == X"`) expressive enough? Consider structured observation log with `/reflect` proposing parameter adjustments instead.
