# AEP 5 CLI scope and validation

The user's [scope correction](../decisions/aep-v5-cli-first.md) defers dashboard
implementation and packaging so the current deliverable can focus on native AEP.
This report covers the local follow-up to `34e5b9a` on `feat/aep-5-native-cli`.

## Resulting scope

- Remove the v5 Ledger page, API adapter, mode switching, and dashboard workflow.
  Existing `apps/`, `packages/`, `package.json`, and `bun.lock` match the pre-v5
  baseline `b47d143`; retaining that source is not a v5 compatibility feature.
- Remove `aep dashboard` and its consumer-specific tests. Native inspection stays
  available through `status`, `query`, `context`, and `timeline`, with JSON output.
- Preserve native record/configuration operations, contracts/specifications,
  migration, worktrees/attempts, verification/reviews/gates, delivery, and learning.
  The copied-binary regression now verifies ordinary native inspection.
- Keep native release jobs independent of web builds. The release artifact
  contains `aep` and `LICENSE`, with a separate SHA-256 checksum.

## Checks

| Check | Observed result |
| --- | --- |
| `cargo fmt --all --check` | Passed |
| `cargo clippy --locked --workspace --all-targets -- -D warnings` | Passed |
| `cargo test --locked --workspace` | 39 passed: 6 CLI unit, 18 lifecycle, 6 core, 9 store |
| `git diff --exit-code b47d143 -- apps packages bun.lock package.json` | Passed; existing web source and dependencies are restored exactly |
| Native workflow YAML parse | Passed; only native build and release jobs remain |
| `git diff --check` | Passed |

A temporary source tree contained only Cargo manifests/lockfile, the pinned
toolchain file, license, Rust crates, and `skills/native/`. It contained no
`apps/`, `packages/`, `package.json`, or `node_modules`. An offline locked
`cargo install` from that tree installed the final CLI. After renaming the build
source directory, the installed binary still served embedded guidance with empty
PATH. A disposable Git project then ran migration plan/apply/verify, `check`,
status, query, context, and timeline with only Git on PATH. Modifying the obsolete
product-context file did not change native status. Help omitted dashboard, and
invoking that removed command returned usage exit 2. A local archive's members
and SHA-256 were verified.

One independent read-only reviewer inspected native command-family coverage and
the dashboard removal. It found concrete implementations for all mandatory native
families and no material native regression. Its separate copied-binary fixture
verified status/query/context/timeline revisions, records, source content, and
unchanged file hashes across reads. This is bounded source/fixture evidence, not
a claim that every production workflow has been exercised.

## Remaining release and adoption evidence

Read-only inspection of [PR #35](https://github.com/memorysaver/agentic-engineering-patterns/pull/35)
showed Linux x86_64, macOS arm64, and macOS x86_64 native checks passing for
`34e5b9a`. Those results precede this local scope correction; the final candidate
still needs its own CI run.

The subsequent [skill migration inventory](2026-09-10-aep-v5-skill-migration-inventory.md)
identifies missing guidance and contracts across the 24 legacy capabilities.
This audit's command-family review does not establish skill behavior parity.
The [implementation plan](../plans/aep-v5-implementation.md) tracks that work
alongside a representative downstream pilot with real project policy/checks,
final candidate platform CI, and an authorized version tag/artifact publication.
GitHub provider tests remain mocks; no production downstream migration or release
publication was performed here. Existing web typecheck issues and richer UI work
are outside this native deliverable.
