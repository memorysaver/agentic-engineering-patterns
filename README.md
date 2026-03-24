# Agentic Engineering Patterns

A Claude Code plugin for structured, spec-driven TypeScript development. Design interactively on main, implement autonomously in isolated worktrees, and merge with confidence. Every feature follows the same lifecycle — from spec to PR — with progress tracking and parallel worktree sessions.

## What You Get

**monorepo-setup** — Scaffolds a modern full-stack TypeScript monorepo via Better-T-Stack. Walks you through stack selection (frontend, backend, database, auth, API layer, addons), then runs the CLI non-interactively.

**openspec-setup** — Initializes spec-driven development with OpenSpec. Creates explore, propose, apply, and archive commands so every feature starts with a spec, not code.

**development-onboarding** — Takes you from zero to ready. Installs the plugin, verifies required tools, and configures recommended Claude Code plugins. Run once on first setup.

**agentic-development-workflow** — Orchestrates the full development lifecycle across five parts: scaffold, design, launch worktree, implement, and post-merge cleanup. Handles 13 phases with checkpoints and resume support.

## The Workflow

Five parts, from scaffold through post-merge cleanup:

```
┌──────────────────────────────────────────────┐
│    Part A — Scaffold (optional)              │
│                                              │
│  /monorepo-setup (Better-T-Stack)            │
│      ▼                                       │
│  /openspec-setup (spec-driven dev)           │
│      ▼                                       │
│  Verify build + OpenSpec ready               │
└──────┬───────────────────────────────────────┘
       │
       ▼
┌──────────────────────────────────────────────┐
│    Part B — Design (on main, interactive)    │
│                                              │
│  Phase 1: /opsx:explore                      │
│      ▼                                       │
│  Phase 2: /opsx:propose                      │
│      ▼                                       │
│  Phase 3: Design review                      │
│      ▼                                       │
│  Commit artifacts to main                    │
└──────┬───────────────────────────────────────┘
       │
       ▼
┌──────────────────────────────────────────────┐
│    Part C — Launch Worktree (on main)        │
│                                              │
│  Verify main is clean                        │
│      ▼                                       │
│  git worktree add + tmux + cmux tab          │
│      ▼                                       │
│  Send initial prompt to spawned agent        │
│  (references worktree-onboarding.md)         │
└──────┬───────────────────────────────────────┘
       │  cmux send key
       ▼
┌──────────────────────────────────────────────┐
│    Part D — Implementation (in worktree)     │
│                                              │
│  Phase 0: Init tracking + dev environment    │
│      ▼                                       │
│  Phase 4: /opsx:apply                        │
│      ▼                                       │
│  Phase 5: Code review & verification         │
│      ▼                                       │
│  Phase 6: Dogfood testing (agent-browser)    │
│      ▼                                       │
│  Phase 7: E2E test scripts                   │
│      ▼                                       │
│  Phase 8: Review results                     │
│      ▼                                       │
│  Phase 9–12: Commit ► PR ► Review ► Merge    │
└──────┬───────────────────────────────────────┘
       │  PR merged
       ▼
┌──────────────────────────────────────────────┐
│    Part E — Post-Merge (on main)             │
│                                              │
│  Phase 13: git checkout main && git pull     │
│      ├► /opsx:archive (spec sync)            │
│      ├► git commit + push archive            │
│      └► Remove worktree · delete branch      │
└──────────────────────────────────────────────┘
```

## Two-Session Model

Design happens interactively with you. Implementation runs autonomously in an isolated worktree — a separate Claude Code session that reads the spec and works through it without interruption.

```
┌─────────────────────────────┐     ┌─────────────────────────────┐
│   Main Session              │     │   Worktree Session          │
│   (interactive)             │     │   (autonomous)              │
│                             │     │                             │
│  Part A: scaffold           │     │  Phase 0: init tracking     │
│  Part B: design             │────►│  Phase 4: apply             │
│  Part C: launch worktree    │     │  Phase 5: code review       │
│                             │     │  Phase 6: dogfood test      │
│                             │     │  Phase 7: E2E tests         │
│                             │◄────│  Phase 8-12: PR + merge     │
│  Part E: archive            │     │                             │
└─────────────────────────────┘     └─────────────────────────────┘
```

Multiple features develop in parallel — each gets its own worktree and cmux tab:

```
main workspace (cmux)
  │
  ├► launch worktree ─► tab: feat-auth
  │    autonomous Claude Code session
  │
  ├► launch worktree ─► tab: feat-notif
  │    autonomous Claude Code session
  │
  │  (each tab runs Part D independently)
  │
  ├► feat-auth merged ─► archive on main
  ├► feat-notif merged ─► archive on main
  │
  openspec/specs/ updated only on main
  ── no conflicts
```

## Project Structure

After scaffolding with `/monorepo-setup`, you get:

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

## Getting Started

New to this plugin? Run `/development-onboarding` to install prerequisites, verify your environment, and configure recommended plugins.

Already set up? Run `/monorepo-setup` to scaffold a project, then `/agentic-development-workflow` to start building.

## Related Projects

- [looplia-skills](https://github.com/memorysaver/looplia-skills) — Search and context management skills
- [Better-T-Stack](https://www.better-t-stack.dev) — Full-stack TypeScript scaffold engine
- [OpenSpec](https://openspec.dev) — Spec-driven development CLI
- [Agent Skills Spec](https://agentskills.io/home) — Open standard for AI agent skills

## License

MIT
