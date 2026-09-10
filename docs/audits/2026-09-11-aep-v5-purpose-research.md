# Purpose-driven research and product-context handoff verification

Date: 2026-09-11
Source commits: `9a23c32ec42e8ef33cac6f32edf257df33a48348` (capability) and `45861f2717219e941972be4b31240a59f1d20d39` (record-update clarification)
Decision: [Purpose-driven research](../decisions/aep-v5-purpose-driven-research.md)
Observation: [MITS product focus](../lessons/2026-09-11-mits-product-focus-observation.md)

## Implemented guidance

The native entrypoint routes product refocusing to roadmap. Roadmap now identifies decision-changing evidence gaps before recommending a priority or benchmark, loads the canonical purpose-research reference as needed, presents a supported recommendation and seeks meaningful user feedback. Once direction is clear, its product-context reference carries authorized work through canonical documents and links in the same task. Design consumes that direction and creates a concrete contract/story when warranted, retaining unresolved exploration as draft and continuing implementation only within the requested scope.

The eight native names and descriptions remain unchanged. No research service, mandatory evaluator topology, fixed research rounds or deterministic semantic router was added. Native state transitions and fingerprint checks remain the existing Rust mechanics; guidance improves the working agent's decisions rather than claiming a machine-enforced semantic completeness check.

Research methods live in `roadmap/references/purpose-research.md`. Canonical document responsibilities and the feedback handoff live in `roadmap/references/product-context.md`. Status guidance distinguishes backlog/readiness from supported product priority. Existing prototype and self-verification guidance remain the path when actual behavior, rather than additional sources, must answer the question.

## Checks and executable proof

- `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, and `cargo test --workspace` passed: **74 tests**, zero failures.
- Generated skill consistency, vocabulary and steering checks passed. The initial package check rejected one extra reference-to-reference Markdown link; the redundant link was removed while retaining the direct CLI read command. Package check then passed without raising navigation or steering ceilings.
- Official skill quick validation passed for native root, roadmap and design. Frontmatter descriptions and the legacy v4 catalog/routing evidence were unchanged.
- Release build passed. The copied executable retained the eight-entry catalog and emitted exact source bytes for roadmap/design and the purpose-research/product-context references.
- `scripts/verify-native-preview.py` now explicitly reads both research/handoff references. Its copied-binary run passed source-preserving migration, repeated apply, offline guidance, read-only native inspection and fixture cleanup.
- Archive contains exactly `aep` and `LICENSE`, with binary bytes matching the copied executable. After the final two-line record-input clarification, all seven human CLI tests, four skill checks, release build, copied-binary probe and exact-source checks passed again.

Executable proof is retained in `~/.local/share/aep/trials/2026-09-11-purpose-research/` for the first candidate and `~/.local/share/aep/trials/2026-09-11-purpose-research-final/` for the installed build, including command transcripts, `summary.json` and `embedded-guidance-proof.json`.

## Isolated agent observation

A fresh agent receives only an isolated synthetic project's context, the candidate executable and an owner's follow-up selecting same-project handoff while leaving an external benchmark tentative. The owner requests context/design completion without production implementation or benchmark execution. The working agent uses installed guidance and actual native commands; it is not given the source diff or intended evaluation verdict. The agent completed and locally committed the requested context/design work at fixture commit `d61436a246f070f5953ef3bea2e61adae0cb5e5a` without another user question or any production implementation/benchmark execution.

It updated the roadmap and design, created accepted direction decision `D-HANDOFF`, left benchmark decision `D-EXPERIENCEQA` pending, accepted a bounded design contract `C-HANDOFF` with seven scenarios, and left future story `S-HANDOFF` pending. The design contract explicitly preserves the current round's documentation-only scope. README links and retained verification notes complete the handoff. Parent read-only checks independently confirmed record/spec validity, decision statuses, no attempt/delivery/check-evidence records and clean committed Git state.

The agent's first roadmap update omitted immutable fields and was rejected. It recovered by reading the complete current record and using its revision. This observed friction led to a small follow-up in the records reference: start updates from `data.record`, preserve immutable/unrelated fields, and supply `data.revision` to `--expect`. The agent behavior observation used build `181c6831f8888c1f`; the final build adds only that record-input clarification to the embedded guidance. No second agent pass is claimed for that clarification.

Fixture history is retained as a verified Git bundle, final documents as an archive, and record/status/check snapshots under `~/.local/share/aep/trials/2026-09-11-purpose-research/agent-trial/`. Its temporary project and executable were removed after retention and inspection.

This local scenario is a behavior observation, not an A/B result or proof that all live agents will choose research at the right time. The initial research-before-recommendation behavior, ambiguous-feedback cases and future live MITS continuation remain separate coverage boundaries.

## Preview artifact

- Version: `5.0.0-preview.1`.
- Installed executable: `~/.local/share/aep/builds/9f32299fdc45ca53/bin/aep`, reached through `~/.local/bin/aep`.
- Binary SHA-256: `9f32299fdc45ca53ed06e69cc88a8aa9f28b0204d4ead1de9d110eb668b8106c`.
- Archive SHA-256: `422a153cbf76132ddedb03654331cb43f54d2610cb0052ff1c406ddc4948d4da`.
- Source, previous binary and installation status: that build directory's `manifest.json`.

The local PATH symlink was atomically switched after the probes passed. Herdr reported MITS `blocked` on its queued product-direction question, with a completed recommendation visible and no visible running tool activity; it was not labeled idle. This cutover was recorded explicitly while it awaited user input. Future skill reads receive the new instructions; existing agent context is not retroactively rewritten. Previous build `aec8e775495a49ff` remains available for rollback. Installed PATH, version and digest were checked. No public release/tag, MITS prompt or MITS product-file change was made.
