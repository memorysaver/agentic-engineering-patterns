---
name: aep-scaffold
description: Creates a new AEP project or converges an existing project's agentic infrastructure. Use for new project, scaffold, infrastructure setup, or skills-layout repair; use /aep-onboard for installation and orientation.
---

# Scaffold

Set up a project for agentic development — either by scaffolding a new monorepo or by onboarding/converging an existing project. Both paths produce a project with OpenSpec, a workspace setup hook, and the canonical BDD e2e-test skill. The existing-project path is an **idempotent audit → confirm → converge** that also repairs drift on re-run.

---

## Mode Selection

Detect whether this is a new or existing project:

```bash
# Check for existing project markers
ls package.json pyproject.toml Cargo.toml go.mod 2>/dev/null
```

- **New project** — empty or near-empty directory, no project config files
  → [New Project Flow](#new-project-flow) (Phase 1-8)
- **Existing project** — has source code and config files
  → [Existing Project Flow — audit → confirm → converge](#existing-project-flow--audit--confirm--converge) (Phase 0E-6E)

---

## Default Tooling

When generating workspace hooks and e2e-test skills, use these defaults unless the project already uses something different:

| Language                | Package Manager | Test Runner            | Dev Server    |
| ----------------------- | --------------- | ---------------------- | ------------- |
| TypeScript / JavaScript | bun             | vitest (via Turborepo) | `bun run dev` |
| Python                  | uv              | pytest                 | `uv run dev`  |
| Rust                    | cargo           | cargo test             | `cargo run`   |
| Go                      | go              | go test                | `go run .`    |

---

# New Project Flow

## Phase 1: Gather Requirements

### Step 1: Understand the project

Ask what the user is building — the answer drives every stack recommendation. For project-type presets,
the four `--template` presets (t3/pern/mern/uniwind), the full flag/options/defaults table, and per-option
decision guidance, read `references/stack-guide.md` — the canonical stack reference.

### Step 2: Present the default and ask about customization

> **Default stack (SaaS/web app):** Hono + TanStack Router + Drizzle + SQLite + Better Auth + tRPC + Turborepo + Biome + Bun
>
> Want to customize anything, or should I use this stack?

If the user says "use defaults" or similar, skip to Phase 2.

### Step 3: Walk through customizations

Group decisions naturally rather than dumping all options at once:

1. **Core stack** (frontend + backend + API layer) — defines the architecture
2. **Data layer** (database + ORM + DB hosting) — skip if Convex
3. **Auth & payments** — usually quick decisions
4. **Addons** — suggest based on project type
5. **Runtime & deploy** — usually defaults are fine

For the tRPC-vs-oRPC choice, framework tradeoffs, and addon guidance, consult `references/stack-guide.md`.
Note the hard constraint: **tRPC is incompatible with nuxt/svelte/solid/astro** — use oRPC for non-React
frontends.

### Default: in-place scaffold

The expected workflow is: **create a git repo → install this plugin → scaffold in-place**. So `.` is the default.

> **Note:** In-place scaffold uses `--directory-conflict merge`, which **overwrites** `README.md`, `.gitignore`, and `package.json`. The repo should be empty/fresh when scaffolding.

---

## Phase 2: Tool Check

```bash
for cmd in bun git gh openspec; do
  printf "%-10s" "$cmd:"
  which $cmd >/dev/null 2>&1 && echo "OK ($(which $cmd))" || echo "MISSING"
done
```

| Tool       | Install command                              |
| ---------- | -------------------------------------------- |
| `bun`      | `curl -fsSL https://bun.sh/install \| bash`  |
| `git`      | `xcode-select --install` (macOS)             |
| `gh`       | `brew install gh`                            |
| `openspec` | `npm install -g @fission-ai/openspec@latest` |

---

## Phase 3: Scaffold Project

Build the `create-better-t-stack` command from gathered requirements and run it non-interactively.

### Default command (in-place)

```bash
bun create better-t-stack@latest . --yes --directory-conflict merge --no-git \
  --frontend <frontend> \
  --backend <backend> \
  --database <database> \
  --orm <orm> \
  --auth <auth> \
  --api <api> \
  --runtime <runtime> \
  --package-manager <pm> \
  --addons <addon1,addon2,...>
```

### Rules

- `.` scaffolds into the current directory; `--directory-conflict merge` merges into it; `--no-git` skips
  git init (the repo already has `.git`)
- Always include `--yes` to skip interactive prompts
- Only include flags that differ from "none"
- If `--database none`, also omit `--orm` and `--dbSetup`
- Deploy flags are separate: `--webDeploy cloudflare` and `--serverDeploy cloudflare`
- Show the user the full command before running it, and wait for confirmation before executing

Before running, check the flag combination against the **Compatibility constraints** table in
`references/stack-guide.md` (tRPC/Clerk need React frontends; `self` needs a meta-framework; Workers needs
Hono; Polar needs Better Auth; pick one of turborepo/nx; Convex needs no DB/ORM).

---

## Phase 4: Post-Scaffold Verification

1. **Verify structure, install, and build:**

   ```bash
   ls apps/ packages/
   bun install
   turbo build
   ```

2. **Ensure workflow directories are gitignored:**

   ```bash
   # Add agentic workflow directories to .gitignore if not already present
   grep -q '.dev-workflow/' .gitignore || printf '\n# Agentic development workflow\n.dev-workflow/\n' >> .gitignore
   grep -q '.feature-workspaces/' .gitignore || printf '.feature-workspaces/\n' >> .gitignore
   ```

3. **Commit the scaffold:**

   ```bash
   git add -A && git commit -m "feat: scaffold monorepo via Better-T-Stack"
   ```

   A fresh repo is single-branch mode — AEP auto-detects `main` as the integration branch, so set
   no override. For the two-branch model, the override key, and how `$BASE` resolves, see
   `/aep-git-ref` "Integration Branch".

---

## Phase 5: Initialize OpenSpec

### Step 1: Run init

```bash
openspec init --tools claude,opencode,pi,codex
```

This creates `openspec/` (`config.yaml` = project config + context; `specs/` = source-of-truth specs;
`changes/` = proposals + artifacts) and the `.claude/skills/openspec-*/SKILL.md` Claude Code skills
(explore, propose, apply, archive).

> The `--tools` flag accepts a comma-separated list. Use `--tools all` to configure every supported tool.

### Step 2: Configure project context

Update `openspec/config.yaml` with the project's tech stack. Read `package.json` and `bts.jsonc` to determine the stack:

```yaml
schema: spec-driven

context: |
  Tech stack: TypeScript, <frontend>, <backend>, <database>/<orm>
  Monorepo: Turborepo + <package-manager>
  Auth: <auth-provider>
  API: <api-layer>
  Conventions: conventional commits, trunk-based development
```

### Step 3: Set up command aliases

Copy this skill's four OpenSpec command aliases from `templates/opsx/` into the project's
`.claude/commands/opsx/`:

```bash
mkdir -p .claude/commands/opsx
# copy explore.md, propose.md, apply.md, archive.md from this skill's templates/opsx/
```

### Step 4: Verify setup

```bash
# Check OpenSpec is initialized
openspec list

# Check skills were created
for skill in openspec-explore openspec-propose openspec-apply-change openspec-archive-change; do
  printf "%-35s" "$skill:"
  [ -f ".claude/skills/$skill/SKILL.md" ] && echo "OK" || echo "MISSING"
done

# Check commands were created
for cmd in explore propose apply archive; do
  printf "%-15s" "/opsx:$cmd:"
  [ -f ".claude/commands/opsx/$cmd.md" ] && echo "OK" || echo "MISSING"
done
```

---

## Phase 6: Commit OpenSpec

```bash
git add -A && git commit -m "feat: initialize OpenSpec for spec-driven development"
```

---

## Phase 7: Generate Workspace Setup Hook

Create the hook that `/aep-build` Phase 0 calls for project-specific setup:

```bash
mkdir -p .claude/hooks
```

Generate `.claude/hooks/workspace-setup.sh` tailored to the stack from Phase 1. It installs deps, scans
for free ports, starts the dev server, calls `skills/e2e-test/scripts/seed.sh` if present, and **must
write** `.dev-workflow/ports.env` — the contract with `/aep-build`:

```
WEB_PORT=<port>
SERVER_PORT=<port>
BASE_URL=http://localhost:<web-port>
SERVER_URL=http://localhost:<server-port>
```

Use the template and the full MUST/MAY contract in [`references/workspace-hook.md`](references/workspace-hook.md), filling in project-specific values from the stack chosen in Phase 1.

```bash
chmod +x .claude/hooks/workspace-setup.sh
```

---

## Phase 8: Generate the E2E Test Skill (delegate)

Hand off to **`/aep-e2e-skill-scaffolding`** — it generates the project-level testing infrastructure that
`/aep-build` Phases 5-8 use, in the **canonical BDD layer-gate three-tier** shape: a journey library
(natural-language Given/When/Then/Verify), a separate `tool-selection.md` (browser/device tool resolved
per environment), and an idempotent `seed.sh`. It reads the stack chosen in Phase 1 to fill its templates
and **owns the canonical cross-tool layout** — the real `skills/e2e-test/` dir plus the `.claude` and
`.agents` discovery symlinks (visible to Claude Code, Codex, and Pi). It also decides what the target
emits (`none` → `policy.md` + `seed.sh` only; `cli`/web/mobile/desktop → adds `journeys/` +
`tool-selection.md`) — those target rules live in `/aep-e2e-skill-scaffolding`.

### Commit

Verify the delegate actually produced the skill before committing (a missing path with an explicit
`git add <path>` would abort the commit), then stage everything it created:

```bash
test -d skills/e2e-test || { echo "ERROR: /aep-e2e-skill-scaffolding did not produce skills/e2e-test — rerun it"; exit 1; }
git add -A   # stages skills/e2e-test/, the two discovery symlinks, and the workspace hook
git commit -m "feat: add workspace hook and BDD e2e-test skill"
```

---

## Next Steps

For the full project layout after scaffolding completes, see [`references/resulting-structure.md`](references/resulting-structure.md).

| Command         | What it does                                                        |
| --------------- | ------------------------------------------------------------------- |
| `/aep-dispatch` | Pick the next story and start building (if product context exists)  |
| `/aep-design`   | Start designing a feature directly (standalone, no product context) |
| `bun run dev`   | Start the dev server                                                |
| `openspec list` | List active changes                                                 |

---

## Guardrails

- **Show the full command and wait for confirmation** before running scaffold; include `--yes` (non-interactive), `--no-git` (in-place), and warn that in-place overwrites README.md, .gitignore, and package.json.
- **Commit OpenSpec artifacts to git** — they are part of the project record.
- **Never overwrite an existing OpenSpec config** — check `openspec/config.yaml` exists before `openspec init`.
- **Never overwrite hand-authored content** (journeys, specs, prose) — the existing-project flow normalizes layout and upgrades generated infra idempotently, leaving hand-written files intact.
- **Version re-pin is recommend-only** — scaffold prints the `npx skills add@<newtag>` commands; the user runs them in a deliberate own-PR re-pin.

---

# Existing Project Flow — audit → confirm → converge

For projects that already have source code. This flow is **idempotent**: run it to onboard an existing
project **or** re-run it later to repair **drift** toward the current AEP standard (canonical cross-tool
layout, BDD e2e skill, current pin). It **reports first, asks, then converges** — and **never overwrites
hand-authored content**. Re-running a fully-converged project is a no-op ("already up to date").

Read [`references/converge-flow.md`](references/converge-flow.md) for how to interpret each audit category,
the observability→telemetry-candidate handling, and the per-category converge detail.

In commands below, replace `<aep-scaffold-dir>` with the absolute directory containing this `SKILL.md`; it is notation, not an environment variable or a target-project-relative path.

## Phase 0E: Status Check (stack + pin)

`scripts/audit.sh` opens by detecting language, package manager, monorepo tool, backend/frontend signal,
and the AEP pin vs latest release. Interpret per `references/converge-flow.md`: recommend **bun** (TS/JS)
or **uv** (Python) if the package manager is undetected; the frontend signal sets the default e2e `target`
(React Native → mobile; Tauri/Electrobun → desktop; else web).

---

## Phase 1E: Audit (drift-aware), grouped by category

Run the read-only audit. Nothing is changed in this phase.

```bash
bash "<aep-scaffold-dir>/scripts/audit.sh"
```

It prints `[ok]`/`[DRIFT]` per check across categories A (canonical layout), B (e2e shape), C (infra) and
`[detected]`/`[ ]` for D (observability), and **exits non-zero while any `[DRIFT]` remains** (exit 0 when
clean). See `references/converge-flow.md` for what each category means and the observability handling.

---

## Phase 2E: Report + Confirm Direction

Present the audit as a **current → target** summary grouped by category (A canonical layout, B e2e shape,
C infra, D observability, E version pin). For each category with drift/gaps, list the **proposed change**
and ask the user which to apply. **Default = fix all drift + gaps.** Use a per-category checklist (e.g.
the AskUserQuestion-style confirm). Only confirmed categories are converged in Phase 3E.

---

## Phase 3E: Converge (idempotent)

Apply only the confirmed changes; each step is a no-op when already satisfied. Run the mechanical fixes,
then the two model-driven steps. Detail for every category is in `references/converge-flow.md`.

```bash
bash "<aep-scaffold-dir>/scripts/converge.sh" --category A --category C
```

Pass only confirmed mechanical categories (`A`, `C`, or `E`), repeating `--category`; skip the script if none were confirmed. E only prints a re-pin recommendation; B and D remain model-driven.

- **B. E2E-test skill** — delegate to **`/aep-e2e-skill-scaffolding`** (creates or upgrades to BDD in
  canonical cross-tool form; migrates a legacy `.claude/skills/e2e-test` real dir; never overwrites
  hand-written journeys).
- **C. Remaining infra** — for each missing item: git repo (`git init -b main && git add -A && git commit
-m "chore: initial commit"`); OpenSpec (follow [Phase 5](#phase-5-initialize-openspec)); workspace hook
  (follow [Phase 7](#phase-7-generate-workspace-setup-hook)).

---

## Phase 4E: Verify

Re-run the audit until it **exits 0** — every confirmed category then reads `[ok]`:

```bash
bash "<aep-scaffold-dir>/scripts/audit.sh"
```

A fully-converged project re-running this flow produces no changes — **idempotent**.

---

## Phase 5E: Commit

```bash
git add -A
git commit -m "feat: converge agentic development infrastructure"
```

---

## Phase 6E: Next Steps

| Command                      | What it does                                              |
| ---------------------------- | --------------------------------------------------------- |
| `/aep-design`                | Start designing a feature (standalone mode)               |
| `/aep-dispatch`              | Pick the next story (if product context exists)           |
| `/aep-e2e-skill-scaffolding` | Generate/upgrade the BDD layer-gate e2e-test skill        |
| `/aep-git-ref`               | AEP git + worktree reference (worktree lifecycle, naming) |
