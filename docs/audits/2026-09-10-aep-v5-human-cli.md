# Human-readable CLI verification

2026-09-10; local 5.0.0-preview.1 candidate. [Decision](../decisions/aep-v5-human-cli.md), [machine evidence](evidence/2026-09-10-human-cli-trial.json).

## Result

Clap defines the command surface, nested help and argument errors. The grouped front page uses those command descriptions. Text output is the default; complete JSON is explicit. Agent guidance has the single `--skill` entrypoint and valid frontmatter. No TUI dependency was introduced.

Rust workspace validation passed: 69 tests, formatting and Clippy with warnings denied. Four skill corpus checks and all nine native SKILL frontmatter validations passed. Actual failed verification is exercised through the CLI; a renderer test preserves external validator failure details. The latter uses a constructed subprocess result, not a live OpenSpec call.

Independent review found two presentation defects: saved failed verification looked successful, and OpenSpec reference errors were omitted. Both were fixed and independently rechecked. Verification now reports FAIL, retained evidence and individual check results. Reference stdout/stderr remain visible.

## Copied binary and downstream observations

The copied release binary (SHA-256 `dcf177ca0c90eb99e991eaf87130b8a7e0444e9bed13b6f2e50b9e7f393008dd`) passed the disposable legacy migration/native-write probe. Its fixture was removed, retained sources stayed unchanged, and the executable served guidance outside this checkout.

Each real project ran nine successful commands: help, root skill, project verification skill, doctor, text status, JSON status, check, migration verification and dry-run init. Before/after hashes of all Git-listed tracked and nonignored untracked files matched: Looplia 4,805; MITS 2,631; Rewarc 2,987. Status summaries were bounded and preserved readiness constraints. The machine evidence records exact commands, exit codes and line counts; full transcripts remain under `~/.local/share/aep/trials/2026-09-10-human-cli-live/`.

Active AGENTS, rules and host adapters in all three migration branches now use `aep --skill`. Independent review confirmed that the 28 edits only replace command names; all four edited TOML files parse and the Rewarc skill frontmatter is unchanged. Historical imported sources were retained. MITS and Rewarc `bun run check` passed, retaining their existing exact-content formatting debt boundaries (86 and 45 files). Rewarc operational-truth validation passed. These interface checks do not establish product acceptance, exercise external providers or change the paused Rewarc worktree.

This is a local preview installation and archive, not a tagged/public release. Previous preview audit hashes continue to identify the old binary; the new artifact lives in a checksum-specific build directory.
