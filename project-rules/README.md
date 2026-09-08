# Project rules

Read this index, then the files relevant to the task. Keep project-specific code, testing, packaging, DevOps, and release rules here. `AGENTS.md` carries the short working agreement and guidance entrypoint.

This repository authors AEP. `crates/` owns the Rust CLI; `skills/native/` supplies embedded instructions; the other skill directories supply the optional legacy v4.1 plugin. `apps/` and `packages/` hold the dashboard and companion apps. Source changes use design documents and implementation branches; do not create a synthetic downstream story loop for repository maintenance.

- [rust.md](rust.md): native ownership, evidence, platform boundaries, and checks.
- [skills.md](skills.md): skill authoring, generated resources, vocabulary, and observation evidence.
- [release.md](release.md): native/legacy versions, changelog, packaging, and release operations.

AEP design decisions stay in `docs/decisions/`. [docs/README.md](../docs/README.md) routes the other documentation. Downstream project ADRs go in `project-roadmap/decisions/`.

Add a rule with its reason and check, then index it here. Project-owned skills can live in `project-rules/skills/<name>/SKILL.md` or in paths registered by `.aep/config.toml`.
