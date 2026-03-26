# Orchestration Patterns

Detailed patterns for the control plane's orchestrator — state management, context assembly, layer gating, and failure handling. Read this when setting up or debugging the execution pipeline.

---

## Work Graph as State Machine

The work graph is a live state machine. Every story node holds a status and transitions based on events.

### State Transitions

```
pending     → ready        (all dependency stories reach 'completed')
ready       → in_progress  (orchestrator dispatches to agent)
in_progress → in_review    (agent submits PR)
in_review   → completed    (verification passes)
in_review   → in_progress  (verification fails, retry initiated)
in_progress → failed       (retry limit exceeded, escalated)
pending     → blocked      (a dependency story enters 'failed')
any         → deferred     (user explicitly postpones)
```

### Orchestrator Loop

The orchestrator is event-driven, not polling-based:

1. **Event received** (story completed, PR submitted, verification result, failure).
2. **Update state** of the affected story in the work graph.
3. **Cascade check**: Does this transition unlock new stories? (completed → check dependents). Does it block stories? (failed → mark dependents as blocked).
4. **Dispatch**: For each newly `ready` story, run conflict detection, assemble context, dispatch to agent per routing rules.
5. **Layer check**: Are all stories in the current layer `completed`? If yes, trigger Integration Gate.
6. **Alert check**: Any cost anomalies? Any critical path blockages? Notify user if needed.

### Concurrency Control

- Maximum parallel agents is configurable. Start with 5–10.
- Two stories with overlapping "Files Likely Affected" must not run in parallel — serialize them.
- If two parallel stories produce merge conflicts, the later PR rebases on the merged one and re-verifies.

---

## Context Assembly

### The Problem Context Assembly Solves

An agent's output quality is directly proportional to the relevance and precision of its input context. Too little context → the agent guesses. Too much context → the agent gets confused or hits token limits. Context assembly is the art of giving each agent exactly what it needs and nothing more.

### Assembly Rules

For each agent role, the Agent Topology document defines a **context window composition** — the ordered list of what goes in. The orchestrator follows this list mechanically:

1. **Read the composition spec** for the target agent role.
2. **Prune the Context Document** to the sections listed in the spec.
3. **Extract the relevant System Map slice** — the story's module and its adjacent interfaces only. Do not include unrelated modules.
4. **Collect dependency artifacts** — for each completed dependency, extract the public interface (types, exports, API surface). Do not include internal implementation unless the composition spec explicitly requires it.
5. **Validate the package** — all required fields present, no references to missing artifacts.
6. **Measure the package** — if it exceeds the target token budget for the role, escalate for manual pruning or split the story.

### Common Assembly Failures

- **Missing dependency artifact**: A dependency is marked `completed` but its output artifact is not found. This usually means the previous agent's output contract was not enforced. Fix: add post-completion validation in the handoff contract.
- **Stale interface contract**: The System Map was amended but the context package still references the old version. Fix: always read interface contracts from the latest System Map, not from cached copies.
- **Context overflow**: The assembled package exceeds the agent's token budget. Fix: either prune more aggressively (summarize dependency artifacts instead of including full source) or split the story into smaller units.

---

## Layer Gating

### Gate Design

Each layer has an Integration Gate — tests that verify stories work together. The gate is NOT the sum of individual story tests. It tests emergent behavior at integration boundaries.

**Layer 0 gate** is the most important test in the pipeline. It executes the exact user journey from the Context Document's Layer 0 MVP Contract. If the walking skeleton doesn't work end-to-end, something is architecturally wrong.

**Subsequent layer gates** test:
1. All previous layer journeys still work (regression).
2. New capabilities added in this layer work end-to-end.
3. Interface contracts honored under realistic conditions (not just mocks).

### Gate Failure Protocol

```
Gate fails
  → Identify failure boundary (which module interface)
  → Check: implementation vs contract mismatch?
     → Implementation wrong: create fix story → Phase 4
     → Contract wrong: trigger Architecture Review → Phase 2
        → Assess impact on completed stories
        → May require re-execution of affected stories
```

Gate failure on a contract issue is the most expensive failure in the pipeline because it can invalidate already-completed work. This is why Phase 2 (System Map approval) is a human-reviewed gate — catching contract errors early prevents cascading rework.

---

## Failure Handling

### Why Fresh-Agent Retry Works

When an agent fails and retries, it carries the full reasoning trajectory from its first attempt. If that trajectory led to a dead end, the retry often follows the same path — the agent is stuck in its own logic. A fresh agent receives only the structured failure log, not the reasoning. It approaches the problem without the stuck trajectory.

The **failure log's "what was NOT tried" field** is the highest-value signal for the fresh agent. It provides starting points the previous agent considered but did not explore.

### Failure Log Schema

```
{
  story_id: string,
  attempt_number: number,
  agent_role: string,

  approach_summary: string,     // What the agent tried to do
  failure_point: string,        // Which verification step failed
  error_output: string,         // Exact error messages or test failures
  hypothesis: string,           // Agent's best guess about root cause
  not_tried: string[],          // Alternative approaches considered but not attempted

  context_issues?: string,      // Any problems with the context package
  time_spent_seconds: number,
  tokens_used: number
}
```

### Cascade Prevention

When a story fails:

1. Mark direct dependents as `blocked`.
2. Continue executing non-blocked stories in the same layer.
3. If the failed story is on the **critical path** → alert user immediately (entire layer is blocked).
4. If NOT on critical path → other work continues. User addresses failure asynchronously.
5. When the failed story is eventually resolved (fixed or deferred), unblock dependents and resume normal dispatch.

### Escalation Format

When a story reaches human escalation, present:

1. The story spec (what was being attempted).
2. All failure logs from all attempts (what happened).
3. The fresh agent's failure log specifically (the most informed analysis).
4. Current impact: which stories are blocked, is this on the critical path?
5. Suggested options: fix the story, simplify the story, defer it, or modify the architecture.

---

## State Persistence

The orchestrator's state must survive crashes.

### Storage Options

- **File-based (JSON in repo)**: Simple, version-controlled. Sufficient for most MVP projects. Limitation: does not support concurrent orchestrators.
- **SQLite**: Supports querying ("show all failed stories") and concurrent access. Better for larger projects.
- **External store (Redis, Postgres)**: For production-grade orchestration with multiple concurrent sessions.

For MVP-stage projects, start with JSON in the repo. Upgrade when the limitation matters.

### State Snapshot Schema

```
{
  project_id: string,
  current_layer: number,
  stories: {
    [story_id]: {
      status: "pending" | "ready" | "in_progress" | "in_review" | "completed" | "failed" | "blocked" | "deferred",
      assigned_agent?: string,
      attempt_count: number,
      last_updated: ISO8601,
      failure_logs?: FailureLog[],
      pr_url?: string,
      completed_at?: ISO8601
    }
  },
  layer_gates: {
    [layer_number]: {
      status: "not_started" | "running" | "passed" | "failed",
      test_results?: TestResult[],
      completed_at?: ISO8601
    }
  },
  cost_summary: {
    total_cost_usd: number,
    cost_by_layer: { [layer]: number },
    cost_by_role: { [role]: number },
    cost_by_story: { [story_id]: number }
  },
  last_updated: ISO8601
}
```

### State Inspection

The user should be able to query the current state at any time:

- Progress per layer: completed / in_progress / pending / failed / blocked
- Critical path status: what is the next bottleneck?
- Cost breakdown: where is the money going?
- Blocked stories: what is waiting on what?

Provide a simple CLI command or dashboard that reads the state file and renders this overview.
