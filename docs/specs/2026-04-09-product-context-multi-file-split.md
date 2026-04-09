# Product Context Multi-File Split

**Date:** 2026-04-09
**Status:** Draft
**Scope:** AEP framework — product-context file structure + skill updates

## Problem

`product-context.yaml` serves as both the product definition (stable, human-authored) and the operational state (frequently modified by agents). At ~2700 lines for a real project (looplia), it's unwieldy. The v2 capability maps added `product/index.yaml` as an additive mirror, creating duplication. We need a single source of truth with no redundancy.

This spec intentionally goes beyond the narrower additive index described in the v2 roadmap. In split mode, `product/index.yaml` becomes the canonical home for stable product definition, not just a lightweight program frame.

## Design

### File Structure

```
product/
  index.yaml                           # Product definition (WHO, WHAT, WHY)
  maps/
    <capability>/
      frame.yaml                       # Scope, boundary, outcome contract
      map.yaml                         # Activities, layers, story stubs
product-context.yaml                   # Operational state (HOW, STATUS)
calibration/
  <type>.yaml                          # Heavy calibration artifacts (unchanged)
openspec/                              # Per-story change packages (unchanged)
```

### `product/index.yaml` — Product Definition

Single source of truth for everything the product IS. Stable after `/envision`, refined by `/reflect`, and updated by light calibrations that change stable product intent. `personas` replaces the legacy singular `product.persona` field from the v1 template.

```yaml
schema: v2
project: <project-name>
version: "0.1.0"

opportunity:
  bet: "..."
  why_now: "..."
  counter_arguments: []
  scale_of_impact: "..."
  kill_criteria: []
  decision: proceed
  decided_at: <ISO date>

personas:
  - id: <persona-id>
    description: "..."
    jtbd: "..."

capabilities:
  - id: <capability-id>
    name: "..."
    description: "..."
    map_path: "maps/<capability-id>/"
    status: active
    depends_on: []

product:
  problem: "..."
  goals: []
  non_goals:
    - statement: "..."
      reasoning: "..."
  mvp_boundary:
    in_scope: []
    out_of_scope: []
    deferred: []
  constraints:
    required_stack: {}
    preferred_stack: {}
    infrastructure: "..."
    external_deps:
      - name: <service>
        provides: "..."
        failure_mode: "..."
  layers:
    - layer: 0
      name: "..."
      user_can: "..."
      verification: "..."
      outcome_contract:
        hypothesis: "..."
        success_metric:
          type: task_completion_rate
          target: ">= 60%"
        decision_rule:
          keep_if: "metric >= target"
          otherwise: "reflect_and_reslice"
  activities:
    - id: <slug>
      name: "..."
      description: "..."
      order: 1
      layer_introduced: 0
  failure_model:
    classes:
      - name: "..."
        examples: "..."
        detection: "..."
        recovery: "..."
        escalation: "..."
    degraded_operation: "..."
  security_model:
    trust_boundaries: "..."
    auth: "..."
    secret_handling: "..."
  success_criteria:
    functional: []
    non_functional: []
  quality_dimensions:
    - dimension: <type>
      criticality: high
      first_calibration_layer: 0.5
      rationale: "..."
  open_questions:
    - question: "..."
      default_assumption: "..."
      revisit_trigger: "..."
  decisions:
    - decision: "..."
      reasoning: "..."
      alternatives: []
  stress_test:
    - challenge: "..."
      angle: product_viability
      resolution: "..."
```

### `product-context.yaml` — Operational State

Frequently modified by agents. Contains execution state, architecture, calibration state, and tracking. In split mode it no longer owns `opportunity` or `product`.

```yaml
schema: v1
project: <project-name>
version: "0.1.0"
updated_at: <ISO 8601>
dispatch_epoch: 0

calibration:
  plan:
    - layer: 0.5
      dimensions: ["visual-design"]
      trigger: "Condition that activates this calibration"
  history:
    - dimension: visual-design
      calibrated_at: <ISO date>
      calibrated_from_layer: 0.5
      mode: establishment
      artifact_path: "calibration/visual-design.yaml"
      sections_updated: []
      summary: "What was decided"

architecture:
  style: "..."
  overview: "..."
  technical_spec: null
  modules:
    - name: <module-name>
      responsibility: "..."
      does_not: "..."
      owns: []
      depends_on: []
      technology: null
      key_concepts: []
  domain_model:
    - name: <entity-name>
      purpose: "..."
      fields:
        - name: id
          type: string
          default: null
          required: true
          notes: "..."
      normalization_rules: []
      invariants: []
  protocol_sequences:
    - name: <protocol-name>
      participants: []
      trigger: "..."
      steps:
        - sender: <module-name>
          message_type: "..."
          timeout_ms: 5000
          payload_example: null
      error_behavior: "..."
      timeout_behavior: "..."
  interfaces:
    - from: <module-name>
      to: <module-name>
      protocol: "HTTP REST | gRPC | function call | message queue"
      endpoint: "..."
      request: {}
      response: {}
      errors:
        - code: <number>
          meaning: "..."
      sla: null
  data_flows:
    - journey: "..."
      path: "..."
  third_party:
    - name: <service>
      provides: "..."
      integration_point: "..."
      failure_mode: "..."
  deployment:
    environments: ["local", "staging", "production"]
    module_runtime: {}
    persistence: {}
  amendment_log:
    - proposed_by: <agent-or-module>
      module_affected: <module-name>
      proposed_change: "..."
      reasoning: "..."
      status: pending
  adrs:
    - id: ADR-001
      title: "..."
      context: "..."
      decision: "..."
      reasoning: "..."
      consequences: "..."

stories:
  - id: <PROJECT>-001
    title: "Short, descriptive"
    layer: 0
    module: <module-name>
    activity: null
    calibration_type: null
    slice: 1
    status: pending
    priority: critical
    business_value: null
    complexity: S
    compile_mode: single_change
    change_group: null
    dependencies: []
    description:
      what_changes: "Observable difference when complete"
      why: "Connection to Context Document / layer"
    acceptance_criteria: []
    interface_obligations:
      implements: []
      consumes: []
      contract_tests_required: false
    files_affected: []
    technical_notes: null
    verification:
      unit: []
      integration: []
      contract: []
    readiness_score: null
    dispatch_score: null
    on_critical_path: false
    assigned_to: null
    openspec_change: null
    dispatched_at_epoch: null
    attempt_count: 0
    max_retries: 4
    cost_usd: null
    started_at: null
    completed_at: null
    pr_url: null
    failure_logs:
      - attempt: 1
        error_class: test_failure
        approach_summary: "..."
        failure_point: "..."
        root_cause: "..."
        unexplored_alternatives: []
        timestamp: <ISO 8601>

topology:
  roles:
    - name: implementer
      purpose: "..."
      does: []
      does_not: []
      input_contract:
        story_spec: "..."
        context_slice:
          context_document: "..."
          system_map_module: "..."
          adjacent_interfaces: "..."
          dependency_artifacts: "..."
      output_contract:
        implementation:
          branch_name: "string"
          files_changed: "FileDiff[]"
          pr_url: "string"
        verification:
          unit_tests: "TestResult[]"
          contract_tests: "TestResult[]"
          all_passing: "boolean"
        status_report:
          story_id: "string"
          outcome: "success | failure"
          error_summary: "string"
          what_was_not_tried: "string[]"
      context_composition: []
      cost_budget:
        input_tokens_max: 50000
        output_tokens_max: 20000
        alert_threshold_total: 100000
  routing:
    dispatch: fifo_within_slice
    concurrency_limit: 5
    conflict_detection: files_affected_overlap
    retry: "2x same agent → failure analyst → fresh agent → human escalation"
    autonomous: false
    auto_design: false
    skip_human_eval: none
    process_learnings: []
  handoffs:
    - from: implementer
      to: evaluator
      trigger: implementation_complete
      payload: "eval-request.md + code diff + contracts.md + feature-verification.json"
      validation: []
      on_validation_failure: "reject handoff, notify source agent"

layer_gates:
  - layer: 0
    status: not_started
    test_definition: "End-to-end user journey from Layer 0 MVP contract"
    results:
      tests_run: 0
      tests_passed: 0
      tests_failed: 0
      failures: []
    completed_at: null

waves:
  - layer: 0
    wave: 1
    stories: []
    theme: "Walking skeleton foundation"

cost:
  total_usd: 0
  by_layer: {}
  by_module: {}
  by_story: {}
  alerts:
    - story_id: null
      type: cost_exceeded
      detail: "..."
      threshold: null
      actual: null
      timestamp: null

changelog:
  - date: <ISO date>
    type: initial
    author: human
    summary: "What changed and why"
    sections_changed: []
    feedback:
      bugs: []
      refinements: []
      discoveries: []
      opportunity_shifts: []
    cost_observations: null
    proposed_changes: []
```

### What Moves Where

| Section                                                                                                                                                                                                                   | From                 | To                                        | Rationale                                                             |
| ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------- | ----------------------------------------- | --------------------------------------------------------------------- |
| `opportunity`                                                                                                                                                                                                             | product-context.yaml | product/index.yaml                        | Stable after /envision                                                |
| `product.problem`, `goals`, `non_goals`, `mvp_boundary`, `constraints`, `layers`, `activities`, `failure_model`, `security_model`, `success_criteria`, `quality_dimensions`, `open_questions`, `decisions`, `stress_test` | product-context.yaml | product/index.yaml                        | Stable product definition                                             |
| `product.persona`                                                                                                                                                                                                         | product-context.yaml | `product/index.yaml` top-level `personas` | Normalize v1 single-persona shape into split-mode multi-persona shape |
| `architecture`                                                                                                                                                                                                            | product-context.yaml | **stays**                                 | Modified by /map, /reflect, /calibrate (api-surface, data-model)      |
| `calibration`                                                                                                                                                                                                             | product-context.yaml | **stays**                                 | Plan/history tracks operational state                                 |
| `stories`                                                                                                                                                                                                                 | product-context.yaml | **stays**                                 | Constantly modified by dispatch/wrap                                  |
| `topology`                                                                                                                                                                                                                | product-context.yaml | **stays**                                 | Modified by /map, updated by /reflect (process_learnings)             |
| `layer_gates`                                                                                                                                                                                                             | product-context.yaml | **stays**                                 | Status tracking                                                       |
| `waves`                                                                                                                                                                                                                   | product-context.yaml | **stays**                                 | Operational grouping                                                  |
| `cost`                                                                                                                                                                                                                    | product-context.yaml | **stays**                                 | Running totals                                                        |
| `changelog`                                                                                                                                                                                                               | product-context.yaml | **stays**                                 | Append-only log                                                       |

### Discovery Convention

Skills use this resolution pattern:

```
product_def_path = "product/index.yaml" if exists else "product-context.yaml"
operational_path = "product-context.yaml"
```

**Backwards compatible:** If `product/index.yaml` does not exist, skills read product definition from `product-context.yaml` as before (v1 single-file mode). In split mode, `product-context.yaml` is expected to omit `opportunity` and `product`, so the fallback only applies to legacy single-file projects, not partially migrated ones.

### Concurrency Model (Unchanged)

- Only the main session writes to YAML files
- Workspace agents report through `.dev-workflow/signals/`
- `/wrap` reads signals and updates `product-context.yaml`

## Skill Changes

### `/envision`

**Before:** Writes opportunity + product to `product-context.yaml`
**After:** Reads existing split files in update mode. Writes stable product definition to `product/index.yaml` (`opportunity`, `personas`, `capabilities`, stable `product.*`). Creates or updates `product-context.yaml` with header, `calibration.plan`, `calibration.history`, and `changelog`, while preserving operational sections. Still creates `product/maps/<capability>/frame.yaml` for multi-journey products.

### `/map`

**Before:** Reads product from `product-context.yaml`, writes architecture/stories/topology to same file
**After:** Reads product definition from `product/index.yaml` (with fallback to legacy `product-context.yaml`). Writes `architecture`, `stories`, `waves`, `topology`, `layer_gates`, `cost`, and `changelog` to `product-context.yaml`. Refines `calibration.plan` there if mapping changes calibration checkpoints. Writes capability `map.yaml` when capability maps exist.

### `/dispatch`

**Before:** Reads everything from `product-context.yaml`
**After:** Reads stable product definition from `product/index.yaml` for context assembly (problem, personas, constraints, layers, activities, success criteria). Reads execution state from `product-context.yaml` (`stories`, `topology`, `layer_gates`, `cost`, `architecture`). Writes story execution fields back to `product-context.yaml` (`status`, `readiness_score`, `dispatch_score`, `assigned_to`, `openspec_change`, `dispatched_at_epoch`, sync results) and increments `dispatch_epoch`. Context assembly for the stable prefix includes the product definition from index plus any referenced technical spec from `architecture.technical_spec`.

### `/calibrate`

**Before:** Reads quality_dimensions and product from `product-context.yaml`
**After:** Reads stable product definition from `product/index.yaml` (`opportunity`, personas, constraints, quality_dimensions, layers, activities, success criteria, failure model) and operational state from `product-context.yaml` (`calibration.plan`, `calibration.history`, `stories`, `architecture`). Heavy calibrations write `calibration/<type>.yaml` and, for visual design, `globals.css`; light calibrations update `architecture.interfaces` or `architecture.domain_model` in `product-context.yaml`, and update stable product sections in `product/index.yaml` for `scope-direction` (`product.goals`, `product.mvp_boundary`, `product.layers`) and `performance-quality` (`product.success_criteria`, `product.failure_model`). All calibrations append `calibration.history` and `changelog` entries in `product-context.yaml`.

### `/reflect`

**Before:** Reads/writes everything in `product-context.yaml`
**After:** Reads stable product definition from `product/index.yaml` and operational state from `product-context.yaml`. Writes new stories, topology routing updates (including `process_learnings`), cost observations, and `changelog` entries to `product-context.yaml`. If reflection changes stable product intent (opportunity, personas, MVP boundary, goals, layers, activities, other stable `product.*` fields), it updates `product/index.yaml`. For capability-map projects, capability narrative files may also need corresponding updates.

### `/validate`

**Before:** Reads everything from `product-context.yaml`
**After:** Reads from both files in split mode and from `product-context.yaml` alone in legacy mode. Validates cross-file consistency (`stories[].activity` and layer references against index, calibration references, dispatch-required story fields, topology/routing compatibility, duplication absence between files). When validating product context artifacts, it may need to patch either or both files so the split remains internally consistent.

### Skills NOT Changed

- `/build` — reads OpenSpec change, not product-context directly
- `/launch` — reads topology from product-context.yaml (no product definition needed)
- `/wrap` — reads signals, updates product-context.yaml stories
- `/autopilot` — orchestrates other skills, no direct file access change
- `/scaffold`, `/design` — read from OpenSpec context (already assembled by /dispatch)

## Migration Path

### For existing projects (looplia, 91app-agent-platform)

1. Expand `product/index.yaml` to be the full stable definition, not the current roadmap-era mirror. Copy in `opportunity.counter_arguments`, `opportunity.scale_of_impact`, `opportunity.decided_at`, and all stable product-definition fields currently still living under `product` in `product-context.yaml`.
2. Normalize persona data. For v1 projects, migrate `product.persona` into top-level `personas`. If an index already exists, merge with any existing personas instead of overwriting them blindly.
3. Preserve or create `capabilities`. For Looplia, retain the existing `remote-relay` capability and `maps/remote-relay/` path rather than regenerating a generic single-capability stub.
4. Keep `calibration`, `architecture`, `stories`, `topology`, `layer_gates`, `waves`, `cost`, and `changelog` in `product-context.yaml`. Remove only `opportunity` and `product` once the split-aware skill updates are in place.
5. Update the skill docs and synced copies together so `/envision`, `/map`, `/dispatch`, `/calibrate`, `/reflect`, and `/validate` all agree on split-mode paths.
6. Validate the migrated project against the real split: YAML parses, no duplicated stable sections remain, and Looplia's current `product/index.yaml` retains its existing personas/capability metadata while gaining the missing stable product fields.

### For new projects

`/envision` creates `product/index.yaml` from the start. `product-context.yaml` is created with operational sections only.

### Backwards compatibility

The convention-based fallback ensures v1 single-file projects continue working without changes. No breaking change to the framework.

## Verification

1. Parse both YAML files independently, for example:
   ```bash
   python3 - <<'PY'
   import yaml
   for path in ("product/index.yaml", "product-context.yaml"):
       with open(path) as fh:
           yaml.safe_load(fh)
   print("YAML OK")
   PY
   ```
2. Run each split-aware skill (`/envision`, `/map`, `/dispatch`, `/calibrate`, `/reflect`, `/validate`) on a test project and verify its documented read/write targets.
3. Verify no duplication: `opportunity` and stable `product` fields exist in exactly one file, while operational sections remain only in `product-context.yaml`.
4. Cross-file consistency: all story layer references exist in `product/index.yaml` `product.layers`; all `stories[].activity` values resolve to `product/index.yaml` `product.activities`; all calibration references point to the correct file.
5. Migration verification against Looplia: after migration, `product-context.yaml` no longer contains `opportunity` or `product`, and `product/index.yaml` contains the full stable definition plus existing personas/capability metadata.
6. Legacy fallback: on a legacy single-file project with no `product/index.yaml`, verify skills still fall back to reading stable definition from `product-context.yaml`.
