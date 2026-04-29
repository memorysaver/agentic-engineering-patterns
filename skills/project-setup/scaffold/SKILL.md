---
name: scaffold
description: Scaffold a new project or onboard an existing one with agentic development infrastructure. Use when creating a new project ("new project", "scaffold", "create app") OR setting up an existing project ("onboard project", "set up existing project", "initialize infrastructure", "add workflow to project"). For new projects, creates a full-stack monorepo via Better-T-Stack. For existing projects, audits and fills infrastructure gaps. Both modes set up OpenSpec, workspace hook, and e2e-test skill.
---

# Scaffold

Set up a project for agentic development — either by scaffolding a new monorepo or by onboarding an existing project. Both paths produce a project with OpenSpec, a workspace setup hook, and an e2e-test skill skeleton.

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
  → [Existing Project Flow](#existing-project-flow) (Phase 1E-6E)

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

| Tool       | Install command                             |
| ---------- | ------------------------------------------- |
| `bun`      | `curl -fsSL https://bun.sh/install \| bash` |
| `git`      | `xcode-select --install` (macOS)            |
| `gh`       | `brew install gh`                           |
| `openspec` | `bun add -g openspec`                       |

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
   grep -q '.dev-workflow/' .gitignore || echo '\n# Agentic development workflow\n.dev-workflow/' >> .gitignore
   grep -q '.feature-workspaces/' .gitignore || echo '.feature-workspaces/' >> .gitignore
   ```

5. **Commit the scaffold:**
   ```bash
   git add -A && git commit -m "feat: scaffold monorepo via Better-T-Stack"
   ```

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

Archive a completed change after its PR/MR has been merged. Run this on the main branch only.

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

Create the hook that `/build` Phase 0 calls for project-specific setup:

```bash
mkdir -p .claude/hooks
```

Generate `.claude/hooks/workspace-setup.sh` tailored to the stack from Phase 1. The hook must:

1. **Install dependencies** — use the package manager from Phase 1 (default: `bun install`)
2. **Scan for available ports** — start from 3000, increment by 10 to avoid parallel workspace collisions
3. **Write `.dev-workflow/ports.env`** — the contract with `/build`:
   ```
   WEB_PORT=<port>
   SERVER_PORT=<port>
   BASE_URL=http://localhost:<web-port>
   SERVER_URL=http://localhost:<server-port>
   ```
4. **Update `.env` files** with assigned ports (detect `.env.example` locations from scaffolded structure)
5. **Start the dev server** if not already running
6. **Call seed script** if `.claude/skills/e2e-test/scripts/seed.sh` exists

Use the template from `/testing-guide` Part 1, filling in project-specific values from the stack chosen in Phase 1.

```bash
chmod +x .claude/hooks/workspace-setup.sh
```

---

## Phase 8: Generate E2E Test Skill Skeleton

Create the project-level testing infrastructure that `/build` Phases 5-8 use:

```bash
mkdir -p .claude/skills/e2e-test/scripts
```

### Generate `.claude/skills/e2e-test/SKILL.md`

```markdown
---
name: e2e-test
description: E2E testing infrastructure for this project. Use when running tests,
  adding test coverage, or understanding what tests exist.
---

# E2E Test Infrastructure

## Prerequisites

- Dev server running (started by `.claude/hooks/workspace-setup.sh`)
- `.dev-workflow/ports.env` exists

## Setup

Source ports before running any test:

\`\`\`bash
source .dev-workflow/ports.env
\`\`\`

## Test Scripts

| Script  | What it tests           | Tools |
| ------- | ----------------------- | ----- |
| seed.sh | DB setup + test account | curl  |

## Adding a New Test

1. Create `.claude/skills/e2e-test/scripts/<feature>-e2e.sh`
2. Follow the E2E script pattern (see `/testing-guide` Part 2)
3. Add the script to the table above
4. Run it: `bash .claude/skills/e2e-test/scripts/<feature>-e2e.sh`
```

### Generate `.claude/skills/e2e-test/scripts/seed.sh`

```bash
#!/usr/bin/env bash
# Seed script — DB migrations + test account creation
# Called by workspace-setup.sh after dev server starts
set -euo pipefail

REPO_ROOT="$(git rev-parse --show-toplevel)"
if [ -f "$REPO_ROOT/.dev-workflow/ports.env" ]; then
  source "$REPO_ROOT/.dev-workflow/ports.env"
fi
SERVER_URL="${SERVER_URL:-http://localhost:3000}"

# Wait for server
echo "Waiting for server at $SERVER_URL..."
for i in $(seq 1 30); do
  curl -s "$SERVER_URL" >/dev/null 2>&1 && break
  sleep 1
done

# TODO: Add project-specific DB migrations here
# TODO: Add test account seeding here

echo "Seed complete."
```

```bash
chmod +x .claude/skills/e2e-test/scripts/seed.sh
```

### Commit

```bash
git add .claude/hooks/ .claude/skills/e2e-test/
git commit -m "feat: add workspace hook and e2e-test skill skeleton"
```

---

## Resulting Structure

```
<project>/
├── .claude/
│   ├── hooks/
│   │   └── workspace-setup.sh    # Project-specific workspace init
│   ├── skills/
│   │   ├── e2e-test/             # Testing infrastructure
│   │   │   ├── SKILL.md
│   │   │   └── scripts/
│   │   │       └── seed.sh
│   │   └── openspec-*/           # OpenSpec skills
│   └── commands/opsx/            # OpenSpec command aliases
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
| `/dispatch`     | Pick the next story and start building (if product context exists)  |
| `/design`       | Start designing a feature directly (standalone, no product context) |
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
- **Existing project mode never overwrites** — only creates missing files, never replaces existing ones

---

# Existing Project Flow

For projects that already have source code and want to add agentic development infrastructure.

---

## Phase 1E: Detect Stack

Scan the project to understand its technology stack:

```bash
echo "=== Detecting stack ==="

# Language
[ -f "package.json" ] && echo "Language: TypeScript/JavaScript"
[ -f "pyproject.toml" ] && echo "Language: Python"
[ -f "Cargo.toml" ] && echo "Language: Rust"
[ -f "go.mod" ] && echo "Language: Go"

# Package manager
[ -f "bun.lockb" ] && echo "Package manager: bun"
[ -f "pnpm-lock.yaml" ] && echo "Package manager: pnpm"
[ -f "package-lock.json" ] && echo "Package manager: npm"
[ -f "yarn.lock" ] && echo "Package manager: yarn"
[ -f "uv.lock" ] && echo "Package manager: uv"

# Monorepo
[ -f "turbo.json" ] && echo "Monorepo: Turborepo"
[ -f "nx.json" ] && echo "Monorepo: Nx"
[ -f "pnpm-workspace.yaml" ] && echo "Monorepo: pnpm workspaces"

# Framework (from package.json or pyproject.toml)
[ -f "package.json" ] && {
  grep -q '"hono"' package.json 2>/dev/null && echo "Backend: Hono"
  grep -q '"express"' package.json 2>/dev/null && echo "Backend: Express"
  grep -q '"fastify"' package.json 2>/dev/null && echo "Backend: Fastify"
  grep -q '"next"' package.json 2>/dev/null && echo "Frontend: Next.js"
  grep -q '"@tanstack/react-router"' package.json 2>/dev/null && echo "Frontend: TanStack Router"
  grep -q '"nuxt"' package.json 2>/dev/null && echo "Frontend: Nuxt"
  grep -q '"svelte"' package.json 2>/dev/null && echo "Frontend: Svelte"
}
```

Present findings to the user and confirm. If package manager is not detected, recommend:

- TypeScript/JavaScript → **bun**
- Python → **uv**

---

## Phase 2E: Audit Checklist

Run through the infrastructure checklist and report what exists vs what's missing:

```bash
echo "=== Infrastructure Audit ==="

# VCS
printf "  %-45s" "git repository (.git/ exists):"
[ -d ".git" ] && echo "[x]" || echo "[ ] MISSING"

# OpenSpec
printf "  %-45s" "openspec/ initialized:"
[ -d "openspec" ] && echo "[x]" || echo "[ ] MISSING"

printf "  %-45s" ".claude/commands/opsx/ aliases:"
[ -d ".claude/commands/opsx" ] && echo "[x]" || echo "[ ] MISSING"

# Workspace hook
printf "  %-45s" ".claude/hooks/workspace-setup.sh:"
[ -f ".claude/hooks/workspace-setup.sh" ] && echo "[x]" || echo "[ ] MISSING"

# E2E test skill
printf "  %-45s" ".claude/skills/e2e-test/SKILL.md:"
[ -f ".claude/skills/e2e-test/SKILL.md" ] && echo "[x]" || echo "[ ] MISSING"

printf "  %-45s" ".claude/skills/e2e-test/scripts/seed.sh:"
[ -f ".claude/skills/e2e-test/scripts/seed.sh" ] && echo "[x]" || echo "[ ] MISSING"

# Gitignore entries for workflow directories
printf "  %-45s" ".dev-workflow/ in .gitignore:"
grep -q '.dev-workflow/' .gitignore 2>/dev/null && echo "[x]" || echo "[ ] MISSING"

printf "  %-45s" ".feature-workspaces/ in .gitignore:"
grep -q '.feature-workspaces/' .gitignore 2>/dev/null && echo "[x]" || echo "[ ] MISSING"
```

Show the user the results. Only proceed to fill gaps for items marked `[ ] MISSING`.

---

## Phase 3E: Fill Gaps

For each missing item, generate it. **Never overwrite existing files.**

### Git repository (if missing)

```bash
git init -b main
git add -A
git commit -m "chore: initial commit"
```

### OpenSpec (if missing)

Follow the same steps as [Phase 5: Initialize OpenSpec](#phase-5-initialize-openspec) from the new project flow — `openspec init`, config, command aliases.

### Workspace setup hook (if missing)

Follow the same steps as [Phase 7: Generate Workspace Setup Hook](#phase-7-generate-workspace-setup-hook), using the detected stack from Phase 1E instead of the chosen stack.

### E2E test skill (if missing)

Follow the same steps as [Phase 8: Generate E2E Test Skill Skeleton](#phase-8-generate-e2e-test-skill-skeleton).

### Workflow gitignore entries (if missing)

```bash
grep -q '.dev-workflow/' .gitignore || echo '\n# Agentic development workflow\n.dev-workflow/' >> .gitignore
grep -q '.feature-workspaces/' .gitignore || echo '.feature-workspaces/' >> .gitignore
```

---

## Phase 4E: Verify

Re-run the audit checklist from Phase 2E. Everything should now be `[x]`.

---

## Phase 5E: Commit

```bash
git add .claude/ openspec/ .gitignore
git commit -m "feat: initialize agentic development infrastructure"
```

---

## Phase 6E: Next Steps

| Command          | What it does                                                |
| ---------------- | ----------------------------------------------------------- |
| `/design`        | Start designing a feature (standalone mode)                 |
| `/dispatch`      | Pick the next story (if product context exists)             |
| `/testing-guide` | Detailed guide for testing strategy and adding test scripts |
| `/git-ref`       | AEP git + worktree reference (worktree lifecycle, naming)   |
