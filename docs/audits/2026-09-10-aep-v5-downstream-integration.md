# Downstream v5 integration

Owner-authorized PR creation and merge completed on 2026-09-10. [Provider evidence](evidence/2026-09-10-v5-downstream-integration.json) records PR identities, merge commits and Rewarc checks.

| Project | Target | Merged PRs |
| --- | --- | --- |
| Looplia | develop | [#361](https://github.com/memorysaver/looplia/pull/361), portability follow-up [#362](https://github.com/memorysaver/looplia/pull/362) |
| MITS | main | [#100](https://github.com/memorysaver/MITS/pull/100) |
| Rewarc | main | [#131](https://github.com/memorysaver/Rewarc-AutoResearch/pull/131) |

All original checkouts and fresh GitHub clones passed native integrity/migration verification after integration. Local checkouts now match their remote integration branches. Looplia retains its four original unrelated untracked files; MITS and Rewarc roots are clean. Rewarc's separate OBS worktree still matches the original HEAD and all 13 dirty-file hashes.

Looplia uses squash per its rules and retains the migration branch for source-commit provenance. Its fresh-clone repair moved only one untracked host hash into source-machine observations; the tracked-host negative control still fails. MITS and Rewarc merge commits preserve migration-source ancestry. No Looplia main promotion or production deployment was performed.

Independent review covered migration scope, source reachability and each integration repair. Local gates included Looplia's 31 Worker SQLite tests and complete commit-range Gitleaks; MITS's repository check and complete commit-range Gitleaks; Rewarc's 117 focused tests, 13 safe-loader tests after correction, 16 dependency/formatter tests, and native/context checks. Looplia/MITS have no tracked PR CI; their absent checks are not represented as CI success.

Rewarc final head b300bb2f357580309fbe43d1f0ea4c47bd2c0afa passed all seven remote check runs: correctness, CI required, dependency audit, full-history/tree secrets, Semgrep, SBOM/license inventory and Supply Chain required. Correctness took 40m13s including setup. Earlier failed/superseded CI runs remain historical; final integration waited for the latest head's success. The safe-loader and current-installation manifest-pin repairs did not suppress security checks or change lockfiles.

These integrations enable native context work. Imported completions, owner holds, MITS Layer 19 and Rewarc OBS-001 remain governed by their existing evidence/authorization boundaries. AEP itself remains a local preview build, not a new public release. New observations are retained in the [migration lessons](../lessons/2026-09-10-three-projects-native-v5-migration.md).
