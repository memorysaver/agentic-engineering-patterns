# System Map Template

Defines the architecture at the module level. Serves two functions: (1) establishes module boundaries so decomposition agents work independently, (2) defines interface contracts so parallel implementation stays compatible.

A module boundary drawn wrong costs more to fix than any implementation bug. Review carefully before proceeding to story decomposition.

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

| Proposed By | Module Affected | Proposed Change | Reasoning | Status |
|------------|----------------|-----------------|-----------|--------|
| | | | | pending/accepted/rejected |

Trigger Architecture Review when: 3+ pending amendments, or any single amendment affects an interface contract.
