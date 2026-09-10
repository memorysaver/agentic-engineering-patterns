---
name: project
description: Install or inspect project entrypoints, assemble project context, and establish the project's runtime verification procedure and checks.
---

# Project setup

Establish the selected workflow from the user's explicit choice, then the project's default. Preserve installed legacy skills and unrelated instructions. The native binary's version identifies its guidance; the entrypoint identifies which workflow owns this task. Carry that choice into handoffs and report missing capabilities explicitly.

Inspect the repository, host discovery, scripts, and Git changes. Read [Context sources](references/context-sources.md) when onboarding or repairing missing project knowledge. Use `aep doctor` for CLI/configuration compatibility; `aep init --dry-run` previews minimal setup and `aep init` applies it. Add `--claude` when that host needs a CLAUDE.md pointer. Existing legacy stores require the migration procedure (`aep skills show migrate`) for an authorized cutover.

Classify existing AGENTS content yourself. Keep the agreement and selection entrypoint short; index applicable code, testing, package, DevOps, release, and workflow rules in the configured rules store, normally `project-rules/`. Preserve project constraints while identifying historical workflow instructions as sources.

Read [Verification setup](references/verification-setup.md) when establishing or repairing how agents operate this product. Reuse actual stack, setup, fixtures, and hand-written journeys; scaffold the missing capabilities within task scope. Rehearse the resulting procedure on a representative path and retain its results.

Configure executable checks as command argument arrays, directories, timeouts, and required environment variable names. Inspect `aep config show --json`; submit settings through `aep config update --file <file> --expect <revision>`. Populated store moves use an explicit migration.

Verify instruction discovery, applicable rules, local procedure discovery via `aep skills`, and `aep check`. A second init should produce no diff. Report selected workflow, working verification entrypoint, observed results, and remaining setup gaps.
