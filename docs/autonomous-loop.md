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
│  /dispatch  — What to build next    │
│  /design    — Spec the details      │
│  /launch    — Spawn workspace       │
│  /build     — Implement + verify    │
│  /wrap      — Archive + cleanup     │
│  (loop)     — Back to /dispatch     │
│                                     │
└─────────────────────────────────────┘
```

The boundary is at `/validate`. Once the product context passes both product design evaluation (Pass 1) and technical validation (Pass 2), the autonomous loop takes over.

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
    skip_human_eval: backend    # or: all, none
```

### 3. Start the loop

```
/dispatch --auto
```

---

## The Autonomous Cycle

```
/dispatch --auto
    │
    ├─ Sync signals from all workspaces
    ├─ Cascade state transitions (pending → ready)
    ├─ Score ready stories by dispatch_score
    ├─ Check layer completion → run gate if needed
    │
    ├─ For each dispatchable story:
    │   │
    │   ├─ Well-specified? → /launch directly
    │   │                     └─ /build (autonomous)
    │   │                         └─ Phase 0-12
    │   │                             └─ PR merged
    │   │
    │   └─ Ambiguous? → /design (Phase 2 only, auto)
    │                    └─ /launch
    │                        └─ /build (autonomous)
    │                            └─ PR merged
    │
    ├─ /wrap (per completed workspace)
    │   ├─ Archive OpenSpec change
    │   ├─ Update product-context.yaml
    │   └─ Trigger next dispatch cycle
    │
    └─ Loop continues until:
        ├─ All stories in all layers complete → notify human
        ├─ Escalation trigger hit → pause and notify human
        └─ No stories ready → wait for in-progress to complete
```

---

## What Changes in Autonomous Mode

| Phase | Interactive (default) | Autonomous (`--auto`) |
|-------|----------------------|----------------------|
| Dispatch Step 4 | Show queue, user picks | Skipped — auto-select by score |
| Dispatch Step 7 | User chooses /launch or /design | Auto-route by spec maturity |
| /design Phase 1 | Interactive explore with user | Skipped — use product context |
| /design Phase 3 | Interactive review with user | Skipped — trust /validate |
| /build Phase 5 | Eval loop (up to 5 rounds) | Same — fully autonomous already |
| /build Phase 11.5 | Human tests running app | Skipped for non-UI, agent-browser for UI |
| /build Phase 12 | User confirms merge | Auto-merge when CI green |
| /wrap next step | Suggests /dispatch | Auto-triggers /dispatch --auto |

---

## Escalation Triggers

The autonomous loop stops and notifies the human when:

| Trigger | Why | What human does |
|---------|-----|----------------|
| Eval loop fails after 5 rounds | Generator can't fix the issues | Review findings, possibly split the story |
| Layer gate test fails | Integration tests don't pass | Debug the integration, possibly add fix stories |
| No stories ready | All blocked or failed | Check what's stuck, reset failed stories |
| Story fails 3 times | Max retries exceeded | Review the story spec, possibly redesign |
| All layers complete | Product is done | Run /reflect, plan next version |

**Escalation is not failure.** It means the agent reached a decision point that requires human judgment. The agent preserves all context (workspace, signals, eval logs) so the human can resume without information loss.

---

## What Humans Focus On

With the implementation loop automated, humans invest time in:

1. **Product vision** (`/envision`, `/reflect`) — Is this the right product? What did we learn from shipping?
2. **Architecture** (`/map`) — Are the module boundaries right? Do we need to refactor?
3. **Design quality** (`/validate` Pass 1) — Does the story map make sense? Are layers ordered correctly?
4. **UI/UX design** — Visual design, interaction patterns, user testing
5. **Escalation handling** — Resolve agent escalations when they hit decision points
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
