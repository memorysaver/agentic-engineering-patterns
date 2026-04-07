# Symphony SPEC.md — Specification Writing Reference

This document extracts reusable documentation patterns from OpenAI's [Symphony SPEC.md](https://github.com/openai/symphony/blob/main/SPEC.md), a ~15,000-word language-agnostic specification for a coding agent orchestration service. Symphony's spec is notable because it is precise enough that any coding agent can implement it in any programming language without clarifying questions.

Use this reference when writing specifications for systems with protocol-level complexity. The patterns here are the standard to aim for.

---

## Source Structure

Symphony's SPEC.md has 15 sections. Each maps to an AEP template:

| #   | Symphony Section                    | What It Does                                                           | AEP Template                                                 |
| --- | ----------------------------------- | ---------------------------------------------------------------------- | ------------------------------------------------------------ |
| 1   | Problem Statement                   | Defines what the service IS, what problems it solves, what it is NOT   | `context-document.md` (Problem Statement)                    |
| 2   | Goals and Non-Goals                 | Behavior-observable goals, explicit non-goals with reasoning           | `context-document.md` (Goals/Non-Goals)                      |
| 3   | System Overview                     | Named components + abstraction layers + external deps                  | `system-map.md` (Modules)                                    |
| 4   | Core Domain Model                   | Typed entity fields, defaults, normalization rules, stable identifiers | `system-map.md` (Domain Model)                               |
| 5   | Workflow Specification              | Repository contract with schema, validation, dynamic reload            | `technical-spec.md` (Configuration)                          |
| 6   | Configuration Specification         | Source precedence, typed getters, dynamic reload, config cheat sheet   | `technical-spec.md` (Configuration)                          |
| 7   | Orchestration State Machine         | Named states, transition triggers, idempotency rules                   | `technical-spec.md` (State Machines)                         |
| 8   | Polling, Scheduling, Reconciliation | Poll loop, candidate selection, concurrency, retry/backoff             | `technical-spec.md` (Protocol Specs)                         |
| 9   | Workspace Management and Safety     | Filesystem lifecycle, hooks, safety invariants                         | `technical-spec.md` (Security)                               |
| 10  | Agent Runner Protocol               | Launch contract, handshake with JSON transcripts, streaming            | `technical-spec.md` (Protocol Specs)                         |
| 11  | Issue Tracker Integration           | Adapter contract, query semantics, normalization rules                 | `system-map.md` (Interface Contracts)                        |
| 12  | Prompt Construction                 | Template rendering, retry semantics, failure handling                  | `technical-spec.md` (Protocol Specs)                         |
| 13  | Logging, Status, Observability      | Structured logs, runtime snapshots, optional HTTP API                  | `technical-spec.md` (Observability)                          |
| 14  | Failure Model                       | 5 failure classes, per-class recovery, restart recovery                | `context-document.md` (Failure Model) / `technical-spec.md`  |
| 15  | Security and Operational Safety     | Trust boundaries, filesystem safety, secret handling                   | `context-document.md` (Security Model) / `technical-spec.md` |

---

## Extracted Patterns

### Pattern 1: Problem-First Framing

Symphony opens with a single sentence saying what the service IS, then lists exactly 4 operational problems it solves, then states what it is NOT.

**How Symphony does it:**

> "Symphony is a long-running automation service that continuously reads work from an issue tracker, creates an isolated workspace for each issue, and runs a coding agent session for that issue inside the workspace."
>
> The service solves four operational problems: [enumerated list]
>
> Important boundary: Symphony is a scheduler/runner and tracker reader. Ticket writes are performed by the coding agent.

**Why it works:** An agent reading this spec knows in 3 paragraphs exactly what to build and — critically — what NOT to build. The "important boundary" prevents the most common scope creep.

**Use in AEP:** `context-document.md` Problem Statement section. Ensure every problem statement includes a "what it is NOT" boundary.

---

### Pattern 2: Behavior-Observable Goals with Explicit Non-Goals

Goals are statements that can be verified by observing the running system. Non-goals are things a reasonable person might expect but the system deliberately excludes.

**How Symphony does it:**

Goals:

- "Poll the issue tracker on a fixed cadence and dispatch work with bounded concurrency."
- "Create deterministic per-issue workspaces and preserve them across runs."
- "Recover from transient failures with exponential backoff."

Non-Goals:

- "Rich web UI or multi-tenant control plane."
- "General-purpose workflow engine or distributed job scheduler."

**Why it works:** Each goal is testable — you can observe the system and confirm it does (or doesn't do) the thing. Non-goals prevent agents from gold-plating.

**Use in AEP:** `context-document.md` Goals/Non-Goals section. Distinct from "In Scope / Out of Scope" — scope defines what the system does, goals define how you know it's working.

---

### Pattern 3: Typed Entity Definitions with Normalization Rules

Every domain entity has typed fields with defaults, and normalization rules that prevent ambiguity.

**How Symphony does it:**

```
Issue:
  id (string) — Stable tracker-internal ID
  identifier (string) — Human-readable ticket key (example: ABC-123)
  priority (integer or null) — Lower numbers are higher priority
  labels (list of strings) — Normalized to lowercase
  blocked_by (list of blocker refs) — Each contains id, identifier, state
```

Normalization rules:

- "Workspace Key: Derive from issue.identifier by replacing any character not in [A-Za-z0-9._-] with \_"
- "Normalized Issue State: Compare states after lowercase"

**Why it works:** No ambiguity about what a field contains, what type it is, what the default is, or how to compare values. A coding agent in any language can implement this without guessing.

**Use in AEP:** `system-map.md` Domain Model section. Replace unstructured "Key internal concepts" bullet lists with typed field tables.

---

### Pattern 4: State Machine Documentation

Stateful entities get explicit state diagrams with named states, transition triggers, and recovery rules.

**How Symphony does it:**

Orchestration states (distinct from tracker states):

1. Unclaimed — not running, no retry scheduled
2. Claimed — reserved to prevent duplicate dispatch
3. Running — worker task exists
4. RetryQueued — worker not running, retry timer exists
5. Released — claim removed

Transition triggers: Poll Tick, Worker Exit (normal), Worker Exit (abnormal), Retry Timer Fired, Reconciliation State Refresh, Stall Timeout.

"Important nuance: A successful worker exit does not mean the issue is done forever."

**Why it works:** Every state is named, every transition has a trigger, and easy-to-miss nuances are called out explicitly.

**Use in AEP:** `technical-spec.md` State Machines section. Also useful in `system-map.md` when a module owns a state machine.

---

### Pattern 5: Protocol Specs with Illustrative JSON Transcripts

Multi-step interactions get exact handshake sequences with example payloads.

**How Symphony does it:**

```json
{"id":1,"method":"initialize","params":{"clientInfo":{"name":"symphony","version":"1.0"},"capabilities":{}}}
{"method":"initialized","params":{}}
{"id":2,"method":"thread/start","params":{"approvalPolicy":"...","sandbox":"...","cwd":"/abs/workspace"}}
{"id":3,"method":"turn/start","params":{"threadId":"<thread-id>","input":[{"type":"text","text":"<rendered prompt>"}]}}
```

Each step includes: what to send, what to expect back, timeout behavior, error mapping.

**Why it works:** An implementor can literally trace through the JSON transcript to verify their implementation. No prose interpretation needed.

**Use in AEP:** `system-map.md` Protocol Sequences section and `technical-spec.md` Protocol Specifications section.

---

### Pattern 6: "Important Boundary" / "Important Nuance" Callouts

Inline blockquote markers flag precision points that are easy to miss.

**How Symphony does it:**

> **Important boundary:** Symphony is a scheduler/runner and tracker reader. Ticket writes are typically performed by the coding agent.

> **Important nuance:** A successful worker exit does not mean the issue is done forever. The orchestrator schedules a short continuation retry.

**Why it works:** Agents scan documents for actionable information. These callouts are semantic anchors that prevent the most common implementation mistakes. They stand out visually and can be grep'd.

**Use in AEP:** Convention across all templates — `system-map.md`, `technical-spec.md`, and anywhere precision matters.

---

### Pattern 7: Agent-Friendly Redundancy (Config Cheat Sheet)

Symphony includes a section explicitly labeled "intentionally redundant" that summarizes all configuration in one flat table.

**How Symphony does it:**

> "This section is intentionally redundant so a coding agent can implement the config layer quickly."
>
> - `tracker.kind`: string, required, currently `linear`
> - `polling.interval_ms`: integer, default `30000`
> - `workspace.root`: path, default `<system-temp>/symphony_workspaces`
>   [... all fields in one list]

**Why it works:** Agents pay a cognitive/context tax for cross-referencing. A redundant summary eliminates cross-referencing entirely for the most common implementation task (reading config).

**Use in AEP:** `story-spec.md` Implementation Cheat Sheet section. Applied at the story level (not product level) because the relevant subset varies per story.

---

### Pattern 8: Enumerated Failure Classes with Per-Class Recovery

Failures are taxonomized into classes, each with detection method and recovery behavior.

**How Symphony does it:**

5 failure classes:

1. Workflow/Config Failures — missing files, invalid YAML, missing credentials
2. Workspace Failures — directory creation, hook timeout, invalid paths
3. Agent Session Failures — handshake failure, turn timeout, subprocess exit
4. Tracker Failures — API errors, non-200 status, malformed payloads
5. Observability Failures — snapshot timeout, dashboard render errors

Each class has explicit recovery: "Dispatch validation failures: Skip new dispatches. Keep service alive. Continue reconciliation."

**Why it works:** Undocumented failure modes become undocumented bugs. By enumerating failure classes early, every implementor handles the same set of failure scenarios the same way.

**Use in AEP:** `context-document.md` Failure Model section (product-level) and `technical-spec.md` Failure Model section (implementation-level).

---

### Pattern 9: Trust Boundary Documentation

Security is not a checklist — it's a boundary declaration about what is trusted and what is not.

**How Symphony does it:**

> "Each implementation defines its own trust boundary."
>
> "Implementations should state clearly whether they are intended for trusted environments, more restrictive environments, or both."
>
> "Hooks are fully trusted configuration. Hooks run inside the workspace directory."

Filesystem safety is mandatory:

- Workspace path must remain under configured workspace root
- Coding-agent cwd must be the per-issue workspace path
- Workspace directory names must use sanitized identifiers

**Why it works:** Rather than prescribing specific security controls, Symphony forces each implementation to explicitly declare its trust posture. This prevents the worst outcome: implicit trust assumptions that no one documented.

**Use in AEP:** `context-document.md` Security Model section (trust boundaries and auth) and `technical-spec.md` Security section (filesystem safety, secret handling).

---

## Key Principle

The overarching principle from Symphony's approach:

> **Define WHAT the system does and HOW it behaves under all conditions. Let implementors decide the programming language, framework, and internal architecture.**

This is what separates a specification from documentation. Documentation describes what was built. A specification defines what must be built — precisely enough that the builder needs no clarifying questions.
