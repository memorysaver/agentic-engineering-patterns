# Agentic Development Workflow

Visual guide to the five-part development workflow with two-session model.
See [SKILL.md](SKILL.md) for the full step-by-step process.

## Five-Part Workflow Overview

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
│    Part C — Launch Workspace (on main)       │
│                                              │
│  Verify main is clean (jj st)                │
│      ▼                                       │
│  jj workspace add + tmux + cmux tab          │
│      ▼                                       │
│  Send initial prompt to spawned agent        │
│  (references worktree-onboarding.md)         │
└──────┬───────────────────────────────────────┘
       │  cmux send key
       ▼
┌──────────────────────────────────────────────┐
│    Part D — Implementation (in workspace)    │
│                                              │
│  Phase 0: Init tracking + jj change stack    │
│      ▼                                       │
│  Phase 4: jj edit each change + /opsx:apply  │
│      ▼                                       │
│  Phase 5: Code review & verification         │
│      ▼                                       │
│  Phase 6: Dogfood testing (agent-browser)    │
│      ▼                                       │
│  Phase 7: E2E test scripts                   │
│      ▼                                       │
│  Phase 8: Review results                     │
│      ▼                                       │
│  Phase 9–12: Publish ► PR ► Review            │
│      ▼                                       │
│  Phase 11.5: Human eval & iteration loop     │
│      ▼                                       │
│  Phase 12: Merge                             │
└──────┬───────────────────────────────────────┘
       │  PR merged
       ▼
┌──────────────────────────────────────────────┐
│    Part E — Post-Merge (on main)             │
│                                              │
│  Phase 13: jj git fetch                      │
│      ├► /opsx:archive (spec sync)            │
│      ├► jj describe + push archive           │
│      └► jj workspace forget                  │
└──────────────────────────────────────────────┘
```

## Two-Session Model

```
Main session (interactive with user):
  Part A: scaffold new project (optional)
  Part B: design phases — explore, propose, review
  Part C: launch workspace, send bootstrap prompt
  Part E: archive after merge

Workspace session (autonomous):
  Part D: implement via jj change stack, test, PR
  Reads worktree-onboarding.md to catch up with full context
```

## Parallel Workspace Sessions via cmux

```
main workspace (cmux)
  │
  ├► jj workspace add ─► tab: feat-auth
  │    autonomous Claude Code session
  │
  ├► jj workspace add ─► tab: feat-notif
  │    autonomous Claude Code session
  │
  │  (each tab runs Part D independently)
  │  (workspaces share the jj store — no extra disk)
  │
  ├► feat-auth merged ─► archive on main
  ├► feat-notif merged ─► archive on main
  │
  openspec/specs/ updated only on main
  ── no conflicts
```
