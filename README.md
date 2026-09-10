# Agentic Engineering Patterns

AEP 5.0 is a Rust CLI for project context, isolated implementation, verification, and learning. Agents choose the work and skills. The CLI serves instructions, maintains records, and checks declared constraints.

This branch prepares **5.0.0-preview.1** for downstream trials. The stable legacy skill release remains **v4.1.0**; the native preview does not replace it.

## Install from source

Install the pinned Rust toolchain with rustup, then run from this checkout:

```bash
cargo install --locked --path crates/aep-cli
aep --version
aep skills
```

In a project with Git history:

```bash
aep init
aep doctor
aep skills show project
```

The installation needs the binary and a short `AGENTS.md` entrypoint. `aep init --claude` also adds an `@AGENTS.md` pointer. Instructions and references are embedded in the binary. The CLI runs independently of this source checkout, Node, and Bun. It needs no LLM key, OpenSpec CLI, or per-runtime skill copies for its normal lifecycle.

Linux and macOS are the native validation targets. Project checks still need their own toolchains. The optional GitHub adapter needs `gh` and repository access.

## Project structure

| Store | Content |
| --- | --- |
| `project-rules/` | Project code, testing, DevOps, release rules, and optional local procedures |
| `project-ledger/` | Stories, changes, layers, waves, releases, gates, attempts, and evidence |
| `project-roadmap/` | Product direction, journeys, architecture, **decisions/ ADRs**, and published specifications |
| `docs/design/` | Design drafts and research supporting a change |
| `lesson-learned/` | Observations and retained execution lessons |
| `.aep/config.toml` | CLI pin, store locations, capacity, and explicit check commands |

Stores are created when needed. A maintenance story can exist without a user journey. IDs are opaque; dependencies and gates determine readiness. Git retains older context.

## Working with AEP

Start with `aep skills`. Read a selected procedure with `aep skills show design`, or a reference with `aep skills show design --ref bdd`. The agent selects procedures from the request and context; the CLI has no model or semantic route command.

Use `aep status`, `aep context <id>`, `aep query`, and `aep timeline` to inspect recorded facts. Use structured files or `--file -` to author records. `aep check` validates the project; verification runs its configured commands in the bound worktree. Self verification produces revision-bound check evidence; explicit project policy can also require independent review. Delivery records actual integration separately from accepted intent and published specifications.

- [Native CLI and configuration](docs/workflow/aep-v5-cli.md)
- [Migration and OpenSpec compatibility](docs/workflow/aep-v5-migration.md)
- [5.0 architecture and accepted design](docs/decisions/aep-v5-rust-cli-architecture.md)
- [Implementation and validation report](docs/audits/2026-09-09-aep-v5-implementation.md)
- [Current CLI scope and validation](docs/audits/2026-09-09-aep-v5-cli-focus.md)
- [Legacy-to-v5 skill capability inventory](docs/audits/2026-09-10-aep-v5-skill-migration-inventory.md)
- [Skill classification: context and self verification](docs/decisions/aep-v5-context-and-self-verification.md)
- [Preview adoption and v4/v5 coexistence](docs/decisions/aep-v5-preview-adoption.md)
- [Preview implementation and review evidence](docs/audits/2026-09-10-aep-v5-preview.md)
- [Downstream preview trial](docs/workflow/aep-v5-preview-trial.md)
- [Looplia, MITS and Rewarc pilot observations](docs/audits/2026-09-10-aep-v5-downstream-pilots.md)
- [Completed original-project migrations and rebuilt CLI trial](docs/audits/2026-09-10-aep-v5-live-migrations.md)
- [Lessons from the three actual v5 migrations](docs/lessons/2026-09-10-three-projects-native-v5-migration.md)
- [Legacy v4.1 installation and skills](docs/workflow/aep-v4.1-guide.md)

## Develop AEP

The Rust crates own the native runtime. `skills/native/` contains its embedded guidance. The current deliverable is the CLI; [dashboard work is deferred](docs/decisions/aep-v5-cli-first.md). Existing `apps/` and `packages/` code is retained at its pre-v5 state and is outside the native release.

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

The native release archive contains `aep` and `LICENSE`, with a separate SHA-256
checksum. Project state is available through `aep status`, `aep query`,
`aep context`, and `aep timeline`; each supports `--json`.

For skill-source maintenance, run the applicable authoring checks:

```bash
bun run skills:check
bun run skills:check-vocab
bun run skills:check-steering
bun run skills:package-check
```

Read [project-rules/README.md](project-rules/README.md) before source changes. AEP's own design decisions remain in `docs/decisions/`; downstream ADRs live in `project-roadmap/decisions/`.
