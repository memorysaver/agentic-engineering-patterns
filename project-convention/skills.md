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
