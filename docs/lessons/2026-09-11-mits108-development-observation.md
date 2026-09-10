# MITS-108 resumed implementation and delivery observation

Date: 2026-09-11, Asia/Taipei
Status: completed observation; follow-up AEP issues recorded, not fixed in this trial

## Scope and entry

The user asked the observer to “觀察 mits agent 開發108”. Herdr reported the existing MITS Codex agent at `w4:p2` working. The observer sent no prompt and made no downstream modifications. The first read at approximately 01:14 found the downstream turn already about 15 minutes into execution; earlier events below were recovered from its visible recent transcript and retained records, not watched from their start.

The downstream user request was “ok 把108 實作完收掉”, following a progress discussion recommending completion of MITS-108 including merge and a local binary update. The downstream agent chose a new attempt on current main because its earlier attempt had been cancelled. It preserved old work, read native/project guidance and accepted a new `mits108-delivery-endpoint` decision to carry the requested endpoint past the original agent-authored “no merge” wording.

## Implementation and review evidence

- New worktree: `MITS-adoption-finish`; attempt `attempt-18d40440b68c4167-1323d8-0`.
- It replayed the two retained implementation commits onto current main, checked compatibility with MITS-109 and initially passed 23 focused adoption/project-status tests plus a disposable CLI probe.
- Independent reviewer `/root/review_mits109` recorded three material findings in `review-18d4045922c88c89-134cf1-0`: paths inside link titles or malformed label syntax were accepted as routes; a valid title containing parentheses was rejected; HTML-comment-shaped text inside a literal code span was stripped and could fabricate a path.
- The agent paused merge, reproduced failures with tests, repaired Markdown construct handling, retained the failing reproduction and added positive/negative cases. The complete test suite later passed 529 tests with one existing ignored benchmark; 12 reviewer CLI probes passed before the final review was recorded.
- A retained regression log was initially ignored by Git; the agent renamed it to `.txt` and retained it in change evidence rather than dropping the failure.

## Product delivery

The agent used native PR/merge commands. [PR #102](https://github.com/memorysaver/MITS/pull/102) merged candidate `b691d97ddee8490467c939e9edde7a63656f9ef6` at `07654b70a9314b43c00342e61979ebb42daae4d6`. It reconciled shared records onto local main `3b339eded7c34b95e9e9ff04306971c828644418` and ran combined checks. It compared product trees and used range-diff to confirm the retained old changes were represented.

The installed MITS binary was updated with the existing experimental local-embedding feature retained. Its SHA-256 is `54b6c5b2f5255607c3554572ab7bfb60c078636187870e7b07661fc890274a81`. The agent retained the previous executable, rollback tooling and an old-candidate Git bundle under `~/.local/state/mits/binary-backups/20260911-mits108-finish-_nxchd0w/`. It reported adoption audit success while preserving the actual deep-status failure `mits_codex_dogfood_evidence_missing`; product repair was not represented as satisfying absent runtime evidence.

## New AEP recovery gap

Spec publication failed with “Publication requires current integration and verification evidence”. The delivery receipt `delivery-18d404d3352b6fb1-14423f-0` binds fingerprint `814758a4996e75c2cfe2da510c681f2033a1579e132c0fc4a7485300f69e72a1`, while the post-integration plan binds `559e7c937c5bad4f94e48addf3bfddd71aa0429b265feedd21b9364df9361153` at the same candidate head.

The agent reran native verification and obtained a fresh review without modifying historical evidence. Publication still failed because the integrated receipt retained its old fingerprint. `deliver plan` and merge dry run reported readiness; actual `deliver merge` then failed with “Expected one active attempt for MITS-108; found 0”. The agent began inspecting native help and AEP source for the proper recovery path. This is a distinct gap from candidate-only stopping: it continued toward closure but reached inconsistent readiness/execution prerequisites and lacked an obvious way to reattest the already-confirmed integration against updated context.

The observer confirmed that merging the candidate into the shared store introduced five new files under the change directory: delivery-context.md and four resume/review probe artifacts. These paths are included in the runtime's contract-file fingerprint input. Thus unchanged product code alone cannot imply an unchanged fingerprint.

## Build provenance and observation limits

The turn began using AEP build `98689e4053158a96`, as shown by the agent's executable/hash inspection. The observer's previously authorized AEP-fix task installed shared PATH build `b75080a5edabd5b0` at 01:05:56 while this downstream turn was underway. The agent's later reading of the new reflect guidance also confirms it encountered the updated executable. This creates a mixed-build observation and must be retained as environmental context, not silently attributed to agent behavior.

Read-only comparison on the same post-integration MITS snapshot produced different fingerprints: old build `bcd48812fe8c6dce48885ed59866fdc50a7c64745e0f857241332e412d690ca2`, new build `559e7c937c5bad4f94e48addf3bfddd71aa0429b265feedd21b9364df9361153`. Neither equals the integration receipt. The old/new comparison demonstrates the intended decision-context change but does not by itself establish that the PATH update caused the publication failure; shared-store contract inputs also changed on integration. Future controlled trials should retain a fixed executable identity through a turn or explicitly record a cutover.

## Native recovery selected by the agent

After inspecting CLI help and AEP source, the agent used `story reopen` and created a separate `MITS-adoption-close` worktree/attempt. It explicitly retained PR #102's merge history and left product code unchanged. It reran native verification and requested independent review for the closure candidate. This is an observed recovery path, not evidence that direct integrated-receipt refresh exists. The extra claim/worktree/check/review cycle is a real cost of the current lifecycle.

The closure attempt `attempt-18d40531fbbf6547-14cd8e-0` obtained fresh checks/review and opened [PR #103](https://github.com/memorysaver/MITS/pull/103), containing work records/evidence rather than product-code changes. It merged at `a1bb2f37e2361209a2a0152f1e80667a4eb07924`. With the new delivery binding, `spec publish` and `change close` succeeded. Reconciling the shared ledger with remote main again produced a conflict in the story snapshot; the agent retained its current integration record, completed the merge and pushed main to `837e1d4` at that observation point. No observer intervention was needed for the recovery.

## Final state and retained evidence

The downstream turn completed in 24 minutes 37 seconds, and Herdr reported the agent idle. Read-only checks independently confirmed both PRs merged, MITS-108 `integrated`, change `adoption-indexed-guidance` `closed`, and publication evidence bound to the closure delivery `delivery-18d40558d009f2c3-1519e7-0`. The original PR #102 receipt remains referenced by the story. Main and origin/main both point to `7af2d5ba96ef042c005ee3139dac863bf8a01719`, with a clean working tree and only the main worktree remaining. The installed binary digest matches the value above.

The agent retained its own lesson at `lesson-learned/observations/mits108-delivery-closure.md`, committed cleanup evidence, and wrote/synchronized a MITS progress memory. The visible memory command reported success; the observer did not independently inspect the memory service. The 529/0/1 test result is retained downstream execution evidence, not a second test run by the observer. Missing selected Codex checkpoint evidence remains an explicitly reported limitation.

Raw transcript snapshots, the old/new build fingerprint comparison, final native records, PR states, Git state and copied downstream evidence are retained locally under `~/.local/share/aep/trials/2026-09-11-mits108-development/`, with SHA-256 inventory in `manifest.json`.

## Lessons and follow-up work

- Delivery continuity worked in this run: the agent continued beyond implementation and verification through merge, publication, closure, installation, cleanup and memory synchronization without an observer prompt.
- Independent review found three meaningful parser errors despite passing initial checks. Counterexamples and positive controls materially improved the implementation; the failing reproduction was preserved.
- AEP readiness and execution prerequisites need to agree. A ready plan/dry run followed by an active-attempt error is an actionable UX/contract gap.
- Integration can introduce previously absent shared context and invalidate publication evidence. Reproduce this under one fixed AEP build before choosing between context stabilization and a supported reattestation path; preserve historical delivery identity and fresh verification requirements either way. The observed reopen/extra-PR recovery succeeded but imposed substantial repeated work.
- Shared-ledger reconciliation produced a story-state conflict during closure. Review whether the lifecycle can avoid requiring manual conflict resolution for records it owns.
- Build provenance needs to cover the whole turn. The downstream lesson records only the starting AEP digest `98689e4053158a96…`, despite later commands encountering the new build. Keep the mixed-build limitation attached to this observation; a starting hash alone cannot establish which executable produced every receipt.

This observation changed only AEP documentation and local observation artifacts. It did not change MITS files, send instructions to its agent, or implement the newly recorded AEP fixes.
