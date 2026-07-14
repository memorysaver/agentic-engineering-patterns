# When to Produce a Technical Specification

Consulted by `/aep-map` Step 1 (System Map) when deciding whether the architecture needs a companion Technical Spec before story decomposition.

Suggest producing a Technical Specification (see `templates/technical-spec.md`) when the System Map reveals any of these conditions:

- 3+ interface contracts require multi-step protocol sequences
- The system has 2+ distinct state machines
- There are explicit failure classes with different recovery behaviors
- Trust boundaries cross module lines

The System Map defines WHAT the modules are and HOW they connect. The Technical Spec defines HOW those connections behave under all conditions (success, failure, timeout, recovery). Write the Technical Spec as a standalone document and reference it from the architecture section:

```yaml
architecture:
  technical_spec: "docs/technical-spec.md"
```

See `templates/references/symphony-spec-reference.md` for the exemplar standard.
