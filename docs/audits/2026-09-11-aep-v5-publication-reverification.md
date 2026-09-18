# MITS-108 findings: fixes and installed preview verification

Date: 2026-09-11
Source commit: `005a89ed91558799b20f8c06e04b8dc161db49c7`
Decision: [Publication after integration context changes](../decisions/aep-v5-publication-reverification.md)
Observation: [MITS-108 development](../lessons/2026-09-11-mits108-development-observation.md)

## Changes and checks

- Publication preserves the original integration receipt, rechecks exact integration/candidate Git trees, and binds fresh verification evidence separately. A fixed-build lifecycle reproduces the old failure and verifies recovery without another attempt/PR. The regression also verifies stale review rejection, moved verification checkout rejection, dry-run non-publication, unchanged delivery/story records and the publisher's executable digest.
- Integrated stories expose `candidate_ready` separately from `eligible`; PR/merge and their dry runs give the same closure direction. Local prerequisite checks also reject a missing PR or dirty integration checkout during dry run. Existing provider tree-mismatch tests still block publication.
- Check, review, integration and publication records retain executing binary provenance. Existing historical records are preserved without backfilling invented identities. The same preview version can now be distinguished by per-operation binary digest.
- Shared-store guidance clarifies reconciliation order, contract-directory freshness, retention of observations and inspecting both conflicting story snapshots against actual delivery/attempt evidence. Git transport/conflict resolution remains explicit; no automatic semantic conflict resolver was introduced.

Validation passed: `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, and `cargo test --workspace` (**74 passed**, including 22 lifecycle tests). All four skill checks passed without raising steering ceilings. Official `quick_validate.py` passed for the changed deliver SKILL.md. `git diff --check` passed.

## Actual executable probes

Two copied release binaries ran the same isolated CLI scenario: create/accept a contract, implement and verify in a worktree, perform local integration, add shared context, and run fresh verification at the same candidate.

- Old build `b75080a5edabd5b0`: publication remained blocked; delivery plan and merge dry run claimed eligibility while actual merge failed. Evidence: `~/.local/share/aep/trials/2026-09-11-publication-old-r4/`.
- New build `aec8e775495a49ff`: publication and change closure succeeded with the original receipt unchanged; publication retained both fingerprints, current check ID and actual producer digest. Plan reported integrated/not eligible for another merge; real/dry-run merge consistently blocked. Evidence and retained harness: `~/.local/share/aep/trials/2026-09-11-publication-new/`.
- Copied new binary passed the source-preserving migration, repeat apply, offline embedded guidance and native inspection procedure: `~/.local/share/aep/trials/2026-09-11-publication-preview/`.

All successful probes removed their temporary fixtures. The first three old-binary harness attempts failed on fixture construction (duplicate TOML checks, an overbroad text replacement, then missing change design description); their logs remain in the preceding `publication-old`, `-r2` and `-r3` directories. Those failures were corrected in the harness, not classified as product failures. The Rust regression initially compared the entire query response, including its expected changing store revision; it now compares the retained delivery records and their individual revisions.

The executable probe uses the default self-verification policy; it does not claim an independent model review. Native Rust fixtures exercise explicit independent-review enforcement with structured test responses. No new live-agent trial or macOS runtime test was performed in this correction.

## Installed artifact

- Version: `5.0.0-preview.1`.
- PATH entry: `~/.local/bin/aep`, atomically switched to `~/.local/share/aep/builds/aec8e775495a49ff/bin/aep`.
- Binary SHA-256: `aec8e775495a49ff17294cdd77282e41c4394863e585c3014fc3b8a3f02657d4`.
- Archive SHA-256: `a1e918994395fd50ba0697ca068476cc811ef44576f09abb7a7d9f776f4b903f`; archive contains exactly `aep` and `LICENSE`, and its binary digest matches the installed executable.
- Source identity, installation time, previous binary and trial paths: `~/.local/share/aep/builds/aec8e775495a49ff/manifest.json`.
- Previous build `b75080a5edabd5b0` remains available for rollback.

Herdr reported MITS `w4:p2` idle immediately before installation. Installed PATH/version/digest were verified, then a read-only `deliver plan --story MITS-108` showed its existing integration with `candidate_ready=true`, `integrated=true`, `eligible=false`. MITS records and product files were not changed. This is a local preview reinstall; no public release/tag was created.
