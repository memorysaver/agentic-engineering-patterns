# Record inputs

Native records use `schema_version`, `kind`, `id`, `title`, `status`, `description`, explicit references, and optional `data` for kind-specific content. Inspect a current record with `aep <kind> show <id> --json`; its revision is required by `update --expect`.

For an update, start from the returned `data.record` rather than the response envelope or a partial patch. Preserve its ID, kind, status, creation time and unrelated fields, edit the intended content, and pass `data.revision` to `--expect`. Lifecycle status changes use their dedicated commands.

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

Attribute delivery limits and non-goals to the user's request or applicable project policy. Separate an implementation worker's assignment from the overall requested endpoint.

Accepted or active records retain their historical content. For a later clarification, create a decision with `refs` pointing to the affected story, change or earlier decision, explain the source of the correction, and accept it under the user's authority. Inspect `aep context <story> --json`: it includes accepted decisions that refer back to linked context, together with the original records. For a replacement decision, use `aep decision supersede <old-id> --by <new-id>` after accepting the new record; context retains the superseded history and successor. Reconcile their meaning against the current request; a newer record alone grants no additional authority. These decisions also participate in verification freshness. A changed acceptance contract or implementation scope needs a new change/story and current verification, rather than a contextual decision used to bypass scope checks.
