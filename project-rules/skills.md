# Skill Corpus Conventions

The rules for authoring skills live in `docs/decisions/skill-authoring-standard.md` (R1–R9), `docs/decisions/claude-5-context-engineering.md` (C1–C6), and `docs/decisions/fable-5-1-behavioral-rebaseline.md` (F1–F8). Read them before editing anything under `skills/`. This file is the short form: what each rule means day to day and which check enforces it.

## Single source, generated copies

Shared skill resources live once in `skills/product-context/_shared/{references,templates}/`. The per-skill copies are generated and marked by a `.aep-generated` manifest. Edit `_shared/`, then run `bun run skills:build`; the pre-commit hook and CI regenerate and verify those copies, so hand edits to a generated file do not survive. Check: `bun run skills:check`.

A `SKILL.md` mention of a shared file's path (`references/<file>`, `scripts/<file>`) is the declaration that materializes it into that skill. Trimming the mention un-declares the copy: `--check` reports the managed-file set as stale, and the next full build deletes the file — which breaks a sibling script that imports it (`validate-state.mjs` and `validate-signal.mjs` import `scripts/json-schema.mjs`). After editing a `SKILL.md` that has generated siblings, run the build and read `git status` for deletions before committing.

## One vocabulary

Enumerated values that more than one skill reads are declared once in `skills/product-context/_shared/references/aep-vocabulary.schema.json`. Every other copy is tagged (`x-aep-vocab` in schemas, `(aep-vocab: <name>)` in prose). Changing an enum means updating every tagged copy in the same change; a deprecated value stays in the enum with its alias named so earlier artifacts still validate. Check: `bun run skills:check-vocab`.

## Steering ceilings

A prohibition in a `SKILL.md`, or in a reference file that is spawned as a prompt, needs a machine check behind it. Per-file ceilings on negations and imperatives live in `evals/steering-baseline.json`; a file may fall below its entry, and raising one is a reviewed decision, not a fix. `SKILL.md` stays under 400 lines (CI warns above 400 and fails above 500). Check: `bun run skills:check-steering`.

## Descriptions are routing metadata

A skill's frontmatter `description` loads into every downstream session. Changing it or a trigger requires re-recording `evals/skill-routing-observations.json` and staying within the description cap enforced by `scripts/check-skills-package.sh`. Check: `bun run skills:package-check`.

## Verification

For skills: `bun run skills:check`, `skills:check-vocab`, `skills:check-steering`, `skills:package-check`; the fixture suites are the `bun run skills:test-*` scripts. For TypeScript: `bun run check` and `bun run check-types`. CI runs the same set from `.github/workflows/skills-check.yml`. New fixtures go beside the existing `scripts/test-*.sh` suites, not into a new harness.

## Native 5.0 guidance

Native procedures under `skills/native/` are embedded by the CLI build and share its version. Their catalog is selected by the working agent, not a deterministic task router. Catalog/show/reference fixtures validate the installed output; agent selection observations are recorded separately in `docs/audits/2026-09-09-aep-v5-implementation.md`.

For native preview, the accepted `docs/decisions/aep-v5-context-and-self-verification.md` and `aep-v5-preview-adoption.md` supersede legacy F1–F3 fixed evaluator topology and R3 slash-command routing. Use `aep skills show <name>` for native cross-skill discovery. Preserve legacy F1–F3/R3 in the v4 corpus. Shared principles still apply: concise progressive disclosure, canonical resources, observable results, source attribution, and honest capability limits. Native guidance may select same-agent execution or independent review according to the task and explicit project policy.

Project-owned verification for this source repository is documented in [verify-aep](skills/verify-aep/SKILL.md). It exercises an actual candidate executable against a disposable legacy project and retains evidence outside the cleaned fixture.

The legacy package check validates the marketplace-declared v4.1 catalog and installs that exact name list. Native procedures are not added to legacy routing observations or runtime packages. Both corpora remain subject to line budgets and per-file steering ceilings. New native ceilings document the limited rules for request scope, stale evidence, and review independence; Rust fixtures enforce mechanical constraints.

Native regression fixtures belong with their Rust crates. Run `python3 ~/.codex/skills/.system/skill-creator/scripts/quick_validate.py skills/native/<name>` for changed native skill frontmatter when that local authoring tool is available; CLI package tests verify the portable artifact.
