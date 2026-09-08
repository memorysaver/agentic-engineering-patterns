---
name: implement
description: Implement an accepted story in an isolated worktree, using its dependencies, project rules, and required validation.
---

# Implement

Read the story and linked change with `aep context <story>`. Check actual readiness with `aep dispatch plan --story <story>`. Resolve missing design through the design skill when the task requires it. Commit the accepted design and dependencies so the base is reproducible.

Create the claim and worktree with `aep dispatch start --story <id> --base <commit> --owner <actor>`. Read the returned worktree, branch, bootstrap, and shared store path. Use the host's agent mechanism when delegation is authorized, binding the worker to that worktree. A prepared claim is not a running agent.

Implement within the declared scope, preserve unrelated work, and inspect results against the change. Use `aep worktree inspect --attempt <id>` before handing off or recovering a worker. The coordinator uses `aep --root <shared-store>` for attempt and verification records; the worker's code stays in its assigned worktree.

Read the validation skill when the implementation is ready for checks and review. Return the actual commit, performed checks, findings, and remaining gaps. Integration follows the user's requested terminal action and the delivery skill.

After the host starts the worker in the recorded worktree, record `aep attempt record <id> --status running`. Record `--status review` when the candidate is ready for review. On resume, inspect `aep attempt status <id>` and `aep worktree inspect --attempt <id>`; reconcile the existing claim with the host before launching another worker.
