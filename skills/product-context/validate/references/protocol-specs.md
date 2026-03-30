# Downstream Protocol Specifications

Reference document for the Protocol Checker agent. Defines the exact requirements that downstream consumers impose on `product-context.yaml`.

---

## Dispatch Protocol Requirements

The `/dispatch` skill reads stories from `product-context.yaml` and computes `dispatch_score` for ranking. Every story MUST have these fields for dispatch to function.

### Required Story Fields

```yaml
- id: string               # kebab-case unique identifier
  title: string             # short descriptive title
  module: string            # package name (e.g., packages/db) or "integration"
  layer: integer            # 0 = walking skeleton, 1+ = enrichment
  slice: integer            # execution slice number (parallel batch)
  status: string            # pending | ready | in_progress | review | done | blocked | failed
  business_value: string    # critical | high | medium | low
  complexity: string        # S | M | L
  description: string       # what changes when complete
  acceptance_criteria: list  # testable conditions (minimum 3 for well-specified stories)
  dependencies: list         # story IDs this depends on (empty list if none)
  files_affected: list       # file paths modified/created
  attempt_count: integer     # starts at 0, incremented on retry
  failure_logs: list         # empty list initially, populated on failure
```

### Scoring Formula

```
dispatch_score = (critical_path_urgency + business_value + unblock_potential) / complexity_cost
```

Where:
- `critical_path_urgency` (0-10): Stories on the longest dependency chain get 10
- `business_value`: critical=10, high=7, medium=4, low=1
- `unblock_potential`: min(10, count_of_direct_dependents * 2)
- `complexity_cost`: S=1, M=2, L=4

### DAG Validation Rules

1. Dependencies must form a directed acyclic graph (no cycles)
2. Every ID in a `dependencies` list must reference an existing story ID
3. Stories in the same slice should have no mutual dependencies
4. Integration stories (module: "integration") should depend on all stories they test

### File Conflict Detection

Stories with overlapping `files_affected` must not be dispatched in parallel. Exceptions:
- Files documented as "append-only" (e.g., router index files) via `conflict_note` field
- Files in `files_affected: []` (no files — cannot conflict)

### Top-Level Fields

```yaml
dispatch_epoch: integer     # starts at 0, incremented each dispatch run
```

### Layer Gate Fields

```yaml
layer_gates:
  - layer: integer
    name: string
    status: string          # pending | passed | failed
    description: string
    tests: list
    pass_criteria: string
    blocks: string
```

---

## Design Protocol Requirements (/design → /launch)

OpenSpec changes created by `/dispatch` and consumed by `/design` must include:

### Story Spec Completeness

For `/design` to skip straight to `/launch` (well-specified path):
- 3+ specific, testable acceptance criteria
- Interface obligations defined (if touching module boundaries)
- Files affected identified
- Complexity S or M

For `/design` to refine first (ambiguous path):
- Fewer than 3 acceptance criteria
- Missing interface details
- Complexity L
- Open questions relevant to this story

### Context Package Structure

```
openspec/changes/<story-id>/
├── proposal.md          # story description + why + business value
├── design.md            # module definition + interface contracts
├── specs/<module>.md    # acceptance criteria + interface obligations
├── tasks.md             # implementation tasks
└── .context/            # pre-assembled context
    ├── stable-prefix.md # shared product/architecture context
    ├── dependencies.md  # public APIs from completed dependencies
    └── retrieval.md     # what to explore at runtime
```

---

## Build Protocol Requirements (/build)

### Feature Verification JSON

If the build agent uses verification tracking:

```json
[
  {
    "task": "string — task description",
    "change_id": "string — jj change short ID",
    "verification_steps": ["step 1", "step 2"],
    "passes": false,
    "evaluated_by": null,
    "round": null,
    "notes": null
  }
]
```

Rules:
- Generator MUST NOT modify `verification_steps` or `passes`
- Only evaluator or human updates these fields

### Signal Protocol

Workspace agents communicate via `.dev-workflow/signals/`:
- `status.json` — phase progress, completion %, PR URL, cost
- `eval-request.md` — generator requests evaluation
- `eval-response-<N>.md` — evaluator returns findings

---

## Topology Requirements

```yaml
topology:
  roles:
    - name: string
      responsibility: string
      does_not: string
      input: string
      output: string
      context_window: list
      cost_budget: string

  handoff_contracts:
    - from: string
      to: string
      trigger: string
      payload: string
      validation: string

  routing:
    dispatch_policy: string
    concurrency_limit: integer    # default 5
    conflict_detection: string
    retry_routing: string
```

---

## Common Protocol Violations

These are the most frequently caught issues (ranked by frequency from real validations):

1. **Missing `business_value` field** — map skill doesn't produce it, dispatch requires it
2. **`estimated_size` vs `complexity`** — map uses `small/medium/large`, dispatch expects `S/M/L`
3. **Missing `dispatch_epoch`** — top-level field required for idempotent dispatch runs
4. **Missing `attempt_count` and `failure_logs`** — required for retry routing
5. **Layer gate missing `status` field** — dispatch checks this for layer advancement
6. **Incomplete `files_affected`** — missing package.json updates, file deletions, config files
7. **DAG violations** — stories referencing non-existent dependency IDs
8. **Undeclared file conflicts** — multiple stories in the same slice modifying the same file
