# MITS adopts layer/wave encapsulation from rebuilt guidance

Date: 2026-09-11, Asia/Taipei
Status: live-observed downstream turn, 5 minutes 10 seconds, after an observer prompt authorized by the user; agent `done`
Previous: [layer/wave steering](2026-09-11-mits-layer-wave-steering.md)
Guidance: [layer and wave encapsulation](../decisions/aep-v5-layer-wave-encapsulation.md)

The user accepted the guidance part of the encapsulation proposal and asked for the native skills to be changed, the CLI rebuilt and installed, and the MITS agent guided to read and follow the new content. This is the first downstream turn on that build.

## Build and installation

Six native files changed: `roadmap/SKILL.md`, `roadmap/references/product-context.md`, `roadmap/references/status.md`, `design/SKILL.md`, `design/references/records.md`, `implement/references/handoff.md`. `bun run skills:check-steering` reported no file above its baseline; `bun run skills:check` reported generated resources in sync. `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings` and `cargo test --workspace` passed. `scripts/verify-native-preview.py` passed against the new executable.

The executable `3fae0e4275dd22f6…` was staged at `~/.local/share/aep/builds/3fae0e4275dd22f6/bin/aep` with a manifest recording source commit `44990cb`, the digest of the uncommitted guidance diff and the checks run. `~/.local/bin/aep` was switched atomically from build `9f32299fdc45ca53`, whose manifest now records the replacement. Version string remains `5.0.0-preview.1`. MITS was idle at the switch.

## Prompt and guidance consumption

The observer sent one prompt: reread six named guidance entries, explain the difference from the previous turn, then under the user's authority encapsulate MITS-110 to MITS-118 in a layer with waves, update story fields and roadmap refs, remove ordering fields, run checks and report saved/committed state. The prompt text is retained in the evidence bundle.

The transcript shows `aep --skill roadmap`, `--ref product-context`, `--ref status`, `aep --skill design`, `--ref records`, `aep --skill implement --ref handoff`, then the project's own `aep --skill verify-mits`, `aep layer --help`, `aep wave --help`, `aep context --help`, `aep layer new --help`, `aep story update --help`, and JSON reads of the target records and existing layer/wave records. The agent's summary of the difference was accurate: the previous turn used a shared roadmap and ordering metadata; the new guidance requires a layer that links outcome, design, decision and members, with order expressed by `depends_on` and waves.

## Native writes and their shape

| Artifact | Observed content |
| --- | --- |
| `project-ledger/layers/layer-20.yaml` | Title names the concept; description states the observable outcome and the completion condition; `refs` list nine stories and both source decisions; `data` names the primary design artifact and two supporting drafts |
| `project-ledger/waves/wave-20-1..4.yaml` | Members in `refs`; 20-2 depends on 20-1; 20-3 and 20-4 both depend on 20-2; descriptions state each batch's intent |
| MITS-110 to MITS-118 | `layer: layer-20`; `wave` set per batch; `data.recommended_order` removed; other fields preserved |
| `evaluation-research-roadmap` | `refs` now include `layer-20`; `data.recommended_execution_order` removed |
| Design draft | New section describing Layer 20 and the four waves with links to the container files |
| 15 events | layer and waves `null → pending`; nine stories and the roadmap `pending → pending` with revisions |

The agent's first version of the design section linked the containers under `project-roadmap/layers` and `project-roadmap/waves`, which do not exist; the files live under the ledger store. The observer saw the broken links before the turn ended. The agent then searched both stores, corrected the links, and reran formatting and checks. All local links resolve at inspection.

## Runtime behavior confirmed by the observer

`aep check --json` passed with 481 records and no diagnostics. Three of the nine `aep spec check` reruns passed; the agent's transcript reported all nine. `aep context layer-20 --json` includes all nine members, the four waves, the nine `eval-*` changes and both decisions with no missing references, while expanding to 190 records including all imported layers, waves and gates. `aep status --json` shows MITS-113 blocked on MITS-110 to MITS-112 through wave-20-1, and MITS-115/116 blocked on MITS-113/114 through wave-20-2; container dependency inheritance works as designed. MITS-110 waits only on acceptance of its change.

## Handoff and retained state

The final response listed actual paths, the checks run, the 0-ready status with its reason, and stated that files were saved but not staged or committed, with `main` still at `7af2d5b`. It named remaining work: the MITS-110 contract, collection entry, data maturity and research spend authorization. No implementation or V2 run occurred. Git retention in MITS has now been deferred across four turns; the user has not asked MITS to commit.

Open observations for AEP: `aep context` on a container still expands through decisions and the roadmap into most of the imported history; the store for containers is the ledger, which the design-store prose initially got wrong; the CLI candidates in the decision (empty-container diagnostic, status grouped by layer) remain unimplemented.

Evidence: `~/.local/share/aep/trials/2026-09-11-mits-layer-wave-adoption/`, including the prompt, pre/post binary digests, preview proof, baseline and final records, transcript snapshots, context/status/check output and a SHA-256 manifest.
