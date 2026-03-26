---
name: scaffold
description: Scaffold a full-stack TypeScript monorepo and initialize spec-driven development. Use when creating a new project, starting fresh, or when the user says "new project", "scaffold", "create app", "setup project". Guides stack selection via Better-T-Stack, then initializes OpenSpec for explore/propose/apply/archive workflow.
---

# Scaffold

Scaffold a modern full-stack TypeScript monorepo using [Better-T-Stack](https://www.better-t-stack.dev/docs), then initialize [OpenSpec](https://openspec.dev) for spec-driven development. One command gets you from empty repo to ready-to-develop project.

For detailed decision guidance on stack options, read `references/stack-guide.md`.

---

## Phase 1: Gather Requirements

Before scaffolding, understand the user's project goals and recommend the right configuration.

### Step 1: Understand the project

Ask what the user is building. The answer shapes every recommendation:

| Project type | Recommended preset |
|---|---|
| **SaaS / web app** | Default stack (see below) |
| **API-first / microservice** | hono + orpc + postgres + drizzle, no frontend |
| **Vue / Nuxt app** | nuxt + hono + orpc (tRPC incompatible) |
| **Svelte app** | svelte + hono + orpc (tRPC incompatible) |
| **Content site / blog** | astro or next + no API layer |
| **Mobile app** | native-uniwind + hono + orpc |
| **Desktop app** | tanstack-router + hono + tauri or electrobun |
| **Browser extension** | tanstack-router + wxt addon |
| **AI / LLM app** | Default stack + ai example + mcp addon |
| **Docs site** | astro + starlight or fumadocs addon |

### Built-in template presets

If the user's project matches a well-known pattern, the CLI has `--template` presets that skip all selection:

| Template | Stack |
|---|---|
| `t3` | Next.js + Prisma + PostgreSQL + tRPC + Better Auth + Biome + Turborepo |
| `pern` | TanStack Router + Express + Drizzle + PostgreSQL + tRPC + Better Auth + Turborepo + Node |
| `mern` | React Router + Express + Mongoose + MongoDB + oRPC + Better Auth + Turborepo + Node |
| `uniwind` | React Native + NativeWind only (no backend/database) |

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

| Topic | Options | Default |
|-------|---------|---------|
| **Scaffold location** | `.` (current directory) or `<project-name>` (new subdirectory) | `.` (in-place) |
| **Frontend** | tanstack-router, react-router, tanstack-start, next, nuxt, svelte, solid, astro, native-bare, native-uniwind, native-unistyles, none | `tanstack-router` |
| **Backend** | hono, express, fastify, elysia, convex, self, none | `hono` |
| **Database** | sqlite, postgres, mysql, mongodb, none | `sqlite` |
| **ORM** | drizzle, prisma, mongoose, none | `drizzle` |
| **Auth** | better-auth, clerk, none | `better-auth` |
| **Payments** | polar, none | `none` |
| **API layer** | trpc, orpc, none | `trpc` |
| **Runtime** | bun, node, workers | `bun` |
| **Package manager** | bun, pnpm, npm | `bun` |
| **Addons** | turborepo, nx, biome, oxlint, ultracite, lefthook, husky, starlight, fumadocs, pwa, tauri, electrobun, mcp, opentui, wxt, skills | `turborepo,biome,skills` |
| **DB setup** | turso, d1, neon, supabase, prisma-postgres, planetscale, mongodb-atlas, docker, none | (depends on database) |
| **Examples** | none, todo, ai | `none` |
| **Deploy** | cloudflare, none | `none` |

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

| Tool | Install command |
|------|----------------|
| `bun` | `curl -fsSL https://bun.sh/install \| bash` |
| `git` | `xcode-select --install` (macOS) |
| `gh` | `brew install gh` |
| `openspec` | `bun add -g openspec` |

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

| Constraint | Rule |
|---|---|
| **tRPC + non-React frontend** | tRPC only works with tanstack-router, react-router, tanstack-start, next. For nuxt/svelte/solid/astro, use `orpc`. |
| **Clerk + non-React frontend** | Clerk only works with React-based frontends. Use `better-auth` for others. |
| **Backend `self`** | Only valid with meta-frameworks: next, tanstack-start, nuxt, astro. |
| **Workers runtime** | Requires `hono` backend. Incompatible with mongodb and docker dbSetup. |
| **Polar payments** | Requires `better-auth` (not clerk). |
| **turborepo + nx** | Cannot use both — pick one. |
| **Convex backend** | Incompatible with solid, astro frontends. No separate database/ORM needed. |

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

4. **Commit the scaffold:**
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

| Path | Purpose |
|------|---------|
| `openspec/` | Root OpenSpec directory |
| `openspec/config.yaml` | Project configuration + context |
| `openspec/specs/` | Specification documents (source of truth) |
| `openspec/changes/` | Change proposals and artifacts |
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
- specs/**/*.md — detailed requirements
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

## Phase 6: Final Commit

```bash
git add -A && git commit -m "feat: initialize OpenSpec for spec-driven development"
```

---

## Resulting Structure

```
<project>/
├── apps/
│   ├── web/              # Frontend (TanStack/React/Next/etc.)
│   └── server/           # Backend (Hono/Express/etc.)
├── packages/
│   ├── config/           # Shared TypeScript/lint config
│   ├── ui/               # Shared UI components (shadcn/ui)
│   ├── db/               # Database schema + migrations
│   ├── auth/             # Auth configuration
│   ├── api/              # API layer (tRPC/oRPC router)
│   └── env/              # Shared environment variables
├── openspec/             # Spec-driven development
├── bts.jsonc             # Better-T-Stack project config
├── turbo.json            # Turborepo pipeline config
└── package.json          # Root workspace config
```

---

## Next Steps

| Command | What it does |
|---------|-------------|
| `/dispatch` | Pick the next story and start building (if product context exists) |
| `/design` | Start designing a feature directly (standalone, no product context) |
| `bun run dev` | Start the dev server |
| `openspec list` | List active changes |

---

## Guardrails

- **Never run scaffold without user confirmation** of the full command
- **Always use `--yes`** to ensure non-interactive execution
- **Show the generated command** to the user before running
- **Warn about overwrites** — in-place scaffold overwrites README.md, .gitignore, and package.json
- **Use `--no-git` for in-place** — the repo already has .git initialized
- **Never overwrite existing OpenSpec config** — check if `openspec/config.yaml` exists first
- **Commit OpenSpec artifacts to git** — they are part of the project record
