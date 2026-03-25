# Agentic Development Workflow

A structured harness for autonomous feature development — from spec to merge.
See [SKILL.md](SKILL.md) for the full step-by-step process.

## Inspired By

This workflow integrates patterns from Anthropic's engineering research on long-running agent harnesses:

- [Harness Design for Long-Running Application Development](https://www.anthropic.com/engineering/harness-design-long-running-apps) — GAN-inspired generator-evaluator pattern, sprint contracts, calibrated QA scoring
- [Effective Harnesses for Long-Running Agents](https://www.anthropic.com/engineering/effective-harnesses-for-long-running-agents) — Initializer/coding agent pattern, structured feature verification lists, session recovery

---

## Harness Architecture

The harness wraps around the agent to provide structure, recovery, and quality assurance. Every `.dev-workflow/` artifact exists because of a specific failure mode observed in Anthropic's research.

```
┌─────────────────────────────────────────────────────────────────────┐
│                         THE HARNESS                                 │
│                                                                     │
│  ┌─────────────┐   ┌──────────────┐   ┌─────────────────────────┐  │
│  │  OpenSpec    │   │  .dev-workflow│   │  Inter-Agent Signals    │  │
│  │  Artifacts   │   │  Harness Dir │   │                         │  │
│  │             │   │             │   │  status.json  ──► main  │  │
│  │  proposal   │   │  contracts  │   │  feedback.md  ◄── main  │  │
│  │  design     │──►│  verify.json│   │  eval-request ──► eval  │  │
│  │  specs/     │   │  init.sh    │   │  eval-response◄── eval  │  │
│  │  tasks      │   │  progress   │   │  ready-review ──► main  │  │
│  └─────────────┘   └──────────────┘   └─────────────────────────┘  │
│         │                  │                      │                  │
│         ▼                  ▼                      ▼                  │
│  ┌─────────────────────────────────────────────────────────────┐    │
│  │                    GENERATOR AGENT                          │    │
│  │                                                             │    │
│  │  Reads specs ► Writes contracts ► Implements via jj stack   │    │
│  │  ► Requests evaluation ► Fixes failures ► Publishes PR      │    │
│  └─────────────────────────────────────────────────────────────┘    │
│                                                                     │
│  ┌─────────────────────────────────────────────────────────────┐    │
│  │                    EVALUATOR AGENT (optional)               │    │
│  │                                                             │    │
│  │  Reads specs + contracts ► Tests running app ► Scores 5     │    │
│  │  dimensions ► Updates verify.json ► Writes eval-response    │    │
│  └─────────────────────────────────────────────────────────────┘    │
│                                                                     │
└─────────────────────────────────────────────────────────────────────┘
```

### Why Each Artifact Exists

```
Problem (from Anthropic research)              Artifact that solves it
───────────────────────────────────────────    ─────────────────────────
Agents self-evaluate too leniently             ► Separate evaluator agent
Evaluator is "terrible" uncalibrated           ► evaluator-criteria.md
Agent builds wrong thing                       ► contracts.md (sprint contracts)
Agent marks features "done" without testing    ► feature-verification.json
Agent can't resume after context reset         ► init.sh (session recovery)
No visibility into workspace progress          ► signals/status.json
No way to send mid-flight feedback             ► signals/feedback.md
Over-scaffolding as models improve             ► Full/light mode selection
```

---

## Harness Tuning

> "Every component in a harness encodes an assumption about what the model
> can't do on its own. Those assumptions deserve stress-testing."
> — Anthropic

```
Model capability ──────────────────────────────────────────►

◄── more scaffolding needed          less scaffolding needed ──►

Sonnet 4.5 era:          Opus 4.6 era:           Future:
┌──────────────┐         ┌──────────────┐        ┌──────────────┐
│ Sprint chunks │         │              │        │              │
│ Context resets│         │ Full sessions│        │ Even simpler │
│ Per-sprint QA │         │ End-of-run QA│        │ evaluator?   │
│ Heavy scaffold│         │ Light scaffold│       │ Fewer phases │
└──────────────┘         └──────────────┘        └──────────────┘

This workflow supports:
  Full mode ── All 13 phases + evaluator   (complex features)
  Light mode ── Skip 3,6,7,8, no evaluator (simple changes)
```

---

## Five-Part Workflow

```
┌──────────────────────────────────────────────────┐
│    Part A — Scaffold (optional)                  │
│                                                  │
│    /monorepo-setup ► /openspec-setup ► verify    │
└──────┬───────────────────────────────────────────┘
       │
       ▼
┌──────────────────────────────────────────────────┐
│    Part B — Design (on main, interactive)        │
│                                                  │
│    Phase 1: /opsx:explore (clarify + research)   │
│        ▼                                         │
│    Phase 2: /opsx:propose (generate artifacts)   │
│        ▼                                         │
│    Phase 3: Design review (security, perf, edge) │
│        ▼                                         │
│    Commit artifacts to main                      │
└──────┬───────────────────────────────────────────┘
       │
       ▼
┌──────────────────────────────────────────────────┐
│    Part C — Launch Workspace (on main)           │
│                                                  │
│    jj st (verify clean)                          │
│        ▼                                         │
│    jj workspace add ► tmux ► cmux tab            │
│        ▼                                         │
│    Send bootstrap prompt to spawned agent        │
│        ▼                                         │
│    Optional: /evaluator-setup (full mode)        │
│        ▼                                         │
│    Monitor via signals/status.json               │
└──────┬───────────────────────────────────────────┘
       │
       ▼
┌──────────────────────────────────────────────────┐
│    Part D — Implementation (in workspace)        │
│                                                  │
│    Phase 0: Init harness ─────────────────────┐  │
│      contracts + verify.json + init.sh        │  │
│      + signals + jj change stack              │  │
│        ▼                                      │  │
│    Phase 4: jj edit each change + /opsx:apply │  │
│        ▼                                      │  │
│    Phase 5: Review ◄──► Evaluator loop        │  │
│        ▼               (if full mode)         │  │
│    Phase 6-8: Dogfood + E2E + Results         │  │
│        ▼                                      │  │
│    Phase 9: Cleanup + push                    │  │
│        ▼                                      │  │
│    Phase 10: Create PR                        │  │
│        ▼                                      │  │
│    Phase 11: PR review loop                   │  │
│        ▼                                      │  │
│    Phase 11.5: Human eval + iteration         │  │
│        ▼                                      │  │
│    Phase 12: Merge                            │  │
└──────┬───────────────────────────────────────────┘
       │  PR merged
       ▼
┌──────────────────────────────────────────────────┐
│    Part E — Post-Merge (on main)                 │
│                                                  │
│    Phase 13: fetch ► archive ► cleanup           │
└──────────────────────────────────────────────────┘
```

---

## Two-Session Model

```
MAIN SESSION (interactive)              WORKSPACE SESSION (autonomous)
━━━━━━━━━━━━━━━━━━━━━━━━━              ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Part A: scaffold ──────────┐
Part B: design ────────────┤
                           │
Part C: launch ────────────┼──cmux──►   Phase 0: init harness
         │                 │                 ▼
         │  poll status.json◄────────   Phase 4: implement
         │  send feedback.md────────►        ▼
         │                 │            Phase 5: evaluator loop
         │  ready-review ◄──────────   Phase 11.5: human eval
         │                 │                 ▼
         │                 │            Phase 12: merge
         │                 │
Part E: archive ◄──────────┘
```

---

## Generator-Evaluator Loop (Full Mode)

```
                    .dev-workflow/signals/
                    ─────────────────────
GENERATOR                                           EVALUATOR
(workspace tab)                                     (evaluator tab)

  Phase 4 ─────────► implement all tasks            reads specs
       │                                            reads contracts
       ▼                                            reads criteria
  Write eval-request.md ─────────────────────────►
                                                    │
                                                    ├─ test running app
                                                    ├─ score 5 dimensions
                                                    ├─ update verify.json
                                                    │
                              ◄──────────────────── Write eval-response-1.md
       │
       ├─ Read response
       ├─ FAIL? Fix issues
       ├─ PASS? Skip to Phase 9
       │
       ▼
  Write eval-request.md (round 2) ───────────────►
                                                    │
                                                    ├─ re-evaluate changes
                                                    │
                              ◄──────────────────── Write eval-response-2.md
       │
       ▼
  All PASS ──► Phase 9: cleanup + publish

  Max 5 rounds, then escalate to human
```

### Evaluator Scoring Dimensions

```
  Completeness ████████░░  4/5  ── All tasks implemented?
  Correctness  ██████████  5/5  ── Works as specified?
  UX Quality   ██████░░░░  3/5  ── Intuitive + accessible?
  Security     ████████░░  4/5  ── Validated + auth'd?
  Code Quality ████████░░  4/5  ── Clean + conventional?

  Hard fail: any < 3, completeness < 4

  Customize dimensions per project type:
    UI-heavy  → weight UX, add Originality
    API-only  → drop UX, add API Design
    Security  → weight Security, add Threat Model
```

---

## .dev-workflow/ Artifact Layout

```
.dev-workflow/                          ← gitignored, per-workspace
├── progress-<change-id>.md             ← checkbox tracking (all 13 phases)
├── contracts.md                        ← per-task success criteria + verification steps
├── feature-verification.json           ← JSON verification list (evaluator updates)
├── init.sh                             ← session recovery script (chmod +x)
├── ports.env                           ← WEB_PORT, SERVER_PORT, BASE_URL
├── code-review-<feature>.md            ← Phase 5 findings
├── dogfood-<feature>.md                ← Phase 6 report
├── human-eval-round-<N>.md             ← Phase 11.5 findings
├── pr-fix-plan-<round>.md              ← Phase 11 fix strategy
└── signals/                            ← inter-agent communication
    ├── status.json                     ← generator writes, main session reads
    ├── feedback.md                     ← main session writes, generator reads
    ├── ready-for-review.flag           ← generator creates when ready for human eval
    ├── eval-request.md                 ← generator writes, evaluator reads
    └── eval-response-<N>.md            ← evaluator writes, generator reads
```

---

## Parallel Workspace Sessions

```
main workspace (cmux)
  │
  ├──► jj workspace add ──► tab: feat-auth ──► tab: evaluator-auth
  │     generator agent          evaluator agent (full mode)
  │     status.json ──────────────────► main can poll
  │
  ├──► jj workspace add ──► tab: feat-notif
  │     generator agent (light mode, no evaluator)
  │     status.json ──────────────────► main can poll
  │
  │   (each workspace runs Part D independently)
  │   (all share the jj store — no extra disk)
  │   (monitor all via signals/status.json per workspace)
  │
  ├──► feat-auth merged ──► archive on main
  ├──► feat-notif merged ──► archive on main
  │
  openspec/specs/ updated only on main — no conflicts
```
