# Agentic Engineering Patterns

AEP 5.0 is a Rust CLI for project context, isolated implementation, verification, and learning. Agents choose the work and skills. The CLI serves instructions, maintains records, and checks declared constraints.

This branch prepares **5.0.0**. The published legacy skill release remains **v4.1.0** until the native release is tagged.

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

The installation needs the binary and a short `AGENTS.md` entrypoint. `aep init --claude` also adds an `@AGENTS.md` pointer. Instructions and references are embedded in the binary. Node, an LLM key, an OpenSpec CLI, and per-runtime skill copies are optional.

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

Use `aep status`, `aep context <id>`, `aep query`, and `aep timeline` to inspect recorded facts. Use structured files or `--file -` to author records. `aep check` validates the project; verification runs its configured commands in the bound worktree. Standard/deep work retains independent review and a two-round limit. Delivery records actual integration separately from accepted intent and published specifications.

- [Native CLI and configuration](docs/workflow/aep-v5-cli.md)
- [Migration and OpenSpec compatibility](docs/workflow/aep-v5-migration.md)
- [5.0 architecture and accepted design](docs/decisions/aep-v5-rust-cli-architecture.md)
- [Implementation and validation report](docs/audits/2026-09-09-aep-v5-implementation.md)
- [Legacy v4.1 installation and skills](docs/workflow/aep-v4.1-guide.md)

## Develop AEP

The Rust crates own the native runtime. `skills/native/` contains its embedded guidance. The existing plugin skill corpus remains available for projects pinned to v4.1. The dashboard is an optional TypeScript consumer; native readiness stays in Rust.

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
bun install --frozen-lockfile
bun run skills:check
bun run skills:check-vocab
bun run skills:check-steering
bun run skills:package-check
bun test packages/api/src/lib/project-ledger-loader.test.ts
bun run --cwd apps/web build
```

For the local dashboard, set `AEP_PROJECT_ROOT` to an initialized project and `AEP_BINARY` to the built binary, then run `bun run dev:dashboard`. The project ledger view reads `aep dashboard --json`. Existing v4.1 views remain available for `PRODUCT_CONTEXT_PATH` consumers.

Read [project-rules/README.md](project-rules/README.md) before source changes. AEP's own design decisions remain in `docs/decisions/`; downstream ADRs live in `project-roadmap/decisions/`.
