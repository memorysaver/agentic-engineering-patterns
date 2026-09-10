# Original-project migrations and rebuilt CLI trial

The owner authorized full migration branches in Looplia, MITS and Rewarc, followed by recording actual lessons, building the CLI and trying the result. All three original working directories now select v5 on `migration/aep-v5-preview`. This supersedes the cutover status in the earlier [isolated pilot report](2026-09-10-aep-v5-downstream-pilots.md); those earlier observations remain historical evidence.

## Committed migration results

| Original repository | Final migration commit | Stories / native records | Custom artifacts mapped |
| --- | --- | --- | --- |
| `looplia` | `e17b05bc0ae33552df9a340fa440e2de82161a0e` | 416 / 786 | 29 |
| `MITS` | `8471b5eeaefc26168e5299f05d33588d81f49665` | 107 / 276 | 41 |
| `Rewarc-AutoResearch` | `f00aa60bf55e5a50266919fb13af219e893492a5` | 96 / 266 | 6 |

Project audits live at `docs/audits/2026-09-10-aep-v5-migration.md` in Looplia/MITS and `docs/audits/2026-09-10-aep-v5/migration.md` in Rewarc. They retain source bases, mapping decisions, local verification policy and recovery constraints. The common [lessons learned](../lessons/2026-09-10-three-projects-native-v5-migration.md) record observed defects and repairs rather than declaring additional workflow requirements.

MITS and Rewarc are clean. Looplia retains exactly its four original unrelated untracked files; its preexisting convention edit was committed separately without changing its bytes. Rewarc's unfinished OBS feature worktree remains at its original HEAD with all 13 dirty-file hashes unchanged. Its old autopilot state is paused and backed up; native OBS-001 remains blocked for an explicit resume.

Independent reviews closed the native scope/provenance, sensitive-prefix, host-role and native-reader findings. Final root review additionally compared all 107 MITS records across its last scope correction: only paths changed on 100 records. No original lifecycle or dependency field changed. No migration branch was pushed or merged by this operation.

## Build identity and installation

Built the current Rust/embedded-guidance inputs at AEP source commit `18cc86ab1f43305c6b51df5d0cf041d4559eec4e` using the pinned Rust 1.98.0 toolchain and lockfile:

```bash
cargo build --release --locked --offline -p aep-cli
cargo fmt --all --check
cargo clippy --locked --offline --workspace --all-targets -- -D warnings
cargo test --locked --offline --workspace
```

All passed, including **62 Rust tests**. Subsequent changes in this source repository are this lesson/report/index evidence only; runtime and embedded guidance inputs are unchanged.

- Version: `aep 5.0.0-preview.1`.
- Binary SHA256: `11195ffddc9010c4cae91fa7062afd5c60defa01532a7c0736ef177862345175`.
- Guidance digest: `52d89d282578dad3ddf6915c3f835de33b27e4141a040613b801a4681cd5f92b`.
- Linux x86_64 archive SHA256: `fec2f068a1a54840c60ddc27735b5bf52f2313b5de8553871e18be84d48ddd4f`.

The archive contains `aep` and `LICENSE`, with a separate checksum. Extraction was tested and the extracted executable matched the binary digest/version. The local installed command at `~/.local/bin/aep` resolves to `~/.local/share/aep/5.0.0-preview.1/bin/aep`; its bytes equal this rebuild. No reinstall or version switch was necessary.

The archive, full local transcripts, build logs and synthetic runtime probe are retained under `~/.local/share/aep/trials/2026-09-10-live-migrations/`. This is a local Linux candidate, not a newly published tag/release or a fresh macOS run.

## Actual executable trial

The standalone candidate outside the source tree ran nine commands in each original repository: `doctor`, `skills`, `skills show validate`, `status`, `query`, selected-story `context` and `timeline`, `check`, and `migrate verify`. All **27 commands passed**. Selected stories were Looplia `L41-006b`, MITS `MITS-107`, and Rewarc `OBS-001`, including retained blocked/imported states.

Before/after hashes of tracked and nonignored untracked regular files were identical: 4,805 files in Looplia, 2,631 in MITS, 2,987 in Rewarc. These inspection commands did not mutate project files or advance work. Exact revisions, status distributions and command outcomes are in [the machine-readable trial evidence](evidence/2026-09-10-v5-live-cli-trial.json).

The same candidate also passed `scripts/verify-native-preview.py` in a disposable project: embedded guidance, v4 initialization without source loss, reviewed migration, repeated apply, read-only inspection and new native story creation. The fixture was removed and its evidence retained. Native writes were exercised in that fixture, not injected as artificial product stories into the three originals.

## Product checks and limits

Earlier in these same live migrations, Looplia's local Worker SQLite suite passed 31 tests; MITS passed its Rust help smoke suite (10 tests), repository/type checks and retained-evidence checks; Rewarc passed 122 governance/reader/documentation/formatter tests and 17 final focused tests, actual operational/documentation truth, provider-free preflight and secret scan. Narrow scanner exceptions retained negative controls. Existing formatter debts remain visible and bound to exact pre-migration bytes (MITS 86; Rewarc 45).

These results establish migration and the exercised local surfaces. They do not make imported stories newly verified, discharge owner product decisions, finish OBS-001, establish all historical runtime features or authorize provider spend/deployment. The project-specific holds remain visible in native context.

To begin inspection in any of the three checked-out branches:

```bash
aep --version
aep skills
aep status
aep context <story-id>
```

Read the chosen native procedure and project rules before selecting new work; new context belongs to the configured native stores.
