# AEP Glossary — Ubiquitous Language

Precise definitions for every term used across AEP skills. Following [Domain-Driven Design](https://martinfowler.com/bliki/UbiquitousLanguage.html), a shared vocabulary eliminates ambiguity — when a skill says "layer" or "slice", it means exactly what this glossary defines.

Terms are grouped by domain and alphabetized within each group. **Bold terms** indicate cross-references to other glossary entries.

---

## Structural / Architectural

### Activity

A discrete step in the user's journey, representing a user-facing verb (e.g., "Upload Selfie", "Generate Avatar"). Activities form the horizontal axis of a Jeff Patton story map. Each activity has an `id`, `name`, `description`, `order` (left-to-right sequence), and `layer_introduced` (which **Layer** first enables it).

**Where it appears:** `product-context.yaml` → `product.activities`; `/envision` extracts them; `/map` maps **Stories** to them.

> "The horizontal axis of a Jeff Patton story map. Each activity is a discrete step in the user's journey, ordered left-to-right."
> — product-context-schema.yaml

See also: **Activity Backbone**, **Story**, **Layer**

### Activity Backbone

The ordered sequence of **Activities** representing the core user journey, read left-to-right. Extracted during `/envision` before defining **Layers** — backbone first, releases second. Infrastructure **Stories** that don't map to any activity leave their `activity` field as `null`.

**Where it appears:** `/envision` Phase 1; `/map` Step 2 (activity mapping).

See also: **Activity**, **Walking Skeleton**

### Context Document

The precise product definition produced by `/envision` Phase 1. Contains the problem statement, persona with JTBD (Jobs to Be Done), MVP boundary (in-scope, out-of-scope, deferred), stack constraints, **Layer** definitions, success criteria, open questions, decisions, and stress-test results. Every statement must be convertible into a verification condition.

**Where it appears:** `product-context.yaml` → `product` section.

> "Every assumption left implicit will be resolved by a downstream agent through guesswork. This phase makes every assumption explicit."
> — /envision SKILL.md

See also: **Opportunity Brief**, **product-context.yaml**

### Execution Slice (Wave)

A batch of **Stories** within a **Layer** that have no mutual dependencies and can run fully in parallel. Also called a **Wave** — the user-facing term used in autopilot reporting (e.g., "Wave 1 launched: auth-setup, db-schema, api-scaffold"). Waves are numbered sequentially — Wave 1 runs first, then Wave 2 after Wave 1 completes. `/dispatch` can batch-dispatch all ready stories in a wave at once.

**Where it appears:** `product-context.yaml` → `stories[].slice`; `/map` Step 3 computes them; `/dispatch` uses them for batch dispatch. Presented as "Wave N" in user-facing output.

See also: **Story**, **Layer**, **Dispatch**

### Interface Contract

The exact API surface between two **Modules** — endpoints, request/response data structures, error codes, and SLA expectations. These are not documentation; they are executable specifications enforced by contract tests. An amendment log tracks when boundaries need to change.

**Where it appears:** `product-context.yaml` → `architecture.interfaces`; `/map` Step 1 defines them.

> "For every module-to-module connection, define the exact API surface — endpoints, data shapes, error contracts."
> — /map SKILL.md

See also: **Module**, **System Map**

### Layer

A horizontal slice of enrichment across the **Activity Backbone**. Layer 0 is the **Walking Skeleton** — the thinnest end-to-end path. Integer layers (1, 2, ...) add capabilities. `.5` layers (0.5, 1.5, 2.5) are **Alignment Layers** — human checkpoints where the team pauses agent execution to **Calibrate** one or more **Quality Dimensions**. All stories within a layer are built before advancing to the next. Advancement is gated by a **Layer Gate**.

**Where it appears:** `product-context.yaml` → `product.layers` and `stories[].layer`; `/envision` defines them; `/map` assigns stories to them.

See also: **Walking Skeleton**, **Layer Gate**, **Release Line**, **Alignment Layer**

### Layer 0 (Walking Skeleton)

The first and most important **Layer**. A horizontal slice across the **Activity Backbone** — the thinnest story from each user activity, strung together so a user can complete the crudest possible end-to-end journey. Validates the architecture at minimum cost before going deep into any module.

**Where it appears:** `/envision` Phase 1 (defines the Layer 0 MVP contract); `/map` Step 2 (ensures every activity has at least one Layer 0 story).

> "Build a skeleton that can walk before building a perfect leg."
> — /map SKILL.md

See also: **Layer**, **Layer Gate**, **Activity Backbone**

### Layer Gate

An integration test that verifies a completed **Layer** works as a whole — testing emergent behavior at integration boundaries, not the sum of individual story tests. Layer 0's gate tests the exact MVP user journey end-to-end. Must pass before advancing to the next layer.

**Where it appears:** `product-context.yaml` → `layer_gates[]`; `/dispatch` checks gate status before advancing; `/autopilot` pauses on gate failure.

See also: **Layer**, **Walking Skeleton**

### Module

A major component with a clear responsibility boundary. Each module defines what it does, what it does NOT do (boundary definition), what state/resources it owns, what it depends on, and its technology stack. Wrong module boundaries cost more than any implementation bug.

**Where it appears:** `product-context.yaml` → `architecture.modules`; `/map` Step 1 defines them.

See also: **Interface Contract**, **System Map**

### Opportunity Brief

A deliberately short (one-page) document produced by `/envision` Phase 0 that captures the core bet: "I believe [target user] has [problem], and I can build [solution] because [advantage]." Includes counter-arguments, kill criteria, and an explicit proceed/kill/defer decision.

**Where it appears:** `product-context.yaml` → `opportunity` section.

See also: **Context Document**, **Kill Point**

### Release Line

A horizontal boundary separating **Layers** in the story map. Conceptually, cutting a release line means "everything above this line is shippable." Layer assignments are pencil marks — `/reflect` can promote or demote stories between layers (re-slicing) without requiring an architecture review, unless **Interface Contracts** changed.

**Where it appears:** `docs/release-line-adjustments.md`; `/reflect` Step 2.5 (re-slice the map).

See also: **Layer**, **Feedback Classification**

### Story

The atomic unit of work in AEP. A self-contained spec that a single agent can implement without asking questions. Each story includes: what changes (observable behavior), acceptance criteria (automatable as tests), layer, module, activity, slice, dependencies, interface obligations, files affected, and verification strategy. Stories follow a state machine: `pending → ready → in_progress → in_review → completed` (or `blocked` / `failed` / `deferred`).

**Where it appears:** `product-context.yaml` → `stories[]`; `/map` creates them; `/dispatch` scores and assigns them; `/build` implements them.

See also: **Execution Slice (Wave)**, **Dispatch Score**, **Story Graph**

### Story Graph

A directed acyclic graph (DAG) of **Stories** organized by **Layer**, showing dependencies and parallelism opportunities. Produced by `/map` Step 3. The graph determines the **Critical Path**, **Execution Slices**, and **Dispatch Scores**.

**Where it appears:** `/map` Step 3 produces it; `/dispatch` Step 3 computes scores from it.

See also: **Story**, **Execution Slice**, **Critical Path**

### System Map

The architectural blueprint produced by `/map` Step 1. Contains **Modules** with responsibility boundaries, **Interface Contracts**, data flows for each user journey, third-party integration points with failure modes, and architecture decision records (ADRs). Requires human review and approval before proceeding to story decomposition.

**Where it appears:** `product-context.yaml` → `architecture` section.

See also: **Module**, **Interface Contract**

---

## Execution / Workflow

### Control Plane

The "thinking" half of the AEP workflow — where humans and AI collaborate on high-leverage decisions: goals, decomposition, architecture, priorities, and feedback. Produces `product-context.yaml` as its primary artifact. Skills: `/envision`, `/map`, `/validate`, `/dispatch`, `/reflect`.

**Where it appears:** README mental model; `/dispatch` bridges Control Plane to **Execution Plane**.

> "You + AI collaborate on high-leverage decisions: goals, decomposition, architecture, priorities, feedback."
> — README.md

See also: **Execution Plane**, **Two-Session Model**

### Critical Path

The longest dependency chain through the **Story Graph** within the active **Layer**. A critical-path story delayed by one hour delays the entire layer by one hour. Stories on the critical path receive maximum urgency in the **Dispatch Score** formula.

**Where it appears:** `/dispatch` Step 3 (critical_path_urgency scoring dimension).

See also: **Dispatch Score**, **Story Graph**

### Dispatch

The act of selecting a **Story** from the ready queue, locking it (`status → in_progress`), creating an **OpenSpec Change** with a **Context Package**, and handing off to `/design` or `/launch`. Follows a 7-step protocol: sync signals → cascade states → score stories → present queue → dispatch lock → create OpenSpec change → hand off.

**Where it appears:** `/dispatch` skill; `/autopilot` Step 6.

See also: **Dispatch Score**, **Dispatch Lock**

### Dispatch Lock

The mechanism that prevents double-dispatch. When a story is selected, its status is updated to `in_progress` and the YAML is committed to git _before_ the workspace is created. A second `/dispatch` run reads `in_progress` from the committed YAML and skips the story.

**Where it appears:** `/dispatch` Step 5.

### Dispatch Score

The priority formula used to rank **Stories** in the ready queue:

```
dispatch_score = (critical_path_urgency + business_value + unblock_potential) / complexity_cost
```

- **Critical path urgency** (0–10): 10 if on critical path, otherwise decreasing with slack
- **Business value** (1–10): critical=10, high=7, medium=4, low=1
- **Unblock potential** (0–10): min(10, count_of_dependent_stories × 2)
- **Complexity cost** (divisor): S=1, M=2, L=4

**Where it appears:** `/dispatch` Step 3; `/autopilot` Step 6; `product-context.yaml` → `stories[].dispatch_score`.

See also: **Critical Path**, **Dispatch**

### Execution Plane

The "doing" half of the AEP workflow — where agents receive precise specs and build autonomously. Agents work in isolation, communicate through structured artifacts and **Signal Files**, and produce merged PRs. Skills: `/design`, `/launch`, `/build`, `/wrap`.

**Where it appears:** README mental model.

See also: **Control Plane**, **Two-Session Model**

### Main Session

The interactive Claude Code session where the human works. Runs on the main branch. Manages the **Control Plane** — dispatching stories, designing features, launching workspaces, wrapping completed work, and reflecting on feedback. Never reads workspace source code directly; observes workspace progress through **Signal Files** only.

**Where it appears:** Every skill specifies its session context; `/dispatch`, `/design`, `/launch`, `/wrap`, `/reflect` run here.

See also: **Workspace Session**, **Two-Session Model**, **Orchestrator Boundaries**

### Phase

A numbered stage in the feature lifecycle. Phases 1–3 (explore, propose, review) happen on main via `/design`. Phase 0 (workspace init) and Phases 4–12 happen in the **Workspace Session** via `/build`:

| Phase | Name                          | Session          |
| ----- | ----------------------------- | ---------------- |
| 0     | Initialize tracking           | Workspace        |
| 1–3   | Explore, propose, review      | Main (`/design`) |
| 4     | Implement via jj change stack | Workspace        |
| 5     | Code review (gen/eval loop)   | Workspace        |
| 6     | Browser testing (dogfood)     | Workspace        |
| 7     | E2E test script generation    | Workspace        |
| 8     | Review results                | Workspace        |
| 9     | Cleanup & publish             | Workspace        |
| 10    | Create PR                     | Workspace        |
| 11    | PR review feedback loop       | Workspace        |
| 11.5  | Human evaluation (optional)   | Workspace        |
| 12    | Pre-merge checks & merge      | Workspace        |
| 13    | Archive & cleanup             | Main (`/wrap`)   |

**Where it appears:** `/design` (1–3); `/build` (0, 4–12); `/wrap` (13).

See also: **Two-Session Model**

### Signal Files

File-based communication between the **Main Session** and **Workspace Sessions**. Located at `.feature-workspaces/<name>/.dev-workflow/signals/`. The concurrency protocol: generators write signals, the main session reads them.

| File                    | Writer       | Reader       | Purpose                                         |
| ----------------------- | ------------ | ------------ | ----------------------------------------------- |
| `status.json`           | Generator    | Main session | Phase, completion %, story status, PR URL, cost |
| `feedback.md`           | Main session | Generator    | Mid-flight instructions                         |
| `eval-request.md`       | Generator    | Evaluator    | Trigger evaluation round                        |
| `eval-response-<N>.md`  | Evaluator    | Generator    | Scoring dimensions, PASS/FAIL                   |
| `ready-for-review.flag` | Generator    | Main session | Signal human review ready                       |

**Where it appears:** `/build` Phase 0 (creates them); `/dispatch` Step 1 (syncs them); `/autopilot` Step 2 (syncs them); `/wrap` (reads them).

See also: **Concurrency Protocol**, **Generator**, **Evaluator**

### Tick

One iteration of the **Autopilot** orchestration loop (default interval: 5 minutes). Each tick is idempotent and targets < 60 seconds of execution. Follows a 7-step protocol: read state → sync signals → wrap completed → guide completion → detect stuck → dispatch new → write state.

**Where it appears:** `/autopilot` skill; `references/tick-protocol.md`.

See also: **Autopilot**, **Tick Lock**

### Tick Lock

A timestamp (`tick_in_progress`) written to `autopilot-state.json` at the start of each **Tick** and cleared at the end. If a new tick fires while a previous one is still running (timestamp < 4 minutes old), it skips. Prevents overlapping ticks.

**Where it appears:** `references/tick-protocol.md` Steps 1 and 7.

See also: **Tick**

### Two-Session Model

The architectural separation between the **Main Session** (human + AI on main branch) and the **Workspace Session** (autonomous agent in an isolated jj workspace). Design happens on main (human judgment). Implementation happens in workspace (autonomous execution). Separation eliminates context reset between planning and building.

**Where it appears:** README; `/launch` creates the workspace session; `/build` runs in it; `/wrap` cleans it up.

See also: **Main Session**, **Workspace Session**, **Control Plane**, **Execution Plane**

### WIP Limit

The maximum number of parallel **Workspace Sessions** (default: 5, from `topology.routing.concurrency_limit`). Based on Little's Law: `Lead Time = WIP / Throughput`. Exceeding WIP limits creates traffic jams, not speed — the bottleneck is usually human PR review, not agent velocity.

**Where it appears:** `/dispatch` Step 5; `/autopilot` Step 6.

See also: **Dispatch**, **Workspace Session**

### Workspace Session

An autonomous Claude Code session running in a tmux window inside an isolated **jj workspace**. Created by `/launch`, executes `/build` phases 0–12 without user interaction. Communicates progress via **Signal Files**. One workspace per **Story**.

**Where it appears:** `.feature-workspaces/<name>/`; `/launch` creates it; `/build` runs in it; `/wrap` removes it.

See also: **Main Session**, **Two-Session Model**, **Signal Files**

---

## Agent / Pattern

### Auto-Rebase

A jj feature where editing an earlier change in the stack automatically rebases all dependent changes. Enables the **Skeleton-First Pattern** — create empty changes, then fill them in any order. Dependent changes stay consistent without manual rebasing.

**Where it appears:** `/build` Phase 4; `/jj-ref`.

See also: **Skeleton-First Pattern**, **jj**

### Autopilot

The fully autonomous orchestration mode. A tick-based state machine that runs the dispatch → launch → monitor → review → wrap → dispatch cycle on a configurable interval (default 5 minutes). Invoked via `/autopilot`. Pauses only for **Design Escalation** or **Layer Gate** failures.

**Where it appears:** `/autopilot` skill; `references/tick-protocol.md`.

See also: **Tick**, **Orchestrator Boundaries**, **Design Escalation**

### Context Assembly

The process of constructing the exact context each agent needs for a **Story**. Consists of three parts: **Stable Prefix** (shared product/architecture context, ~10K tokens, cacheable), story-specific payload (full spec + module + adjacent interfaces + dependency outputs, ~20K tokens), and retrieval instructions (~500 tokens). Measured against the role's token budget to prevent overflow.

**Where it appears:** `/dispatch` Step 6; `product-context.yaml` → `topology.roles[].context_composition`.

See also: **Stable Prefix**, **Context Package**

### Context Package

The pre-assembled directory of context files created by `/dispatch` for each dispatched **Story**. Located at `openspec/changes/<story-id>/.context/` with three files: `stable-prefix.md`, `dependencies.md`, and `retrieval.md`.

**Where it appears:** `/dispatch` Step 6; `/build` Phase 0 reads it.

See also: **Context Assembly**, **Stable Prefix**, **OpenSpec Change**

### Evaluator

The separate agent that reviews a **Generator**'s work. Calibrated toward skepticism because agents praise their own work. Scores across dimensions (e.g., Completeness, Correctness, UX Quality, Security, Code Quality) on a 1–5 scale with hard failure thresholds. Spawned at **Phase** 5 via tmux split, not at launch.

**Where it appears:** `/build` Phase 5; `/gen-eval` skill; `/launch` (brainstorms criteria).

> "When asked to evaluate work they've produced, agents tend to respond by confidently praising the work — even when, to a human observer, the quality is obviously mediocre."
> — Anthropic, "Harness Design for Long-Running Application Development"

See also: **Generator**, **Generator/Evaluator Separation**

### Feature Verification

A structured JSON file (`.dev-workflow/feature-verification.json`) listing all verification checks for a feature — each acceptance criterion mapped to a testable condition. Only the **Evaluator** or a human modifies this file; the **Generator** cannot mark its own work as verified.

**Where it appears:** `/build` Phase 0 (created); Phase 5 (**Evaluator** checks against it).

See also: **Evaluator**, **Sprint Contract**

### Generator

The agent that creates artifacts — implements code, writes designs, produces proposals. In the **Two-Session Model**, the workspace agent acts as generator. Cannot honestly evaluate its own work, which is why the **Evaluator** exists as a separate agent.

**Where it appears:** `/build` (workspace agent); `/gen-eval` skill.

See also: **Evaluator**, **Generator/Evaluator Separation**

### Generator/Evaluator Separation

The single most durable pattern from Anthropic's research on agent harnesses. The agent that creates work (generator) and the agent that evaluates it (evaluator) must be separate. Self-evaluation produces inflated scores and rationalized problems. The cost of a second agent is trivial compared to shipping broken work.

**Where it appears:** `/gen-eval` skill (defines the pattern); `/build` Phase 5 (applies it); `/validate` (uses it).

See also: **Generator**, **Evaluator**

### Harness

The infrastructure surrounding an agent to help it succeed — tracking files, signal protocols, recovery scripts, evaluation criteria, verification checklists. Each harness component exists because of a specific failure mode observed in Anthropic's research. Components should be stress-tested as models improve and removed when no longer needed.

**Where it appears:** `/build` Phase 0 (sets up the harness); `.dev-workflow/` directory contains harness artifacts.

| Problem                         | Harness artifact          | Solution                        |
| ------------------------------- | ------------------------- | ------------------------------- |
| Agents self-evaluate leniently  | evaluator-criteria.md     | Separate evaluator              |
| Agent builds wrong thing        | contracts.md              | Sprint contracts                |
| Agent doesn't test thoroughly   | feature-verification.json | Verification checklist          |
| Agent can't resume after reset  | init.sh                   | Session recovery script         |
| No visibility into progress     | status.json               | Signal files                    |
| No mid-flight course correction | feedback.md               | Main session sends instructions |

See also: **Generator/Evaluator Separation**, **Sprint Contract**, **Feature Verification**

### Orchestrator

The main session agent (or **Autopilot**) that coordinates work across **Workspace Sessions**. Decides what to build, dispatches stories, monitors progress, and triggers wraps. Operates strictly within **Orchestrator Boundaries**.

**Where it appears:** `/dispatch`; `/autopilot`; `/wrap`.

See also: **Orchestrator Boundaries**, **Autopilot**

### Orchestrator Boundaries

The strict separation rules the **Orchestrator** must follow:

1. **Never read workspace source code** — only read **Signal Files**
2. **Never spawn Agent tools for code review** — use `tmux send-keys` instead
3. **Never call `gh pr merge`** — workspace agents own **Phase** 12
4. **Never write eval-response files** — that is the **Evaluator**'s job

These boundaries prevent the orchestrator from becoming an executor and eliminate cross-cutting concerns.

**Where it appears:** `/autopilot` SKILL.md (STOP section); `references/tick-protocol.md`.

See also: **Orchestrator**, **Two-Session Model**

### Skeleton-First Pattern

Create empty jj changes with descriptions for each task _before_ implementing any of them. Then implement by editing changes in any order — **Auto-Rebase** keeps dependents consistent. Separates planning (change structure) from execution (filling in code). A natural fit for agent workflows where rough code is generated first, then cleaned up with `split`/`squash`.

**Where it appears:** `/build` Phase 0 (creates the change stack); `/jj-ref` (documents the pattern).

See also: **Auto-Rebase**, **jj**

### Sprint Contract

A file (`.dev-workflow/contracts.md`) that explicitly states what success looks like for the current feature — the definition of done, key behaviors, integration requirements. Exists because without explicit success criteria, agents build the wrong thing.

**Where it appears:** `/build` Phase 0 (generated from OpenSpec artifacts).

See also: **Feature Verification**, **Harness**

### Stable Prefix

The shared portion of **Context Assembly** (~10K tokens) that is identical across all agents working in the same **Layer**. Contains: problem statement, constraints, active layer MVP contract, and architecture overview. Cacheable — when dispatching multiple stories, write it once.

**Where it appears:** `/dispatch` Step 6 → `openspec/changes/<id>/.context/stable-prefix.md`.

See also: **Context Assembly**, **Context Package**

---

## Product / Planning

### Alignment Layer

A `.5` **Layer** (0.5, 1.5, 2.5) where the team pauses agent execution to recalibrate human intent across one or more **Quality Dimensions**. Formerly called "UI polish layers" — the concept was generalized because the gap between "works correctly" and "is what we actually want" exists across many dimensions, not just visual design.

**Where it appears:** `product-context.yaml` → `stories[].layer` (fractional values); `/map` Step 3 plans them; `/reflect` Step 2 identifies the need; `/calibrate` executes them.

See also: **Layer**, **Calibration**, **Quality Dimension**

### Calibration

The act of pausing agent execution to let a human inspect what was built and capture what "right" actually means in a format agents can consume. Agents optimize for correctness against spec, but specs are lossy compressions of human intent. Calibration corrects the loss. Each calibration targets a specific **Quality Dimension** and follows a two-phase pattern: generate brief → capture decisions.

Calibrations split into two classes:

| Class     | Dimensions                                                    | Method                             | Artifact                                    |
| --------- | ------------------------------------------------------------- | ---------------------------------- | ------------------------------------------- |
| **Heavy** | visual-design, ux-flow, copy-tone                             | External tools or deep exploration | Standalone YAML in `calibration/` directory |
| **Light** | api-surface, data-model, scope-direction, performance-quality | Conversation + code review         | Inline updates to `product-context.yaml`    |

**Where it appears:** `/calibrate` skill (executes); `/envision` (declares quality dimensions); `/reflect` (identifies needs); `product-context.yaml` → `calibration.history`.

See also: **Quality Dimension**, **Alignment Layer**, **Calibration Artifact**

### Calibration Artifact

The machine-readable output of a **Calibration** capture. Heavy calibrations produce standalone YAML files in `calibration/` (e.g., `calibration/visual-design.yaml`, `calibration/ux-flow.yaml`, `calibration/copy-tone.yaml`). Light calibrations update existing sections of `product-context.yaml` directly (e.g., `architecture.interfaces`, `architecture.domain_model`, `product.success_criteria`). Agents query these artifacts for specific values (`palette.dark.primary`, `voice.personality`, `components.border_radius`).

**Where it appears:** `calibration/` directory (heavy types); `product-context.yaml` sections (light types); `/dispatch` Step 6 includes them in **Context Packages** for calibrated stories.

See also: **Calibration**, **Context Assembly**

### Changelog

A semantic history of how `product-context.yaml` evolved. Unlike git diffs (which show _what_ changed), the changelog records _why_ things changed. Types include: initial, envision_update, map_update, dispatch, reflect, build, wrap, layer_gate_pass/fail, architecture_review. Appended by every skill that modifies the YAML.

**Where it appears:** `product-context.yaml` → `changelog[]`.

### Concurrency Protocol

The rule that only the **Main Session** writes to `product-context.yaml`. **Workspace Sessions** report status through **Signal Files** only. The main session (via `/wrap`, `/dispatch`, `/reflect`) reads signals and updates the YAML. This prevents git merge conflicts from concurrent writers.

**Where it appears:** `product-context.yaml` header comment; `/build` (workspace agent respects it); `/wrap` (reads signals, writes YAML).

See also: **Signal Files**, **Main Session**, **Workspace Session**

### Cost Tracking

Accumulated execution cost data in `product-context.yaml` → `cost` section. Tracks total USD, cost by layer, by module, and by story. Alerts flag anomalies: cost overruns, retry concentration, and timeout patterns. Reviewed by `/reflect` for optimization opportunities.

**Where it appears:** `product-context.yaml` → `cost`; `/build` reports cost via signals; `/wrap` syncs to YAML; `/reflect` reviews.

### Design Escalation

A condition that causes **Autopilot** to pause and request human input via `/design`. Triggers:

- Complexity L with fewer than 3 acceptance criteria
- UI-heavy activity with complexity M or L
- `attempt_count >= 2` (repeated failures)
- Fewer than 3 acceptance criteria with missing interface details

**Where it appears:** `/autopilot` Step 6; `/dispatch` Step 7 (routes ambiguous stories to `/design`).

See also: **Autopilot**, **Dispatch**

### Feedback Classification

The categories used by `/reflect` to route real-world observations back to the right phase:

| Type                         | Definition                                | Action                                                                                           |
| ---------------------------- | ----------------------------------------- | ------------------------------------------------------------------------------------------------ |
| **Bug**                      | Specified behavior that doesn't work      | New high-priority fix story in current layer                                                     |
| **Refinement**               | Working behavior that needs improvement   | New story in next layer (or promote earlier)                                                     |
| **Refinement → Calibration** | Works as specified but doesn't feel right | `/calibrate <dimension>` — heavy types create `.5` layer stories, light types update YAML inline |
| **Discovery**                | New requirement or invalidated assumption | Update product/architecture via `/envision` or `/map`                                            |
| **Opportunity Shift**        | Fundamental bet is wrong                  | Back to `/envision` Phase 0                                                                      |
| **Process**                  | Workflow/tooling observation, not product | Document in `lessons-learned/`, propose skill amendments                                         |

**Where it appears:** `/reflect` Step 2.

See also: **Release Line**, **Calibration**, **Quality Dimension**

### Kill Point

The explicit decision point at the end of `/envision` Phase 0 where the **Opportunity Brief** is challenged. If the opportunity does not survive an honest five-minute challenge, it should not consume further resources. Options: proceed, kill, or defer with a revisit condition.

**Where it appears:** `/envision` Phase 0.

See also: **Opportunity Brief**

### OpenSpec

A spec-driven development CLI tool that manages feature specifications as structured artifact bundles. Provides commands: `explore` (research), `propose` (generate specs), `apply` (implement), `archive` (preserve). AEP uses OpenSpec to manage the lifecycle of each dispatched **Story**.

**Where it appears:** `/scaffold` Phase 5 (initializes OpenSpec); `/design` (runs explore/propose); `/wrap` (runs archive).

See also: **OpenSpec Change**

### OpenSpec Change

A specification artifact bundle created by `/dispatch` for each dispatched **Story**. Located at `openspec/changes/<story-id>/` containing:

```
proposal.md          — story description + business value
design.md            — module definition + interface contracts
specs/<module>.md    — acceptance criteria + interface obligations
tasks.md             — implementation tasks
.context/            — pre-assembled Context Package
```

**Where it appears:** `/dispatch` Step 6 (creates); `/design` (refines); `/build` Phase 0 (reads); `/wrap` (archives).

See also: **OpenSpec**, **Context Package**

### Quality Dimension

An aspect of a product where human judgment is needed that agents cannot provide — where "correct" and "right" diverge. Declared during `/envision` Phase 1. Seven standard dimensions:

| Dimension               | The gap agents can't judge                                        |
| ----------------------- | ----------------------------------------------------------------- |
| **visual-design**       | Brand identity, color, typography, layout                         |
| **ux-flow**             | User journey, information architecture, page transitions          |
| **api-surface**         | Endpoint naming, grouping, error contracts                        |
| **data-model**          | Entity naming, field semantics, domain language                   |
| **scope-direction**     | Whether what was built matches what the PM actually wanted        |
| **copy-tone**           | Brand voice, error messages, empty states, terminology            |
| **performance-quality** | Latency thresholds, retry behavior, caching, degradation strategy |

**Where it appears:** `product-context.yaml` → `product.quality_dimensions`; `/envision` Phase 1 declares them; `/reflect` checks them; `/calibrate` addresses them.

See also: **Calibration**, **Alignment Layer**

### product-context.yaml

The single source of truth for the entire product. A YAML file committed to git that evolves incrementally across skills:

| Section        | Populated by                                   |
| -------------- | ---------------------------------------------- |
| `opportunity`  | `/envision` Phase 0                            |
| `product`      | `/envision` Phase 1 (incl. quality_dimensions) |
| `calibration`  | `/envision` (plan) + `/calibrate` (history)    |
| `architecture` | `/map` Step 1 + `/calibrate` (light types)     |
| `stories`      | `/map` Steps 2–3                               |
| `topology`     | `/map` Step 4                                  |
| `layer_gates`  | `/map` Step 3                                  |
| `cost`         | `/build` → `/wrap`                             |
| `changelog`    | All skills that modify the file                |

Version format: major (opportunity shift), minor (architecture/map change), patch (dispatch/build/wrap). Only the **Main Session** writes to this file (**Concurrency Protocol**).

**Where it appears:** Root of the project directory; referenced by every AEP skill.

See also: **Concurrency Protocol**, **Changelog**

### Upstream Candidate

A lesson or observation from a downstream project build that may warrant changes to AEP skills, documentation, or patterns. Marked during `/workflow-feedback` capture. Only upstream candidates are reviewed during the AEP-side pull process. Items classified as `process` or `tech-stack` are almost always upstream candidates; `discovery` items are upstream only if they reveal a pattern applicable beyond one project.

**Where it appears:** `.dev-workflow/feedback.md` (downstream); `/workflow-feedback` review mode (AEP).

See also: **Workflow Feedback**

### Workflow Feedback

The standardized process for capturing and routing observations about AEP workflow effectiveness. Downstream projects write feedback during or after builds using `/workflow-feedback` capture; the AEP repo reviews and pulls improvements using `/workflow-feedback` review. Distinct from product feedback (handled by `/reflect`): workflow feedback concerns the development process, not the product being built.

**Where it appears:** `/workflow-feedback` skill; `.dev-workflow/feedback.md` (downstream output); `docs/lessons/` and `docs/tech-stack/` (AEP destinations).

See also: **Upstream Candidate**

---

## Tools

### Better-T-Stack

A full-stack TypeScript monorepo scaffold engine used by `/scaffold`. Default stack: Hono + TanStack Router + Drizzle + SQLite + Better Auth + tRPC + Turborepo + Biome + Bun. Supports customization of frontend, backend, database, ORM, auth, API layer, runtime, package manager, and addons. Has built-in template presets (t3, pern, mern, uniwind).

**Where it appears:** `/scaffold` Phase 3.

### Bookmark (jj)

A publishing-time branch marker in **jj**. Unlike git branches that move with commits, jj bookmarks are explicitly created when changes are ready to push. Created at **Phase** 9 (cleanup & publish), pushed to remote for PR creation.

**Where it appears:** `/build` Phase 9; `/jj-ref`.

See also: **jj**

### cmux

A Claude Code tab multiplexer that creates visual tabs for parallel **Workspace Sessions**. Each workspace gets its own cmux tab for terminal display, making it easy to monitor multiple agents working in parallel.

**Where it appears:** `/launch` (creates cmux tab for new workspace).

See also: **tmux**, **Workspace Session**

### Colocated Mode

Running **jj** and git in the same repository (`.jj/` + `.git/`), managed by jj. Use `jj` for all local work and `jj git` subcommands for remote operations. Never use raw `git commit` or `git add` in colocated mode. Initialized via `jj git init --colocate`.

**Where it appears:** `/onboard` Phase 2.5; `/jj-ref`.

See also: **jj**

### jj (Jujutsu)

A change-oriented version control system used for all local work in AEP. Key mental model shifts from git: the working copy IS a change, there's no staging area, changes are mutable until published, and you clean up history after generation (not during). Provides workspaces (isolated working copies sharing one object store) with no extra disk space — essential for parallel agent execution.

**Where it appears:** Every execution-plane skill; `/jj-ref` (full reference); `/onboard` (verifies installation).

See also: **Colocated Mode**, **Bookmark**, **Auto-Rebase**, **Skeleton-First Pattern**

### OpenSpec CLI

The command-line tool for spec-driven development. Commands: `openspec init` (initialize), `openspec explore` (research phase), `openspec propose` (generate specs), `openspec apply` (implementation), `openspec archive` (preserve completed work). Installed as a project dependency and aliased via `/scaffold`.

**Where it appears:** `/scaffold` Phase 5; `/design` (explore/propose); `/wrap` (archive).

See also: **OpenSpec**, **OpenSpec Change**

### tmux

A terminal multiplexer that hosts **Workspace Session** Claude Code instances. Each workspace runs in a tmux session. The **Orchestrator** communicates with workspace agents exclusively via `tmux send-keys` — this is the enforcement mechanism for **Orchestrator Boundaries**.

**Where it appears:** `/launch` (creates tmux session); `/autopilot` (sends commands via tmux); `/build` Phase 5 (spawns evaluator in tmux split).

See also: **cmux**, **Orchestrator Boundaries**, **Workspace Session**
