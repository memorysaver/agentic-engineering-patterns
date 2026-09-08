# Record inputs

Native records use `schema_version`, `kind`, `id`, `title`, `status`, `description`, explicit references, and optional `data` for kind-specific content. Inspect a current record with `aep <kind> show <id> --json`; its revision is required by `update --expect`.

Create the change before its story so the story can reference an existing ID. A minimal change input is:

```yaml
kind: change
id: C-42
title: Describe the intended behavior
description: Outcome, scope, constraints, and verification reasoning.
data:
  specs:
    - capability: example
      path: project-ledger/changes/C-42/specs/example/spec.md
      baseline: null
```

Create the declared delta file, validate, and accept the change under the task's authority. Then create its story:

```yaml
kind: story
id: S-42
title: Implement the accepted behavior
description: Implementation scope and explicit non-goals.
change: C-42
paths: [apps/example]
required_checks: [tests]
```

Configure the named checks in `.aep/config.toml` with explicit command argument arrays. Acceptance prose belongs in the change and scenarios; the story supplies work organization and references. IDs remain stable across layers, waves, releases, and attempts.

A story may add `changes: [other-contract]` alongside its primary `change`. All linked contracts must be accepted. Release records can select story IDs through `refs`; their `required_gates` govern promotion. Story/layer/wave prerequisite gates govern dispatch.
