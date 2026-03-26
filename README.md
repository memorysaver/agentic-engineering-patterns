# Agentic Engineering Patterns

A Claude Code plugin for structured, spec-driven TypeScript development. Design interactively on main, implement autonomously in isolated jj workspaces, and merge with confidence. Every feature follows the same lifecycle — from spec to PR — with progress tracking and parallel workspace sessions.

## The Workflow

Four verbs, one mental model:

```
/onboard → /scaffold → [ /design → /launch → /build → /wrap ]
   once       once           (repeat per feature)
```

| Command | What it does | Session |
|---------|-------------|---------|
| `/onboard` | Verify tools, install plugin, configure environment | Main, once |
| `/scaffold` | Scaffold monorepo (Better-T-Stack) + initialize OpenSpec | Main, once per project |
| `/design` | Explore + propose + review a feature with user | Main, interactive |
| `/launch` | Spawn workspace + optional evaluator agent | Main, automated |
| `/build` | Init → implement → test → PR → merge | Workspace, autonomous |
| `/wrap` | Archive OpenSpec change + cleanup workspace | Main, post-merge |
| `/jj-ref` | jj command reference (on-demand) | Any |

## Two-Session Model

Design happens interactively with you. Implementation runs autonomously in an isolated jj workspace — a separate Claude Code session that reads the spec and works through its change stack without interruption.

```
┌─────────────────────────────┐     ┌─────────────────────────────┐
│   Main Session              │     │   Workspace Session         │
│   (interactive)             │     │   (autonomous)              │
│                             │     │                             │
│  /design (explore, propose, │     │  /build                     │
│   review, commit to main)   │────►│    Phase 0: init + jj stack │
│                             │     │    Phase 4: implement       │
│  /launch (spawn workspace,  │     │    Phase 5: code review     │
│   optional evaluator)       │     │    Phase 6-8: test          │
│                             │◄────│    Phase 9-12: PR + merge   │
│  /wrap (archive + cleanup)  │     │                             │
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
  ├► feat-auth merged ─► /wrap on main
  ├► feat-notif merged ─► /wrap on main
  │
  openspec/specs/ updated only on main
  ── no conflicts
```

## Project Structure

After scaffolding with `/scaffold`, you get:

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
├── openspec/          # Spec-driven development (after /scaffold)
├── bts.jsonc          # Better-T-Stack project config
├── turbo.json         # Turborepo pipeline
└── package.json       # Root workspace
```

## Getting Started

New to this plugin? Run `/onboard` to install prerequisites, verify your environment, and configure recommended plugins.

Already set up? Run `/scaffold` to create a project, then `/design` to start building.

## Related Projects

- [looplia-skills](https://github.com/memorysaver/looplia-skills) — Search and context management skills
- [Better-T-Stack](https://www.better-t-stack.dev) — Full-stack TypeScript scaffold engine
- [OpenSpec](https://openspec.dev) — Spec-driven development CLI
- [Agent Skills Spec](https://agentskills.io/home) — Open standard for AI agent skills

## License

MIT
