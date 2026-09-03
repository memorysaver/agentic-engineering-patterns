# Project Conventions

This directory is the project's convention surface. `AGENTS.md` carries the generic working agreement; everything specific to this repository lives here, one file per topic, linked from this index. Read this file first; open only the files the task touches.

## How this project runs

This repository is AEP itself. The product is the skill corpus under `skills/`; `apps/` and `packages/` hold the companion Turborepo apps and shared packages. The `/aep-*` loop skills are authored here and run in downstream projects, so there is no `product-context.yaml` and no story loop in this repository. Changes to how AEP works start with a decision document in `docs/decisions/` (design only), followed by implementation commits reviewed against it, and ship as a tagged release.

## Where documents go

All documents live under `docs/`. `docs/README.md` is the routing guide: decisions, workflow, tech-stack, lessons, plans, research, audits, articles, each with its naming convention. `docs/glossary.md` fixes the vocabulary. The repository root holds only `README.md`, `AGENTS.md`, `CLAUDE.md`, `CHANGELOG.md`, `LICENSE`, and tool configuration.

## Conventions

- [`skills.md`](skills.md) — authoring rules for the skill corpus: the shared-source build, the vocabulary schema, steering ceilings, routing evals, and the checks that enforce them.
- [`release.md`](release.md) — version bump, changelog, tag, and downstream propagation.

## Adding a convention

Write it as a file in this directory: the rule, its reason, and how it is checked. Link it above with one line. Nothing project-specific goes into `AGENTS.md`.
