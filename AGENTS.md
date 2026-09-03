# AGENTS.md

## Project Context

- This repository is Agentic Engineering Patterns (AEP), an Agent Skills plugin. The product is the skill corpus under `skills/`; `apps/` and `packages/` hold the companion Turborepo apps and shared packages.
- Read `README.md` before starting work to understand the project's purpose and concepts. `docs/README.md` routes documentation by category; `docs/glossary.md` fixes the vocabulary.
- Treat the repository's documentation and code as the source of truth. Do not invent architecture, commands, conventions, or requirements.
- The rules for authoring skills live in `docs/decisions/skill-authoring-standard.md` (R1–R9) and `docs/decisions/claude-5-context-engineering.md` (C1–C6). Read them before editing anything under `skills/`.

## Working Agreement

- The user's request or approved plan defines the deliverable. Do not silently narrow, widen, or replace it.
- Work autonomously on reversible actions within scope. Ask only when materially different interpretations would change the result, or when an action is destructive or outside scope.
- When the user asks for analysis or diagnosis, report findings without applying a fix unless requested.
- Complete all work that does not depend on missing input. If blocked, state exactly what remains and why.
- Do not fix, optimize, or extend unrelated behavior. Report noteworthy issues as follow-ups.
- Do not stop after announcing a next step. Execute it, verify the result, and finish the requested work.

## Engineering Principles

- Follow repository conventions and the surrounding code.
- Make the smallest coherent change that fully satisfies the request.
- Make surgical edits when they produce the same result as rewriting a file.
- Before changing state, confirm that the evidence supports that specific change.

## Repository Conventions

- Shared skill resources live once in `skills/product-context/_shared/{references,templates}/`. The per-skill copies are generated and marked by a `.aep-generated` manifest. Edit `_shared/`, then run `bun run skills:build`; the pre-commit hook and CI regenerate and verify those copies, so hand edits to a generated file do not survive.
- Enumerated values that more than one skill reads are declared once in `skills/product-context/_shared/references/aep-vocabulary.schema.json`. Every other copy is tagged (`x-aep-vocab` in schemas, `(aep-vocab: <name>)` in prose) and checked by `bun run skills:check-vocab`. Changing an enum means updating every tagged copy in the same change.
- A prohibition in a `SKILL.md` needs a machine check behind it. Per-skill ceilings on negations and imperatives live in `evals/steering-baseline.json`; raising an entry is a reviewed decision, not a fix. `SKILL.md` stays under 400 lines (CI warns above 400 and fails above 500).
- A skill's frontmatter `description` is routing metadata that loads into every downstream session. Changing it or a trigger requires re-recording `evals/skill-routing-observations.json` and staying within the description cap enforced by `scripts/check-skills-package.sh`.
- Changes to how AEP works start with a decision document in `docs/decisions/` (design only, no schema or skill edits), followed by implementation PRs reviewed against it.
- Every bump of `metadata.version` in `.claude-plugin/marketplace.json` ships with a matching `CHANGELOG.md` entry in the same PR. Additive capability is a minor bump, fixes are a patch, and breaking a skill contract is a major bump. Tag `vX.Y.Z` on merge to `main`; downstream projects see nothing until they re-pin.

## Tools and Verification

- First privately identify what information is needed; gather independent items in parallel when possible.
- Inspect relevant files before editing. Prefer `rg` and `rg --files` for searches.
- Verify changes in proportion to their risk using the repository's existing checks. For skills: `bun run skills:check`, `bun run skills:check-vocab`, `bun run skills:check-steering`, and `bun run skills:package-check`; the fixture suites are the `bun run skills:test-*` scripts. For TypeScript: `bun run check` and `bun run check-types`. CI runs the same set from `.github/workflows/skills-check.yml`.
- Add permanent tests only when requested or when the repository already tests that kind of behavior. Keep them focused on the requested behavior. Here that means a fixture beside the existing `scripts/test-*.sh` suites, not a new harness.
- Use temporary checks when useful, but do not commit disposable artifacts. Scratch files belong outside the repository.
- Never claim a command or test passed unless it was run successfully. Report the exact checks performed and any checks that could not run.

## Communication

- Before substantial work, briefly state what you will inspect or change. Provide concise progress updates during long tasks.
- End with a self-contained summary of what changed, what was verified, and any remaining issue.
- Cite retrieved sources near the claims they support. Paraphrase by default and clearly mark direct quotations.
- Use structure only when it improves clarity or the user requests it.
- Please remove all mannered prose.
