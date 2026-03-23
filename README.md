# Agentic Engineering Patterns

A Claude Code plugin that packages skills for scaffolding and maintaining modern TypeScript monorepos with agentic development workflows.

Install this plugin, then use the bundled skills to scaffold new projects (via Better-T-Stack), initialize spec-driven development (via OpenSpec), and follow a structured development lifecycle with worktree isolation and progress tracking.

## Installation

```bash
# Add the marketplace
/plugin marketplace add memorysaver/agentic-engineering-patterns

# Install plugin groups
/plugin install project-scaffold@agentic-engineering-patterns
/plugin install development-workflow@agentic-engineering-patterns
```

## Plugin Groups

| Group | Skills | Purpose |
|-------|--------|---------|
| **project-scaffold** | monorepo-setup, openspec-setup | Scaffold new monorepo projects and configure spec-driven development |
| **development-workflow** | agentic-development-workflow | Full-lifecycle feature development with worktree isolation |

## Prerequisites

### Required Tools

| Tool | Purpose | Install |
|------|---------|---------|
| `bun` | Package manager & runtime | `curl -fsSL https://bun.sh/install \| bash` |
| `git` | Version control + worktrees | `xcode-select --install` (macOS) |
| `claude` | Claude Code CLI | `npm install -g @anthropic-ai/claude-code` |
| `gh` | GitHub CLI for PRs | `brew install gh` |
| `openspec` | Spec-driven development | `bun add -g openspec` |
| `tmux` | Terminal multiplexer | `brew install tmux` |
| `cmux` | Claude Code tab multiplexer | `bun add -g cmux` |

### Optional Tools

| Tool | Purpose | Install |
|------|---------|---------|
| `agent-browser` | Browser automation testing | Claude Code plugin: `agent-browser@agent-browser` |
| `portless` | Port management (.localhost) | `bun add -g portless` |
| `oxlint` | Fast linter | Included via Better-T-Stack `--addons oxlint` |

### Recommended Claude Code Plugins

| Plugin | Purpose |
|--------|---------|
| `superpowers@superpowers-marketplace` | Planning, debugging, TDD, code review workflows |
| `agent-browser@agent-browser` | Browser automation for testing |
| `frontend-design@claude-plugins-official` | High-quality UI generation |
| `code-review@claude-plugins-official` | PR code review |
| `mgrep@Mixedbread-Grep` | Semantic search (local + web) |
| `skill-creator@claude-plugins-official` | Create and test new skills |

## Quick Start

### 1. Scaffold a new project

```
/monorepo-setup
```

The skill walks you through choosing your stack, then runs `create-better-t-stack` non-interactively to scaffold a full monorepo.

### 2. Initialize spec-driven development

```
/openspec-setup
```

Runs `openspec init --tools claude,opencode,pi,codex` to set up OpenSpec with multi-tool skills and command aliases.

### 3. Start feature development

```
/agentic-development-workflow
```

Follow the structured workflow: design on main, implement in an isolated worktree, test, PR, merge, archive.

## Skills Reference

### monorepo-setup

Interactive scaffold skill that:
1. Gathers your requirements (frontend, backend, database, auth, API, etc.)
2. Verifies required CLI tools are installed
3. Builds and runs `create-better-t-stack` with all flags non-interactively
4. Verifies the scaffold completed successfully

**Default stack:** Hono + TanStack Router + Drizzle + SQLite + Better Auth + tRPC + Turborepo + Bun

### openspec-setup

Post-scaffold initialization that:
1. Runs `openspec init --tools claude,opencode,pi,codex`
2. Configures project context in `openspec/config.yaml`
3. Creates `/opsx:explore`, `/opsx:propose`, `/opsx:apply`, `/opsx:archive` command aliases

### agentic-development-workflow

Full-lifecycle development workflow with five parts:

```
Part A — Scaffold (optional)
  └► /monorepo-setup + /openspec-setup

Part B — Design (on main, interactive)
  └► /opsx:explore → /opsx:propose → design review

Part C — Launch Worktree
  └► git worktree + tmux + cmux → autonomous agent

Part D — Implementation (in worktree, autonomous)
  └► init → apply → code review → test → PR → merge

Part E — Post-Merge (on main)
  └► /opsx:archive → cleanup worktree
```

Features:
- **Two-session model** — main (interactive) + worktree (autonomous)
- **Parallel worktrees** — multiple features developed simultaneously via cmux tabs
- **Progress tracking** — checkpoint files in `.dev-workflow/` for resume support
- **13 phases** with guardrails and skip support

## Better-T-Stack Reference

[Better-T-Stack](https://www.better-t-stack.dev/docs) scaffolds modular TypeScript monorepos. Key options:

| Category | Options | Default |
|----------|---------|---------|
| Frontend | tanstack, react, next, nuxt, svelte, solid, astro | tanstack |
| Backend | hono, express, fastify, elysia, convex | hono |
| Database | sqlite, postgres, mysql, mongodb | sqlite |
| ORM | drizzle, prisma, mongoose | drizzle |
| Auth | better-auth, clerk | better-auth |
| API | trpc, orpc | trpc |
| Runtime | bun, node, workers | bun |
| Addons | turborepo, nx, biome, oxlint, skills, pwa, tauri | turborepo,oxlint,skills |

### Resulting structure

```
<project>/
├── apps/
│   ├── web/           # Frontend (TanStack/React/Next/etc.)
│   └── server/        # Backend (Hono/Express/etc.)
├── packages/
│   ├── config/        # Shared TypeScript/lint config
│   ├── ui/            # Shared UI components (shadcn/ui)
│   ├── db/            # Database schema + migrations
│   ├── auth/          # Auth configuration
│   ├── api/           # API layer (tRPC/oRPC router)
│   └── env/           # Shared environment variables
├── openspec/          # Spec-driven development (after /openspec-setup)
├── bts.jsonc          # Better-T-Stack project config
├── turbo.json         # Turborepo pipeline
└── package.json       # Root workspace
```

## Development Workflow Diagram

```
┌─────────────────────────┐     ┌─────────────────────────┐
│   Main Session          │     │   Worktree Session      │
│   (interactive)         │     │   (autonomous)          │
│                         │     │                         │
│  Part A: scaffold       │     │  Phase 0: init tracking │
│  Part B: design         │────►│  Phase 4: apply         │
│  Part C: launch worktree│     │  Phase 5: code review   │
│                         │     │  Phase 6: dogfood test  │
│                         │     │  Phase 7: E2E tests     │
│                         │◄────│  Phase 8-12: PR + merge │
│  Part E: archive        │     │                         │
└─────────────────────────┘     └─────────────────────────┘
```

## Related Projects

- [looplia-skills](https://github.com/memorysaver/looplia-skills) — Search and context management skills
- [Better-T-Stack](https://www.better-t-stack.dev) — Full-stack TypeScript scaffold engine
- [OpenSpec](https://openspec.dev) — Spec-driven development CLI
- [Agent Skills Spec](https://agentskills.io/home) — Open standard for AI agent skills

## License

MIT
