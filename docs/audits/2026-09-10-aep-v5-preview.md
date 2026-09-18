# AEP 5.0.0-preview.1 convergence and release preparation

The user accepted [v4/v5 coexistence and source-preserving migration](../decisions/aep-v5-preview-adoption.md), the [context/self-verification classification](../decisions/aep-v5-context-and-self-verification.md), and separate subagent reviews before preparing a downstream trial. This report covers `feat/aep-5-native-cli` after `34e5b9a`, including the earlier uncommitted CLI-first correction. Publication, platform CI and production adoption are separate facts below.

## Implemented scope

- The Rust workspace and embedded catalog identify `5.0.0-preview.1`; legacy marketplace metadata remains v4.1.0. The tag workflow marks prereleases `--prerelease --latest=false`.
- Eight native skills retain task-oriented entrypoints. Eleven added references cover context sources, project-owned verification/Feature Map, product framing/status, design criteria/prototypes, handoff/Git, self verification, closure and feedback. The total native corpus is eight skills and thirteen references.
- Fresh configuration defaults to self verification. Explicit independent-review policy is enforced through the shared gate/delivery/spec/rule-adoption requirements. Fixed evaluator rounds and effort prescriptions are removed. Concrete unresolved review findings still block completion across revisions and attempts; current evidence is required to resolve them.
- Initialization preserves v4 default in detected legacy projects and uses v5 for fresh projects. Managed AGENTS routing carries explicit user choice, project default and handoff identity. Migration selects v5 and preserves source data and installed skill bytes, with collision, overlap and drift checks.
- A tracked `--consumer-review` note records the working agent's assessment of detected ambiguous host consumers. Installed legacy skills and exact known read-only guards do not create that requirement. A note records attribution/provenance; it cannot prove an external scheduler stopped.
- The repository's [verify-aep](../../project-rules/skills/verify-aep/SKILL.md) procedure and executable [probe](../../scripts/verify-native-preview.py) exercise the public binary against a disposable legacy project and retain evidence after fixture cleanup.
- Dashboard integration remains deferred. Existing web code and dependencies match `b47d143`; native archives contain `aep` and `LICENSE`, with a separate checksum.

## Reviews and closure

Three subagents first reviewed the decisions against existing code/guidance without edits. They then implemented disjoint fixes. Migration and verification reviewers subsequently cross-reviewed one another's code; a separate fresh agent evaluated guidance selection. This separates author validation, independent code review, and behavior observations.

| Review surface | Concrete findings and handling |
| --- | --- |
| Migration and version routing | Legacy rules/lessons deletion, unconditional v5 append, init-before-migrate ambiguity, unchecked target collisions, missing source/store overlap guards and custom-root routing were found and addressed with regression fixtures |
| Verification and completion | Default/forced independent review contradicted autonomy; fixed rounds blocked valid later reviews. Changing the default also required retaining optional findings, stale-response checks and obligations across replacement attempts. Shared enforcement and CLI fixtures cover these paths |
| Native guidance | Missing context selection, operational verification, prototype path, closure/feedback ownership, legacy topology wording and `monet-*` residue were corrected. Legacy skill corpus remains unchanged |
| Cross-review of migration | Real-binary probes found external relative links broken by nested custom rules stores and deleted imported files resurrected during later scoped migration. These required explicit relocation diagnostics and preservation of native deletion rather than silent recopy |
| Downstream migration | Looplia exposed binary evidence decoding; MITS exposed overlapping OpenSpec/story IDs, lost deferred holds and prospective configuration validation. Fixes include source-scoped alias review and fresh/existing configuration reproductions; all bounded review findings are closed |
| Release helper | A timed-out subprocess omitted its command transcript. The helper now persists argv/partial output/timeout before reporting failure; a real short-timeout subprocess probe confirmed evidence survives cleanup |

## Validation evidence

Local final candidate checks are complete. Platform checks are reported separately on the exact PR head; early snapshots below retain their bounded observation scope.

- Final full native run after downstream and cross-review fixes: **62 passed** (6 CLI unit, 18 lifecycle, 2 migration asset, 14 migration preview, 6 verification preview, 7 core, 9 store). `cargo fmt --all --check`, `cargo clippy --locked --workspace --all-targets -- -D warnings`, and `cargo test --locked --workspace` passed. Both cross-review reproductions and the ignored host-settings drift probe pass; no review findings remain open.
- All four skill-source checks passed: generated resources, vocabulary, steering and package installation. The 24 legacy skills remain valid with their existing routing evidence; those legacy observations are not native behavior evidence.
- Eight native frontmatters and the project-owned procedure validate using `uv` with PyYAML. System Python alone lacks PyYAML; no system package installation was needed.
- The final offline installed binary served all 21 embedded skill/reference bodies byte-for-byte outside its renamed build-source tree and with empty PATH. Bundle digest: `52d89d282578dad3ddf6915c3f835de33b27e4141a040613b801a4681cd5f92b`.
- [Final extracted-archive runtime proof](evidence/2026-09-10-v5-preview-runtime.json) verified legacy byte preservation, v4 init then v5 cutover, repeated apply, read-only inspection, and new native writes. Evidence survives deletion of the fixture; full transcripts remain in the local artifact directory.
- The guidance author independently constructed an additional disposable runtime fixture: direct migration without init, native documentation change acceptance, linked story/context and dispatch planning passed. This is an actual runtime usability observation, not an unbiased cold-agent selection result.
- Offline execution of the release shell branch selected prerelease/non-latest flags for preview and normal flags for stable. Native-only workflow YAML parsed successfully.

A [fresh, non-author observer](evidence/2026-09-10-v5-preview-cold-guidance.json) read the actual catalog and selected skills/references for six requests: disposable v4 migration, establishing runtime verification, API exploration, implementation under explicit review policy, status/decision reporting, and stale-journey feedback. All had a usable route and concrete context/evidence criteria; no blocking contradiction was found. This is a cold selection/content observation, not six completed downstream tasks.

The source repository intentionally has no downstream `.aep/config.toml`. Its `verify-aep` procedure is read through the project-rules index; local CLI skill discovery is tested in initialized downstream fixtures. This distinction was exposed during the fresh-agent pass and clarified in the procedure.

## Remaining evidence and publication state

This candidate has no production downstream migration or real GitHub-provider execution evidence. Provider fixtures are mocks. Context packets only traverse explicit links/requested files, and process receipts do not certify arbitrary screenshot/trace artifact identity or acceptance completeness. Those are disclosed preview limits, not hidden passing gates.

An offline `cargo +1.98.0 install --locked` succeeded from a reduced source tree containing only Rust inputs, native guidance and license. The original build-source path was then removed by renaming. The Linux archive was extracted, member names and SHA-256 verified, and the public runtime probe rerun using the extracted executable.

Local artifact directory: `/tmp/aep-preview-final-7x626jh8`. Archive SHA-256: `66ee7230ec5a17a6edaefb8c8e1404f20c758a95175b227fb35896b8dcb19128`. Its `candidate-manifest.json` binds every build input and binary digest. This is a prepared local candidate; no public tag or release has been created.

Linux and both macOS targets passed on `808fa3e`; the additional downstream fixes require checks on the updated [PR #35](https://github.com/memorysaver/agentic-engineering-patterns/pull/35) head. The owner selected Looplia, MITS and Rewarc. Their [isolated pilot observations](2026-09-10-aep-v5-downstream-pilots.md) distinguish native context conversion, actual project verification, and remaining executable consumer or specification gaps. The original three projects remain untouched.
