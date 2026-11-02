# Adopt AEP 5.0 preview in an existing project

Migration converts useful current context and preserves the project's constraints. It does not turn legacy completion, gate, archive, or task-checkbox labels into passing implementation evidence. Legacy source files and installed v4 skills remain in place, with exact source versions also retrievable from Git. The [preview trial](aep-v5-preview-trial.md) explains version selection and the first downstream task.

## Plan and apply

1. Inspect the current project instructions, consumers, runtime claims, and source layout. Commit the exact source files so the plan can bind each imported path to a retrievable Git blob. Finish or explicitly checkpoint active legacy attempts before cutover.
2. Run `aep migrate plan --output /tmp/aep-migration.json`. The default source is `product-context.yaml`; `--source <relative-path>` selects another supported source. `--story <id>` selects a scope and includes its dependency/barrier closure. `--dry-run` writes no plan or project files.
3. Review records, writes, diagnostics, source hashes, and the selected scope. The agent resolves custom policy and semantic mappings; the CLI does not guess them. Unknown legacy hooks or inline host workflow instructions require a tracked factual consumer-disposition note via `--consumer-review <relative-path>`; merely installed skills and recognized read-only guard hooks do not require one. Unresolved operational constraints block apply. A saved plan must be regenerated after relevant drift.
4. Run `aep migrate apply --plan /tmp/aep-migration.json`, then `aep migrate verify` and `aep check`. Commit the resulting context and entrypoint. A repeated apply is idempotent for the same receipt; another story scope gets its own plan.
5. Inspect the returned context with `aep status`, `aep context`, `aep --skill`, and a local procedure. Complete applicable contract mappings and current verification before dispatching production work.

The plan covers selected stories, completed dependencies, legacy wave barriers, prior-layer gate occurrences, product direction and split product sources, ADRs, rules, and lessons. Native IDs remain opaque. Non-completed story states such as blocked or deferred remain non-dispatchable; migration does not turn holds into pending work. Legacy numeric layer/wave labels, including fractional values and quoted numbers, retain their ordering; labels without a numeric ordering require explicit mapping. Known legacy arrays must have the supported shape; malformed dependencies cannot become empty dependencies.

`project-convention/` is copied into `project-rules/` with v5 references adjusted; original bytes stay in place. Applicable old AGENTS content is retained as an indexed imported instruction source, while the selected v5 entrypoint gives native AEP ownership of workflow state. Existing `lessons-learned/` notes are copied under `lesson-learned/` at the same directory depth; relative note/evidence and external project links retain their targets, and `aep lesson find` searches them. Nested custom lesson-store roots and reserved `observations/*.md` paths require explicit mapping. Binary evidence remains at its original source path outside the native text stores. The receipt records its source commit, path and digest. Recognized Markdown, quoted and backtick references are rebased to the retained asset; unfamiliar reference syntax requires explicit mapping. Oversized or unsupported sources still require project review.

The importer reports project-specific operational fields such as topology, release gates, execution slices, routing, and governance instead of silently dropping their authority. Map these to native capacity/dependencies/gates/checks and indexed rules in a reviewed plan. Preserve the generated plan and record each diagnostic, its concrete resolution, and its source/target evidence before editing that plan. Remove only a diagnostic whose semantics are actually represented; update receipt targets when adding native files. A review note alone cannot replace an executable legacy consumer. Archived change references currently require this explicit historical-provenance mapping; retaining an archived source does not create a live change or verified completion. Generic schema conversion alone does not establish that such a project is ready to resume.

Legacy product/context, rule and lesson inputs remain historical sources. Preserve installed v4 skills and their lock entries. Inspect old writers and scheduled consumers at cutover; record whether they are inactive, read-only, or explicitly redirected/stopped under the task authority. The selected v5 entrypoint and CLI use native stores; no automatic dual-write or fallback is performed. Preview migration retains source files for comparison rather than removing them. Store/source overlap and conflicting existing native targets block conversion. Repeating migration does not synchronize later legacy edits over native work. Inspect the migrated state with `aep --root <project> status --json` and `aep query`; dashboard work is deferred and is not required for migration.

## Resolve old completion evidence

An old completed dependency is imported with `status: imported` and its original claim. After inspecting the code and Git evidence, use:

```bash
aep story reconcile OLD-42 --commit <integration-commit> --by <actor>
```

This records an attributed Git inspection and creates a worktree for current verification. It creates no passing check or review. Configure/run current project checks and satisfy any explicit independent-review policy before evaluating a gate that covers the imported work. `aep story reopen <id>` retains a historical delivery reference and returns a completed/imported story to pending work when implementation or design must change.

## Native specifications and retired compatibility

AEP 5.0 retires OpenSpec commands, profile configuration, automatic import/export and external reference validation. Existing source files remain untouched; they are not interpreted as active contracts. Translate requirements deliberately into native change records and BDD deltas, preserving source attribution in the change description. Check and publish them through `aep spec check`, `aep spec diff` and `aep spec publish` after normal verification and delivery.

## Preview to stable 5.0

Back up `.aep/config.toml`. Remove its retired `openspec_profile` key and change `cli_version` to `"5.0.1"`, preserving every other setting. This explicit configuration migration is required because 5.0 rejects unknown fields. Run `aep doctor` and `aep check`, inspect historical imported records and translate any remaining constraints into native rules and change contracts before resuming work. Prior receipts remain historical; rerun candidate verification with the new binary before delivery. The record schema remains 1; no old source files are deleted automatically.
