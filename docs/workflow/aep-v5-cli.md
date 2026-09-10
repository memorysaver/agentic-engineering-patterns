# AEP 5.0 native CLI preview

AEP serves instructions and maintains evidence-bearing project records. The working agent interprets intent, chooses skills, writes designs, and decides the requested terminal action. The CLI has no semantic routing engine or model dependency.

## Installation and guidance

From this source checkout, run `cargo install --locked --path crates/aep-cli`. The pinned Rust toolchain and Cargo.lock build one `aep` binary containing the native skill Markdown and templates. A tagged native release also provides platform archives and SHA-256 checksums; an implementation branch does not publish those artifacts.

The installed executable runs without this checkout, Node, Bun, or a web server.
The native archive contains only `aep` and `LICENSE`, with a
separate checksum. Native release jobs depend only on native validation. Git and
tools explicitly required by the requested operation remain external dependencies.
Dashboard implementation is deferred; native inspection uses the commands below.

`aep --help` groups commands by purpose and shows examples. Bare command groups such as `aep verify` show their help without doing work. Normal command output is human-readable; `--json` returns complete structured results, revisions and diagnostics. Large human summaries explicitly report omitted entries. JSON remains the interface for scripts and exhaustive context retrieval.

`aep --skill` prints the complete agent entrypoint in SKILL.md format (YAML frontmatter and Markdown), including discovered procedures. `aep --skill design` reads one procedure; `aep --skill design --ref bdd` reads one reference. Named procedure/reference output is byte-exact source text without transport footers. These commands work outside Git and without project setup. Add `--json` for release, source, digest, completeness and reference metadata. Project procedures in `project-rules/skills/<name>/SKILL.md`, or configured `skill_paths`, use the same interface. Duplicate names and the reserved entrypoint name `aep` are errors.

This unreleased preview replaces `aep skills` and `aep skills show` with the single `--skill` surface; no alias or second skill entrypoint remains. Update active project instructions and scripts together with the binary. Historical migration sources retain their original commands.

Run `aep init` in an existing Git repository. It creates `.aep/config.toml`, a rules index if needed, and an explicit AGENTS workflow route. A detected legacy project defaults to v4; a fresh project defaults to v5. An explicit user choice takes precedence. Existing project instructions and legacy skill files are preserved; `--claude` adds `@AGENTS.md`. `--dry-run` exposes planned files. Repeated initialization preserves the route. Migration explicitly switches ownership to v5 while retaining legacy source files. See the [preview trial](aep-v5-preview-trial.md).

## Configuration

`aep config show --json` returns configuration and its revision. Submit reviewed configuration with `aep config update --file settings.toml --expect <revision>`. YAML/JSON input also works. Moving stores containing records, rules, procedures, or specification prose requires an explicit migration.

This complete example defines a project check. Replace its argv with the project's actual check; no shell is inserted automatically.

```toml
schema_version = 1
cli_version = "5.0.0-preview.1"
openspec_profile = "1.12.0-common"
skill_paths = []

[stores]
ledger = "project-ledger"
roadmap = "project-roadmap"
rules = "project-rules"
lessons = "lesson-learned"
designs = "docs/design"

[policy]
max_parallel = 2
independent_review = false
required_checks = ["test"]
protected_paths = [".aep", "project-rules", ".github"]

[[checks]]
id = "test"
command = ["cargo", "test", "--workspace"]
cwd = "."
environment = "local"
timeout_seconds = 300
required = true
paths = []
env = []
```

`env` names required environment variables; values are not written to receipts. `paths` selects required checks by actual changed paths. `required = false` makes a check explicitly selectable with `verify run --check <id>`; a gate can still require it. `environment` identifies the configured check target. A label alone cannot prove that a command tested the intended environment; that command is reviewed project policy.

Self verification through current checks is the default. Explicit `independent_review = true` requires current independent review for every risk level. Setting it false permits checks-only completion when no unresolved review findings remain. Risk still derives from the actual diff: light is eligible only for documentation changes without contract obligations; protected paths derive deep. Review has no fixed model topology or round cap.

## Records and context

The common record schema has `kind`, `id`, `title`, `status`, `description`, explicit references, scope, containers, dependencies, and extension data. IDs are opaque and globally unique. YAML duplicate keys, unknown structural fields, unsupported schemas, missing references, and dependency cycles are errors. Canonical Markdown records use YAML frontmatter plus a prose body.

```yaml
kind: story
id: FIX-retry
title: Preserve a completed retry result
description: Implement the accepted retry behavior.
change: retry-result
changes: []
paths: [src/retry, tests/retry]
depends_on: []
required_gates: []
risk: standard
```

`change` names a primary contract; `changes` can name additional contracts. Each must be accepted. A change may serve several stories. Optional `layer`, `wave`, and `release` fields refer to containers; product journey references are optional. Release records can select stories through `refs`, including stories from several layers. Dependencies and gates determine readiness, not numeric IDs or folder order.

Use `aep story new --file story.yaml`. The same new/show/list/update shape applies to changes, layers, waves, releases, gates, roadmap records, decisions, lessons, and rule proposals. Use `--file -` for stdin. Updates require `--expect <record revision>` and preserve identity/status. Protected transitions have explicit commands; accepted decisions are superseded by a new accepted record.

- `aep status`: recorded state, readiness, and diagnostics.
- `aep query --kind story --status pending`: filtered records and revisions.
- `aep context FIX-retry --source docs/design/retry.md`: explicitly linked records and requested source content.
- `aep timeline FIX-retry`: recorded events; unknown occurrence times remain unknown.
- `aep decision accept ADR-001 --by <actor>`: attributed acceptance; ADR files live in `project-roadmap/decisions/`.
- `aep lesson record --file lesson.yaml` and `aep lesson find retry`: observations and searchable imported notes.

Read commands never decide that a new story, design, repair, or merge is wanted. Context packets state their scope; agents inspect additional project sources as needed.

## Design and specifications

Changes contain design prose and `data.specs`, a list of capability, delta path, and baseline digest. Delta paths must belong to a configured context store. `baseline: null` means a new capability.

```yaml
kind: change
id: retry-result
title: Preserve retry results
description: Explain the desired behavior, alternatives, and tradeoff.
data:
  specs:
    - capability: retry
      path: project-ledger/changes/retry-result/specs/retry/spec.md
      baseline: null
```

`aep spec diff --change retry-result` previews the candidate. `aep change accept retry-result --by <actor>` records accepted intent after BDD/baseline checks. Documentation-only changes declare `data.documentation_only: true` and need no artificial behavior contract.

After every linked story is integrated with current check/review evidence, `aep spec publish --change retry-result` updates `project-roadmap/specs/<capability>/spec.md` and its evidence file in one transaction. `aep change close retry-result` closes the published change. Acceptance, integration, publication, and release are distinct facts.

## Isolated execution and evidence

1. Commit the design, rules, configuration, and delta files. `aep dispatch plan --story FIX-retry` reports declared readiness.
2. Run `aep dispatch start --story FIX-retry --base main --owner builder`. The base must contain the current contract and every completed dependency's integration commit. The command claims capacity/scope and creates a Git worktree.
3. Continue in the returned worktree as the current agent, or use an authorized host worker. A prepared worktree is not a running worker. Record actual start with `aep attempt record <id> --status running`; record handoff with `--status review`.
4. Implement and commit in that worktree. `aep verify plan --story FIX-retry` shows actual scope, derived risk, and required checks. `aep verify run --story FIX-retry` runs them there with a timeout and bounded output capture.
5. When project policy requires independent review, or an additional review is chosen, `aep review request --story FIX-retry` returns a revision-bound packet and response shape. `aep review record --file response.json` validates attribution and freshness. Subsequent reviews retain unresolved blocking/material findings and require current closure evidence. Optional review findings remain obligations even when independent review is not mandatory. In that mode the builder can attest a fix with current passing check evidence; explicitly required independent blocking review still needs the independent response. A pending chosen review remains unresolved until recorded or replaced after it becomes stale.
6. Run `aep deliver plan --story FIX-retry`. The skill proceeds to PR creation or authorized merge according to the request.

Workers use `aep --root <control-checkout>` for shared records. Inspections verify the recorded root, Git repository, branch, base, and worktree. Duplicate attempts, overlapping scopes, unavailable dependencies, stale gates, and exhausted capacity block dispatch. A host must reconcile worker liveness before relaunching a claim.

Verification receipts include command, check identity, candidate head, contract/rule/config fingerprint, linked decision/context revisions, instruction files, environment, result, and timestamps. Failed, blocked, interrupted, truncated, and stale results remain visible. Exact worktree head and cleanliness are checked before and after execution, including revalidation of integrated work. Checks that modify candidate inputs cannot certify the old inputs.

## Delivery, gates, and release

`aep deliver pr --story FIX-retry --base main` uses the optional `gh` adapter. It records intent, looks up an existing branch PR, pushes, and records the confirmed URL. `aep deliver merge --story FIX-retry --local` performs a verified fast-forward in the control checkout. Uncommitted changes outside the ledger block local integration.

Without `--local`, merge uses `gh`, checks the current PR head/base/state, and records the provider result. The integration tree is compared with the verified candidate. A different or unavailable tree is recorded as an integration with unverified composition and blocks specification/release promotion. Inspect the result and use `aep story reopen <id>` to retain its historical receipt while preparing current work.

After a lost provider response, inspect `aep deliver status --story <id>`, then run `aep deliver reconcile --story <id>` for the recorded intent. PR retries look up the existing branch. Reconciliation records confirmed facts; it does not assume a timeout means failure or overwrite concurrent prose edits.

`aep gate evaluate <id>` evaluates its explicit story/container scope, required checks, child gates, and optional `data.environment`. Changed membership, current receipts, rules, contract content, or criteria invalidate the old verdict. Content invalidation is deliberately conservative across configured rules/change text; it can require reevaluating a gate after unrelated context changes. Release promotion also changes scope records; reevaluate its gates before using them for subsequent dispatch. The promotion receipt retains the proof used at promotion.

`aep release promote <id> --environment staging --by <actor>` requires integrated members and current release gates for that environment. Release `required_gates` govern promotion; story/layer/wave prerequisite gates govern dispatch. The release captures member revisions and attribution. CLI attribution does not prove a person's real identity or authority, and local results do not become deployment proof.

## Learning and rules

Capture a concrete observation, its source/evidence, and relevant scope with `aep lesson record`. `aep reflect propose --file proposal.yaml` creates a draft rule; it does not silently install a hook or change project policy. Implement and evaluate the committed proposal under a story. `aep rule adopt <id> --by <actor> --evidence <check-id>` requires current integrated check/review evidence whose candidate contains that exact rule proposal. The adoption records its digest and evidence. `aep rule retire <id>` retains history.

Agents decide whether a lesson belongs in a local rule, local procedure, or an AEP release. Built-in instructions change through source review and release; local procedures remain project files.

## Transactions and machine output

`--json` produces one schema-versioned result on stdout, with verdict, CLI version, data, diagnostics, and `side_effects`. Human guidance also supports Markdown. Saved migration plans bind source revisions/hashes. Runtime transactions live under Git's per-worktree metadata; partial writes require `aep recover` inspection, followed by `--apply` or `--rollback`. Recovery rejects later content or permission edits rather than overwriting them.

| Exit | Meaning |
| --- | --- |
| 0 | Requested operation completed; a status result may describe blocked work |
| 1 | Validation or test failure |
| 2 | Invalid input or usage |
| 3 | Unmet prerequisite |
| 4 | Unsupported capability, version, or unmapped semantics |
| 5 | Revision conflict or recovery required |
| 6 | External execution or I/O failure |

Managed stores contain UTF-8 text, bounded to 16 MiB per file. Large or binary evidence belongs in an artifact store or Git source with explicit pointers. Git history protects migration context; the transaction journal protects interrupted current writes. These serve different purposes.

References accept their catalog name or exact Markdown link path: `aep --skill roadmap --ref status` and `aep --skill roadmap --ref references/status.md` return identical content. Both resolve only resources in the selected procedure catalog. Native skill links include the command beside the relative link.
