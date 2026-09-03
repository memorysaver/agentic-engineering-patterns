# Project Convention and the Upgrade Path: One Template, One Marker, One Directory

> **Status:** Accepted — implemented on branch `release/v4.1.0` as **v4.1.0**, alongside [fable-5-1-behavioral-rebaseline.md](fable-5-1-behavioral-rebaseline.md). It changes what `/aep-onboard` and `/aep-scaffold` write into a downstream repository and how a downstream repository upgrades, so it lives in `decisions/` per the [docs routing guide](../README.md).
>
> **Sourcing note:** the owner's direction (2026-09-03) after reviewing this repository's own `AGENTS.md` (commit `9b281fd`), the `AGENTS.md` files of looplia and 91app-agent-platform, and their `docs/` trees.

## Diagnosis

**Three places author `AGENTS.md` wording, none of them versioned.** `/aep-onboard` Phase 1 describes an `## AEP Workflow` section in prose; the README's agent prompt (step 4) carries a second wording; `/aep-scaffold`'s converge flow (category A) creates `CLAUDE.md = @AGENTS.md`. Nothing records which release a downstream `AGENTS.md` was written against, so `audit.sh` greps for the prose `pinned at **vX**` — and 91app-agent-platform's says v2.1.0 while its `skills-lock.json` is far newer. An upgrade therefore has no hook: re-pinning the skills leaves the instruction files as they were.

**The instruction files carry the wrong content for the current generation.** Both downstream repos open `AGENTS.md` with a downstream-authored "Agent behavioral guidelines" block (Think Before Coding / Simplicity First / Surgical Changes / Goal-Driven Execution). Two of its lines — "If uncertain, ask" and "If something is unclear, stop. Name what's confusing. Ask." — are the opposite of the Fable 5.1 guidance for autonomous work, and "loop until verified" with per-step checks steers the main agent into small local fixes. AEP never shipped that block (no copy exists in this corpus), so removing it is a migration step, not a corpus edit.

**Conventions are inlined, so `AGENTS.md` grows and the repo layout drifts anyway.** Each downstream `AGENTS.md` carries a `## Project Conventions (authoritative)` section followed by monorepo layout, stack, commands, release flow, and more (91app: ~370 lines). Meanwhile `docs/` has no routing rule: 91app's has more than thirty top-level directories (research, prototypes, postmortems, fixes, decisions, all peers); looplia keeps `DEVELOPMENT.md` and `LICENSING.md` at the root and `docs/` beside `product/`, `calibration/`, `project-memory/`, `lessons-learned/`. Every session pays for the whole `AGENTS.md`, and the parts that would answer "where does this file go" are the parts nobody wrote.

## The standard (normative)

- **P1 — One template, shipped by `/aep-onboard`.** `skills/project-setup/onboard/templates/` holds `AGENTS.md.tmpl`, `CLAUDE.md.tmpl`, and `project-convention/README.md.tmpl`. The README agent prompt and `/aep-scaffold`'s converge flow point at these; neither carries its own wording. `AGENTS.md.tmpl` is the *generic* working agreement (project context, working agreement, engineering principles, tools and verification, communication) plus two short pointers: `## AEP Workflow` (the pinned release, the `aep-` prefix, the `/aep-easy-explain` register) and `## Project Conventions`, which says only: conventions live in `project-convention/`; read its README first and follow the links that apply to the task. Nothing project-specific is inlined.
- **P2 — A version marker is the upgrade hook.** The template's first line is `<!-- aep-agents-template: vX.Y.Z -->`. `audit.sh` reads it (replacing the prose grep). `/aep-onboard` `references/migrations.md` keeps one section per release, starting at v4.1.0; an upgrade applies every section between the file's marker and the installed release, then rewrites the marker. A file with no marker is treated as pre-v4.1.0.
- **P3 — `project-convention/` is the convention surface, by reference.** The directory holds a README (the index) and one file per convention. The README opens with the AEP workflow (this project runs on `/aep-*`; start with `/aep-onboard`; the loop is product context → dispatch → design → launch → build → wrap), then the documentation rule (**all documents live under `docs/`**, with a routing table adapted from this repository's `docs/README.md`), then the project-record directories AEP owns and a downstream must not reorganize (`product-context.yaml`, `product/`, `maps/`, `openspec/`, `lessons-learned/{,process,retrospectives,distillations}`, `calibration/`, `dogfood-output/`, `docs/layer-gates/`, `docs/human-alignment/`, `skills/e2e-test/`, `.dev-workflow/`), then the root-stays-clean rule (only `README.md`, `AGENTS.md`, `CLAUDE.md`, `CHANGELOG.md`, `LICENSE`, and tool config at the root; setup and licensing notes go under `docs/`), then how to add a convention (one file, one-line link from the README, nothing inlined into `AGENTS.md`). Progressive disclosure is the design: a session reads the README's index lines and opens only the files its task touches.
- **P4 — Migration is model-driven and never clobbers.** Moving a downstream's existing `## Project Conventions (authoritative)` content into `project-convention/<topic>.md` files, and removing the behavioral-guidelines block, is done by the onboarding agent reading the file — `converge.sh` stays mechanical and only reports. A hand-authored `CLAUDE.md` is flagged for manual merge, as today.
- **P5 — This repository dogfoods the template.** AEP's own `AGENTS.md` is the template with the marker; its repository conventions live in `project-convention/README.md`.

## The v4.1.0 migration plan (what `migrations.md` carries)

1. Re-pin every runtime to v4.1.0 (the existing `/aep-onboard` Phase 1 commands).
2. `AGENTS.md`: if absent, write it from the template. If present, remove the behavioral-guidelines block (`## 1. Think Before Coding` … `## 4. Goal-Driven Execution` and the "These guidelines are working if" footer), replace the `## AEP Workflow` section with the template's (v4.1.0 pin, `aep-` prefix, easy-explain register), delete `## Memory & Learning Loop` (v4.0.0 already stopped authoring it), replace `## Project Conventions (authoritative)` and everything after it with the template's pointer, and prepend the marker.
3. `CLAUDE.md`: `@AGENTS.md` when Claude Code is installed; a hand-authored file is merged into `AGENTS.md` by hand first.
4. `project-convention/`: create `README.md` from the template. Move the removed convention content into one file per topic (`layout.md`, `stack.md`, `commands.md`, `release.md`, …) and link each from the README with one line.
5. `docs/`: apply the routing table. Move root-level notes (`DEVELOPMENT.md`, `LICENSING.md`, …) under `docs/setup/`; leave AEP-owned directories where they are.
6. `skills/e2e-test/scripts/derive-verification-recipe.sh` is **rendered from an AEP template** — a re-pin alone does not update it. Re-run `/aep-e2e-skill-scaffolding` in upgrade mode (or patch `MAX_ROUNDS`/`EFFORT` by hand) so the two-round cap and pinned effort actually apply.
7. Verify: `audit.sh` reports the marker current, `project-convention/README.md` present, `CLAUDE.md = @AGENTS.md`; commit the instruction files with the re-pin.

## Anti-patterns this prevents

- **The lying pin.** An `AGENTS.md` that says one version while the installed bytes are another.
- **The kitchen-sink instruction file.** Every session paying for release flow and admin runbooks it will never touch.
- **Root sprawl.** Notes that live at the repository root because no rule said where else.
- **The fourth copy.** A new place that authors `AGENTS.md` wording.

## References

- [fable-5-1-behavioral-rebaseline.md](fable-5-1-behavioral-rebaseline.md) — the execution-plane half of the same release; F5 is why the bootstrap does not repeat the generic agreement.
- [claude-5-context-engineering.md](claude-5-context-engineering.md) — C5 (`/aep-onboard` stops authoring memory-loop prose) is completed by step 2 above.
- `docs/README.md` — the routing table the `project-convention` README adapts.
- looplia and 91app-agent-platform `AGENTS.md` and `docs/` trees (2026-09-03) — the diagnosis.
