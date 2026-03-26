# Agentic Engineering Patterns

A Claude Code plugin for structured, spec-driven TypeScript development. Plan products with a persistent context layer, design features interactively on main, implement autonomously in isolated jj workspaces, and iterate with structured feedback loops.

## The Workflow

Three layers, one mental model:

```
Product context:   /envision → /map → ─────────────────────────── → /reflect → loop
                                      ↓                         ↑
Project setup:     /onboard → /scaffold                         │
                                      ↓                         │
Feature lifecycle: [ /design → /launch → /build → /wrap ] ──────┘
                     (repeat per feature/story)
```

### Product Context (persistent, evolves)

| Command | What it does | When to use |
|---------|-------------|-------------|
| `/envision` | Opportunity brief + context document | Starting a product, revisiting direction |
| `/map` | System map + story graph + agent topology | Decomposing a product into executable work |
| `/reflect` | Classify feedback + update context | After shipping, after user testing |

### Project Setup (one-time)

| Command | What it does |
|---------|-------------|
| `/onboard` | Verify tools, install plugin, configure environment |
| `/scaffold` | Scaffold monorepo (Better-T-Stack) + initialize OpenSpec |

### Feature Lifecycle (per-feature)

| Command | What it does | Session |
|---------|-------------|---------|
| `/design` | Explore + propose + review (reads product context) | Main, interactive |
| `/launch` | Spawn workspace + optional evaluator agent | Main, automated |
| `/build` | Init → implement → test → PR → merge | Workspace, autonomous |
| `/wrap` | Archive + suggest `/reflect` | Main, post-merge |
| `/jj-ref` | jj command reference (on-demand) | Any |

## Two-Session Model

Design happens interactively with you. Implementation runs autonomously in an isolated jj workspace — a separate Claude Code session that reads the spec and works through its change stack without interruption.

```
┌─────────────────────────────┐     ┌─────────────────────────────┐
│   Main Session              │     │   Workspace Session         │
│   (interactive)             │     │   (autonomous)              │
│                             │     │                             │
│  /envision (product vision) │     │  /build                     │
│  /map (decomposition)       │     │    Phase 0: init + jj stack │
│  /design (feature spec)     │────►│    Phase 4: implement       │
│  /launch (spawn workspace)  │     │    Phase 5: code review     │
│                             │◄────│    Phase 6-12: test + merge │
│  /wrap (archive)            │     │                             │
│  /reflect (feedback loop)   │     │                             │
└─────────────────────────────┘     └─────────────────────────────┘
```

Multiple features develop in parallel — each gets its own jj workspace and cmux tab:

```
main workspace (cmux)
  │
  ├► jj workspace add ─► tab: feat-auth
  │    autonomous Claude Code session
  │
  ├► jj workspace add ─► tab: feat-notif
  │    autonomous Claude Code session
  │
  │  (each tab runs /build independently)
  │  (workspaces share the jj store — no extra disk)
  │
  ├► feat-auth merged ─► /wrap → /reflect
  ├► feat-notif merged ─► /wrap → /reflect
```

## Project Structure

After scaffolding with `/scaffold`, you get:

```
<project>/
├── product-context/     # Product planning artifacts (after /envision + /map)
├── openspec/            # Per-feature change artifacts (after /design)
├── apps/
│   ├── web/             # Frontend (TanStack/React/Next/etc.)
│   └── server/          # Backend (Hono/Express/etc.)
├── packages/
│   ├── config/          # Shared TypeScript/lint config
│   ├── ui/              # Shared UI components (shadcn/ui)
│   ├── db/              # Database schema + migrations
│   ├── auth/            # Auth configuration
│   ├── api/             # API layer (tRPC/oRPC router)
│   └── env/             # Shared environment variables
├── bts.jsonc            # Better-T-Stack project config
├── turbo.json           # Turborepo pipeline
└── package.json         # Root workspace
```

## Getting Started

New to this plugin? Run `/onboard` to install prerequisites, verify your environment, and configure recommended plugins.

Have a product idea? Run `/envision` to validate the opportunity and frame the product, then `/map` to decompose it into executable work.

Already set up? Run `/scaffold` to create a project, then `/design` to start building.

## Related Projects

- [looplia-skills](https://github.com/memorysaver/looplia-skills) — Search and context management skills
- [Better-T-Stack](https://www.better-t-stack.dev) — Full-stack TypeScript scaffold engine
- [OpenSpec](https://openspec.dev) — Spec-driven development CLI
- [Agent Skills Spec](https://agentskills.io/home) — Open standard for AI agent skills

## License

MIT
