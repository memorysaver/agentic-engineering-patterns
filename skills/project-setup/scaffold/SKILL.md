---
name: aep-scaffold
description: Scaffold a new project or onboard/converge an existing one with agentic development infrastructure. Use when creating a new project ("new project", "scaffold", "create app") OR setting up / repairing an existing project ("onboard project", "set up existing project", "initialize infrastructure", "add workflow to project", "fix the skills layout", "upgrade to canonical"). For new projects, creates a full-stack monorepo via Better-T-Stack. For existing projects, runs an idempotent audit → confirm → converge that repairs drift (canonical cross-tool skills layout, BDD e2e skill, pin). Both modes set up OpenSpec, workspace hook, and the BDD e2e-test skill.
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

For detailed decision guidance on stack options, read `references/stack-guide.md`.

## Phase 1: Gather Requirements

Before scaffolding, understand the user's project goals and recommend the right configuration.

### Step 1: Understand the project

Ask what the user is building. The answer shapes every recommendation:

| Project type                 | Recommended preset                            |
| ---------------------------- | --------------------------------------------- |
| **SaaS / web app**           | Default stack (see below)                     |
| **API-first / microservice** | hono + orpc + postgres + drizzle, no frontend |
| **Vue / Nuxt app**           | nuxt + hono + orpc (tRPC incompatible)        |
| **Svelte app**               | svelte + hono + orpc (tRPC incompatible)      |
| **Content site / blog**      | astro or next + no API layer                  |
| **Mobile app**               | native-uniwind + hono + orpc                  |
| **Desktop app**              | tanstack-router + hono + tauri or electrobun  |
| **Browser extension**        | tanstack-router + wxt addon                   |
| **AI / LLM app**             | Default stack + ai example + mcp addon        |
| **Docs site**                | astro + starlight or fumadocs addon           |

### Built-in template presets

If the user's project matches a well-known pattern, the CLI has `--template` presets that skip all selection:

| Template  | Stack                                                                                    |
| --------- | ---------------------------------------------------------------------------------------- |
| `t3`      | Next.js + Prisma + PostgreSQL + tRPC + Better Auth + Biome + Turborepo                   |
| `pern`    | TanStack Router + Express + Drizzle + PostgreSQL + tRPC + Better Auth + Turborepo + Node |
| `mern`    | React Router + Express + Mongoose + MongoDB + oRPC + Better Auth + Turborepo + Node      |
| `uniwind` | React Native + NativeWind only (no backend/database)                                     |

Usage: `bun create better-t-stack@latest . --yes --template t3 --directory-conflict merge --no-git`

Only suggest templates if they match the user's needs exactly.

### Step 2: Present the default and ask about customization

> **Default stack (SaaS/web app):** Hono + TanStack Router + Drizzle + SQLite + Better Auth + tRPC + Turborepo + Biome + Bun
>
> Want to customize anything, or should I use this stack?

If the user says "use defaults" or similar, skip to Phase 2.

### Step 3: Walk through customizations

Don't dump all options at once. Group naturally:

1. **Core stack** (frontend + backend + API layer) — defines the architecture
2. **Data layer** (database + ORM + DB hosting) — skip if Convex
3. **Auth & payments** — usually quick decisions
4. **Addons** — proactively suggest based on project type
5. **Runtime & deploy** — usually defaults are fine

For each customization, explain the tradeoff briefly. Key decisions:

- **tRPC vs oRPC** — tRPC is battle-tested; oRPC has native OpenAPI, file uploads, contract-first. **tRPC is incompatible with nuxt/svelte/solid/astro** — use oRPC for non-React frontends.
- **Frontend framework** — depends on SSR needs, React vs Vue/Svelte ecosystem.
- **Addons** — proactively suggest relevant addons rather than reading the full list.

### All available options

| Topic                 | Options                                                                                                                              | Default                  |
| --------------------- | ------------------------------------------------------------------------------------------------------------------------------------ | ------------------------ |
| **Scaffold location** | `.` (current directory) or `<project-name>` (new subdirectory)                                                                       | `.` (in-place)           |
| **Frontend**          | tanstack-router, react-router, tanstack-start, next, nuxt, svelte, solid, astro, native-bare, native-uniwind, native-unistyles, none | `tanstack-router`        |
| **Backend**           | hono, express, fastify, elysia, convex, self, none                                                                                   | `hono`                   |
| **Database**          | sqlite, postgres, mysql, mongodb, none                                                                                               | `sqlite`                 |
| **ORM**               | drizzle, prisma, mongoose, none                                                                                                      | `drizzle`                |
| **Auth**              | better-auth, clerk, none                                                                                                             | `better-auth`            |
| **Payments**          | polar, none                                                                                                                          | `none`                   |
| **API layer**         | trpc, orpc, none                                                                                                                     | `trpc`                   |
| **Runtime**           | bun, node, workers                                                                                                                   | `bun`                    |
| **Package manager**   | bun, pnpm, npm                                                                                                                       | `bun`                    |
| **Addons**            | turborepo, nx, biome, oxlint, ultracite, lefthook, husky, starlight, fumadocs, pwa, tauri, electrobun, mcp, opentui, wxt, skills     | `turborepo,biome,skills` |
| **DB setup**          | turso, d1, neon, supabase, prisma-postgres, planetscale, mongodb-atlas, docker, none                                                 | (depends on database)    |
| **Examples**          | none, todo, ai                                                                                                                       | `none`                   |
| **Deploy**            | cloudflare, none                                                                                                                     | `none`                   |

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

Key flags:

- `.` — scaffold into current directory
- `--directory-conflict merge` — merge into existing directory
- `--no-git` — skip git init (repo already has .git)

### Rules

- Always include `--yes` to skip interactive prompts
- Only include flags that differ from "none"
- If `--database none`, also omit `--orm` and `--dbSetup`
- Deploy flags are separate: `--webDeploy cloudflare` and `--serverDeploy cloudflare`
- Show the user the full command before running it
- Wait for confirmation before executing

### Compatibility constraints

| Constraint                     | Rule                                                                                                               |
| ------------------------------ | ------------------------------------------------------------------------------------------------------------------ |
| **tRPC + non-React frontend**  | tRPC only works with tanstack-router, react-router, tanstack-start, next. For nuxt/svelte/solid/astro, use `orpc`. |
| **Clerk + non-React frontend** | Clerk only works with React-based frontends. Use `better-auth` for others.                                         |
| **Backend `self`**             | Only valid with meta-frameworks: next, tanstack-start, nuxt, astro.                                                |
| **Workers runtime**            | Requires `hono` backend. Incompatible with mongodb and docker dbSetup.                                             |
| **Polar payments**             | Requires `better-auth` (not clerk).                                                                                |
| **turborepo + nx**             | Cannot use both — pick one.                                                                                        |
| **Convex backend**             | Incompatible with solid, astro frontends. No separate database/ORM needed.                                         |

---

## Phase 4: Post-Scaffold Verification

1. **Verify the structure:**

   ```bash
   ls apps/ packages/
   ```

2. **Install dependencies:**

   ```bash
   bun install
   ```

3. **Verify build:**

   ```bash
   turbo build
   ```

4. **Ensure workflow directories are gitignored:**

   ```bash
   # Add agentic workflow directories to .gitignore if not already present
   grep -q '.dev-workflow/' .gitignore || printf '\n# Agentic development workflow\n.dev-workflow/\n' >> .gitignore
   grep -q '.feature-workspaces/' .gitignore || printf '.feature-workspaces/\n' >> .gitignore
   ```

5. **Commit the scaffold:**

   ```bash
   git add -A && git commit -m "feat: scaffold monorepo via Better-T-Stack"
   ```

   A fresh repo is single-branch mode — AEP auto-detects `main` as the integration branch, so
   **do not pin `aep.integration-branch`**. The repo can adopt two-branch mode later just by
   creating `develop` (auto-detected, no reconfiguration). Only set the config for a non-standard
   integration branch name: `git config aep.integration-branch <name>`.

---

## Phase 5: Initialize OpenSpec

### Step 1: Run init

```bash
openspec init --tools claude,opencode,pi,codex
```

This creates:

| Path                                 | Purpose                                               |
| ------------------------------------ | ----------------------------------------------------- |
| `openspec/`                          | Root OpenSpec directory                               |
| `openspec/config.yaml`               | Project configuration + context                       |
| `openspec/specs/`                    | Specification documents (source of truth)             |
| `openspec/changes/`                  | Change proposals and artifacts                        |
| `.claude/skills/openspec-*/SKILL.md` | Claude Code skills (explore, propose, apply, archive) |

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

Create OpenSpec command aliases in `.claude/commands/opsx/`:

#### `.claude/commands/opsx/explore.md`

```markdown
---
name: "OPSX: Explore"
description: "Enter explore mode — think through ideas, investigate, clarify requirements"
category: Workflow
tags: [workflow, explore, thinking]
---

Enter explore mode for thinking and investigation.

**IMPORTANT:** Explore mode is for thinking, not implementing. Read files and search code freely, but never write code. You MAY create OpenSpec artifacts if asked — that's capturing thinking, not implementing.

Invoke the openspec-explore skill to begin.
```

#### `.claude/commands/opsx/propose.md`

```markdown
---
name: "OPSX: Propose"
description: "Create a new change proposal with all artifacts"
category: Workflow
tags: [workflow, propose, change]
---

Create a new OpenSpec change proposal. This generates:

- proposal.md — what and why
- design.md — how, key decisions, risks
- specs/\*_/_.md — detailed requirements
- tasks.md — implementation checklist

Invoke the openspec-propose skill to begin.
```

#### `.claude/commands/opsx/apply.md`

```markdown
---
name: "OPSX: Apply"
description: "Implement tasks from an existing change proposal"
category: Workflow
tags: [workflow, apply, implement]
---

Implement tasks from an existing OpenSpec change. Reads the change artifacts and works through each task.

Invoke the openspec-apply-change skill to begin.
```

#### `.claude/commands/opsx/archive.md`

```markdown
---
name: "OPSX: Archive"
description: "Archive a completed change after merge"
category: Workflow
tags: [workflow, archive, cleanup]
---

Archive a completed change after its PR/MR has been merged. Run this on the integration branch only.

Invoke the openspec-archive-change skill to begin.
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

Generate `.claude/hooks/workspace-setup.sh` tailored to the stack from Phase 1. The hook must:

1. **Install dependencies** — use the package manager from Phase 1 (default: `bun install`)
2. **Scan for available ports** — start from 3000, increment by 10 to avoid parallel workspace collisions
3. **Write `.dev-workflow/ports.env`** — the contract with `/aep-build`:
   ```
   WEB_PORT=<port>
   SERVER_PORT=<port>
   BASE_URL=http://localhost:<web-port>
   SERVER_URL=http://localhost:<server-port>
   ```
4. **Update `.env` files** with assigned ports (detect `.env.example` locations from scaffolded structure)
5. **Start the dev server** if not already running
6. **Call seed script** if `skills/e2e-test/scripts/seed.sh` exists

Use the template in [`references/workspace-hook.md`](references/workspace-hook.md), filling in project-specific values from the stack chosen in Phase 1.

```bash
chmod +x .claude/hooks/workspace-setup.sh
```

---

## Phase 8: Generate the E2E Test Skill (delegate)

Hand off to **`/aep-e2e-skill-scaffolding`** — it generates the project-level testing infrastructure that
`/aep-build` Phases 5-8 use, in the **canonical BDD layer-gate three-tier** shape: a journey library
(natural-language Given/When/Then/Verify), a separate `tool-selection.md` (browser/device tool resolved
per environment), and an idempotent `seed.sh`. It reads the stack chosen in Phase 1 to fill its templates
and places the skill in **canonical cross-tool form** (visible to Claude Code, Codex, and Pi).

Invoke `/aep-e2e-skill-scaffolding`. When it returns, the project has:

```
skills/e2e-test/                              # REAL dir (canonical source of truth)
├── SKILL.md  ├── policy.md  └── scripts/seed.sh
├── journeys/{README.md, 00-walking-skeleton.md} + tool-selection.md   # only when dogfood_target ≠ none
.claude/skills/e2e-test → ../../skills/e2e-test   # symlink (Claude Code)
.agents/skills/e2e-test → ../../skills/e2e-test   # symlink (Codex / Pi)
```

> A `none`-target (CLI/library) project ships `policy.md` + `seed.sh` only — no `journeys/` or
> `tool-selection.md` (Tier-2 is N/A; the gate is Tier-1 + coverage).

### Commit

Verify the delegate actually produced the skill before committing (a missing path with an explicit
`git add <path>` would abort the commit), then stage everything it created:

```bash
test -d skills/e2e-test || { echo "ERROR: /aep-e2e-skill-scaffolding did not produce skills/e2e-test — rerun it"; exit 1; }
git add -A   # stages skills/e2e-test/, the two discovery symlinks, and the workspace hook
git commit -m "feat: add workspace hook and BDD e2e-test skill"
```

---

## Resulting Structure

```
<project>/
├── skills/
│   └── e2e-test/                # REAL dir — canonical, BDD layer-gate e2e (cross-tool)
│       ├── SKILL.md  ├── policy.md  └── scripts/seed.sh
│       └── journeys/ + tool-selection.md   # only when dogfood_target ≠ none (omitted for CLI/library)
├── .claude/
│   ├── hooks/
│   │   └── workspace-setup.sh    # Project-specific workspace init
│   ├── skills/
│   │   ├── e2e-test → ../../skills/e2e-test   # symlink (Claude Code)
│   │   └── openspec-*/           # OpenSpec skills
│   └── commands/opsx/            # OpenSpec command aliases
├── .agents/
│   └── skills/
│       └── e2e-test → ../../skills/e2e-test   # symlink (Codex / Pi)
├── apps/
│   ├── web/                      # Frontend (TanStack/React/Next/etc.)
│   └── server/                   # Backend (Hono/Express/etc.)
├── packages/
│   ├── config/                   # Shared TypeScript/lint config
│   ├── ui/                       # Shared UI components (shadcn/ui)
│   ├── db/                       # Database schema + migrations
│   ├── auth/                     # Auth configuration
│   ├── api/                      # API layer (tRPC/oRPC router)
│   └── env/                      # Shared environment variables
├── openspec/                     # Spec-driven development
├── bts.jsonc                     # Better-T-Stack project config
├── turbo.json                    # Turborepo pipeline config
└── package.json                  # Root workspace config
```

---

## Next Steps

| Command         | What it does                                                        |
| --------------- | ------------------------------------------------------------------- |
| `/aep-dispatch` | Pick the next story and start building (if product context exists)  |
| `/aep-design`   | Start designing a feature directly (standalone, no product context) |
| `bun run dev`   | Start the dev server                                                |
| `openspec list` | List active changes                                                 |

---

## Guardrails

- **Never run scaffold without user confirmation** of the full command
- **Always use `--yes`** to ensure non-interactive execution
- **Show the generated command** to the user before running
- **Warn about overwrites** — in-place scaffold overwrites README.md, .gitignore, and package.json
- **Use `--no-git` for in-place** — the repo already has .git initialized
- **Never overwrite existing OpenSpec config** — check if `openspec/config.yaml` exists first
- **Commit OpenSpec artifacts to git** — they are part of the project record
- **Existing project mode is audit → confirm → converge** — reports drift, asks per category, then
  applies only confirmed changes; idempotent (no-op when already converged). It normalizes layout and
  upgrades generated infra, but **never overwrites hand-authored content** (journeys, specs, prose)
- **Version re-pin is recommend-only** — scaffold prints the `npx skills add@<newtag>` commands; the
  user runs them in a deliberate own-PR re-pin

---

# Existing Project Flow — audit → confirm → converge

For projects that already have source code. This flow is **idempotent**: run it to onboard an existing
project **or** re-run it later to repair **drift** toward the current AEP standard (canonical cross-tool
layout, BDD e2e skill, current pin). It **reports first, asks, then converges** — and **never overwrites
hand-authored content**. Re-running a fully-converged project is a no-op ("already up to date").

---

## Phase 0E: Status Check (stack + pin)

```bash
echo "=== Detecting stack ==="
[ -f "package.json" ] && echo "Language: TypeScript/JavaScript"
[ -f "pyproject.toml" ] && echo "Language: Python"
[ -f "Cargo.toml" ] && echo "Language: Rust"
[ -f "go.mod" ] && echo "Language: Go"
for lk in "bun.lockb:bun" "pnpm-lock.yaml:pnpm" "package-lock.json:npm" "yarn.lock:yarn" "uv.lock:uv"; do
  [ -f "${lk%%:*}" ] && echo "Package manager: ${lk##*:}"
done
[ -f "turbo.json" ] && echo "Monorepo: Turborepo"; [ -f "nx.json" ] && echo "Monorepo: Nx"
[ -f "package.json" ] && {
  for f in '"hono":Backend:Hono' '"express":Backend:Express' '"next":Frontend:Next.js' \
           '"@tanstack/react-router":Frontend:TanStack Router' '"nuxt":Frontend:Nuxt' '"svelte":Frontend:Svelte' \
           '"native-uniwind":Frontend:React Native' '"@tauri-apps/api":Frontend:Tauri' '"electrobun":Frontend:Electrobun'; do
    grep -q "${f%%:*}" package.json 2>/dev/null && echo "${f#*:}" | tr ':' ' '
  done
}

# AEP pin (skills CLI) + latest release
echo "=== AEP pin ==="
[ -f "skills-lock.json" ] && echo "skills-lock.json: present" || echo "skills-lock.json: MISSING (skills CLI not used here)"
grep -oE 'pinned at \*\*v[0-9.]+\*\*' AGENTS.md 2>/dev/null || echo "AGENTS.md pin note: none"
echo "latest release: https://github.com/memorysaver/agentic-engineering-patterns/releases/latest"
```

If package manager is undetected, recommend bun (TS/JS) or uv (Python). The frontend signal also sets the
default e2e `target` (React Native → mobile; Tauri/Electrobun → desktop; else web).

---

## Phase 1E: Audit (drift-aware), grouped by category

Report **current vs target** for every category — not just "missing file" but **drift** (wrong layout,
thin/legacy e2e, stale pin). Nothing is changed in this phase.

```bash
echo "=== A. Canonical skills layout (cross-tool) ==="
chk() { printf "  %-52s" "$1:"; shift; "$@" && echo "[ok]" || echo "[DRIFT]"; }
chk "skills-lock.json present"            test -f skills-lock.json
chk ".agents/skills exists (codex install)" test -d .agents/skills
chk "AGENTS.md present"                    test -f AGENTS.md
chk "AGENTS.md has AEP Workflow section"   bash -c 'grep -q "AEP Workflow" AGENTS.md 2>/dev/null'
chk "CLAUDE.md = @AGENTS.md import"        bash -c '[ "$(head -1 CLAUDE.md 2>/dev/null | tr -d "[:space:]")" = "@AGENTS.md" ]'
# project-owned skills must be real in skills/ and symlinked into both runtimes
for s in $( [ -d skills ] && ls skills 2>/dev/null ); do
  chk "skills/$s exposed to .claude" bash -c "r=\$(readlink -f skills/$s 2>/dev/null); l=\$(readlink -f .claude/skills/$s 2>/dev/null); [ -n \"\$r\" ] && [ \"\$l\" = \"\$r\" ]"
  chk "skills/$s exposed to .agents" bash -c "r=\$(readlink -f skills/$s 2>/dev/null); l=\$(readlink -f .agents/skills/$s 2>/dev/null); [ -n \"\$r\" ] && [ \"\$l\" = \"\$r\" ]"
done

echo "=== B. E2E-test skill shape ==="
# Canonical = policy.md (the single source of truth, always emitted) OR a BDD journeys/ library.
# A none-target (CLI/library) project legitimately has policy.md and NO journeys/ — keying on
# journeys/README.md alone would mislabel it as DRIFT forever (breaking idempotency).
if   [ -f skills/e2e-test/policy.md ] || [ -f skills/e2e-test/journeys/README.md ]; then echo "  canonical     [ok]"
elif [ -d skills/e2e-test ];                     then echo "  real-non-bdd  [DRIFT → upgrade to BDD]"
elif [ -d .claude/skills/e2e-test ] && [ ! -L .claude/skills/e2e-test ]; then echo "  thin-legacy   [DRIFT → migrate to skills/ + BDD]"
else echo "  absent        [DRIFT → generate]"; fi

echo "=== C. Infrastructure ==="
chk "openspec/ initialized"               test -d openspec
chk ".claude/commands/opsx/ aliases"      test -d .claude/commands/opsx
chk ".claude/hooks/workspace-setup.sh"    test -f .claude/hooks/workspace-setup.sh
chk ".dev-workflow/ gitignored"           bash -c 'grep -q ".dev-workflow/" .gitignore 2>/dev/null'
chk ".feature-workspaces/ gitignored"     bash -c 'grep -q ".feature-workspaces/" .gitignore 2>/dev/null'

echo "=== D. Observability (telemetry candidates for /aep-map) ==="
deps="$(cat package.json 2>/dev/null) $(cat pyproject.toml 2>/dev/null)"
for probe in "sentry:error_stream" "datadog:monitoring" "posthog:analytics" "amplitude:analytics" "@opentelemetry:monitoring" "newrelic:monitoring"; do
  printf "  %-45s" "${probe%%:*} (${probe##*:}):"; echo "$deps" | grep -qi "${probe%%:*}" && echo "[detected]" || echo "[ ]"
done
printf "  %-45s" "health endpoint (/healthz|/readyz|/health):"
grep -rqiE '/(healthz|readyz|health)\b' . --include='*.ts' --include='*.js' --include='*.py' 2>/dev/null && echo "[detected]" || echo "[ ]"
```

**Observability → telemetry candidates.** For each `[detected]` tool, record a **candidate** entry under
`topology.routing.telemetry_sources` (`kind` + a `token_env` name — never the secret; leave
`endpoint`/`metric_map` for `/aep-map`). If nothing is detected, note it so `/aep-map` knows quantitative
metrics may need a tool added or stay qualitative.

---

## Phase 2E: Report + Confirm Direction

Present the audit as a **current → target** summary grouped by category (A canonical layout, B e2e shape,
C infra, D observability, E version pin). For each category with drift/gaps, list the **proposed change**
and ask the user which to apply. **Default = fix all drift + gaps.** Use a per-category checklist (e.g.
the AskUserQuestion-style confirm). Only confirmed categories are converged in Phase 3E.

---

## Phase 3E: Converge (idempotent)

Apply only the confirmed changes. Each step is a no-op when already satisfied. **Never overwrite
hand-authored content.**

### A. Canonical skills layout

Normalize `.claude/skills/aep-*` to symlinks into `.agents/skills` so both runtimes share one copy (the
README "gotcha"), and ensure `AGENTS.md` / `CLAUDE.md`:

```bash
# Share one copy of each aep-* skill across runtimes. NEVER `rm` a real dir unless the canonical
# copy is known to exist — otherwise PROMOTE it (preserve content) before linking. A Claude-only
# install (real dirs in .claude/skills, none in .agents/skills) is the exact case this must not destroy.
if [ -d .claude/skills ] && [ -d .agents/skills ]; then
  ( cd .claude/skills
    for d in aep-*; do
      [ -e "$d" ] || continue                 # nothing to normalize
      [ -L "$d" ] && continue                  # already a symlink — leave it
      if [ -e "../../.agents/skills/$d" ]; then
        rm -rf "$d" && ln -s "../../.agents/skills/$d" "$d"          # canonical copy exists → safe to replace
      else
        mv "$d" "../../.agents/skills/$d" && ln -s "../../.agents/skills/$d" "$d"   # only copy → promote, don't destroy
      fi
    done )
fi
# CLAUDE.md = @AGENTS.md import — only create when ABSENT (never clobber a hand-authored CLAUDE.md).
if [ -f AGENTS.md ] && [ ! -f CLAUDE.md ]; then printf '@AGENTS.md\n' > CLAUDE.md; fi
# A real-content CLAUDE.md that isn't the import is flagged for manual merge — NOT overwritten.
if [ -f CLAUDE.md ] && [ "$(head -1 CLAUDE.md | tr -d '[:space:]')" != "@AGENTS.md" ]; then
  echo "NOTE: CLAUDE.md has hand-authored content — merge it into AGENTS.md by hand, then set CLAUDE.md to '@AGENTS.md'."
fi
```

Project-owned skill exposure (real `skills/<name>` + both symlinks) is handled by the skill's own
generator — for e2e-test that's `/aep-e2e-skill-scaffolding` (next step).

### B. E2E-test skill

Delegate to **`/aep-e2e-skill-scaffolding`** — it creates (absent) or upgrades (thin-legacy / real-non-bdd
→ BDD) the skill in canonical cross-tool form, idempotently. It migrates a legacy `.claude/skills/e2e-test`
real dir into `skills/` first and never overwrites hand-written journeys.

### C. Infrastructure (fill gaps)

For each `[DRIFT]`/missing infra item, generate it — **never overwrite existing files**:

- **Git repo:** `git init -b main && git add -A && git commit -m "chore: initial commit"` (AEP auto-detects `main`).
- **OpenSpec:** follow [Phase 5: Initialize OpenSpec](#phase-5-initialize-openspec).
- **Workspace hook:** follow [Phase 7](#phase-7-generate-workspace-setup-hook) using the detected stack.
- **Gitignore:**
  ```bash
  grep -q '.dev-workflow/' .gitignore || printf '\n# Agentic development workflow\n.dev-workflow/\n' >> .gitignore
  grep -q '.feature-workspaces/' .gitignore || printf '.feature-workspaces/\n' >> .gitignore
  ```

### E. Version pin — detect + recommend (do NOT auto-run)

Re-pinning AEP is a **deliberate, own-PR action** (README), so scaffold only **recommends** it. When the
local pin lags the latest release, print the commands for the user to run themselves:

```bash
echo "Local pin lags latest. To re-pin (run yourself, in its own PR):"
echo "  npx skills add memorysaver/agentic-engineering-patterns@<newtag> -a claude-code --skill '*' -y"
echo "  npx skills add memorysaver/agentic-engineering-patterns@<newtag> -a codex        --skill '*' -y"
echo "  then normalize .claude/skills/aep-* symlinks (step A), bump the AGENTS.md pin note, commit --no-verify"
```

---

## Phase 4E: Verify

Re-run the Phase 1E audit. Every confirmed category should now read `[ok]`. A fully-converged project
re-running this flow produces no changes — **idempotent**.

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
