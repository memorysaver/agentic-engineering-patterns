# Context sources

Build enough context to explain this task's purpose, present behavior, constraints, observable success, and continuity. Prefer a source index with identified gaps over copying the repository into another document.

| Question | Useful sources | Evidence to retain |
| --- | --- | --- |
| Why this work? | Request, roadmap, journey, story/change | Outcome, scope, authority, acceptance source |
| What happens now? | Relevant code/tests, actual runtime, diff, Git history | Observed behavior and base/head; separate intent from implementation |
| What constrains it? | Indexed rules, ADRs, interface contracts, user's criteria | Applicable scope, source revision, unresolved conflicts |
| How can it be checked? | Local verification skill, fixtures, checks, environment | Operation, expected result, observation method, missing access |
| What already happened? | Dependencies, attempts/worktrees, checks, lessons | Current work, valid evidence, unknowns and recovery point |

`aep context <id>` follows explicit record links. Add relevant files using `--source <path>` and inspect runtime/source separately. Its output is a bounded packet, not a completeness verdict. A statement such as “implemented” needs code or runtime evidence beyond roadmap status.

When a fact changes a decision, keep its source and revision or observation time with the task context. Persist reusable constraints in the indexed rules or ADRs; preserve task-specific unknowns with the change/handoff. Existing records and a short note usually suffice; a second progress database adds another writer to reconcile.

For setup, discover the project's own bootstrap/check commands before adding tools. Record required tool versions, working directory, environment variable names, seed/reset method, and isolation needs. Keep secrets in the project's established secret mechanism. Distinguish a missing dependency or service from a product defect by completing preflight before interpreting a failed journey.
