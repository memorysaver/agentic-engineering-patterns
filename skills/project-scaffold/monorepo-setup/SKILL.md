---
name: monorepo-setup
description: Interactive monorepo scaffold using Better-T-Stack. Use when creating a new project, starting fresh, or when the user says "new project", "scaffold", "create app", "monorepo setup", or wants to bootstrap a full-stack TypeScript monorepo.
---

# Monorepo Setup

Scaffold a modern full-stack TypeScript monorepo using [Better-T-Stack](https://www.better-t-stack.dev/docs). This skill gathers your requirements through conversation, then runs the CLI non-interactively to create a production-ready project.

---

## Phase 1: Gather Requirements

Before scaffolding, discuss the project with the user to determine the right configuration. Ask about each option, offering the recommended default.

### Questions to ask

| Topic | Options | Default |
|-------|---------|---------|
| **Scaffold location** | `.` (current directory) or `<project-name>` (new subdirectory) | `.` (in-place) |
| **Frontend** | tanstack, react, next, nuxt, svelte, solid, astro, none | `tanstack` |
| **Backend** | hono, express, fastify, elysia, convex, self, none | `hono` |
| **Database** | sqlite, postgres, mysql, mongodb, none | `sqlite` |
| **ORM** | drizzle, prisma, mongoose, none | `drizzle` |
| **Auth** | better-auth, clerk, none | `better-auth` |
| **API layer** | trpc, orpc, none | `trpc` |
| **Runtime** | bun, node, workers | `bun` |
| **Package manager** | bun, pnpm, npm | `bun` |
| **Addons** | turborepo, nx, biome, oxlint, lefthook, husky, starlight, fumadocs, pwa, tauri, mcp, skills | `turborepo,oxlint,skills` |
| **DB setup** | turso, d1, neon, supabase, planetscale, mongodb-atlas, docker, none | (depends on database) |
| **Examples** | none, todo, ai | `none` |
| **Deploy** | cloudflare, none | `none` |

### Default: in-place scaffold

The expected workflow is: **create a git repo → install this plugin → scaffold in-place**. So `.` (current directory) is the default.

> **Note:** In-place scaffold uses `--directory-conflict merge`, which **overwrites** `README.md`, `.gitignore`, and `package.json` with Better-T-Stack's versions. This is expected — the user's repo should be empty/fresh when scaffolding.

Only suggest a new subdirectory (`<project-name>`) if the user explicitly asks or if the current directory already has a `package.json` (indicating an existing project).

### Gathering style

- Present the recommended stack first: "The default stack is Hono + TanStack + Drizzle + Better Auth + tRPC + Turborepo + Bun. Want to customize anything?"
- If the user says "use defaults" or similar, skip to Phase 2
- For each customization, briefly explain the tradeoff
- Collect all answers before proceeding

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
  --addons <addon1,addon2,...>
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
- If `--database none`, also omit `--orm` and `--db-setup`
- If user chose addons, pass them as comma-separated: `--addons turborepo,oxlint,skills`
- Show the user the full command before running it
- Wait for confirmation before executing

### Example (default stack, in-place)

```bash
bun create better-t-stack@latest . --yes --directory-conflict merge --no-git \
  --frontend tanstack \
  --backend hono \
  --database sqlite \
  --orm drizzle \
  --auth better-auth \
  --api trpc \
  --runtime bun \
  --package-manager bun \
  --addons turborepo,oxlint,skills
```

### Example (default stack, new subdirectory)

```bash
bun create better-t-stack@latest my-app --yes \
  --frontend tanstack \
  --backend hono \
  --database sqlite \
  --orm drizzle \
  --auth better-auth \
  --api trpc \
  --runtime bun \
  --package-manager bun \
  --addons turborepo,oxlint,skills
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
