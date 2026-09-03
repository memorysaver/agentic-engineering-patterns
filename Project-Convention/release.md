# Release Conventions

Every bump of `metadata.version` in `.claude-plugin/marketplace.json` ships with a matching `CHANGELOG.md` entry in the same PR, under `[X.Y.Z] - YYYY-MM-DD`. Additive capability is a minor bump, fixes are a patch, and breaking a skill contract is a major bump; a deprecated alias that keeps earlier artifacts valid keeps a change minor. Tag `vX.Y.Z` on merge to `main`.

Downstream projects see nothing until they re-pin. From v4.1.0 every release that touches the instruction files or a rendered template carries a section in `skills/project-setup/onboard/references/migrations.md`; a release that touches neither still adds a one-line section saying so, so the marker on a downstream `AGENTS.md` can move forward without guesswork.

Version strings to update on a release: `.claude-plugin/marketplace.json`, `CHANGELOG.md`, `README.md` (quick start, agent prompt, common commands), `skills/project-setup/onboard/SKILL.md` Phase 1, and the `AGENTS.md.tmpl` marker and `## AEP Workflow` line when the template changed.
