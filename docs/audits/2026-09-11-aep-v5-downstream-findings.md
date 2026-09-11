# Verification of fixes from downstream delivery

Date: 2026-09-11
Implementation commit: `2d5d81915301b1f01176d7b1937086f0498fa08b`
Decision: [Corrections from the live downstream delivery trial](../decisions/aep-v5-downstream-delivery-findings.md)

## Changes and observed results

| Finding | Correction | Verification |
| --- | --- | --- |
| Project verification skill appeared to be an AEP dependency | Human catalog groups built-in and project procedures, with project source paths; exact named skill content and structured metadata preserved | Human CLI fixture plus copied binary reading the real MITS catalog and exact verify-mits bytes |
| A decision referring to an immutable story was missing from story context/freshness | Shared incoming-decision traversal includes accepted decisions and superseded history; existing outgoing links preserved | Actual CLI lifecycle checks for discoverability, unchanged code head, stale evidence, correction chains, unrelated records and native supersession |
| Corrective story update failed without useful next step | Update diagnostic and records reference explain linked decisions for contextual corrections and new contracts for changed scope | Existing immutable story remains unchanged after failed update; linked decision becomes visible |
| Shared ledger copied into candidate triggered scope rejection and later integration conflicts | Git/closure guidance explains shared-root transport and evidence-based reconciliation | Same fixture verifies separate-store checks succeed, while a copied out-of-scope ledger file still blocks verification |
| Publisher added an EOF blank line | Requirement separators and final newline rendered separately; prefix/suffix and no-op bytes preserved | Spec tests cover creation, multiple requirements, no-op historical bytes and preserved suffix; actual published spec passes staged Git whitespace check |
| Reflect-generated skills needed a unified discovery path | Feedback reference now describes reusable local skill creation, applicability metadata, discovery and representative-use validation | Embedded reference output tests; local catalog fixture and installed-product inspection; no new semantic search claim |

Accepted and superseded decisions are retained as context, not interpreted as automatic authority or an automatic conflict-resolution policy. Adding or changing relevant decision context intentionally makes old verification evidence stale. Operational receipts and unlinked retrospective lessons do not become new inputs merely by referring to the story.

## Independent review and resolved findings

`/root/review_v5_downstream_findings` independently reproduced the original missing incoming-decision edge using a disposable Git repository and real CLI calls. After the first implementation, it found a material boundary case: native `decision supersede D --by D2` removed the sole incoming accepted edge, which could restore the pre-decision fingerprint and revive old evidence. Retaining superseded anchors fixed the problem. A native supersession regression checks both historical/successor visibility and that the old fingerprint does not return. The reviewer reran its independent reproduction and confirmed the fix.

The reviewer also identified that the dispatch fixture initially lacked both its new story and decision in the base. The fixture now commits the story before adding the decision, isolating the missing-decision requirement. Follow-up review reported no remaining material findings. Disposable reviewer fixtures were cleaned. No downstream mutations were performed for review.

## Checks

- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: 72 passed, 0 failed. Includes 20 lifecycle tests and seven human CLI tests.
- Four required skills checks passed: generated consistency, vocabulary, steering and package validation. Steering reran after the supersession wording change; no ceiling increase.
- Official skill quick validation passed for native root/design/implement/deliver/reflect using isolated `uv run --with pyyaml`.
- `cargo build --locked --release -p aep-cli`: passed.
- Copied executable passed `scripts/verify-native-preview.py`: source-preserving migration, repeated apply, offline guidance and native inspection; fixture removed.
- Read-only old/new binary comparison against MITS-109: old context omitted `mits109-delivery-endpoint`; new context included it. New catalog placed verify-mits under Project procedures with its source path. Named verify-mits output matched the actual project SKILL.md bytes; MITS Git state unchanged.
- Release archive contains `aep` and `LICENSE`; archived binary bytes match the copied executable.

## Local preview artifact

- Version: `5.0.0-preview.1`.
- Installed executable: `/home/memorysaver/.local/share/aep/builds/b75080a5edabd5b0/bin/aep`, via `~/.local/bin/aep`.
- Binary SHA-256: `b75080a5edabd5b0c3cba8e9308ab80312fb3cad0a21a9b24ab7ca205ef36106`.
- Archive SHA-256: `3c6e0db27c98ac89eba57fc2a1b9bbe121b3e02c22407ab2d8f0d07352c32386`.
- Provenance, source commit and previous binary: `~/.local/share/aep/builds/b75080a5edabd5b0/manifest.json`.
- Copied-binary proof and MITS read-only comparisons: `~/.local/share/aep/trials/2026-09-11-downstream-findings/`.

Previous executable remains available for rollback. No public AEP release/tag or downstream implementation was made. MITS's historical published spec bytes were left intact. Full live agent behavior for this additional correction and cross-platform runtime checks remain separate from the executed local tests.
