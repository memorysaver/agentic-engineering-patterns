# Agentic Engineering Patterns

AEP is a native CLI, `aep`, that keeps a software project's context, work and verification together so coding agents can plan, implement and deliver with evidence. The agent decides what to do; `aep` serves the procedures, maintains the records, isolates implementation in Git worktrees, runs the project's checks and records what actually happened.

Current release: **5.0.0-preview.1** (Linux x86_64, macOS Apple Silicon). The earlier skill bundle, v4, is kept for existing installs and is being retired; see [Coming from v4](#coming-from-v4).

## Install

```bash
curl -fsSL https://raw.githubusercontent.com/memorysaver/agentic-engineering-patterns/main/scripts/install.sh | bash
```

The script picks the archive for your platform from the newest v5 release, verifies its SHA-256, keeps the binary under `~/.aep/builds/<sha16>/` and links `~/.local/bin/aep`. `AEP_VERSION=v5.x.y` pins a tag; `AEP_HOME` and `AEP_BIN_DIR` change the locations. Runtime needs are `git` on `PATH` and nothing else: no Node, no LLM key, no per-agent skill copies.

To build from source, install the pinned Rust toolchain with rustup and run `cargo install --locked --path crates/aep-cli` from this checkout.

## Quick start

In a Git repository:

```bash
aep init            # .aep/config.toml, AGENTS.md, project-rules/README.md, .gitignore entry
aep doctor          # installation, project compatibility, available tools
aep --skill         # the agent entrypoint: how to work in this project
aep --skill project # set up stores, checks and the verification procedure
```

The generated `AGENTS.md` is two sentences: read `README.md`, run `aep --skill`. `aep init --claude` also writes a `CLAUDE.md` that points at `AGENTS.md`. Existing `AGENTS.md` content is kept and only gains that pointer if it lacks one. Running `init` again changes nothing.

From there the agent reads the procedure it needs (`aep --skill design`, `--skill implement`, and so on), creates records with `aep <kind> new --file`, and moves work through the lifecycle below. A human can follow along with `aep status`, `aep context <id>` and `aep timeline`.

## How it works

### Records and stores

Everything is a YAML or Markdown record with an ID, status, references and typed data, kept in plain directories that Git retains:

| Store | Default path | Holds |
| --- | --- | --- |
| Ledger | `project-ledger/` | Stories, changes with BDD specs, layers, waves, releases, gates, attempts, reviews, evidence, deliveries, events |
| Roadmap | `project-roadmap/` | Product direction, journeys, decisions (ADRs), published specifications |
| Rules | `project-rules/` | Project-specific code, testing and release rules, indexed for agents |
| Designs | `docs/design/` | Design drafts and research behind a change |
| Lessons | `lesson-learned/` | Observations that should change future work |

Stores are created when first needed. `aep check` validates every record, reference and specification and warns about containers with no members. Every write records an event with the affected IDs, status transitions and revision hashes; add `--note "<reason>"` to any write command and the reason is stored with that event.

### Lifecycle

The agent chooses skills and order; the CLI enforces structure.

1. **roadmap**: state direction, group a multi-story concept in a layer, record decisions with attribution.
2. **design**: write the change contract with BDD scenarios, accept it under the user's authority, create its stories.
3. **implement**: `aep dispatch start` checks readiness (dependencies, gates, capacity) and opens an isolated worktree for one attempt.
4. **validate**: `aep verify run` executes the project's configured checks in that worktree and records revision-bound evidence; policy can require an independent review.
5. **deliver**: `aep deliver merge` or `deliver pr` records the actual integration with a receipt tied to the verified tree, then `spec publish` and `change close`.
6. **reflect**: turn what was observed into lessons, rules or context updates.

Readiness comes from explicit dependencies, gates and accepted contracts, never from numbering or folder order. Delivery is recorded separately from intent, so `aep status` reports what is integrated, what is ready and why something is blocked.

### Commands

| Group | Commands |
| --- | --- |
| Start | `init`, `doctor`, `status`, `context`, `check` |
| Plan | `query`, `timeline`, `story`, `roadmap`, `change`, `decision`, `layer`, `wave` |
| Implement and verify | `dispatch`, `worktree`, `attempt`, `verify`, `review`, `gate` |
| Deliver and learn | `deliver`, `release`, `spec`, `lesson`, `reflect`, `rule` |
| Maintain | `config`, `migrate`, `openspec`, `recover` |
| Observe | `eval` |

Every command accepts `--json` for structured output, `--dry-run` for supported writes and `--note` for the reason. `aep --skill [name] [--ref name]` prints the embedded guidance; it is the only place agents read procedures from.

### Verification and policy

`.aep/config.toml` pins the CLI version, names the stores and declares the project's checks as explicit command arrays, together with policy: parallel attempt capacity, whether independent review is required, protected paths. Checks run inside the attempt's worktree and their results are stored as evidence bound to the revision they checked. Passing `aep check` proves structure; product behavior is proved by the project's own verification procedure, which `aep --skill project --ref verification-setup` helps establish.

### Observing a project from outside

`aep eval` is a neutral view of how a project is run. `aep eval snapshot` collects structural facts (records, verification policy, delivery receipts, Git state, legacy residue, CI and secret scanning) into a run under `~/.aep/eval/<project>/<run>/` and `aep eval report` applies fixed rules that name the engineering practice behind each finding. Nothing is written into the project. Under Herdr, `aep eval watch` starts a separate observer agent that follows `aep --skill eval`: it reads the working agent's transcript at each milestone, compares its claims with records and Git, and stores observations with `aep eval record`. The observer never sends input to the working agent and never judges product decisions or feature value.

The machine-level AEP home is `~/.aep` (`AEP_HOME` overrides it): `config.toml` from `aep eval init`, `eval/` for runs, and `builds/` from the installer.

## For coding agents

`aep init` writes an `AGENTS.md` that tells any agent to run `aep --skill` and follow the project's rules index. The guidance is embedded in the binary and versioned with it, so every agent working in the project reads the same procedures. Handoffs carry story, change, container and worktree identity; a later agent resumes from `aep context <id>` rather than from chat history.

## Coming from v4

v4 was a bundle of Agent Skills installed with `npx skills`. It is kept on the [`v4` branch](https://github.com/memorysaver/agentic-engineering-patterns/tree/v4) and its tags for existing installs, receives no new features, and is retired once downstream projects deliver on v5 and a v5 stable release exists. Details:

- [v4 legacy README and installation](docs/workflow/aep-v4.1-guide.md), including the `@v4.1.0` and `@v4` install refs
- [v4 sunset decision](docs/decisions/aep-v4-sunset.md)
- [Migrating a v4 project to v5](docs/workflow/aep-v5-migration.md): `aep migrate` converts useful legacy context into native records, preserves provenance and switches the workflow owner explicitly

A v4 project does not change behavior by installing the binary; the switch is an explicit migration.

## Documentation

- [Native CLI and configuration](docs/workflow/aep-v5-cli.md)
- [Migration and OpenSpec compatibility](docs/workflow/aep-v5-migration.md)
- [Architecture and accepted design](docs/decisions/aep-v5-rust-cli-architecture.md)
- [Human-readable CLI](docs/decisions/aep-v5-human-cli.md)
- [Skill classification: context and self verification](docs/decisions/aep-v5-context-and-self-verification.md)
- [Purpose-driven research and product context](docs/decisions/aep-v5-purpose-driven-research.md)
- [Layer and wave encapsulation](docs/decisions/aep-v5-layer-wave-encapsulation.md)
- [Context reverse edges and container completeness](docs/decisions/aep-v5-context-reverse-edges.md)
- [Delivery continuity and closure](docs/decisions/aep-v5-delivery-continuity.md)
- [Publication after integration context changes](docs/decisions/aep-v5-publication-reverification.md)
- [Preview adoption and v4/v5 coexistence](docs/decisions/aep-v5-preview-adoption.md)
- [Migration cleanliness and projects new to AEP](docs/decisions/aep-v5-migration-cleanliness.md)
- [Downstream pilots and live migrations](docs/audits/2026-09-10-aep-v5-downstream-pilots.md), [lessons](docs/lessons/)
- [Changelog](CHANGELOG.md)

## Develop AEP

The Rust workspace has three crates: `aep-core` (records, validation, readiness), `aep-store` (transactions, Git, worktrees) and `aep-cli` (commands, embedded guidance from `skills/native/`).

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
bun run skills:check-steering   # guidance stays within its steering ceilings
bun run skills:check            # generated resources are in sync
```

Releases are tagged `v5.*` on `main`; CI builds both platforms, publishes the archives with checksums and the installer picks them up. Read [project-rules/README.md](project-rules/README.md) before changing source. AEP's own design decisions live in `docs/decisions/`; a downstream project's ADRs live in its `project-roadmap/decisions/`.

## License

MIT. See [LICENSE](LICENSE).
