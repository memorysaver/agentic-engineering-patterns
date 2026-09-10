# Try AEP 5.0.0-preview.1 in an existing project

The preview keeps v4.1 skills available and creates separate v5 project stores. Selecting a workflow controls which procedures and records the agent uses. It does not synchronize the two data models. The [adoption decision](../decisions/aep-v5-preview-adoption.md) records this boundary.

## Obtain a candidate

Before publication, install from the reviewed candidate checkout into a separate prefix:

```bash
cargo install --locked --path crates/aep-cli --root /tmp/aep-v5-preview
/tmp/aep-v5-preview/bin/aep --version
/tmp/aep-v5-preview/bin/aep skills
```

Use that exact executable throughout the trial or put its `bin` directory first on the trial shell's PATH. Verify the selected version before using the short `aep` commands below. An existing installed executable or legacy skill package is not removed by this isolated install. The installed binary carries the preview guidance and needs no source checkout or JavaScript runtime.

After a preview release is actually published, download the archive for the matching platform plus its `.sha256` file and verify the checksum before extracting into a dedicated directory. Supported release targets are Linux x86_64, macOS arm64, and macOS x86_64. Publication and platform results are reported by the candidate audit; a document describing this process does not establish that assets already exist.

For an AEP candidate smoke run, use the source repository's [verify-aep procedure](../../project-rules/skills/verify-aep/SKILL.md). The helper writes a new evidence directory and cleans its own disposable project. It never migrates the caller's repository.

## Select the downstream scope

Choose the project and inspect its own AGENTS, applicable maintenance rules, uncommitted changes, current worktrees, and active automation. Preserve the installed v4 skills. Select either an isolated checkout for the first trial or an explicitly authorized in-place cutover. Commit the exact legacy input files so migration can bind their Git blobs; avoid including unrelated changes.

Tell the working agent the selected version and task, for example:

```text
Use AEP v5 preview for this migration and subsequent trial.
Read the project's rules and `aep skills`.
Preserve v4 skills and migration sources.
Preview the conversion, resolve concrete source/consumer conflicts,
then apply it and use v5 stores for the selected work.
Report missing native capabilities rather than falling back to v4.
```

Reading native guidance does not itself change project ownership. `aep init` keeps a detected legacy project's default at v4; a fresh project defaults to v5. The generated AGENTS route supports explicit user selection. `migrate apply` is the actual v5 cutover. Existing project policy, authentication and execution authority still apply.

## Convert and inspect

Run from the selected downstream checkout:

```bash
aep migrate plan --output /tmp/aep-preview-migration.json
```

Inspect writes, source hashes, dependencies, historical gates, and diagnostics. Use `--story <id>` to select a narrower scope. Source files stay in place; custom native stores must not overlap legacy sources. Existing conflicting native targets block conversion. A drifted plan must be regenerated and reviewed.

When the plan detects unknown legacy hooks or inline host workflow instructions, inspect the actual consumers and record their disposition in a tracked project note, then pass `--consumer-review <relative-path>`. This is the working agent's factual audit, not a new human-approval ritual. Keep nonconflicting guard hooks; stop or redirect conflicting active writers only within the task's authority. The receipt binds the note and relevant source versions. An arbitrary assertion is not evidence that an external scheduler stopped.

After the plan is coherent:

```bash
aep migrate apply --plan /tmp/aep-preview-migration.json
aep migrate verify
aep check
aep status --json
aep query --json
aep context <imported-story-id> --json
```

Inspect the actual generated `project-ledger/`, `project-roadmap/`, `project-rules/`, `lesson-learned/`, `.aep/config.toml` and AGENTS route. Verify the selected default is v5 and compare legacy source/skill bytes with the recorded base. Root entrypoints are intentionally updated; their original content remains an attributed source. Imported completion is still an unverified historical claim.

Preserve explicitly configured downstream review/check requirements. A fresh native config defaults to self verification. Checks-only completion is permitted when policy allows it and no unresolved review findings remain; a project requiring independent review still needs that evidence. Use the [configuration guide](aep-v5-cli.md) and `aep skills show project` to configure the project's actual checks.

## Run one real task

Select a bounded task with an observable outcome and load its context. Use `aep skills show design` when intent or a technical decision remains unresolved; use its prototype reference when an experiment can answer the question. Discover or establish the project's own runtime verification procedure before claiming behavior works.

Implement in the proper worktree, execute applicable checks and real user paths, then perform only the requested delivery action. Preserve results and relevant lessons. New work writes to v5; original v4 files remain historical input, not a second status database.

Capture:

- Selected CLI version and guidance digest, source base, store paths, and task identity.
- Whether a cold agent found intent, rules, dependencies, current implementation and verification entrypoints.
- Actual checks/runtime observations, artifact locations, environment, and unverified surfaces.
- Routing mistakes, stale context, missing native capabilities, migration diagnostics, and useful lessons.
- Whether another session can resume the same work from native records without the previous conversation.

Returning to v4 after new v5 work requires deliberate handoff of that progress. Restoring old entrypoint text does not convert new records back to the old schema. Recover through the reviewed Git history and preserve new work/evidence before changing ownership.
