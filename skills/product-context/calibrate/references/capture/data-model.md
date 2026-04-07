# Data Model — Capture Questions

Ask these questions one at a time during `/calibrate capture` for data-model.

## Questions

1. **Entity naming:** For each entity, what's the name the team actually uses in conversation? Map code name → human name.
2. **Field naming:** For fields that feel wrong, what should they be called? (provide old name → new name)
3. **Relationships:** For each entity pair, what's the relationship? (owns, references, derives from, contains)
4. **Lifecycle stages:** Are there entities that represent the same concept at different stages? Should they be unified or kept separate?
5. **Invariants:** What must always be true? (e.g., "A completed X always has a non-null Y", "An X cannot exist without a parent Z")
6. **Normalization rules:** How should values be compared and stored? (e.g., "identifiers lowercased", "dates always UTC")
7. **Missing entities:** Are there domain concepts the code doesn't model yet but should?

## Output

Update `architecture.domain_model` in `product-context.yaml` with calibrated entity names, fields, relationships, and invariants.
