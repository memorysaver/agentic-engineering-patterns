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

```
Main session (interactive with user):
  Part A: scaffold new project (optional)
  Part B: design phases — explore, propose, review
  Part C: launch worktree, send bootstrap prompt
  Part E: archive after merge

Worktree session (autonomous):
  Part D: implement, test, PR — no user interaction needed
  Reads worktree-onboarding.md to catch up with full context
```

## Parallel Worktree Sessions via cmux

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
