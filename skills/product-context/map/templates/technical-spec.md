# Technical Specification Template: [System Name]

A production-grade system specification for protocol-heavy systems. Use this template when the Context Document and System Map don't capture enough behavioral detail for agents to implement without ambiguity — typically when the system has multi-step protocols, multiple state machines, or complex failure/recovery semantics.

> **Important boundary:** This template is opt-in. Most projects go directly from context-document to system-map. Use this only when the system has protocol-level complexity that those templates don't capture.

**When to use this template:** During `/map`, if the System Map reveals 3+ interface contracts requiring protocol sequences, 2+ distinct state machines, explicit failure classes with different recovery behaviors, or trust boundaries crossing module lines.

**Reference exemplar:** See `references/symphony-spec-reference.md` for an annotated analysis of OpenAI's Symphony SPEC.md — the standard this template is modeled after.

Quality standard: **every statement must be convertible into a verification condition.** If it cannot be tested, it is not precise enough.

---

## 1. Service Identity

**[System name]** is [one sentence defining what this service IS — what it does, for whom, in what context].

The service solves [N] operational problems:

- [Problem 1 — concrete operational pain point this service eliminates]
- [Problem 2]
- [Problem 3]

> **Important boundary:** [What this service is NOT. State the most likely misunderstanding about scope. Example: "Symphony is a scheduler/runner and tracker reader. Ticket writes are performed by the coding agent."]

### Trust Posture

[State explicitly whether this service is intended for trusted environments, restricted environments, or both. This shapes every downstream security and approval decision.]

---

## 2. Goals and Non-Goals

### 2.1 Goals

[Behavior-observable statements. Each must be verifiable by observing the running system. Not aspirations — observable behaviors.]

- [Goal — e.g., "Poll the issue tracker on a fixed cadence and dispatch work with bounded concurrency."]
- [Goal — e.g., "Recover from transient failures with exponential backoff."]
- [Goal — e.g., "Support restart recovery without requiring a persistent database."]

### 2.2 Non-Goals

[Things a reasonable person might expect this system to do, but it deliberately will NOT. Each explains why.]

- [Non-goal — e.g., "Rich web UI or multi-tenant control plane."]
- [Non-goal — e.g., "General-purpose workflow engine or distributed job scheduler."]

---

## 3. System Overview

### 3.1 Components

[Named components, each with a one-line responsibility. Number them for cross-referencing.]

1. `[Component Name]`
   - [One-line responsibility. What it does and — if ambiguous — what it does NOT do.]

2. `[Component Name]`
   - [One-line responsibility.]

### 3.2 Abstraction Layers

[Group components into named layers. This makes the system easier to port and reason about.]

1. `[Layer Name]` ([layer purpose])
   - [What lives in this layer]

2. `[Layer Name]` ([layer purpose])
   - [What lives in this layer]

### 3.3 External Dependencies

- [Dependency — what it provides, how failure is handled]
- [Dependency]

---

## 4. Domain Model

### 4.1 Entities

#### [Entity Name]

[One sentence — what this entity represents in the system.]

Fields:

| Field        | Type      | Default   | Required | Notes                                   |
| ------------ | --------- | --------- | -------- | --------------------------------------- |
| `id`         | string    | —         | yes      | Stable identifier. Derived from [rule]. |
| `status`     | enum      | `pending` | yes      | See state machine in Section 5.         |
| `created_at` | timestamp | —         | no       | ISO-8601.                               |

#### [Entity Name]

[Repeat for each domain entity.]

### 4.2 Normalization Rules

[How values are compared, derived, and sanitized across the system.]

- `[Identifier Type]` — [Derivation rule, e.g., "Replace any character not in [A-Za-z0-9._-] with \_"]
- `[State comparison]` — [e.g., "Compare states after lowercase"]

### 4.3 Invariants

[Conditions that must always hold across the system, not just within one entity.]

- [Invariant — e.g., "A running entity always has a non-null started_at timestamp"]
- [Invariant — e.g., "At most max_concurrent entities may be in Running state"]

---

## 5. State Machines

### [Stateful Entity] States

[These are the system's internal states, which may differ from external/user-visible states.]

1. `[State Name]` — [When the entity is in this state, what is true about it]
2. `[State Name]` — [Description]
3. `[State Name]` — [Description]

> **Important nuance:** [Easy-to-miss detail about state semantics, e.g., "A successful exit does not mean the entity is done forever."]

### Transition Triggers

| Trigger | From State(s) | To State | Side Effects                                     |
| ------- | ------------- | -------- | ------------------------------------------------ |
| [Event] | [State]       | [State]  | [What happens — cleanup, notifications, retries] |

### Idempotency and Recovery Rules

- [Rule — e.g., "Claimed checks are required before launching any worker"]
- [Rule — e.g., "Restart recovery is tracker-driven, no durable DB required"]

---

## 6. Configuration Specification

### 6.1 Source Precedence

[Where configuration comes from, in priority order.]

1. [Highest priority — e.g., CLI arguments]
2. [e.g., Configuration file values]
3. [e.g., Environment variable indirection]
4. [Lowest priority — built-in defaults]

### 6.2 Dynamic Reload Semantics

[Can config change at runtime? What happens when it does?]

- [e.g., "Watch config file for changes. Re-apply without restart."]
- [e.g., "Invalid reloads must not crash the service. Keep last known good config."]

### 6.3 Validation Rules

[What must be true before the system starts dispatching work?]

- [e.g., "Config file can be loaded and parsed"]
- [e.g., "API key is present after environment variable resolution"]

### 6.4 Config Cheat Sheet

> This section is intentionally redundant so a coding agent can implement the config layer quickly.

| Key          | Type    | Default     | Notes              |
| ------------ | ------- | ----------- | ------------------ |
| `[key.path]` | string  | `[default]` | [What it controls] |
| `[key.path]` | integer | `[default]` | [What it controls] |

---

## 7. Protocol Specifications

### [Protocol Name]: [Participant A] <> [Participant B]

**Purpose:** [What this protocol accomplishes]

**Compatibility note:** [What must be preserved for interoperability vs. what can vary]

#### Launch Contract

- Command: `[how the subprocess/service is started]`
- Working directory: [where it runs]
- Communication: [stdio, HTTP, gRPC, etc.]

#### Startup Handshake

[Illustrative transcript showing the exact message sequence. Equivalent payload shapes are acceptable.]

```json
{"id":1,"method":"initialize","params":{...}}
// wait for response
{"method":"initialized","params":{}}
{"id":2,"method":"[next step]","params":{...}}
```

1. [Step 1 — what is sent, what to expect back, timeout]
2. [Step 2]
3. [Steady-state interaction begins]

#### Streaming / Turn Processing

[How the steady-state interaction works.]

- [e.g., "Read line-delimited JSON from stdout"]
- [e.g., "Buffer partial lines until newline"]
- [e.g., "Stderr is diagnostics only, not protocol"]

Completion conditions:

- [e.g., "turn/completed → success"]
- [e.g., "subprocess exit → failure"]
- [e.g., "turn timeout → failure"]

#### Timeout and Error Mapping

| Timeout  | Default | Applies To                    |
| -------- | ------- | ----------------------------- |
| `[name]` | [value] | [which phase of the protocol] |

| Error Category | Cause              | System Response        |
| -------------- | ------------------ | ---------------------- |
| `[error_name]` | [what triggers it] | [what the system does] |

---

## 8. Failure Model

### 8.1 Failure Classes

| #   | Class  | Examples           | Detection                   | Recovery             | Escalation                   |
| --- | ------ | ------------------ | --------------------------- | -------------------- | ---------------------------- |
| 1   | [Name] | [What triggers it] | [How the system detects it] | [Automatic recovery] | [When/how human is notified] |
| 2   | [Name] | [Triggers]         | [Detection]                 | [Recovery]           | [Escalation]                 |

### 8.2 Partial State Recovery (Restart)

[What state survives a restart and what must be reconstructed?]

- [e.g., "No retry timers are restored from prior process memory"]
- [e.g., "Service recovers by fresh polling of active items and re-dispatching eligible work"]

### 8.3 Operator Intervention Points

[How operators control behavior without code changes.]

- [e.g., "Edit config file — changes detected and re-applied automatically"]
- [e.g., "Change entity states in external system — running sessions stopped when reconciled"]
- [e.g., "Restart service — for process recovery or deployment"]

---

## 9. Security and Operational Safety

### 9.1 Trust Boundaries

[What is trusted and what is not. Be explicit.]

- [e.g., "The config file is fully trusted configuration"]
- [e.g., "External tracker data is not assumed trustworthy"]

### 9.2 Filesystem Safety

[Mandatory filesystem invariants.]

- [e.g., "Working paths must remain under configured root"]
- [e.g., "Directory names must use sanitized identifiers"]

### 9.3 Secret Handling

- [e.g., "Support $VAR indirection in config"]
- [e.g., "Do not log API tokens or secret values"]
- [e.g., "Validate secret presence without printing them"]

### 9.4 Script/Hook Safety

[If the system executes user-provided scripts or hooks:]

- [e.g., "Hooks are fully trusted configuration"]
- [e.g., "Hook output should be truncated in logs"]
- [e.g., "Hook timeouts are required to avoid hanging the system"]

---

## 10. Observability

### 10.1 Logging Conventions

Required context fields:

- [e.g., `entity_id`, `session_id`]

Message format:

- [e.g., "Stable key=value phrasing"]
- [e.g., "Include action outcome: completed, failed, retrying"]
- [e.g., "Avoid logging large raw payloads"]

### 10.2 Runtime Snapshot

[If the system exposes a monitoring interface, define the snapshot shape.]

```json
{
  "running": [],
  "retrying": [],
  "totals": { "input_tokens": 0, "output_tokens": 0, "seconds_running": 0 }
}
```

### 10.3 Optional HTTP API

[If applicable — endpoints, response shapes, error envelopes.]

---

## 11. Operational Boundaries

> **Important boundary:** [System-level boundary that shapes all design decisions]

> **Important constraint:** [Hard limit — resource, scaling, or architectural]

### Resource Limits and Backpressure

- [e.g., "Maximum N concurrent workers"]
- [e.g., "Backoff formula: min(10000 * 2^(attempt-1), max_backoff_ms)"]

### Scaling Constraints

- [e.g., "Single-process, in-memory state — no distributed coordination"]
- [e.g., "Horizontal scaling requires partitioning by project"]
