# AEP 5.0 implementation and validation

This audit covers the native implementation on `feat/aep-5-native-cli`, based on design PR [#34](https://github.com/memorysaver/agentic-engineering-patterns/pull/34), merged at `b47d1431832c5e28a7e5175416e2f44a271c1709`. It records local verification and independent agent review. The branch prepares 5.0.0; it does not publish a release or migrate downstream production projects.

This is a historical report of the initial implementation. The subsequent
[CLI-focused validation](2026-09-09-aep-v5-cli-focus.md) records the user's scope
correction: dashboard integration is removed from v5, existing web source is
restored to its prior baseline, and native inspection and release remain independent.

## Implemented boundaries

The [accepted architecture](../decisions/aep-v5-rust-cli-architecture.md) is implemented in three Rust crates:

- [`aep-core`](../../crates/aep-core/src/lib.rs): versioned records, references, dependencies, scope/capacity, and gate freshness.
- [`aep-store`](../../crates/aep-store/src/lib.rs): canonical text stores, normalized paths, optimistic revisions, file locking, journaled transactions, and recovery.
- [`aep-cli`](../../crates/aep-cli/src/main.rs): JSON/exit contract, embedded guidance, context maintenance, process/Git/provider operations, specification interoperability, and migration.

The eight procedures under [`skills/native/`](../../skills/native/) share the binary release. `skills`, `skills show`, and reference reads work outside a repository with an empty PATH. The working agent selects procedures and the terminal action. No semantic route command, model dependency, or host-specific skill installation is present.

The [CLI contract](../workflow/aep-v5-cli.md) documents record creation/update, ADR acceptance/supersession, lessons/rule adoption, explicit configuration, worktree claims, checks/reviews, integration, publication, gates, and release promotion. The [migration runbook](../workflow/aep-v5-migration.md) documents current-context conversion, custom policy mapping, and the optional OpenSpec profile. The dashboard consumes Rust readiness through a validated JSON adapter.

## Local checks

Environment: Linux x86_64, Rust 1.98.0 (`88d9e12ae`, 2026-08-18), Bun 1.4.2. Test fixtures use disposable Git repositories and real local subprocesses. GitHub delivery fixtures use a mock `gh` provider and isolated local Git remotes.

| Check | Observed result |
| --- | --- |
| `cargo fmt --all --check` | Passed |
| `cargo clippy --locked --workspace --all-targets -- -D warnings` | Passed |
| `cargo test --workspace` | 38 passed: 6 CLI unit, 17 lifecycle, 6 core, 9 store; no failures |
| `cargo install --locked --path crates/aep-cli --root /tmp/aep-v5-install-check` | Installed release binary; `aep --version` reports 5.0.0 |
| Installed catalog and BDD reference with empty PATH, outside Git | Passed; JSON reports no side effects |
| Native archive with binary, LICENSE, and SHA-256 | Archive members and checksum generated/checked locally |
| Native guide frontmatter validation | All 8 guides passed the local skill-creator validator, using an isolated `uv` environment for PyYAML |
| `bun run skills:check` / `skills:check-vocab` | Passed; legacy generated resources and vocabulary remain consistent |
| `bun run skills:check-steering` | Passed: 64 negation lines, 0 hard imperatives in 35 checked files; legacy ceilings preserved |
| `bun run skills:package-check` | Passed: 24 legacy packages; existing 40 routing observations retained |
| `bun test packages/api/src/lib/project-ledger-loader.test.ts` | 2 tests passed |
| Targeted `oxlint` and `oxfmt` on six changed dashboard/API files | Passed |
| `bun run --cwd apps/web build` | Client and server production builds passed |
| Workflow YAML parse | Both workflow files parsed |
| `aep openspec check --reference-cli`, OpenSpec 1.12.0 | Passed on a native authored/exported fixture: 2 items, 0 failures (`change/native`, `spec/result`) |

`bun run check-types` is blocked by an untouched unused React import in `packages/ui/src/components/scroll-area.tsx`. A direct web TypeScript check also reports existing one-argument `z.record` calls in `packages/api/src/lib/product-context-schema.ts` and missing `cloudflare:workers` declarations in `packages/env/src/server.ts`. Those files were not changed by this implementation. The native adapter tests and dashboard build pass; this audit does not claim a clean monorepo typecheck.

A named `agent-browser` session exercised a synthetic project through the local API and web app. Root navigation selected Project Ledger, displayed Rust's missing-contract readiness reason, filtered by record kind, searched records, and displayed an empty result. The event view and rendered screenshot were inspected. No browser errors were reported. The browser and both owned development servers were closed afterward.

## Independent code review

Three read-only subagents reviewed separate areas and reproduced failures with disposable fixtures. Reported blockers were fixed and reviewed again. The final bounded rechecks reported no remaining blockers in their reviewed areas.

| Review area | Corrections checked |
| --- | --- |
| Core and store | Parent/child gate drift, new scope members and failed evidence, all linked contracts, failed dependency-container gates, unsafe/aliased paths, duplicate YAML keys, configuration drift/removal, `SAMPLE` roots, moved recovery journals, file modes, stale writes, and lock cleanup |
| Configuration and output | Repair invalid configuration against proposed constraints; block relocation of stores containing prose/procedures; include every linked change in review packets and PR descriptions |
| Workflow and delivery | Exact candidate before/after checks, dependency commit ancestry, terminal attempt guards, review-round carryover, explicit rule evaluation, environment evidence, provider lost-response recovery, preservation of concurrent PR prose, and actual integration-tree comparison |
| Unverified integration | Retain the actual integration receipt while blocking dependent dispatch and specification promotion when `tree_verified` is false |
| Specifications | Preserve surrounding text/order, rename before modify, reject prose/fenced BDD substitutes, preserve scenario identities, generate native Purpose and exported Why/What Changes sections, reject empty capability publication, and compare with pinned reference validation |
| Migration and memory | Git-bound source provenance, scoped retries, ID collisions, incremental change linking, quoted numeric layer ordering, repeated gate occurrences, split product maps, ADR unions, empty active selections, malformed arrays, custom constraints, mapping drift, and retained lesson links |

Permanent regressions live in the Rust crates and [`lifecycle.rs`](../../crates/aep-cli/tests/lifecycle.rs). Temporary reviewer fixtures were not committed. They add focused evidence beyond the permanent suite; they are not a production migration result.

## Guidance observations

One independent agent inspected the same fixed temporary project under four requests: inspect status, implement, review only, and authorized merge. It used the embedded catalog, a project-local procedure, context, dispatch planning, and delivery planning. File hashes, including Git state, remained unchanged during the observation.

| Request | Observed selection and boundary |
| --- | --- |
| Inspect | Read context/status without creating work |
| Implement | Selected implementation and validation guidance; used readiness facts |
| Review only | Selected validation/review guidance and stopped at review scope |
| Authorized merge | Selected delivery guidance; recognized missing attempt/evidence prerequisites |

The observation exposed missing reviewer-only guidance and missing host start/handoff instructions. The native guides now state those branches. The reviewer found the delivery terminal action easier to identify than in the combined legacy build procedure. This is a qualitative observation by one agent, not a model comparison, measured routing benchmark, or proof of autonomous production success.

## Operational limits and release evidence

Gate freshness is conservative. A release promotion changes scope records and can stale the gate that supported it. Reevaluate affected gates before using them for subsequent dispatch; the promotion receipt retains the proof used at promotion. An invalidated gate does not silently remain passing.

The OpenSpec common profile checks explicit requirement obligations and scenario steps. Optional strict reference checks also enforce OpenSpec's document completeness/length conventions. Imported custom schemas and changed mapping targets remain blocked until attributed mappings are renewed. Final-capability retirement and unsupported custom semantics require an explicit design instead of publishing an empty or guessed specification.

Local checks cover Linux. CI defines Linux x86_64, macOS arm64, and macOS x86_64 build/test/archive jobs; their remote results belong to the implementation PR checks. Windows is not a supported native target in this release. The CLI is not an OS sandbox or a reviewer identity service. Provider authentication and real downstream deployment checks were not exercised by local mocks.

The first remote macOS run found that export destination validation rejected the system `/var` temporary-directory alias. Destination resolution now accepts only macOS's fixed `/var` and `/tmp` aliases to `/private/var` and `/private/tmp`; project-controlled symlinks remain rejected. The existing export lifecycle fixture covers this platform regression, and the store fixture checks that other symlink parents stay blocked.

No v5 tag, package registry publication, production downstream cutover, or implementation PR merge was performed for this audit. The optional legacy marketplace stays at v4.1.0. The source repository keeps its own AEP design decisions under `docs/decisions/`; downstream ADRs use `project-roadmap/decisions/`.
