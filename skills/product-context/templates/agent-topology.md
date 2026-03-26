# Agent Topology Template

Defines every agent role in the execution pipeline, how they communicate, and how the orchestrator routes work to them. This is an architectural document — changes here affect the behavior of the entire execution plane.

The core rule: **agents communicate through structured artifacts, not free text.** Every input and output is schema-defined. Every handoff has validation. No exceptions.

---

## Agent Roles

### Role: [Role Name]

**Purpose**: [One sentence — what this agent does in the pipeline.]

**Responsibility boundary**:
- Does: [Specific responsibilities]
- Does not: [Explicit exclusions — prevents role creep]

**Input contract**:
```
[Schema definition of the work object this agent receives. Use TypeScript types, JSON Schema, or equivalent. Be explicit about required fields.]

Example:
{
  story_spec: StorySpec,           // The full story specification
  context_slice: {
    context_document: string,       // Relevant sections only
    system_map_module: ModuleDef,   // This story's module definition
    adjacent_interfaces: InterfaceContract[],
    dependency_artifacts: Artifact[] // Public interfaces from completed dependencies
  }
}
```

**Output contract**:
```
[Schema definition of what this agent produces.]

Example:
{
  implementation: {
    branch_name: string,
    files_changed: FileDiff[],
    pr_url: string
  },
  verification: {
    unit_tests: TestResult[],
    contract_tests: TestResult[],
    all_passing: boolean
  },
  status_report: {
    story_id: string,
    outcome: "success" | "failure",
    error_summary?: string,
    what_was_not_tried?: string[]   // Critical for fresh-agent retries
  }
}
```

**Context window composition**:
[Exactly what goes into this agent's context and in what order. Less is more — irrelevant context degrades performance.]

1. Story Spec (full)
2. Context Document (pruned to: Purpose, Technical Constraints, relevant Layer in MVP Contract)
3. System Map (this module + adjacent interface contracts only)
4. Dependency artifacts (public API surface only, not internals)

**Cost budget**:
- Expected tokens: [range, e.g., 10k–50k input, 5k–20k output]
- Expected duration: [range, e.g., 2–10 minutes]
- Alert threshold: [e.g., > 100k total tokens or > 20 minutes]

[Repeat for each agent role]

---

## Standard Roles

Most projects will need at least these roles. Add or remove based on project complexity.

### `implementer`
Takes a story spec and produces code + tests + PR. The workhorse of the execution plane.

### `contract-verifier`
Takes a PR and runs interface contract tests against the System Map. Catches integration incompatibilities before merge.

### `integration-tester`
Runs end-to-end tests for layer gates. Operates on the combined codebase, not individual stories.

### `failure-analyst`
Takes a failed story's trace and produces a structured failure log with root cause hypothesis and unexplored alternatives. Feeds into fresh-agent retries.

---

## Handoff Contracts

### [Source Role] → [Target Role]

**Trigger event**: [What causes this handoff — e.g., "implementer completes PR submission"]

**Payload schema**:
```
[Exact structure passed from source to target]
```

**Pre-handoff validation**:
[Checks that run before the target agent starts. If validation fails, the handoff is rejected and the source agent is notified.]

- [ ] Payload matches schema
- [ ] All required fields present and non-empty
- [ ] Referenced artifacts (files, PRs) actually exist
- [ ] [Domain-specific checks]

**Failure handling**: [What happens if validation fails — retry source? escalate?]

---

## Routing Rules

### Dispatch Policy

**Queue model**: [FIFO within execution slice / priority-based / other]

**Assignment**: When a story transitions to `ready` in the work graph, the orchestrator:
1. Checks conflict detection (see below)
2. Checks concurrency limit
3. Assembles context package per the implementer role's context window composition
4. Dispatches to the next available agent instance

### Concurrency

- **Maximum parallel agents**: [Number — start with 5–10, increase as stability is proven]
- **Per-module limit**: [Optional — prevent one module from consuming all agent capacity]

### Conflict Detection

Stories that modify overlapping files must not run in parallel. The orchestrator checks each ready story's "Files Likely Affected" against in-progress stories. Conflicts are serialized — the later story waits until the earlier one completes.

### Retry Routing

```
Attempt 1:   Same agent, error context appended to input
Attempt 2:   Same agent, second retry
Attempt 3:   failure-analyst produces structured log
             → fresh implementer agent receives story spec + failure log
Attempt 4:   Human escalation — story marked 'failed', user notified
```

Between attempt 2 and 3, the failure-analyst role intervenes to extract useful signal from the failures before handing off to a fresh agent.

---

## Cost Tracking Schema

Every agent invocation produces a trace record appended to the project's cost log:

```
{
  story_id: string,
  agent_role: string,
  attempt_number: number,
  start_time: ISO8601,
  end_time: ISO8601,
  tokens_input: number,
  tokens_output: number,
  cost_usd: number,
  outcome: "success" | "failure" | "escalated",
  error_class?: string    // e.g., "test_failure", "timeout", "context_overflow"
}
```

### Cost Alerts

- **Per-story alert**: Triggered when total cost across all attempts for a single story exceeds [threshold].
- **Per-layer alert**: Triggered when cumulative layer cost exceeds [threshold].
- **Anomaly alert**: Triggered when a story's cost is > 3x the median for its complexity class (S/M/L).

---

## Topology Diagram

[Optional but recommended. A simple flow diagram showing agent roles, handoff directions, and the orchestrator's position.]

```
                    ┌─────────────────┐
                    │   Orchestrator   │
                    │  (Control Plane) │
                    └────────┬────────┘
                             │ dispatches
                    ┌────────▼────────┐
                    │   Implementer    │
                    │ (Execution Plane)│
                    └────────┬────────┘
                             │ PR submitted
                    ┌────────▼────────┐
                    │Contract Verifier │
                    └────────┬────────┘
                             │ pass/fail
                    ┌────────▼────────┐
              ┌─────│   Orchestrator   │─────┐
              │     └─────────────────┘     │
              │ (all layer stories done)     │ (failure)
     ┌────────▼────────┐          ┌─────────▼────────┐
     │Integration Tester│          │ Failure Analyst   │
     └────────┬────────┘          └─────────┬────────┘
              │                             │
         Layer Gate                   Fresh Implementer
         Decision                     or Escalation
```
