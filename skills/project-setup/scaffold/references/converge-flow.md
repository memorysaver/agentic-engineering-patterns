# Existing-Project Converge Flow

How to read `scripts/audit.sh` output and apply the converge steps. Read this when running the
existing-project branch (Phases 0E–5E). The flow is **idempotent**: run it to onboard an existing project
**or** re-run it later to repair **drift** toward the current AEP standard. It **reports first, asks, then
converges** — and **never overwrites hand-authored content** (journeys, specs, prose). A fully-converged
project re-running the flow is a no-op ("already up to date").

The AEP plugin install contract is **plain per-agent installs**: the skills CLI copies real `aep-*` dirs
into `.claude/skills/` (claude-code) and/or `.agents/skills/` (codex), and either side alone is a complete
install. A symlink layout already in place (`.claude/skills/aep-*` → `../../.agents/skills/aep-*`) stays
legal — this flow verifies health and normalizes toward nothing. The runtime roots
themselves must be real directories when present (a whole-directory alias fails closed). Project-owned
skills are unchanged: `/aep-e2e-skill-scaffolding` owns that layout (real `skills/<name>/` dir +
`.claude/skills/<name>` and `.agents/skills/<name>` symlinks into both runtimes).

---

## Reading the audit (Phases 0E–1E)

`scripts/audit.sh` is read-only. It prints, in one run:

- **Phase 0E — stack + pin.** Detected language, package manager, monorepo tool, backend/frontend signal,
  and the AEP pin vs latest release. If the package manager is undetected, recommend **bun** (TS/JS) or
  **uv** (Python). The frontend signal sets the default e2e `target`: React Native → mobile;
  Tauri/Electrobun → desktop; else web.
- **Phase 1E — drift, grouped by category.** `[ok]`/`[DRIFT]` for **A** skills layout, **B** e2e shape,
  **C** infrastructure; `[detected]`/`[ ]` for **D** observability. The script **exits non-zero while any
  `[DRIFT]` remains** and `0` once clean — that exit code is the convergence gate (Phase 4E loops on it).
  Category B is three-valued: `canonical` (ok), `real-non-bdd`/`thin-legacy`/`absent` (each a distinct
  drift with its own converge action). Categories D and E never count toward the exit code.

**Observability → telemetry candidates (D).** For each `[detected]` tool, record a **candidate** entry
under `topology.routing.telemetry_sources` (`kind` + a `token_env` name — never the secret; leave
`endpoint`/`metric_map` for `/aep-map`). If nothing is detected, note it so `/aep-map` knows quantitative
metrics may need a tool added or stay qualitative.

---

## Report + confirm (Phase 2E)

Present the audit as a **current → target** summary grouped by category (A skills layout, B e2e shape,
C infra, D observability, E version pin). For each category with drift/gaps, list the **proposed change**
and ask the user which to apply. **Default = fix all drift + gaps.** Use a per-category checklist (e.g. the
AskUserQuestion-style confirm). Only confirmed categories are converged.

---

## Converge (Phase 3E) — apply only confirmed changes

Each step is a no-op when already satisfied.

- **A. Skills layout + instruction files** — `scripts/converge.sh` verifies the layout is healthy (plain
  per-agent installs or an existing symlink layout) and creates `CLAUDE.md = @AGENTS.md` only when a
  Claude skill install exists and the file is absent (a hand-authored `CLAUDE.md` is flagged for manual
  merge, never clobbered). The two instruction-file checks — the
  `<!-- aep-agents-template: vX.Y.Z -->` marker on `AGENTS.md` and `project-convention/README.md` —
  are **model-driven**: follow `/aep-onboard` Phase 1.5 and its `references/migrations.md` from the
  file's marker to the pinned release (the script reports them and changes nothing). It
  moves, deletes, and links nothing: aliased runtime roots and foreign links fail closed, and **version
  skew** — both agents carrying real but different copies of one skill — fails closed toward the category
  E re-pin. Project-owned skill exposure (real `skills/<name>` + both symlinks) is handled by the skill's
  own generator — for e2e-test that's `/aep-e2e-skill-scaffolding` (next step).
- **B. E2E-test skill** — delegate to **`/aep-e2e-skill-scaffolding`**. It creates (absent) or upgrades
  (thin-legacy / real-non-bdd → BDD) the skill in canonical cross-tool form, idempotently, migrating a
  legacy `.claude/skills/e2e-test` real dir into `skills/` first.
- **C. Infrastructure (fill gaps)** — for each `[DRIFT]`/missing item, generate it, never overwriting
  existing files:
  - **Git repo:** `git init -b main && git add -A && git commit -m "chore: initial commit"`. AEP
    auto-detects the integration branch (`main` in single-branch mode) — see `/aep-git-ref`
    "Integration Branch"; the standard case needs no config override.
  - **OpenSpec:** follow `/aep-scaffold` **Phase 5: Initialize OpenSpec**.
  - **Workspace hook:** follow `/aep-scaffold` **Phase 7** using the detected stack.
  - **Gitignore:** applied by `scripts/converge.sh` (`.dev-workflow/`, `.feature-workspaces/`).
- **E. Version pin — detect and recommend only.** Re-pinning AEP is a deliberate, own-PR action
  (README), so `scripts/converge.sh` only **prints** the `npx skills add@<newtag>` commands for the user
  to run themselves.

---

## Verify + commit (Phases 4E–5E)

Re-run `scripts/audit.sh` until it **exits 0** — every confirmed category then reads `[ok]`. A
fully-converged project re-running the flow produces no changes (idempotent). Then commit:
`git add -A && git commit -m "feat: converge agentic development infrastructure"`.
