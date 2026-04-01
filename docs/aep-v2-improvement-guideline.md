# AEP v2 Improvement Guideline

**From Single-File Context to Multi-Map Product Architecture**

> Synthesized from three independent deep analyses of AEP's design, User Story Mapping (Jeff Patton), OpenSpec integration, and the SDD tool ecosystem. This document serves as the authoritative improvement roadmap for evolving AEP from v1 to v2.

---

## 1. Executive Summary

### Where AEP Sits in the SDD Ecosystem

The Spec-Driven Development (SDD) ecosystem in 2026 includes several frameworks — OpenSpec (change-level artifact flow), BMAD (agent-simulated agile teams), Spec Kit (PRD-to-implementation generation), GSD (goal-driven specs), and PRP (prompt-ready proposals). Each solves a piece of the puzzle: how to give AI agents enough structure to produce useful code.

**AEP answers a question the others avoid: what should the agent build, and why?**

OpenSpec handles "how to isolate and hand off a change." BMAD handles "how to simulate a team." But these frameworks assume you already know what to build, for whom, and in what order. AEP encodes User Story Mapping's cognitive structure — walking skeleton, layered delivery, left-to-right user narrative — directly into `product-context.yaml`, making **product decisions themselves machine-parseable artifacts** rather than fuzzy concepts in human heads.

### The Core Thesis

The bottleneck in agentic software engineering is not code generation. It is:

1. **Specification quality** — agents build the wrong thing when specs are ambiguous
2. **Decomposition quality** — wrong story boundaries create integration failures
3. **Handoff quality** — agents lose context at every transition point
4. **Parallel consistency** — concurrent agents must not conflict
5. **Learning velocity** — the system must improve from its own execution history

AEP v1 addresses all five through its control plane / execution plane architecture, single-writer concurrency, generator/evaluator separation, and dispatch scoring. These innovations are real and should be preserved.

### What v2 Changes

v2 separates **human sensemaking** from **machine execution** into distinct document layers, while preserving AEP's closed feedback loop. The core change: replace the single monolithic `product-context.yaml` with a **multi-map document architecture** that is the universal default for all products — not a scaling escape hatch for complex ones.

```
Human Maps (product narrative)  →  /compile  →  Machine Graph (agent control plane)  →  /dispatch  →  OpenSpec Change Packets (execution)
```

This separation ensures:

- Humans keep the story (Jeff Patton's territory) without drowning in dispatch state
- Agents get a clean, compiled work graph without parsing irrelevant product narrative
- Products grow naturally from one capability map to many, with no restructuring needed

---

## 2. What v1 Gets Right

These are AEP's genuine innovations. v2 must preserve all of them.

### 2.1 Machine-Parseable Product Decisions

`product-context.yaml` (`skills/product-context/templates/product-context-schema.yaml`) transforms product planning from conversation artifacts into structured, versionable, machine-readable data. No other SDD framework does this at the product level — they all start at the change/task level.

### 2.2 Control Plane / Execution Plane Separation

The split between decision-making skills (`/envision`, `/map`, `/validate`, `/dispatch`, `/reflect`) and execution skills (`/design`, `/launch`, `/build`, `/wrap`) correctly identifies that **deciding what to build** and **building it** are fundamentally different activities requiring different capabilities.

### 2.3 Single-Writer Concurrency Protocol

Only the main session writes to `product-context.yaml`. Workspace agents report status through `.dev-workflow/signals/status.json`. The main session reads signals on `/wrap` and `/dispatch` and updates the YAML atomically. This prevents merge conflicts from concurrent writers and is the most pragmatic multi-agent coordination pattern in the ecosystem.

### 2.4 Walking Skeleton Discipline

Layer 0 must be a thin end-to-end path across the activity backbone before any depth is added. This is directly from Jeff Patton and is correctly enforced through layer gates, activity coverage validation, and the dispatch active-layer mechanism.

### 2.5 Generator/Evaluator Separation

Agents self-evaluate leniently. The separate evaluator pattern — spawned at Phase 5 of `/build`, calibrated toward skepticism, updating `feature-verification.json` — catches real quality issues. The research basis (Anthropic's harness design findings) is sound.

### 2.6 File-Based Inter-Agent Communication

Signal files (`.dev-workflow/signals/`) implement structured, asynchronous, file-based communication between agents. This is more robust than chat-based coordination and naturally integrates with git version control.

### 2.7 Cost Tracking from Day One

The `cost` section in the schema, per-story `cost_usd` tracking, and cost alerts (`cost_exceeded`, `retry_concentration`, `timeout_pattern`) make agentic engineering accountable. Most frameworks ignore this entirely.

---

## 3. Known Issues

### 3.1 Schema Inconsistencies

These are bugs in the current schema that create confusion between skills and between the YAML template and the Zod runtime schema.

**Issue 1: `business_value` field missing from YAML template**

The dispatch scoring formula in `skills/product-context/dispatch/SKILL.md` explicitly uses `business_value` as a scoring term:

```
dispatch_score = (critical_path_urgency + business_value + unblock_potential) / complexity_cost
```

The autonomous loop prerequisites in `docs/autonomous-loop.md` require all stories to have `business_value`. However, the YAML template schema at `skills/product-context/templates/product-context-schema.yaml` line 187 only defines `priority: critical | high | medium | low` — there is no `business_value` field on stories. The dispatch skill implicitly maps priority to business_value (critical=10, high=7, medium=4, low=1), but this mapping is buried in the skill text, not in the schema.

**Fix:** Add `business_value` as an explicit numeric field (1-10) on stories in the schema template. Keep `priority` as the human-friendly enum but make `business_value` the dispatch-facing field with a clear derivation rule.

**Issue 2: `layer_gates.status` enum mismatch**

The YAML template (line 295) defines: `not_started | running | passed | failed`. The autopilot skill references `status: pending` as a prerequisite state. These are inconsistent — an autopilot checking for `pending` will never match a schema that starts at `not_started`.

**Fix:** Standardize on `not_started | running | passed | failed`. Update autopilot to check for `not_started` instead of `pending`.

**Issue 3: Story status enum divergence**

The YAML template defines an 8-state machine: `pending | ready | in_progress | in_review | completed | blocked | failed | deferred`. In practice, some code paths may produce `done` (instead of `completed`) or `review` (instead of `in_review`).

**Fix:** Canonicalize on the YAML template's 8 states. Add explicit validation that rejects non-canonical values.

**Issue 4: `execution_slices` undocumented in YAML template**

The concept of execution slices (parallel batches within a layer) is described in `/map` Step 3 but has no corresponding section in the YAML template schema.

**Fix:** Add `execution_slices` as a top-level section in the schema template, documenting the structure that `/map` produces.

**Issue 5: Autonomous routing fields undocumented**

`docs/autonomous-loop.md` requires `topology.routing.autonomous: true`, `auto_merge`, `auto_design`, `skip_human_eval`. None of these appear in the YAML template schema.

**Fix:** Add all autonomous routing fields to the template schema under `topology.routing` with documentation of their behavior.

### 3.2 Structural Tensions

**`/design` is in the wrong plane.** It currently lives in `skills/agentic-development-workflow/` (Execution Plane) but runs on the main session interactively with the user, refines specifications (not code), and produces artifacts that feed INTO execution. It is the "last mile" of the Control Plane, not the first step of the Execution Plane. See Section 9 for the proposed reclassification.

**Single YAML becomes unwieldy.** At 30+ stories, `product-context.yaml` becomes difficult to navigate, slow to parse for context assembly, and semantically overloaded (product narrative + dispatch state + cost telemetry in one file). See Section 4 for the multi-map solution.

**Evaluator is optional but shouldn't be.** "Light mode" skips the evaluator entirely. Self-evaluation is unreliable by the framework's own documentation ("agents tend to confidently praise work they produced"). See Section 10 for risk-proportional evaluation.

**jj is adoption friction.** Every skill references jj commands directly. Teams using git cannot adopt AEP without also adopting jj, which in 2026 remains a niche tool. See Section 11 for the VCS abstraction proposal.

---

## 4. Multi-Map Document Architecture

**This is the core v2 change.**

### Design Principle

**Multi-map is the universal default for ALL products, not a scaling escape hatch.** Even the simplest MVP starts with `index.yaml` + one capability map. As the product grows, you add more maps — no migration, no restructuring, just more files.

This principle follows directly from Jeff Patton's guidance: each story map should tell one complete user narrative that you can walk through left-to-right. A product with multiple user journeys needs multiple maps. But even a product with one journey benefits from the separation between human narrative and machine state.

### 4.1 The Problem with Single-File Context

1. **Semantic overload**: `product-context.yaml` simultaneously serves as product narrative (opportunity, persona, JTBD), system architecture document (modules, interfaces), work queue (stories with dispatch state), agent routing table (topology, handoffs), and telemetry database (cost, failure_logs, changelog). These concerns evolve at different rates and serve different audiences.

2. **Context pollution**: When `/dispatch` assembles context for an agent, it must parse the entire file to extract the relevant subset. The agent's context window receives product framing it doesn't need, stories for other modules it won't touch, and topology definitions for roles it won't play.

3. **No growth seam**: A product that starts simple and grows complex must eventually restructure its single YAML into multiple files. This restructuring is expensive and risky. Better to start with an extensible structure.

4. **Human readability degrades**: A 500-line YAML with opportunity brief, architecture, 40 stories, topology, cost tracking, and changelog is no longer a document a human can scan to understand the product. The story map — the thing Jeff Patton designed to maintain shared understanding — gets buried under machine state.

### 4.1b Human Alignment Risks of Multi-Map

The multi-map architecture solves real problems but introduces new ones that must be addressed:

**Map fragmentation.** Jeff Patton's story map is one artifact seen at once — that's the entire point. Splitting into per-capability files means humans must mentally reassemble the big picture across `index.yaml` + N `frame.yaml` + N `map.yaml` files. For a two-capability product, that's six files minimum. Mitigation: the dashboard (`apps/web`) must render all capability maps as a unified visual story map. Consider a `/view` command that renders the human-relevant view in-terminal.

**Staleness during execution.** Between `/compile` and `/reflect`, Human Maps freeze while the Machine Graph drifts (story statuses, cost, failures change via `/dispatch` and `/wrap`). The human's map becomes misleading during the period they most need accurate state. Mitigation: `/dispatch` should auto-detect stale maps (`map_hash` mismatch) and either auto-recompile or warn before proceeding. This behavior must be specified, not left implicit.

**"Where do I edit?" confusion.** When a human spots a wrong acceptance criterion, they must decide: edit the stub in `map.yaml` and recompile, or edit `machine-graph.yaml` directly (creating divergence). Mitigation: define a clear policy — direct Machine Graph edits are allowed but flagged as `compile_override: true` on the story, and `/compile` preserves overrides rather than clobbering them.

**Alternative considered:** Single source of truth with computed views (`/view --human`, `/view --machine`) instead of separate documents. This avoids fragmentation and staleness but loses the clean separation of concerns. This alternative should be prototyped alongside multi-map to determine which works better in practice before committing to full implementation.

### 4.2 Three-Layer, Multi-Map Architecture

#### Layer 1: Human Maps

Product narrative, organized by capability. One set of files per value stream or user journey.

```
product/
  index.yaml                           # Program frame
  maps/
    <capability-a>/
      frame.yaml                       # Scope + boundary
      map.yaml                         # Backbone + layers + story stubs
    <capability-b>/
      frame.yaml
      map.yaml
```

- Even a simple product starts with `index.yaml` + `maps/<main-capability>/frame.yaml` + `map.yaml`
- Each map tells ONE complete user narrative
- `/envision` writes `index.yaml`; `/map` writes capability-level files
- `/reflect` updates maps based on product learnings (bug classification, re-slicing, discovery)

**What goes in Human Maps:**

- Opportunity brief, persona, JTBD (in `index.yaml`)
- Backbone activities as left-to-right narrative
- Layers with outcome hypotheses and success metrics
- Story stubs (title, layer, activity, acceptance criteria sketch)
- Planned omissions and open questions
- Product decisions with reasoning

**What does NOT go in Human Maps:**

- Dispatch state (scores, assigned_to, epoch)
- Agent topology (roles, routing, concurrency)
- Execution tracking (cost_usd, pr_url, failure_logs)
- Full interface contracts (those belong in architecture, which compiles into Machine Graph)

#### Layer 2: Machine Graph

Unified agent control plane, compiled from all Human Maps.

```
product/
  machine-graph.yaml                   # Single compiled file
```

- Compiled by `/compile` from all Human Maps — deterministic: same inputs = same output
- Contains everything agents need: full stories with state machine, dispatch fields, acceptance criteria, interface obligations, topology, routing, layer gates, cost, learning policy, changelog
- Written by: `/compile` (initial), `/dispatch` (story state), `/wrap` (completion state)
- Read by: `/dispatch`, `/autopilot`, `/wrap`, dashboard

**What goes in Machine Graph:**

- Stories with full state machine (`pending → ready → in_progress → in_review → completed`)
- Dispatch scoring fields (`business_value`, `readiness_score`, `ambiguity_penalty`, `dispatch_score`)
- Architecture (modules, interfaces, data flows) — compiled from capability maps
- Topology (agent roles, routing policy, concurrency limits, conflict detection, retry strategy)
- Layer gates with integration test definitions
- Cost tracking (total, by_layer, by_module, by_story, alerts)
- Learning policy (auto-adjustments from execution history)
- Changelog (semantic history of all modifications)

#### Layer 3: Change Packets (OpenSpec)

Per-story execution packets. Already exists and works well.

```
openspec/
  changes/<story-id>/
    proposal.md
    design.md
    specs/
    tasks.md
    .context/                          # Pre-assembled context for agents
```

- Written by `/design` and `/build`
- Read by `/build`
- Archived by `/wrap` via `/opsx:archive`

### 4.3 Index File Schema

```yaml
# product/index.yaml
schema: v2
project: <project-name>
version: "0.1.0"
updated_at: <ISO 8601>

opportunity:
  bet: "Core hypothesis"
  why_now: "Market timing"
  counter_arguments: []
  scale_of_impact: "Magnitude of change if successful"
  kill_criteria: []
  decision: proceed # proceed | kill | defer
  decided_at: <ISO date>

personas:
  - id: <persona-id>
    description: "Concrete user with context, skill level, tools, constraints"
    jtbd: "When [situation], I want to [motivation], so I can [outcome]"

capabilities:
  - id: <capability-id>
    name: "e.g., Order to Cash"
    description: "One-line description of the user journey"
    map_path: "maps/<capability-id>/"
    status: active # active | planned | completed | deprecated
    owner: null # team or person
    depends_on: [] # other capability IDs (cross-capability dependencies)

constraints:
  technical: []
  business: []
  regulatory: []

success_criteria:
  - criterion: "Program-level success measure"
    metric: "How to measure"
    target: "What number to hit"

decisions:
  - id: DEC-001
    title: "Decision title"
    context: "What prompted this"
    decision: "What was decided"
    reasoning: "Why"
    consequences: "What this enables and constrains"
    date: <ISO date>
```

### 4.4 Capability Map Files

#### `frame.yaml`

```yaml
# product/maps/<capability>/frame.yaml
capability: <capability-id>
scope: "What this map covers"
primary_user: <persona-id>
secondary_users: []

boundary:
  start_trigger: "What initiates this journey"
  end_success_state: "What 'done' looks like for the user"

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

open_questions:
  - question: "Unresolved question"
    default_assumption: "What we assume until resolved"
    revisit_trigger: "When to revisit"
```

#### `map.yaml`

```yaml
# product/maps/<capability>/map.yaml
capability: <capability-id>

# Backbone: left-to-right user narrative (Jeff Patton's core concept)
activities:
  - id: <activity-id>
    verb_phrase: "e.g., Authenticate"
    description: "What the user does and why"
    layer_introduced: 0 # which layer first touches this activity

# Layers: horizontal slices through the backbone
layers:
  - layer: 0
    name: "Walking Skeleton"
    outcome_hypothesis: "Users can complete crudest end-to-end journey"
    success_metric:
      type: task_completion_rate
      target: ">= 60%"
    decision_rule:
      keep_if: "metric >= target"
      otherwise: "reflect_and_reslice"
    verification: "End-to-end user journey test definition"

  - layer: 1
    name: "Core Experience"
    outcome_hypothesis: "Users can complete primary job-to-be-done without workarounds"
    success_metric:
      type: task_success_rate
      target: ">= 80%"
    decision_rule:
      keep_if: "metric >= target"
      otherwise: "reflect_and_reslice"
    verification: "Functional test suite for core flows"

# Story stubs: enough for human understanding, not full dispatch-ready specs
stories:
  - id: <CAPABILITY>-001
    title: "Short, descriptive"
    layer: 0
    activity: <activity-id> # null for infrastructure stories
    description: "What changes when complete (observable behavior)"
    acceptance_criteria_sketch:
      - "High-level criterion (refined during /compile)"
    complexity_estimate: S # S | M | L
    depends_on_stubs: [] # other stub IDs within this map
    notes: null # known pitfalls, design considerations

planned_omissions:
  - "What we deliberately exclude and why"

architecture_hints:
  modules_involved: [] # which modules this capability touches
  key_interfaces: [] # critical integration points
  technology_notes: null
```

### 4.5 Machine Graph Schema

```yaml
# product/machine-graph.yaml
schema: v2
project: <project-name>
compiled_from:
  - capability: <capability-id>
    map_hash: <git-hash> # for staleness detection
compiled_at: <ISO 8601>
dispatch_epoch: 0

# Architecture (compiled from capability maps + enriched during /compile)
architecture:
  style: "e.g., modular monolith"
  overview: "2-3 sentences"
  modules:
    - name: <module-name>
      responsibility: "What this module does"
      does_not: "Boundary definition"
      owns: []
      depends_on: []
      technology: null
      key_concepts: []
  interfaces:
    - from: <module>
      to: <module>
      protocol: "HTTP REST | gRPC | function call | message queue"
      endpoint: "Specific API path or function signature"
      request: {}
      response: {}
      errors: []
      sla: null
  data_flows:
    - journey: "User journey name"
      path: "User -> [Module] -> action -> [Module] -> response"
  third_party: []
  deployment: {}
  amendment_log: []
  adrs: []

# Stories (compiled from stubs, enriched with dispatch fields)
stories:
  - id: <PROJECT>-001
    title: "Short, descriptive"
    source_capability: <capability-id>
    source_stub: <CAPABILITY>-001
    layer: 0
    module: <module-name>
    activity: <activity-id>
    slice: 1 # execution slice within layer (parallel batch)
    status: pending
    priority: critical # critical | high | medium | low
    business_value: 10 # 1-10 numeric (derived from priority or explicitly set)
    complexity: S # S | M | L
    readiness_score: null # 0.0-1.0 (computed by /compile or /validate)
    ambiguity_score: null # 0-5 (computed: higher = more ambiguous)
    dependencies: []
    description:
      what_changes: "Observable difference when complete"
      why: "Connection to capability map layer"
    acceptance_criteria:
      - "Specific, automatable test criterion"
    interface_obligations:
      implements: []
      consumes: []
      contract_tests_required: false
    files_affected:
      - "path/to/likely/file.ts"
    technical_notes: null
    verification:
      unit: []
      integration: []
      contract: []
    compile_mode: single_change # single_change | grouped_change | shared_enabler
    change_group: null # group ID for grouped_change mode
    # Dispatch scoring (computed by /dispatch)
    dispatch_score: null
    on_critical_path: false
    # Execution tracking (updated by /dispatch, /wrap)
    assigned_to: null
    openspec_change: null
    dispatched_at_epoch: null
    attempt_count: 0
    max_retries: 4
    cost_usd: null
    started_at: null
    completed_at: null
    pr_url: null
    failure_logs: []

# Execution slices (computed by /compile)
execution_slices:
  - layer: 0
    slice: 1
    stories: []
    theme: "Walking skeleton foundation"
  - layer: 0
    slice: 2
    stories: []
    theme: "Walking skeleton integration"

# Topology (agent roles, routing, handoffs)
topology:
  roles:
    - name: implementer
      purpose: "Takes story spec, produces code + tests + PR"
      does: []
      does_not: []
      input_contract: {}
      output_contract: {}
      context_composition: []
      cost_budget:
        input_tokens_max: 50000
        output_tokens_max: 20000
        alert_threshold_total: 100000
  routing:
    dispatch: fifo_within_slice
    concurrency_limit: 5
    conflict_detection: files_affected_overlap
    retry: "2x same agent -> failure analyst -> fresh agent -> human escalation"
    autonomous: false
    auto_merge: false
    auto_design: false
    skip_human_eval: none # none | backend | all
  handoffs: []

# Layer gates
layer_gates:
  - layer: 0
    status: not_started # not_started | running | passed | failed
    test_definition: "End-to-end user journey from Layer 0"
    results:
      tests_run: 0
      tests_passed: 0
      tests_failed: 0
      failures: []
    completed_at: null

# Cost tracking
cost:
  total_usd: 0
  by_layer: {}
  by_module: {}
  by_story: {}
  alerts: []

# Learning policy
learning_policy:
  auto_writeback: true
  reflect_threshold: 3 # auto-trigger /reflect after N completed stories
  cost_alert_ratio: 2.0 # alert if actual cost > 2x estimated
  retry_pattern_window: 5 # look at last N stories for failure patterns
  patterns: []
  # Example patterns (populated by /reflect based on execution history):
  # - when: "error_class == context_overflow"
  #   action: "reduce_context_slice"
  # - when: "error_class == merge_conflict"
  #   action: "lower_concurrency_for_module"
  # - when: "error_class == test_failure AND module == api"
  #   action: "require_contract_review"

# Changelog
changelog:
  - date: <ISO date>
    type: initial # initial | compile | envision_update | map_update | dispatch | reflect | build | wrap | layer_gate_pass | layer_gate_fail | architecture_review
    author: human # human | agent
    summary: "What changed and why"
    sections_changed: []
```

### 4.6 New Commands

#### `/slice`

**Purpose:** Cut a release from Human Maps — interactive, human-driven.

**When to use:** After `/envision` and `/map` have populated the Human Maps, before `/compile`.

**What it does:**

1. Read `index.yaml` and all capability maps
2. Present the backbone activities across capabilities
3. Human confirms which layers to target in this release cycle
4. For each capability, human confirms:
   - Which activities are in scope for this release
   - Outcome hypothesis for this slice
   - Planned omissions (what we deliberately skip)
5. Update `map.yaml` files with release slice decisions
6. Commit updated maps

**Key principle:** `/slice` is a human sensemaking activity. It's where Jeff Patton's "release line is a pencil line" philosophy lives. The human decides what's in and out based on product judgment, not machine scoring.

#### `/compile`

**Purpose:** Transform all Human Maps into a unified Machine Graph.

**When to use:** After `/slice` (or after `/map` if skipping explicit slicing), before `/dispatch`.

**What it does:**

1. Read `index.yaml` and all capability `frame.yaml` + `map.yaml` files
2. Resolve cross-capability dependencies from `index.yaml`
3. Generate full stories from story stubs:
   - Expand acceptance criteria sketches into specific, automatable criteria
   - Assign modules based on architecture hints and system map
   - Compute dependencies across capabilities
   - Assign execution slices (parallel batches within each layer)
4. Build architecture section from capability architecture hints
5. Compute readiness scores and ambiguity scores per story
6. Seed topology from capability map patterns (or reuse existing topology)
7. Create layer gates from layer verification definitions
8. Initialize cost tracking structure
9. Write `machine-graph.yaml`
10. Commit

**Key property:** Compile merges, doesn't overwrite — re-compiling after updating maps preserves execution tracking fields (dispatch state, cost, failure logs).

#### Compilation Complexity Tiers

Not all `/compile` operations are equal. Some are mechanical; others require intelligence:

**Tier 1 — Mechanical (truly deterministic):**

- Copy `layer`, `activity`, `complexity_estimate` from stub to full story
- Derive `business_value` from priority enum
- Compute `dispatch_score` from formula
- Assemble `execution_slices` from dependency DAG
- Build `layer_gates` from layer verification definitions
- Initialize cost tracking structure

**Tier 2 — AI-assisted (requires human review):**

- Expand `acceptance_criteria_sketch` into specific, automatable criteria
- Assign modules from `architecture_hints` + system map
- Infer `interface_obligations` from module boundaries
- Estimate `files_affected` from module ownership
- Compute `readiness_score` and `ambiguity_score`

Tier 1 operations are deterministic. Tier 2 operations use AI inference and MUST present their expansions to the human for review before committing the Machine Graph. `/compile` should output a diff showing all Tier 2 expansions so the human can confirm or correct.

This replaces the binary "deterministic vs AI-inferred" framing in Decision D2. The revisit trigger for full AI compilation without review remains: only when model quality reaches the point where Tier 2 expansions are reliably correct.

### 4.7 How Existing Commands Adapt

| Command      | v1 Behavior                                                                         | v2 Behavior                                                                                                                                                                                                   |
| ------------ | ----------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `/envision`  | Writes `product-context.yaml` opportunity + product sections                        | Writes `product/index.yaml` (opportunity, personas, capabilities) + creates first capability map directory with `frame.yaml`                                                                                  |
| `/map`       | Writes architecture, stories, topology, layer_gates, cost to `product-context.yaml` | Writes `product/maps/<capability>/map.yaml` (backbone, layers, story stubs, architecture hints). Can be run per-capability.                                                                                   |
| `/validate`  | Single-pass validation of `product-context.yaml`                                    | Two-pass: (1) Product design pass on Human Maps (narrative coherence, walking skeleton coverage, outcome contracts), (2) Technical pass on Machine Graph (dispatch fields, DAG validity, interface contracts) |
| `/dispatch`  | Reads `product-context.yaml`, scores stories, creates OpenSpec change               | Reads `product/machine-graph.yaml` only. No change to dispatch logic.                                                                                                                                         |
| `/build`     | Reads OpenSpec change in workspace                                                  | No change. Agents never see Human Maps or Machine Graph directly.                                                                                                                                             |
| `/wrap`      | Updates story status in `product-context.yaml`                                      | Updates story status in `product/machine-graph.yaml`. Signal sync unchanged.                                                                                                                                  |
| `/reflect`   | Updates `product-context.yaml` with classified feedback                             | Updates both: Human Maps (product learnings, re-slicing, discoveries) AND Machine Graph (new stories, cost observations). May trigger re-compile for structural changes.                                      |
| `/autopilot` | Reads `product-context.yaml` for prerequisites and dispatch                         | Reads `product/machine-graph.yaml` for all orchestration state.                                                                                                                                               |

### 4.8 Migration Path from v1

1. **v1 continues working.** The loader detects `schema: v1` (single file) vs `schema: v2` (multi-file). All v1 skills remain functional. No forced migration.

2. **Migration tool.** A one-time script splits an existing `product-context.yaml` into:
   - `product/index.yaml` (opportunity, persona)
   - `product/maps/<project>/frame.yaml` (extracted from product section)
   - `product/maps/<project>/map.yaml` (activities, layers, story stubs extracted from stories)
   - `product/machine-graph.yaml` (architecture, full stories, topology, layer_gates, cost, changelog)

3. **Gradual adoption.** Teams can migrate one project at a time. The dashboard and loader support both formats during the transition period.

---

## 5. Enhanced Dispatch Formula

### 5.1 Current Formula

From `skills/product-context/dispatch/SKILL.md`:

```
dispatch_score = (critical_path_urgency + business_value + unblock_potential) / complexity_cost
```

Where:

- `critical_path_urgency` (0-10): Stories on critical path = 10; others: `max(0, 10 - slack)`
- `business_value` (1-10): Derived from priority enum (critical=10, high=7, medium=4, low=1)
- `unblock_potential` (0-10): `min(10, count_dependent_stories * 2)`
- `complexity_cost` (divisor): S=1, M=2, L=4

### 5.2 Proposed Enhanced Formula

```
dispatch_score = (business_value + unblock_potential + critical_path_urgency + reuse_leverage) / (complexity_cost + ambiguity_penalty + interface_risk)
```

**New numerator term:**

- **`reuse_leverage`** (0-10): Stories that produce shared enablers (auth middleware, base components, shared utilities) score higher. Computed as `min(10, count_of_modules_depending_on_output * 3)`. This prioritizes foundational work that unblocks multiple streams.

**New denominator terms:**

- **`ambiguity_penalty`** (0-5): Stories with fewer than 3 acceptance criteria (+2), missing interface obligations (+1), unresolved open questions relevant to the story (+1), or no `files_affected` specified (+1). This replaces the current binary "well-specified vs ambiguous" routing decision in dispatch Step 7 with a continuous signal that degrades the score proportionally.

- **`interface_risk`** (0-3): Stories that create or modify interface contracts (+1 per interface touched, max 3). Cross-module interface changes carry integration risk that compounds in parallel execution.

### 5.3 Readiness Score

A per-story score computed during `/compile` or `/validate`, used by `/dispatch` for routing:

```
readiness_score = (
  min(3, acceptance_criteria_count)      # 0-3
  + (interface_obligations_defined ? 2 : 0)  # 0 or 2
  + (files_affected_identified ? 1 : 0)      # 0 or 1
  + (verification_defined ? 2 : 0)           # 0 or 2
  + (no_relevant_open_questions ? 2 : 0)     # 0 or 2
) / 10
```

Routing thresholds:

- `readiness_score < 0.5` → route to `/design` (spec needs refinement before agent execution)
- `readiness_score >= 0.7` → route to `/launch` (spec is dispatch-ready)
- Between 0.5 and 0.7 → present to user for decision

### 5.4 Story-to-Change Mapping

Currently the schema assumes 1:1 (one story = one OpenSpec change). v2 supports three modes:

| Mode             | When                                                          | Example                                                       |
| ---------------- | ------------------------------------------------------------- | ------------------------------------------------------------- |
| `single_change`  | Default. Vertical slice, independently mergeable.             | "Add user login endpoint"                                     |
| `grouped_change` | Tightly coupled stories that must be atomically verified.     | API contract + consumer integration + database migration      |
| `shared_enabler` | Infrastructure story consumed by multiple downstream stories. | Auth middleware, shared component library, base configuration |

Add to Machine Graph story schema:

```yaml
compile_mode: single_change # single_change | grouped_change | shared_enabler
change_group: null # group ID for grouped_change mode
```

When `compile_mode: grouped_change`, `/dispatch` dispatches the entire group as one unit, creates one OpenSpec change containing all grouped stories, and tracks the group atomically.

### 5.5 Grouped Change Specification

When `compile_mode: grouped_change`, the following rules apply:

**Dispatch:** The group is scored using the minimum `readiness_score` and maximum `complexity_cost` of any story in the group. This prevents premature dispatch when one story in the group is under-specified.

**OpenSpec packet:** One change packet with:

- `proposal.md` covering the full group rationale
- `design.md` explaining the cross-story contract
- `specs/<story-id>/` subdirectory per story
- `group-verification.md` defining atomic acceptance criteria for the group as a whole

**Build mode:** `/build` implements all stories in dependency order within the group. A single PR contains all changes. The evaluator verifies cross-story contracts in addition to per-story criteria.

**Failure handling:** Failure of any story fails the entire group. The group is retried as a unit. If a single story fails repeatedly, the group should be re-examined in `/design` — the coupling assumption may be wrong.

**Size limit:** Maximum 3 stories per group. Larger groups should be split or require explicit human design review.

---

## 6. Learning Loop

### 6.1 Current Gap

v1's feedback loop is human-initiated: `/wrap` updates story status and suggests running `/reflect`. But `/wrap` discards valuable execution telemetry — actual cost vs estimated, eval round count, failure patterns — that could inform future dispatch decisions automatically.

### 6.2 Auto-Writeback from `/wrap`

When `/wrap` processes a completed story, it should automatically:

1. **Record execution metrics** in the story's Machine Graph entry:
   - `actual_cost_usd` vs `cost_budget` from topology
   - `actual_complexity` (inferred from: eval rounds, lines changed, time elapsed)
   - `eval_rounds_used` (number of generator/evaluator cycles)

2. **Extract failure patterns** (if `attempt_count > 1`):
   - Classify the failure pattern from `failure_logs`
   - Check if the pattern matches existing `learning_policy.patterns`
   - If new pattern: add to policy with `confidence: low` (human confirms later in `/reflect`)

3. **Flag spec quality degradation** (if `eval_rounds > 3`):
   - The story's spec was likely ambiguous — too many review cycles indicate the spec didn't communicate intent clearly
   - Mark `ambiguity_score` as degraded for future reference
   - `/reflect` should review the story stub in the Human Map for clarity improvements

4. **Append structured learning** to changelog:
   ```yaml
   - date: <ISO date>
     type: wrap_learning
     author: agent
     summary: "Story X completed: cost $Y (budget $Z), N eval rounds, pattern: P"
     metrics:
       actual_cost: 3.20
       budgeted_cost: 2.00
       eval_rounds: 4
       attempt_count: 1
       failure_pattern: null
   ```

### 6.3 Learning Policy

A new section in `machine-graph.yaml` (included in schema above, Section 4.5):

```yaml
learning_policy:
  auto_writeback: true
  reflect_threshold: 3 # auto-trigger /reflect after N stories complete
  cost_alert_ratio: 2.0 # alert if actual > 2x estimated
  retry_pattern_window: 5 # analyze last N stories for patterns
  patterns:
    - when: "error_class == context_overflow"
      action: "reduce_context_slice"
      confidence: high
    - when: "error_class == merge_conflict AND module == api"
      action: "lower_concurrency_for_module"
      confidence: medium
    - when: "eval_rounds > 3 AND complexity == S"
      action: "flag_spec_ambiguity"
      confidence: low
```

`/dispatch` consults `learning_policy.patterns` when assembling context and setting routing parameters. For example:

- If `reduce_context_slice` is triggered for a module, `/dispatch` assembles a smaller context package
- If `lower_concurrency_for_module` is triggered, `/dispatch` avoids dispatching multiple stories to that module in parallel
- If `flag_spec_ambiguity` matches, `/dispatch` routes to `/design` regardless of readiness score

### 6.4 From Harness to Learning System

The progression:

1. **v1 (current):** Harness executes stories. Human reviews outcomes in `/reflect`.
2. **v2 phase 1:** `/wrap` auto-records metrics. `/reflect` has richer data to classify.
3. **v2 phase 2:** Learning policy auto-adjusts dispatch parameters. `/reflect` confirms or overrides.
4. **Future:** Learning policy patterns are refined across projects (cross-project learning via `.aep/` sync).

---

## 7. Outcome Contracts

### 7.1 Why Outcomes Matter

Jeff Patton emphasizes that release slices should be anchored in outcomes, not just feature completeness. A walking skeleton that "works" technically but doesn't help users complete their job isn't a valid walking skeleton.

v1 has `layer.verification` (test definition) and `layer.user_can` (capability description). These describe _what_ but not _whether it matters_. v2 adds outcome contracts that make the connection to product learning explicit.

### 7.2 Outcome Contract Schema

Already included in the `frame.yaml` and `map.yaml` schemas above (Section 4.4):

```yaml
outcome_contract:
  hypothesis: "If users can complete X in Y minutes, this slice is viable"
  success_metric:
    type: task_completion_rate # task_completion_rate | time_to_complete | error_rate | satisfaction_score
    target: ">= 80%"
  decision_rule:
    keep_if: "metric >= target"
    otherwise: "reflect_and_reslice"
```

### 7.3 How Outcome Contracts Flow

1. **`/envision`** defines program-level success criteria in `index.yaml`
2. **`/map`** defines capability-level outcome contract in `frame.yaml` and layer-level outcome hypotheses in `map.yaml`
3. **`/compile`** propagates outcome contracts into `machine-graph.yaml` layer gates
4. **`/reflect`** evaluates outcomes after layer completion:
   - If `keep_if` condition met → advance to next layer
   - If `otherwise` triggered → re-slice: promote stories from later layers, demote others, adjust backbone if needed
5. **Outcome evaluation is not test automation.** It may require user testing, analytics review, or qualitative assessment. `/reflect` prompts the human to evaluate against the contract.

---

## 8. `/design` Reclassification

### 8.1 The Case

`/design` currently lives in `skills/agentic-development-workflow/` (Execution Plane). But consider what it actually does:

- Runs on the **main session**, interactively with the user
- **Refines specifications**, not code
- Produces OpenSpec artifacts that feed **INTO** the execution plane
- Involves human judgment on trade-offs (interface width, error handling strategy, dependency decisions)

Compare with actual Execution Plane skills:

- `/launch` — spawns autonomous workspace (no human interaction)
- `/build` — autonomous implementation (no human interaction)
- `/wrap` — autonomous cleanup (minimal human interaction)

`/design` is the **last mile of the Control Plane**, not the first step of the Execution Plane.

### 8.2 Proposed Change

Move `/design` to `skills/product-context/design/SKILL.md`.

Updated mental model:

**CONTROL PLANE** (human decides):

```
/envision → /map → /slice → /compile → /validate → /dispatch → /design
```

**EXECUTION PLANE** (agents execute autonomously):

```
/launch → /build → /wrap
```

The boundary between planes becomes crisp: everything before `/launch` involves human judgment; everything after is autonomous agent work.

### 8.3 Files Affected

- `skills/agentic-development-workflow/design/SKILL.md` → move to `skills/product-context/design/SKILL.md`
- `README.md` — update mental model diagram
- `docs/autonomous-loop.md` — update boundary diagram
- `skills/agentic-development-workflow/README.md` — remove `/design` references
- `.aep/config.yaml` — update skill group mappings for downstream sync

---

## 9. Evaluator Depth Scaling

### 9.1 The Problem with Optional Evaluation

v1 offers "light mode" which skips the evaluator entirely. The framework's own documentation explains why this is risky: "agents tend to confidently praise work they produced, even when quality is mediocre." Making evaluation optional under time pressure means it will be skipped when it matters most — on complex, risky changes.

### 9.2 Risk-Proportional Evaluation

Replace the binary optional/required with evaluation depth scaled to story risk:

| Risk Level | Trigger (risk_score)                                                                              | Evaluation Approach                                                      |
| ---------- | ------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------ |
| **Low**    | risk_score 1-3: S complexity, single module, no interface changes, readiness_score >= 0.8         | Automated verification only (all tests must pass, no separate evaluator) |
| **Medium** | risk_score 4-7: M complexity OR touches interfaces OR readiness_score < 0.8                       | Full generator/evaluator with separate evaluator agent                   |
| **High**   | risk_score 8+: L complexity OR security-sensitive OR cross-module integration OR previous failure | Full generator/evaluator + protocol checker (three agents)               |

### 9.3 Implementation

In `machine-graph.yaml` topology:

```yaml
routing:
  evaluation_policy:
    low_risk: automated_verification # all tests must pass, no evaluator agent
    medium_risk: generator_evaluator
    high_risk: generator_evaluator_protocol
  risk_computation: "max(1, complexity_score + interface_count + (failure_history * 2) + security_flag)"
  # complexity_score: S=1, M=3, L=5
  # security_flag: 0 (normal) or 3 (security-sensitive module)
  # Risk tiers: Low 1-3, Medium 4-7, High 8+
```

The risk formula is additive with a floor of 1, ensuring no story ever scores zero risk. The previous multiplicative formula (`complexity * interface_count * (1 + failure_history)`) collapsed to zero when any factor was zero — a story rewriting the database migration layer with `interface_count: 0` would score zero risk, which is clearly wrong.

**Note on low-risk evaluation:** The low-risk tier uses "automated verification" (all defined tests must pass), not "self-review checklist." Sections 2.5 and 9.1 establish that agents self-evaluate leniently — routing low-risk stories to self-review would contradict this finding. Automated test verification is both cheaper than spawning an evaluator and more reliable than self-assessment.

The `/dispatch` scoring already knows complexity, interface obligations, and failure history. Adding `evaluation_depth` as a computed field on dispatched stories is a natural extension.

### 9.4 Gate at `/wrap`

If a story was completed without the appropriate evaluation depth (e.g., a medium-risk story used self-review only), `/wrap` should flag this:

- In autonomous mode: add a `review_gap` alert to the changelog
- In interactive mode: prompt the human to confirm acceptance

This replaces the current binary "evaluator or not" with a proportional system that never fully skips evaluation.

---

## 10. VCS Abstraction

### 10.1 The Friction

jj (Jujutsu) provides genuine engineering value for AEP:

- `jj workspace add` gives each agent an isolated working copy
- Mutable changes and auto-rebase match the agent workflow (generate rough code, then refine)
- `jj split` and `jj squash` enable clean commit history from messy agent output

However, jj remains a niche tool in 2026. Requiring it means every team that adopts AEP must also adopt a new version control system. This is the single largest adoption barrier.

### 10.2 Proposed Abstraction

Create a VCS abstraction that every skill references instead of calling jj directly:

```
skills/
  project-setup/
    vcs-ref/
      SKILL.md           # VCS abstraction reference
      references/
        jj-backend.md    # jj-specific commands (current jj-ref content)
        git-backend.md   # git equivalents
```

Every skill instruction that currently says "run `jj workspace add`" would instead say "create an isolated workspace (see `/vcs-ref`)".

### 10.3 Git Compatibility Mode

| jj Operation       | git Equivalent                                              | Capability Loss                             |
| ------------------ | ----------------------------------------------------------- | ------------------------------------------- |
| `jj workspace add` | `git worktree add`                                          | None (equivalent)                           |
| `jj edit <change>` | `git checkout <branch>`                                     | No mutable changes                          |
| `jj squash`        | `git rebase -i` (non-interactive via `GIT_SEQUENCE_EDITOR`) | Harder to automate                          |
| `jj split`         | `git add -p` + `git commit`                                 | Manual, harder to automate                  |
| `jj describe`      | `git commit --amend`                                        | Equivalent                                  |
| auto-rebase        | Manual `git rebase`                                         | No auto-rebase; agents must handle manually |

**Accepted trade-offs in git mode:**

- No automatic rebase — agents must explicitly rebase before creating PR
- No mutable changes — agents commit and amend rather than edit in place
- Less clean commit history — squash-on-merge compensates at PR level

### 10.4 Implementation Priority

This is P2 — important for adoption but not blocking for teams already using jj. The abstraction layer should be designed first (what operations skills actually need), then the git backend implemented as a compatibility shim.

---

## 11. Signal File Schema Evolution

### 11.1 Current State

Signal files (defined in `skills/agentic-development-workflow/launch/references/signals-spec.md`) are untyped JSON. The spec describes the structure informally, but there's no formal schema or version field.

### 11.2 Proposed Changes

1. **Add a formal schema** for `status.json`:

   ```yaml
   signal_version: 1
   phase: 0-13
   phase_name: "string"
   task_current: "string"
   task_index: 0
   task_total: 0
   blockers: []
   completion_pct: 0
   started_at: "ISO 8601"
   last_updated: "ISO 8601"
   # Product-cycle fields
   story_status: "in_progress | in_review | completed | failed"
   pr_url: null
   cost_usd: null
   completed_at: null
   failure_log: null
   ```

2. **Add version field** for forward compatibility. When the schema evolves, readers can detect which fields are available.

3. **Define lifecycle formally:**
   - Created: Phase 0 of `/build`
   - Updated: At every phase transition and significant progress milestone
   - Read: By `/dispatch` (signal sync), `/wrap` (completion sync), `/autopilot` (monitoring)
   - Cleaned up: By `/wrap` when workspace is removed

### 11.3 Autopilot Signal Staleness (From Real-World Feedback)

**Problem observed:** Autopilot misidentifies working agents as stuck because signal file updates are too infrequent. In practice, a workspace agent may read skill files (hundreds of lines), analyze existing code patterns, plan its approach, write multiple files, and create a PR — all without a single signal file update. The agent goes from "initialized" (phase 0, 5% completion) straight to "PR created" in one burst. Meanwhile, autopilot reads the same stale signal values across multiple ticks and increments the stuck counter.

**Root cause:** Signal-based monitoring has a fundamental blind spot — it only sees what the workspace agent explicitly writes. Long bursts of productive work between signal writes appear as stagnation to the monitoring system.

**Proposed fixes (ordered by implementation simplicity):**

1. **Tmux liveness check (recommended first fix).** Before incrementing the stuck counter, autopilot should run `tmux capture-pane -t <name>:0 -p -S -20` to check if the agent is actively producing output. If the tmux pane shows recent activity, do NOT count as stuck. This is the simplest change with the highest impact — it adds a secondary progress signal that doesn't require any changes to the workspace agent.

2. **jj diff check.** If the workspace has uncommitted changes (`jj diff` produces output), the agent is making progress even if signals are stale. This provides a file-system-level liveness signal independent of the agent's signal writing discipline.

3. **Phase-specific grace periods.** Phase 0 (initialization) naturally takes longer — reading skills, understanding the codebase, planning the approach. The stuck threshold should be higher for early phases:
   - Phase 0 (init): 12 ticks before stuck (currently 6)
   - Phase 1-4 (implementation): 6 ticks (current default)
   - Phase 5+ (review/PR): 8 ticks (reviews involve back-and-forth)

4. **More frequent signal writes in `/build`.** Update `status.json` at every task start, not just phase transitions. Add granular progress markers:
   - Task-level: `"task_current": "reading testing guide"`, `"task_index": 2, "task_total": 7"`
   - Sub-phase: `"sub_phase": "analyzing_codebase"` within phase 0

5. **Heartbeat protocol (recommended for autonomous mode).** Instead of inferring liveness from side-effects, have `/build` write a heartbeat timestamp to `status.json` at every major code generation step (minimum every 60 seconds during active work). If the heartbeat is stale beyond the phase-specific grace period, the agent is actually stuck or dead. No inference needed. This is more reliable than tmux capture + jj diff heuristics because it provides a direct, unambiguous liveness signal.

6. **Non-disruptive nudges.** Instead of sending text via `tmux send-keys` (which injects into the agent's input mid-task and can cause the agent to abandon its current approach), write nudge messages to `feedback.md`. The agent already checks `feedback.md` at phase boundaries. This ensures nudges arrive at natural breakpoints, not mid-generation. The current `tmux send-keys` approach risks causing intervention cascades: nudge → agent confusion → errors → more nudges → failure loop.

7. **Dead agent detection.** After the tmux liveness check, also verify the Claude process is still running: check the PID from `tmux list-panes -F '#{pane_pid}'`. If the process is dead, skip nudging and go straight to workspace cleanup and re-dispatch. This catches silent crashes (context window exhaustion, API errors) that leave a stale tmux pane.

8. **Intervention budget.** Track cumulative nudges per workspace. After 3 nudges with no progress change (same phase, same `completion_pct`), kill the workspace and re-dispatch with a fresh agent. In fully autonomous mode, there is no human to escalate to — the system needs a self-repair path rather than an infinite nudge loop.

**Impact on implementation roadmap:** Fix #1 (tmux liveness) should be added to Phase 1 (P0) as it's low effort and addresses a real operational issue. Fixes #3 and #4 belong in Phase 2 alongside signal schema evolution. Fixes #5-#8 are recommended for Phase 2 or Phase 3, with #5 (heartbeat) and #6 (non-disruptive nudges) being the highest priority for autonomous mode reliability.

**Files affected:**

- `skills/patterns/autopilot/references/tick-protocol.md` — add liveness check before stuck counter increment (Step 6), add dead agent detection, add intervention budget tracking
- `skills/patterns/autopilot/SKILL.md` — document phase-specific grace periods, document nudge-via-feedback.md policy
- `skills/agentic-development-workflow/build/SKILL.md` — add heartbeat writes to status.json, add more frequent signal write points

---

## 12. Implementation Roadmap

### Phase Dependencies

```
Phase 1 (bugfixes)  ─── independent
Phase 4 (/design)   ─── independent
Phase 7 (VCS)       ─── independent
Phase 2 (dispatch)  ─── soft dependency on Phase 5 (readiness_score computed by /compile)
Phase 3 (learning)  ─── soft dependency on Phase 5 (references Human Map stubs)
Phase 5 (multi-map) ─── independent (core change)
Phase 6 (outcomes)  ─── HARD dependency on Phase 5 (frame.yaml only exists in multi-map)
```

**Ordering risk:** Phases 1-4 improve `product-context.yaml`. Phase 5 replaces it with multi-map. Schema additions (Phase 1), dispatch formula updates (Phase 2), and learning policy (Phase 3) will need rework when Phase 5 lands. Recommendation: split Phase 1 into "bugfixes" (fix now — enum mismatches, autopilot stuck detection) and "schema additions" (defer to Phase 5 schema — `business_value` field, `execution_slices` section, autonomous routing fields).

### Minimum Viable v2

If scope must be reduced, the minimum viable v2 is:

1. Schema bugfixes (Phase 1 bugfixes only — enum mismatches, missing fields)
2. Multi-map architecture + `/compile` (Phase 5 — the core thesis)
3. Updated loader with adapter pattern for both formats

Everything else is independently valuable and can ship as v2.x:

- `/slice` (humans can edit `map.yaml` directly)
- Enhanced dispatch formula (current formula not proven insufficient)
- Learning loop (works on v1 or v2)
- Outcome contracts (product philosophy overlay)
- VCS abstraction (address when git-only team adopts)

### Validation Gate

Before committing to full Phase 5 implementation:

1. Run AEP v1 on a real project to 15-20 stories. Document actual pain points with single-file format.
2. Prototype `/compile` — hand-craft a `map.yaml`, run expansion logic, compare against hand-crafted `machine-graph.yaml`. If Tier 2 expansion is too lossy, revise the multi-map thesis.
3. Prototype the "single source + computed views" alternative (Section 4.1b) for comparison.

### Phase 1: Schema Fixes (P0, Low Effort)

**Goal:** Eliminate all schema inconsistencies that cause confusion between skills.

| Task                                  | File                                              | Change                                                                                    |
| ------------------------------------- | ------------------------------------------------- | ----------------------------------------------------------------------------------------- |
| Add `business_value` field to stories | `product-context-schema.yaml`                     | Add `business_value: null # 1-10, derived from priority if not set`                       |
| Standardize `layer_gates.status`      | `product-context-schema.yaml`, autopilot SKILL.md | Use `not_started` everywhere, remove `pending`                                            |
| Canonicalize story status enum        | `product-context-schema.yaml`                     | Document canonical 8-state machine, add validation note                                   |
| Add `execution_slices` section        | `product-context-schema.yaml`                     | Add section matching `/map` Step 3 output                                                 |
| Document autonomous routing fields    | `product-context-schema.yaml`                     | Add `autonomous`, `auto_merge`, `auto_design`, `skip_human_eval` under `topology.routing` |
| Autopilot tmux liveness check         | autopilot tick-protocol.md                        | Add `tmux capture-pane` check before stuck counter increment                              |
| Phase-specific stuck thresholds       | autopilot SKILL.md                                | Phase 0: 12 ticks, Phase 1-4: 6 ticks, Phase 5+: 8 ticks                                  |

### Phase 2: Enhanced Dispatch + Evaluator Scaling (P0, Medium Effort)

**Goal:** Improve dispatch quality and make evaluation proportional to risk.

| Task                               | File                              | Change                                                      |
| ---------------------------------- | --------------------------------- | ----------------------------------------------------------- |
| Add readiness/ambiguity scoring    | dispatch SKILL.md, schema         | New fields + computation logic                              |
| Update dispatch formula            | dispatch SKILL.md                 | Add `reuse_leverage`, `ambiguity_penalty`, `interface_risk` |
| Implement evaluation depth routing | dispatch SKILL.md, build SKILL.md | Risk computation → evaluation depth assignment              |
| Add evaluation gate to `/wrap`     | wrap SKILL.md                     | Flag review gaps                                            |

### Phase 3: Learning Loop (P1, Medium Effort)

**Goal:** Close the open-loop gap between execution and planning.

| Task                                  | File              | Change                                                      |
| ------------------------------------- | ----------------- | ----------------------------------------------------------- |
| Auto-writeback in `/wrap`             | wrap SKILL.md     | Record metrics, extract failure patterns, flag spec quality |
| Add `learning_policy` schema          | schema            | New section with patterns and thresholds                    |
| `/dispatch` reads learning policy     | dispatch SKILL.md | Consult patterns for context assembly and routing           |
| Structured learning changelog entries | schema            | New `wrap_learning` changelog type with metrics             |

### Phase 4: `/design` Reclassification (P1, Low Effort)

**Goal:** Clarify the Control/Execution Plane boundary.

| Task                 | File                          | Change                                   |
| -------------------- | ----------------------------- | ---------------------------------------- |
| Move skill file      | design SKILL.md               | Move to `skills/product-context/design/` |
| Update documentation | README.md, autonomous-loop.md | Update diagrams and references           |
| Update sync config   | `.aep/config.yaml`            | Update skill group mappings              |

### Phase 5: Multi-Map Document Architecture (P1, High Effort)

**Goal:** Implement the three-layer, multi-map document structure as the universal default.

| Task                                     | File                      | Change                                           |
| ---------------------------------------- | ------------------------- | ------------------------------------------------ |
| Design `index.yaml` schema               | New template file         | Program frame structure                          |
| Design `frame.yaml` + `map.yaml` schemas | New template files        | Capability map structure                         |
| Design `machine-graph.yaml` schema       | New template file         | Compiled agent control plane                     |
| Implement `/slice` command               | New skill                 | Human-driven release cutting                     |
| Implement `/compile` command             | New skill                 | Human Map → Machine Graph compilation            |
| Update `/envision`                       | envision SKILL.md         | Write `index.yaml` + create first capability map |
| Update `/map`                            | map SKILL.md              | Write per-capability `frame.yaml` + `map.yaml`   |
| Update `/validate`                       | validate SKILL.md         | Two-pass validation (maps + graph)               |
| Update `/reflect`                        | reflect SKILL.md          | Update both Human Maps and Machine Graph         |
| Update loader                            | product-context-loader.ts | Support both v1 and v2 formats                   |
| Migration tool                           | New script                | Split v1 YAML into v2 multi-file structure       |
| Update dashboard                         | apps/web                  | Compose data from multiple sources               |

### Phase 6: Outcome Contracts (P2, Medium Effort)

**Goal:** Make product learning an explicit, machine-verifiable part of the workflow.

| Task                                      | File             | Change                                                    |
| ----------------------------------------- | ---------------- | --------------------------------------------------------- |
| Add outcome contract to `frame.yaml`      | Template         | `outcome_contract` with hypothesis, metric, decision rule |
| Add layer-level outcomes to `map.yaml`    | Template         | Per-layer `outcome_hypothesis` + `success_metric`         |
| Propagate to layer gates in Machine Graph | `/compile`       | Layer gates include outcome evaluation criteria           |
| Update `/reflect`                         | reflect SKILL.md | Evaluate outcomes against contracts, trigger re-slice     |

### Phase 7: VCS Abstraction (P2, Medium Effort)

**Goal:** Enable git-based teams to adopt AEP without switching to jj.

| Task                                 | File               | Change                                |
| ------------------------------------ | ------------------ | ------------------------------------- |
| Create VCS abstraction skill         | New skill          | Operation-level abstraction           |
| Document git backend                 | New reference      | Git equivalents for all jj operations |
| Update all skills to use abstraction | All SKILL.md files | Replace direct jj references          |
| Test git compatibility               | Integration tests  | Verify full workflow with git backend |

---

## 13. Decision Log

### D1: Multi-map as universal default (not conditional)

**Decision:** Every product uses `index.yaml` + capability maps from day one, even simple MVPs.

**Alternatives considered:**

- (a) Single file for simple products, multi-map for complex ones (v2 Analysis 2)
- (b) Single file always, with sections for multiple capabilities
- (c) Multi-map always (chosen)

**Rationale:** Option (a) creates a migration cliff — the moment a product outgrows single-file, it requires restructuring. Option (c) has minimal overhead for simple products (one `index.yaml` + one capability map) but provides a natural growth path. The cost of starting with multi-map is one extra directory and one extra file. The cost of migrating later is restructuring all tooling and rewriting the YAML.

**Revisit trigger:** If user feedback consistently shows that the multi-file structure is confusing for simple projects, consider a `--simple` flag that generates a flattened single-file view.

### D2: `/compile` as tiered transformation

**Decision:** `/compile` uses tiered compilation: Tier 1 (mechanical fields — copying, formula computation, DAG assembly) is deterministic; Tier 2 (acceptance criteria expansion, module assignment, interface inference) is AI-assisted with mandatory human review. See Section 4.6 "Compilation Complexity Tiers" for the full breakdown.

**Alternatives considered:**

- (a) `/compile` uses AI for all operations (fully non-deterministic)
- (b) `/compile` is purely template-based (fully deterministic) — originally chosen, revised after design review
- (c) No `/compile` — humans write Machine Graph directly
- (d) Tiered: mechanical operations are deterministic, inference operations are AI-assisted with human review (chosen)

**Rationale:** Option (b) was the original choice, but design review revealed that expanding acceptance criteria sketches into automatable criteria and inferring interface obligations from architecture hints are not template operations — they require intelligence. Claiming determinism for these operations creates a false sense of reliability. Option (d) is honest about what is mechanical and what requires judgment, while preserving determinism where it genuinely applies and adding a human review gate where it doesn't.

**Revisit trigger:** If model quality improves to the point where Tier 2 expansions are reliably correct without human review, the review gate can be relaxed to opt-in confirmation.

### D3: `/design` belongs in Control Plane

**Decision:** Move `/design` from Execution Plane to Control Plane.

**Alternatives considered:**

- (a) Keep `/design` in Execution Plane (v1 status quo)
- (b) Move to Control Plane (chosen)
- (c) Split `/design` into Control Plane (spec refinement) and Execution Plane (OpenSpec artifact generation)

**Rationale:** `/design` runs interactively on the main session, involves human trade-off decisions, and produces specs that feed into execution. It shares more characteristics with `/dispatch` than with `/build`. Option (c) over-engineers the split — the skill is cohesive as-is.

**Revisit trigger:** If autonomous design mode (`auto_design: true`) becomes the common case, `/design` effectively becomes an Execution Plane activity and should move back.

### D4: Evaluator is never fully skippable

**Decision:** Replace binary optional/required with risk-proportional evaluation depth.

**Alternatives considered:**

- (a) Keep evaluator optional ("light mode" — v1 status quo)
- (b) Make evaluator always required
- (c) Risk-proportional depth (chosen)

**Rationale:** Option (a) means evaluation gets skipped under pressure, exactly when it matters most. Option (b) adds overhead to trivial changes (typo fixes, config changes). Option (c) scales effort to risk — simple changes get a checklist, complex changes get full gen/eval.

**Revisit trigger:** If model quality improves to the point where self-review reliably catches issues, lower the risk thresholds. If quality problems persist, raise them.

### D5: jj remains recommended but not required

**Decision:** Create a VCS abstraction with jj as recommended backend and git as compatible backend.

**Alternatives considered:**

- (a) Require jj (v1 status quo)
- (b) Switch to git entirely
- (c) Abstract with both backends (chosen)

**Rationale:** jj provides genuine value (workspace isolation, mutable changes, auto-rebase) that improves the agent workflow. But requiring it blocks adoption. The abstraction lets teams start with git and upgrade to jj when they see the benefits.

**Revisit trigger:** If jj reaches mainstream adoption (available in standard package managers, CI/CD platforms support it natively), the abstraction overhead may not be worth maintaining.

### D6: Learning policy starts simple, grows with data

**Decision:** Learning policy begins as a threshold-based system with human-confirmed patterns.

**Alternatives considered:**

- (a) No learning policy (v1 — human runs `/reflect` manually)
- (b) Full ML-based learning from execution history
- (c) Rule-based patterns with human confirmation (chosen)

**Rationale:** Option (a) wastes execution telemetry. Option (b) requires more data than most projects generate and adds unpredictability. Option (c) provides immediate value (auto-writeback, cost alerts) while building toward intelligence (patterns accumulate and are human-reviewed).

**Revisit trigger:** When a project accumulates 50+ completed stories with learning data, evaluate whether pattern-based rules are sufficient or whether statistical approaches would add value.

---

## 14. Guiding Principles for v2 Evolution

These principles should guide all implementation decisions:

1. **Human Maps are for humans. Machine Graph is for agents.** Never pollute Human Maps with dispatch state. Never force agents to parse product narrative.

2. **Start extensible, not minimal.** Multi-map from day one. One capability map is fine. The structure supports growth without restructuring.

3. **Compilation is the bridge.** The `/compile` step is where human intent becomes machine instruction. Tier 1 (mechanical) operations should be deterministic and re-runnable. Tier 2 (AI-assisted) operations should be auditable with human review.

4. **Every execution teaches.** `/wrap` doesn't just close stories — it records what happened so the system improves. Learning is not optional.

5. **Evaluation scales with risk.** No story ships without some form of review. The depth is proportional to the risk, not to the team's time pressure.

6. **The control plane's weight will shift.** As models improve, less upfront specification may be needed. AEP's "every harness component earns its place" principle applies here: periodically stress-test whether each control plane step is still necessary.

7. **Preserve the story.** Jeff Patton's core insight is that shared understanding comes from narrative, not from backlog items. The Human Map must always be readable as a story about users, not as a queue of work items.

---

## 15. Open Design Questions

These questions surfaced during design review and must be resolved before or during implementation.

### Q1: Cross-Story Context Assembly

When Story B depends on Story A's output, how does Story B's agent get accurate context about Story A's ACTUAL implementation (not just the pre-implementation interface contract)? The `architecture.interfaces` section describes intended contracts, but Story A's agent may have deviated — different error handling, additional middleware, or runtime data shapes not captured in the contract. Story B's agent discovers these deviations at runtime, wasting eval rounds.

**Proposed solution:** `/wrap` extracts a dependency manifest from each completed story (actual exported symbols, endpoint signatures as implemented, deviation notes from the interface contract). `/dispatch` includes this manifest in downstream story context packages instead of relying solely on pre-implementation contracts.

### Q2: `files_affected` Accuracy

`files_affected` is estimated during `/map` when the project may not even exist yet. These estimates will frequently be wrong, causing false-positive file conflicts (blocking parallel dispatch unnecessarily) and false-negative conflicts (allowing actually conflicting stories to run in parallel).

**Proposed solution:** Two-phase field — `files_affected_planned` (from `/map`, used for initial conflict detection) and `files_affected_actual` (populated by `/wrap` from merged PR diff, used for future conflict detection against in-progress stories). The actual values feed back into the Machine Graph for remaining stories.

### Q3: Machine Graph Write Coordination

If `/compile` runs while `/autopilot` ticks trigger `/dispatch` and `/wrap`, concurrent writes to `machine-graph.yaml` can corrupt state. The commit-as-lock pattern from `/dispatch` protects against concurrent dispatches but not against `/compile` overwriting dispatch state.

**Proposed solution:** Explicit `write_lock` field at the top of `machine-graph.yaml` with `holder` (skill name), `acquired_at`, and `expires_at` (auto-expire after 5 minutes to prevent deadlock). Every skill that writes to the Machine Graph must check and acquire the lock. `/autopilot` skips tick if lock is held by another skill.

### Q4: Learning Policy Expressiveness

The rule-based pattern system (`when: "error_class == X", action: "Y"`) uses string-equality conditions against 4 enum values. Real failure patterns are contextual and cross-dimensional: "context overflow when module X interacts with module Y's legacy API, because the generated adapter code is verbose."

**Open question:** Is the rule-based system expressive enough, or should the learning loop be restructured as a structured observation log with `/reflect` analyzing observations and proposing parameter adjustments (e.g., `auth_module.max_context_tokens: 30000`) rather than pattern-matching rules?

### Q5: Multi-Map vs Single Source + Views

The multi-map architecture (Section 4) and the "single source of truth with computed views" alternative (Section 4.1b) both solve the semantic overload problem. Multi-map provides cleaner separation of concerns but risks map fragmentation, staleness during execution, and "where do I edit?" confusion. Single source with views keeps one authoritative file but requires better tooling (dashboard, `/view` command).

**Open question:** Which approach should be prototyped first? What metrics determine which is better? Candidate metrics: time-to-understand for new team members, frequency of stale-state errors, human error rate (editing wrong file), and time from idea to dispatched story.

### Q6: `/compile` Human Review UX

If `/compile` Tier 2 operations require human review (Section 4.6), what does that review experience look like? Options: a full diff of every expanded story, a summary of changes with drill-down, or an interactive confirm/reject per story. The UX of this review determines whether humans actually engage with it or rubber-stamp it.

### Q7: Direct Machine Graph Edits

When a human edits `machine-graph.yaml` directly (fixing a wrong acceptance criterion, adjusting a dispatch score), how is this handled? Options:

- (a) Disallow — require editing the stub + recompile (strict but frustrating)
- (b) Allow and flag as `compile_override: true` on the story — `/compile` preserves overrides (recommended in Section 4.1b)
- (c) Allow and auto-propagate back to Human Maps via a `/sync` command

The choice directly affects the integrity of the human/machine separation and how much friction users experience.
