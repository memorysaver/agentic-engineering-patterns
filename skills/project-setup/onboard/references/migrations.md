# AEP Upgrade Migrations

One section per release, newest first, tracked from v4.1.0. On upgrade: read the marker on the first line of `AGENTS.md` (`<!-- aep-agents-template: vX.Y.Z -->`; a file with no marker counts as pre-v4.1.0), apply every section between that marker and the release you just pinned, in order, then rewrite the marker to the pinned release. Each section says what changed in AEP and what a downstream repository does about it. Steps marked _mechanical_ are safe to script; every other step is done by reading the file — nothing here overwrites hand-authored content.

## v4.1.0 — Fable 5.1 re-baseline, Project-Convention, two-round evaluator

**What changed in AEP** ([fable-5-1-behavioral-rebaseline](https://github.com/memorysaver/agentic-engineering-patterns/blob/main/docs/decisions/fable-5-1-behavioral-rebaseline.md), [project-convention-and-upgrade-path](https://github.com/memorysaver/agentic-engineering-patterns/blob/main/docs/decisions/project-convention-and-upgrade-path.md)):

- The evaluator loop caps at **two rounds for every tier**; round 2 is bought only by a `blocking` finding; `material` findings are fixed and attested without a round; the `standard → deep` auto-escalation is gone (`tier_escalated` is deprecated, always `false`).
- Every evaluator spawn pins **`high`** effort; `evaluator_effort` gains `high`, and `default`/`highest` are deprecated aliases.
- `/aep-launch` prepends an operating-agreement preamble to every workspace bootstrap.
- `AGENTS.md`, `CLAUDE.md`, and `Project-Convention/README.md` are shipped as templates by `/aep-onboard`, versioned by the marker on `AGENTS.md`'s first line. The `## Project Conventions (authoritative)` pattern of inlining conventions into `AGENTS.md` is retired.

**Downstream steps:**

1. _Mechanical._ Re-pin every runtime to v4.1.0 with the `/aep-onboard` Phase 1 commands; commit the skill files and `skills-lock.json`.
2. `AGENTS.md`. If absent, copy `templates/AGENTS.md.tmpl` as-is. If present:
   - remove the behavioral-guidelines block (`## 1. Think Before Coding` through `## 4. Goal-Driven Execution`, including the "These guidelines are working if" footer). Its "if uncertain, ask / if unclear, stop" lines contradict unattended operation on the current generation, and its per-step verify loop steers the main agent into local fixes;
   - replace the `## AEP Workflow` section with the template's (v4.1.0 pin, `aep-` prefix rule, `/aep-easy-explain` register);
   - delete `## Memory & Learning Loop` if present (v4.0.0 already stopped authoring it; `lessons-learned/` is the record);
   - replace `## Project Conventions (authoritative)` **and every section after it** with the template's `## Project Context` pointer line. Keep the removed text — step 4 moves it;
   - prepend `<!-- aep-agents-template: v4.1.0 -->` as the first line.
   The result should match the template section-for-section; the only project-specific text left is the project name in `## Project Context` if you add one.
3. `CLAUDE.md`. Where Claude Code is installed, the file is `@AGENTS.md`. A hand-authored `CLAUDE.md` is merged into `AGENTS.md` by hand first (project content goes to step 4), then replaced.
4. `Project-Convention/`. Copy `templates/Project-Convention/README.md.tmpl` to `Project-Convention/README.md`. Move the content removed in step 2 into one file per topic — `layout.md` (monorepo structure, where new code goes), `stack.md`, `commands.md`, `release.md`, `testing.md`, `admin.md`, … — and add one line per file under `## Conventions` in the README. Rules that only restate what the code already enforces can be dropped; rules with a reason stay, with the reason.
5. `docs/`. Apply the README's routing table: move root-level notes (`DEVELOPMENT.md`, `LICENSING.md`, setup guides) under `docs/setup/`; group loose top-level `docs/` directories into the categories; leave the AEP-owned directories (`docs/layer-gates/`, `docs/human-alignment/`, `lessons-learned/`, `openspec/`, …) where they are. Do this in a separate commit so the move is reviewable on its own.
6. `skills/e2e-test/scripts/derive-verification-recipe.sh` is **rendered from an AEP template**, so the re-pin alone leaves it at the old caps. Re-run `/aep-e2e-skill-scaffolding` (upgrade mode) to regenerate it, or patch by hand: `standard` and `deep` → `MAX_ROUNDS=2; EFFORT="high"`, the negative-assertion-delta line → `EFFORT="high"`. Existing `.dev-workflow/verification-recipe.json` files need no edit — the protocol's cap governs and the old effort values are read as `high`.
7. _Mechanical._ Verify with `/aep-scaffold`'s `scripts/audit.sh`: `AGENTS.md` carries the v4.1.0 marker, `CLAUDE.md = @AGENTS.md` where owed, `Project-Convention/README.md` present. Commit the instruction files together with the re-pin.
