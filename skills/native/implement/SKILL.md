---
name: implement
description: Select ready work and implement an accepted story in its worktree, preserving task context, verification, and recovery information.
---

# Implement

Read the story and linked change with `aep context <story>`. Check readiness with `aep dispatch plan --story <story>`. Select work using requested priority, value, dependency unlocks, and uncertainty; CLI readiness establishes mechanical prerequisites. Resolve missing intent through `aep --skill design` when needed.

Read [Task handoff](references/handoff.md) when assembling context, starting work or resuming it; read [Git and worktree operations](references/git.md) for base selection, conflicts or recovery. Commit accepted design and dependencies so the chosen base is reproducible.

Create the claim/worktree with `aep dispatch start --story <id> --base <commit> --owner <actor>`. Inspect its returned branch, bootstrap, and shared store path. Work there yourself or use the host's agent mechanism when delegation is authorized. A prepared claim reports preparation only.

Keep code in the assigned worktree and use `aep --root <shared-store>` for coordinated attempt/verification records. Record `aep attempt record <id> --status running` once work actually starts; record `--status review` when the candidate is ready for validation. On resume, inspect `aep attempt status <id>` and `aep worktree inspect --attempt <id>` before reconciling an existing claim with the host.

Exercise changed behavior during implementation using the project's verification procedure. Read `aep --skill validate` to establish completion evidence. Handoff the actual commit, performed checks, findings and remaining gaps; use `aep --skill deliver` for the requested integration action.
