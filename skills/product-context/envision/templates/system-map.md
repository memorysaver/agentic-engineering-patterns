# System Map Template

Defines the architecture at the module level. Serves two functions: (1) establishes module boundaries so decomposition agents work independently, (2) defines interface contracts so parallel implementation stays compatible.

A module boundary drawn wrong costs more to fix than any implementation bug. Review carefully before proceeding to story decomposition.

### Callout Conventions

Use these blockquote markers throughout this document to flag precision points that agents must not miss:

> **Important boundary:** Where a responsibility stops and another begins
> **Important nuance:** Easy-to-miss detail that changes implementation
> **Important constraint:** Hard limit that shapes design choices

---

## System Overview

**Architecture style**: [e.g., microservices, modular monolith, serverless functions]

**High-level description**: [2–3 sentences on structure and why this architecture was chosen, referencing Context Document constraints.]

---

## Modules

### [Module Name]

**Responsibility**: [What this module does and does not do. The "does not" part defines the boundary.]

**Owns**: [Data, state, or resources this module is the authority on. No other module directly modifies these.]

**Depends on**: [Other modules this one calls or consumes from.]

**Technology**: [If different from default stack.]

**Key internal concepts**: [Domain objects, patterns, or abstractions implementers need to understand.]

[Repeat for each module]

---

## Domain Model

Domain entities that span module boundaries or require precise typing. Module-specific concepts remain in the Modules section above. Each entity has typed fields so implementers in any language know exactly what to build. See `references/symphony-spec-reference.md` Pattern 3 for the standard.

### [Entity Name]

**Purpose**: [One sentence — what this entity represents in the system.]

**Fields**:

| Field    | Type   | Default   | Required | Notes                                        |
| -------- | ------ | --------- | -------- | -------------------------------------------- |
| `id`     | string | —         | yes      | Stable across restarts. Derived from [rule]. |
| `status` | enum   | `pending` | yes      | See state machine if applicable.             |

**Normalization rules**:

- [e.g., "All identifiers are lowercased and slugified"]
- [e.g., "Replace characters not in [A-Za-z0-9._-] with \_"]

**Invariants**:

- [Conditions that must always hold, e.g., "A completed entity always has a non-null completed_at timestamp"]

[Repeat for each domain entity]

---

## Interface Contracts

For every module-to-module connection. These will be enforced by automated contract tests in Phase 4. An undefined interface is a guaranteed integration failure.

### [Module A] → [Module B]

**Protocol**: [HTTP REST, gRPC, message queue, function call, etc.]

**Endpoint / Channel**: [Specific API path, queue name, or function signature.]

**Request shape**:

```
[Exact data structure — TypeScript types, JSON Schema, or equivalent. Specify required vs optional, types, constraints.]
```

**Response shape**:

```
[Same specificity as request.]
```

**Error contract**:

```
[What errors can be returned, their shape, what the caller should do for each.]
```

**SLA**: [Expected latency, throughput, availability. "TBD" is acceptable if noted as open question.]

---

## Protocol Sequences

For interface contracts that involve multi-step interactions (handshakes, streaming, request-response chains), document the sequence here. Simple request-response contracts don't need this — use it when the interaction has ordering, state, or timing constraints. See `references/symphony-spec-reference.md` Pattern 5 for the standard.

### [Protocol Name]: [Module A] <> [Module B]

**Trigger**: [What initiates this protocol]

**Sequence**:

1. [Module A] sends [message type]:
   ```json
   { "type": "init", "payload": { "..." } }
   ```
2. [Module B] responds with [message type]:
   ```json
   { "type": "ack", "session_id": "..." }
   ```
3. [Steady-state interaction description]

**Timeout behavior**: [What happens if step N takes too long]
**Error behavior**: [What happens if step N fails]

> **Important nuance:** [Easy-to-miss detail about this protocol]

---

## Data Flow

For each primary user journey in the Layered MVP Contract, trace the data path:

### [Journey Name]

```
User → [Module] → action → [Module] → action → response
```

Show which module handles each step, what data passes between them, where state is persisted.

---

## Third-Party Boundaries

### [Service Name]

**Provides**: [Specific capability used.]
**Integration point**: [Which module, how.]
**Failure mode**: [Behavior when service is down — graceful degradation or hard fail?]
**Limitations**: [Rate limits, quotas, latency.]

---

## Deployment Topology

**Environments**: [Local dev, staging, production.]
**Module → Runtime mapping**: [Which modules run where.]
**Persistence**: [Databases/storage, which modules own them.]

---

## Architecture Decision Records

### ADR-001: [Title]

**Context**: [What prompted this decision.]
**Decision**: [What was decided.]
**Reasoning**: [Why, over alternatives.]
**Consequences**: [What this enables and constrains.]

---

## Amendment Log

During story decomposition, agents may discover boundary or contract issues. Collected here, reviewed in batch.

| Proposed By | Module Affected | Proposed Change | Reasoning | Status                    |
| ----------- | --------------- | --------------- | --------- | ------------------------- |
|             |                 |                 |           | pending/accepted/rejected |

Trigger Architecture Review when: 3+ pending amendments, or any single amendment affects an interface contract.
