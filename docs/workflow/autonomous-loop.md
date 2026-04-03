# Autonomous Feature Loop

How the AEP workflow supports fully autonomous implementation. Humans own the product design; agents own the implementation loop.

---

## The Boundary

```
┌─────────────────────────────────────┐
│          HUMAN DOMAIN               │
│                                     │
│  /envision  — What to build         │
│  /map       — How to decompose it   │
│  /validate  — Is the design right?  │
│  UI/UX      — How it should look    │
│  /reflect   — What did we learn?    │
│                                     │
├─────────────────────────────────────┤
│          AGENT DOMAIN               │
│                                     │
│  /autopilot — Orchestrate the loop  │
│  /dispatch  — What to build next    │
│  /design    — Spec the details      │
│  /launch    — Spawn workspace       │
│  /build     — Implement + verify    │
│  /wrap      — Archive + cleanup     │
│                                     │
└─────────────────────────────────────┘
```

The boundary is at `/validate`. Once the product context passes both product design evaluation (Pass 1) and technical validation (Pass 2), the autonomous loop takes over via `/autopilot`.

---

## Enabling Autonomous Mode

### 1. Validate the product context

```
/validate    ← runs product design + technical evaluation
```

Both passes must clear:

- **Pass 1 (Product Design):** Walking skeleton validity >= 3, vision alignment >= 3, INVEST compliance >= 3
- **Pass 2 (Technical):** All dispatch fields present, DAG valid, codebase claims verified

### 2. Set autonomous routing in product-context.yaml

```yaml
topology:
  routing:
    autonomous: true
    concurrency_limit: 5
    auto_merge: true
    auto_design: true
    skip_human_eval: backend # or: all, none
```

### 3. Start the autopilot

```
/autopilot
```

One command. Initializes state, runs the first tick, and starts a recurring loop (default: every 5 minutes). Use `--loop` to customize the interval:

```
/autopilot --loop 10m
```

---

## The Autonomous Cycle

Autopilot runs as a tick-based state machine. Every 5 minutes, a tick executes:

```
/autopilot tick
    │
    ├─ ① Sync signals from all workspace status.json files
    ├─ ② Wrap completed workspaces (max 1 per tick)
    │   ├─ Archive OpenSpec change
    │   ├─ Update product-context.yaml
    │   └─ Remove workspace
    │
    ├─ ③ Merge ready PRs (CI green + eval PASS + reviews OK)
    │
    ├─ ④ Trigger code review via tmux for workspaces that need it
    │   └─ Workspace runs its own gen/eval loop (Phase 5)
    │
    ├─ ⑤ Detect stuck workspaces → nudge or escalate
    │
    ├─ ⑥ Dispatch new work (max 1 launch per tick)
    │   ├─ Score ready stories by dispatch_score
    │   ├─ Check design escalation → pause if needed
    │   ├─ /dispatch + /launch for top story
    │   └─ Check layer completion → gate test if needed
    │
    └─ ⑦ Write state + history + status
```

The cycle continues until:

- All stories in all layers complete → stop and notify human
- Design escalation → pause and notify human
- Layer gate fails → pause and notify human
- No stories ready → wait for in-progress workspaces

---

## What Changes with Autopilot

| Phase                   | Interactive (manual)                | Autopilot                                          |
| ----------------------- | ----------------------------------- | -------------------------------------------------- |
| Dispatch                | User picks stories                  | Auto-select by score, 1 per tick                   |
| Design routing          | User chooses /launch or /design     | Auto-route; escalate ambiguous to human            |
| Code review             | Workspace self-orchestrates Phase 5 | Autopilot triggers via tmux if workspace misses it |
| Human eval (Phase 11.5) | User tests running app              | Skipped for non-UI, agent-browser for UI           |
| Merge (Phase 12)        | User confirms merge                 | Auto-merge when CI green + eval PASS               |
| Wrap                    | User runs /wrap                     | Autopilot runs /wrap on next tick                  |
| Next dispatch           | User runs /dispatch                 | Autopilot dispatches on next tick                  |

---

## Escalation Triggers

The autopilot pauses and notifies the human when:

| Trigger                        | Why                            | What human does                                  |
| ------------------------------ | ------------------------------ | ------------------------------------------------ |
| Design needed                  | Story is ambiguous or UI-heavy | Run `/design` to refine, then `/autopilot start` |
| Eval loop fails after 5 rounds | Generator can't fix the issues | Review findings, possibly split the story        |
| Layer gate test fails          | Integration tests don't pass   | Debug the integration, possibly add fix stories  |
| No stories ready               | All blocked or failed          | Check what's stuck, reset failed stories         |
| Story fails 3 times            | Max retries exceeded           | Review the story spec, possibly redesign         |
| All layers complete            | Product is done                | Run `/reflect`, plan next version                |
| Workspace stuck 60 min         | Agent unresponsive             | Check tmux session, restart if needed            |

**Escalation is not failure.** It means the agent reached a decision point that requires human judgment. All context (workspace, signals, eval logs) is preserved so the human can resume without information loss.

When autopilot pauses, it writes detailed guidelines to `.dev-workflow/autopilot-status.md` explaining why, what needs attention, and what human feedback is expected.

---

## Separation of Concerns: Gen/Eval

Autopilot maintains strict separation between two gen/eval uses:

| Concern                    | Owner                    | What it evaluates                                  |
| -------------------------- | ------------------------ | -------------------------------------------------- |
| **Code quality**           | Workspace agent          | Code correctness, security, completeness           |
| **Orchestration learning** | Autopilot (main session) | Cross-workspace patterns: failures, costs, retries |

Autopilot **triggers** workspace code review via tmux but never evaluates code itself. The main session gen/eval analyzes orchestration quality and produces findings for `/reflect`.

---

## What Humans Focus On

With the implementation loop automated, humans invest time in:

1. **Product vision** (`/envision`, `/reflect`) — Is this the right product? What did we learn from shipping?
2. **Architecture** (`/map`) — Are the module boundaries right? Do we need to refactor?
3. **Design quality** (`/validate` Pass 1) — Does the story map make sense? Are layers ordered correctly?
4. **UI/UX design** — Visual design, interaction patterns, user testing
5. **Escalation handling** — Resolve autopilot pauses when stories need human design input
6. **Layer gate review** — Verify integration tests pass before advancing to next layer

---

## Cost Model

Autonomous operation has a predictable cost structure:

```
Per story:
  Implementation:  ~50K tokens (agent builds the feature)
  Evaluation:      ~10K tokens (separate agent reviews)
  Integration test: ~30K tokens (per integration story)

Per layer:
  Stories × implementation + evaluation + integration tests
  Layer 0 example: 15 stories × 60K = ~900K tokens + 90K integration = ~990K tokens

Per dispatch cycle:
  Signal sync + cascade + scoring + context assembly: ~5K tokens
  OpenSpec change creation: ~10K tokens
```

The cost scales linearly with story count. The biggest cost driver is the number of eval loop rounds — converging in 1-2 rounds is 3-5x cheaper than hitting the 5-round max.

---

## Prerequisites

Before enabling autonomous mode, verify:

- [ ] `product-context.yaml` has `opportunity` + `product` sections (from /envision)
- [ ] `architecture`, `stories`, `topology` sections present (from /map)
- [ ] `/validate` passed both product design and technical evaluation
- [ ] All stories have `business_value`, `complexity`, `attempt_count`, `failure_logs`
- [ ] `dispatch_epoch: 0` is set at the top level
- [ ] Layer gates are defined with `status: pending`
- [ ] `topology.routing.autonomous: true` is set
- [ ] Project has `.claude/hooks/workspace-setup.sh` for workspace bootstrapping
- [ ] CI pipeline is configured (GitHub Actions or equivalent) for auto-merge to work
