# Data Model Brief: [project name]

## Product Identity

- **What:** [from opportunity.bet + product.problem]
- **Domain:** [the real-world domain this models]

## Current Domain Model

[from architecture.domain_model, listed with key fields]

| Entity   | Key Fields            | Purpose              |
| -------- | --------------------- | -------------------- |
| [entity] | [field1, field2, ...] | [what it represents] |

## Language Mismatch

[from /reflect observation or ambient detection]

- **Code says:** [term] → **Team says:** [term]
- **Entity [X]** has fields [a, b, c] → but the domain concept is really [...]
- **Relationship confusion:** [X] owns [Y]? Or [Y] references [X]?

## Schema/Migration State

[scan ORM models, migration files, or schema definitions]

- Current ORM: [from product.constraints]
- Migration count: [approximate]
- Known schema debt: [any migrations that feel wrong]

## Questions for Calibration

1. When you talk about [entity] in conversation, what word do you use?
2. Does [field_name] mean what you think it means? What would you call it?
3. Are [Entity A] and [Entity B] actually the same thing with different lifecycle stages?
4. What's the relationship between [X] and [Y] — ownership, reference, or derivation?
5. Are there domain terms the code doesn't model yet but should?
6. What invariants must always hold? (e.g., "A completed [entity] always has a non-null [field]")

---

## Extraction Map

| Brief section | Source                                                |
| ------------- | ----------------------------------------------------- |
| Domain model  | `architecture.domain_model`                           |
| Entity fields | `architecture.domain_model[].fields`                  |
| Normalization | `architecture.domain_model[].normalization_rules`     |
| Schema files  | File scan: ORM models, migrations, schema definitions |
