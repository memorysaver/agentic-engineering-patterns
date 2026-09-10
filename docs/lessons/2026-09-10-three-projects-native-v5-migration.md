# Actual v5 migrations: Looplia, MITS and Rewarc

Observed on 2026-09-10 after the owner requested migration branches in the three original repositories. These observations follow the earlier [isolated pilots](../audits/2026-09-10-aep-v5-downstream-pilots.md); they describe real cutover, including project-specific repairs. They do not claim that the unassisted importer performs those repairs.

## Result and evidence boundary

All three originals now select v5 on `migration/aep-v5-preview`. Looplia contains all 416 source stories and 29 custom context artifacts; MITS contains all 107 stories and 41 custom artifacts; Rewarc contains all 96 stories and six custom artifacts. Legacy source data remains available with Git/path/digest provenance. Imported completion remains unverified, and deferred/blocked states survive.

Project-local audits are `docs/audits/2026-09-10-aep-v5-migration.md` in Looplia/MITS and `docs/audits/2026-09-10-aep-v5/migration.md` in Rewarc. The [live migration and executable trial report](../audits/2026-09-10-aep-v5-live-migrations.md) records final revisions, executable identity and actual trial outcomes. The source repository uses `docs/lessons/` for downstream observations; downstream native projects use their configured `lesson-learned/` store.

## 1. An isolated pilot does not establish original-project cutover

The first Rewarc pilot correctly stopped with unresolved context and a live legacy consumer; MITS had seven BDD errors; Looplia had custom artifacts left unmapped. Those were useful observations, not completed migrations. The original runs then explicitly selected every story, bound the actual source commits, resolved each diagnostic and mapped every custom artifact.

The live runs also encountered state absent from the pilot: Looplia's owner edit plus four unrelated untracked files, and Rewarc's unfinished OBS worktree. Project branch, source snapshot, native owner, actual readers, host instructions and validation must be observed separately. Empty `aep check` success cannot prove import coverage.

## 2. Context completeness includes custom semantics and consumer code

Rewarc needed original topology and story extensions available to its capability/gate registry. Copying only standard story fields would have dropped operational meaning. Its native reader preserves historical extensions while overlaying current native lifecycle and dependency fields. Product index, registry, maps, evidence classifications and published specs now have explicit native owners.

Review caught a subtler partial cutover: operational truth used native records, but documentation lifecycle/tasks/spec checks still read the legacy tree. Regression cases now prove that valid stale legacy files cannot hide a bad native purpose, missing native task file or missing published requirement. A timestamp parsing discrepancy also appeared when PyYAML interpreted a native ISO string as a datetime; an isolated loader fixes that without changing global YAML behavior.

The concrete lesson is to inspect executable readers as well as directory output. Context integrity and application-specific semantic integrity are different checks.

## 3. Host roles and active procedures can silently retain v4 writers

Root `AGENTS.md` selection alone left Codex builder/evaluator roles and Looplia prompt aliases capable of issuing old workflow writes. Active host instructions were migrated; exact originals were retained with provenance. Rewarc's project E2E guide also needed native lifecycle commands and an explicit temporary recipe output instead of the legacy default state file.

These repairs retain project independent review and security floors. They do not reinstate a prescribed evaluator model or fixed gen/eval round count. The useful requirement is complete task context and observable verification, with independent review where project policy requires it.

## 4. Historical context links are not writable story scope

Reviewed migration plans initially appended archived change directories to `story.paths`. That field is writable implementation scope, so the addition was semantically wrong despite passing structural checks. Historical references now remain under attributed data, not paths.

Native path matching also uses literal directory prefixes rather than shell globs. Legacy `**`, old product-context paths, conventions and OpenSpec paths therefore needed explicit scope translation. Looplia corrected 196 stories; Rewarc corrected 75; MITS corrected its affected scopes as well. Each correction compared parsed records before/after and proved that fields other than paths were unchanged, including status and original metadata.

The CLI rejects edits to accepted/imported records through ordinary story updates. These corrections were explicitly audited migration-structure repairs with narrow paths-block edits, not lifecycle transitions or fresh implementation acceptance. This friction is a candidate for a dedicated migration correction mechanism; no such command was added in this run.

## 5. A human-owned verification floor must reach executable configuration

Project procedures listed sensitive directories, but initial native `protected_paths` did not cover every inherited floor. Independent review found this in Looplia and Rewarc. All relevant policies were translated into literal prefixes; non-prefix wildcard patterns were conservatively widened to their directory. MITS's Rust check scope also needed a literal `crates/mits` prefix.

Prose alone does not make the CLI derive the intended risk or select conditional checks. Verify the actual configuration and representative paths, while preserving the original policy as the semantic source.

## 6. Source preservation interacts with formatters and Git hooks

Formatting the whole migrated tree would rewrite retained snapshots and invalidate their evidence. Conversely, hiding every legacy/native directory would conceal unrelated new violations. MITS and Rewarc now use non-mutating formatter gates that accept a preexisting violation only when both its current bytes and original Git blob match an exact recorded digest: 86 retained debts in MITS and 45 in Rewarc. New, changed, forged-baseline and unrecognized formatter failures are rejected.

CLI-serialized records and specified exact source copies have explicit exclusions. Commit-hook target exclusions must match them: a native-record-only commit otherwise caused an all-targets-excluded formatter error. Actual commits and an empty-target probe exercised the fix. Source-derived EOF/hardbreak warnings remain labeled rather than silently cleaned.

This is evidence-bound preservation of existing debt, not a claim that raw whole-repository formatting is clean. Future edits to debt files must pass current formatting.

## 7. Migration receipts can resemble secrets

Gitleaks classified some public SHA256 provenance fields, plus one public model literal in Looplia, as secrets. The fixes are finite exact value AND exact generated path exceptions; source values were independently recomputed. There is no generic hash, migration-directory or scanner-rule exemption.

Negative controls matter: changed values and paths still trigger findings, and a synthetic never-issued credential-shaped canary is detected in the same receipt location. A clean scan without that boundary check could have hidden a weakened scanner.

## 8. Normalize contract syntax without inventing new acceptance

MITS's native BDD checks exposed eight scenario-trigger repairs resolving seven diagnostics. Native triggers were clarified while original source text and Then/And assertions stayed intact. Rewarc's unsupported delta `Purpose` was moved intact into design before the committed source snapshot; requirement assertions stayed unchanged.

Both cases require inspecting the source meaning and recording exact before/after transformations. Removing diagnostics, weakening assertions or fabricating a passing product result would not be a migration repair.

## 9. Preserve unfinished work as unfinished

Rewarc OBS-001 had a real feature worktree with 13 dirty-file hashes and an old running autopilot signal. The actual legacy runtime state was paused, its original state retained locally, and native OBS-001 marked blocked for an explicit resume. Its worktree HEAD and dirty bytes were independently rechecked; no merge or duplicate implementation occurred.

Looplia's existing convention edit was captured unchanged in a separate provenance commit. Four unrelated untracked files remained untouched. Migration authorization was sufficient to perform these context operations; it did not choose product owner decisions, approve a candidate or authorize provider spend.

## 10. Verification results must retain their scope

Native check/migration verification passed in all three originals. Additional observed checks included Looplia's actual local Worker SQLite suite (31 tests), MITS's offline Rust help smoke suite (10 tests), its 31-member retained-evidence verifier/self-test and isolated public fixtures, and Rewarc's governance/native-reader/documentation/formatter suite (122 tests, plus 17 focused tests after its final parser change).

These do not verify every imported feature. Looplia's broad typecheck/E2E limitations remain explicit. MITS's full Rust suite, deferred Layer 19 work and model/benchmark downloads were not claimed. Rewarc's blocked OBS dispatch correctly refuses its state and two unverified imported dependencies; a lifecycle command requiring a feature worktree correctly refuses the migration root. No paid provider run or deployment was used to make migration look complete.

## Candidates for future AEP work

Observed recurring gaps suggest targeted improvements: scope/provenance separation during import; detection of active legacy consumer paths; clearer archive mapping support; a migration-aware correction path for immutable records; and visibility into whether human-owned risk floors reached native config. These are grounded follow-up candidates, not accepted changes or extra mandatory workflow phases. The current run repaired the three projects and retained evidence before proposing broader automation.
