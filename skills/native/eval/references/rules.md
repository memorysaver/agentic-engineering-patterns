# Rules

`aep eval report` applies these rules to the latest snapshot of a run. Severity: `warn` means a structural gap that undermines evidence; `advice` means a practice worth adopting; `info` is context. Every rule reads structure only; none judges what the project chose to build.

| Id | Practice | Fires when | Does not claim |
| --- | --- | --- | --- |
| VER-001 | verification | `.aep/config.toml` has no checks | that the project has no tests; only that AEP cannot record them |
| VER-002 | verification | checks exist but `required_checks` is empty | that deliveries skipped checks; only that policy does not require them |
| VER-003 | verification | policy requires independent review and every review is `host_reported` | that reviews were wrong; only that independence is not established |
| VER-004 | verification | stale evidence is attached to active stories | anything about the code itself |
| REC-000 | records | `aep check` fails | — |
| REC-001 | records | imported stories remain unreconciled more than 14 days after migration | that the imported work is wrong; only that its completion is unverified |
| REC-002 | records | fewer than half of the last 30 days' events carry a note (only with 5 or more events) | — |
| REC-003 | records | a layer or wave has no description | — |
| REC-004 | records | imported decisions are still pending | — |
| DEL-001 | delivery | a closed change has no published specification | — |
| DEL-002 | delivery | a story is integrated without a delivery receipt | — |
| GIT-001 | git | the oldest unpushed commit is more than 3 days old | that the project's branch policy requires pushing; check the project's own Git rules |
| GIT-002 | git | uncommitted files exist at snapshot time | — |
| GIT-003 | git | a running attempt is more than 7 days old | — |
| GIT-004 | git | an attempt worktree has no running attempt record | — |
| LEG-001 | legacy | the project is migrated but still carries v4 skills or the AGENTS.md route | — |
| LEG-002 | legacy | `aep migrate verify` fails | — |
| LEG-003 | legacy | AGENTS.md does not point at `aep --skill` | — |
| OPS-001 | devops | no workflow under `.github/workflows` | that CI is required; only that checks run solely when an agent runs them |
| OPS-002 | devops | workflows exist but run none of the configured checks | — |
| OPS-003 | devops | no secret scanning in checks, hooks or CI | — |
| OPS-004 | devops | no dependency lockfile at the root | — |
| OPS-005 | devops | no `CHANGELOG.md` | — |

Thresholds (14 days, 3 days, 7 days, half of recent events) are fixed in this version. A finding is a prompt to look, not a verdict: the observer checks the evidence path before repeating it, and the project's own rules decide whether a practice applies.

Facts behind the rules are in the run's `snapshot.json`: record counts by kind and status, event note ratios, verification policy and review attribution, delivery and release counts, Git branch, dirty and unpushed state, worktrees and attempts, legacy residue and migration verification, CI workflows, whether they run the configured checks, secret scanning, lockfiles and changelog. The observer can cite them by path.
