---
name: design
description: Define or revise a change contract, BDD scenarios, and decisions before implementing behavior whose intent or tradeoffs need resolution.
---

# Design

Read the relevant story, accepted contracts, project rules, and ADRs through `aep context <id>` and additional sources selected for the task. Establish the desired outcome, boundaries, failure cases, interfaces, and required verification. A maintenance task can have no journey mapping.

Author one change contract and link its story. Keep design drafts in `docs/design/`, accepted tradeoffs in `project-roadmap/decisions/`, and proposed scenario deltas under the change. Use `aep change new --file <file>` and `aep decision new --file <file>` to create records. Inspect `--help` for required fields and operation syntax.

Read [BDD contracts](references/bdd.md) when writing or modifying specification scenarios. Read [Record inputs](references/records.md) when creating the initial records.

Run `aep spec check --change <id>` and `aep check`. Resolve semantic ambiguity using project evidence and the user when needed; parser success establishes structure only. Accept the contract through `aep change accept <id> --by <actor>` when the current task authority supports that decision. Report unresolved intent without upgrading it to accepted behavior.
