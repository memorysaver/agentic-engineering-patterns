# Downstream v5 integration

Owner-authorized PR creation and merge completed on 2026-09-10. [Provider evidence](evidence/2026-09-10-v5-downstream-integration.json) records PR identities, merge commits and Project C checks.

| Project | Target | Merged PRs |
| --- | --- | --- |
| Project B | develop | [#361](https://github.com/memorysaver/project-b/pull/361), portability follow-up [#362](https://github.com/memorysaver/project-b/pull/362) |
| Project A | main | [#100](https://github.com/memorysaver/Project A/pull/100) |
| Project C | main | [#131](https://github.com/memorysaver/Project C/pull/131) |

All original checkouts and fresh GitHub clones passed native integrity/migration verification after integration. Local checkouts now match their remote integration branches. Project B retains its four original unrelated untracked files; Project A and Project C roots are clean. Project C's separate OBS worktree still matches the original HEAD and all 13 dirty-file hashes.

Project B uses squash per its rules and retains the migration branch for source-commit provenance. Its fresh-clone repair moved only one untracked host hash into source-machine observations; the tracked-host negative control still fails. Project A and Project C merge commits preserve migration-source ancestry. No Project B main promotion or production deployment was performed.

Independent review covered migration scope, source reachability and each integration repair. Local gates included Project B's 31 Worker SQLite tests and complete commit-range Gitleaks; Project A's repository check and complete commit-range Gitleaks; Project C's 117 focused tests, 13 safe-loader tests after correction, 16 dependency/formatter tests, and native/context checks. Project B/Project A have no tracked PR CI; their absent checks are not represented as CI success.

Project C final head b300bb2f357580309fbe43d1f0ea4c47bd2c0afa passed all seven remote check runs: correctness, CI required, dependency audit, full-history/tree secrets, Semgrep, SBOM/license inventory and Supply Chain required. Correctness took 40m13s including setup. Earlier failed/superseded CI runs remain historical; final integration waited for the latest head's success. The safe-loader and current-installation manifest-pin repairs did not suppress security checks or change lockfiles.

These integrations enable native context work. Imported completions, owner holds, Project A Layer 19 and Project C OBS-001 remain governed by their existing evidence/authorization boundaries. AEP itself remains a local preview build, not a new public release. New observations are retained in the [migration lessons](../lessons/2026-09-10-three-projects-native-v5-migration.md).
