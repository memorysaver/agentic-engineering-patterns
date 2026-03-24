---
name: monorepo-setup
description: Interactive monorepo scaffold using Better-T-Stack. Use when creating a new project, starting fresh, or when the user says "new project", "scaffold", "create app", "monorepo setup", or wants to bootstrap a full-stack TypeScript monorepo. Guides users through tech stack selection with opinionated recommendations for their use case.
---

# Monorepo Setup

Scaffold a modern full-stack TypeScript monorepo using [Better-T-Stack](https://www.better-t-stack.dev/docs). This skill gathers your requirements through conversation, recommends a stack tailored to your use case, then runs the CLI non-interactively to create a production-ready project.

For detailed decision guidance on each option, read `references/stack-guide.md`.

---

## Phase 1: Gather Requirements

Before scaffolding, understand the user's project goals and recommend the right configuration. Don't just list options — help them choose.

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

Only suggest templates if they match the user's needs exactly. Otherwise, use the custom flag approach.

### Step 2: Present the default and ask about customization

Present the recommended stack based on their project type:

> **Default stack (SaaS/web app):** Hono + TanStack Router + Drizzle + SQLite + Better Auth + tRPC + Turborepo + Biome + Bun
>
> Want to customize anything, or should I use this stack?

If the user says "use defaults" or similar, skip to Phase 2.

### Step 3: Walk through customizations

Don't dump all 13 options at once. Group the conversation naturally:

1. **Core stack** (frontend + backend + API layer) — these define the app's architecture
2. **Data layer** (database + ORM + DB hosting) — skip if they chose Convex
3. **Auth & payments** — usually quick decisions
4. **Addons** — proactively suggest based on their project type rather than listing all 16
5. **Runtime & deploy** — usually defaults are fine

For each customization, explain the tradeoff briefly. Refer to `references/stack-guide.md` for detailed guidance, especially:

- **tRPC vs oRPC** — the most common question. tRPC is battle-tested with a huge ecosystem; oRPC is newer but has native OpenAPI support, file uploads, and a contract-first workflow. If they need external API consumers or file handling, lean toward oRPC. Note: **tRPC is incompatible with nuxt/svelte/solid/astro** — use oRPC for non-React frontends.
- **Frontend framework** — depends on SSR needs, React vs Vue/Svelte ecosystem, and whether they need a meta-framework.
- **Addons** — many users don't know what's available. Proactively suggest relevant addons based on their project type rather than reading out the full list.

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

The expected workflow is: **create a git repo → install this plugin → scaffold in-place**. So `.` (current directory) is the default.

> **Note:** In-place scaffold uses `--directory-conflict merge`, which **overwrites** `README.md`, `.gitignore`, and `package.json` with Better-T-Stack's versions. This is expected — the user's repo should be empty/fresh when scaffolding.

Only suggest a new subdirectory (`<project-name>`) if the user explicitly asks or if the current directory already has a `package.json` (indicating an existing project).

---

## Phase 2: Tool Check

Verify required CLI tools are installed. Run:

```bash
for cmd in bun git gh; do
  printf "%-10s" "$cmd:"
  which $cmd >/dev/null 2>&1 && echo "OK ($(which $cmd))" || echo "MISSING"
done
```

### Install missing tools

| Tool | Install command |
|------|----------------|
| `bun` | `curl -fsSL https://bun.sh/install \| bash` |
| `git` | `xcode-select --install` (macOS) |
| `gh` | `brew install gh` |

If `bun` is missing, it must be installed before proceeding — the scaffold depends on it.

---

## Phase 3: Scaffold

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
  --addons <addon1,addon2,...> \
  # Include only if selected:
  # --payments <payments>
  # --dbSetup <db-setup>
  # --examples <example>
  # --webDeploy <deploy> --serverDeploy <deploy>
```

Key flags:
- `.` — scaffold into current directory
- `--directory-conflict merge` — merge into existing directory
- `--no-git` — skip git init (repo already has .git)

### Alternative: new subdirectory (only if user requests)

```bash
bun create better-t-stack@latest <project-name> --yes \
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

- Always include `--yes` to skip interactive prompts
- Only include flags that differ from "none" — omit flags for features the user doesn't want
- If `--database none`, also omit `--orm` and `--dbSetup`
- Deploy flags are separate: `--webDeploy cloudflare` and `--serverDeploy cloudflare`
- If user chose payments, include `--payments polar`
- If user chose addons, pass them as comma-separated: `--addons turborepo,biome,skills`
- Show the user the full command before running it
- Wait for confirmation before executing

### Compatibility constraints

These combinations are invalid — the CLI will reject them. Check before building the command:

| Constraint | Rule |
|---|---|
| **tRPC + non-React frontend** | tRPC only works with tanstack-router, react-router, tanstack-start, next. For nuxt/svelte/solid/astro, use `orpc` or `none`. |
| **Clerk + non-React frontend** | Clerk only works with tanstack-router, react-router, tanstack-start, next. For others, use `better-auth`. |
| **Backend `self`** | Only valid with meta-frameworks: next, tanstack-start, nuxt, astro. |
| **Workers runtime** | Requires `hono` backend. Incompatible with mongodb and docker dbSetup. |
| **Polar payments** | Requires `better-auth` (not clerk). |
| **turborepo + nx** | Cannot use both — pick one. |
| **Convex backend** | Incompatible with solid, astro frontends. No separate database/ORM needed. |

### Example (default stack, in-place)

```bash
bun create better-t-stack@latest . --yes --directory-conflict merge --no-git \
  --frontend tanstack-router \
  --backend hono \
  --database sqlite \
  --orm drizzle \
  --auth better-auth \
  --api trpc \
  --runtime bun \
  --package-manager bun \
  --addons turborepo,biome,skills
```

### Example (API-first with oRPC, in-place)

```bash
bun create better-t-stack@latest . --yes --directory-conflict merge --no-git \
  --backend hono \
  --database postgres \
  --orm drizzle \
  --auth better-auth \
  --api orpc \
  --runtime bun \
  --package-manager bun \
  --addons turborepo,biome,skills \
  --dbSetup docker
```

### Example (SaaS with payments, new subdirectory)

```bash
bun create better-t-stack@latest my-saas --yes \
  --frontend tanstack-router \
  --backend hono \
  --database postgres \
  --orm drizzle \
  --auth better-auth \
  --payments polar \
  --api trpc \
  --runtime bun \
  --package-manager bun \
  --addons turborepo,biome,skills \
  --dbSetup neon
```

---

## Phase 4: Post-Scaffold Verification

After the scaffold completes:

1. **Verify the structure exists:**
   ```bash
   ls apps/ packages/
   ```

2. **Install dependencies** (if not already done by scaffold):
   ```bash
   bun install
   ```

3. **Verify build works:**
   ```bash
   turbo build
   ```

4. **Commit the scaffold:**
   ```bash
   git add -A && git commit -m "feat: scaffold monorepo via Better-T-Stack"
   ```

### If scaffolded into a new subdirectory (alternative flow)

1. **Navigate into the project:**
   ```bash
   cd <project-name>
   ```

2. **Verify the structure exists:**
   ```bash
   ls apps/ packages/
   ```

3. **Install dependencies** (if not already done by scaffold):
   ```bash
   bun install
   ```

4. **Verify build works:**
   ```bash
   turbo build
   ```

5. **Initialize git** (if not already done):
   ```bash
   git init && git add -A && git commit -m "Initial scaffold via Better-T-Stack"
   ```

### Suggest next steps

- Run `/openspec-setup` to initialize spec-driven development
- Run `bun run dev` to start the dev server
- Check `bts.jsonc` for the project configuration record

---

## Resulting Structure

The scaffold produces a Turborepo monorepo:

```
<project>/
├── apps/
│   ├── web/              # Frontend (TanStack/React/Next/etc.)
│   └── server/           # Backend (Hono/Express/etc.)
├── packages/
│   ├── config/           # Shared TypeScript/lint config
│   ├── ui/               # Shared UI components (shadcn/ui)
│   ├── db/               # Database schema + migrations (Drizzle/Prisma)
│   ├── auth/             # Auth configuration (Better Auth)
│   ├── api/              # API layer (tRPC/oRPC router)
│   └── env/              # Shared environment variables
├── bts.jsonc             # Better-T-Stack project config
├── turbo.json            # Turborepo pipeline config
└── package.json          # Root workspace config
```

---

## Guardrails

- **Never run scaffold without user confirmation** of the full command
- **Always use `--yes`** to ensure non-interactive execution
- **Show the generated command** to the user before running
- **Warn about overwrites** — in-place scaffold (`--directory-conflict merge`) overwrites README.md, .gitignore, and package.json
- **Use `--no-git` for in-place** — the repo already has .git initialized
- **Check directory state first** — detect if empty repo vs existing project before recommending scaffold location
