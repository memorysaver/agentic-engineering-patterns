# Agentic Development Workflow

A structured harness for autonomous feature development — from spec to merge.

Four skills, one lifecycle:

```
/design → /launch → /build → /wrap
```

| Skill | What it does | Session |
|-------|-------------|---------|
| [/design](design/SKILL.md) | Explore + propose + review | Main, interactive |
| [/launch](launch/SKILL.md) | Spawn workspace + evaluator | Main, automated |
| [/build](build/SKILL.md) | Init → implement → test → PR → merge | Workspace, autonomous |
| [/wrap](wrap/SKILL.md) | Archive + cleanup | Main, post-merge |
| [/jj-ref](jj-ref/SKILL.md) | jj command reference | On-demand |

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
  Full mode ── All phases + evaluator    (complex features)
  Light mode ── Skip review phases, no evaluator (simple changes)
```

---

## Two-Session Model

```
MAIN SESSION (interactive)              WORKSPACE SESSION (autonomous)
━━━━━━━━━━━━━━━━━━━━━━━━━              ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

/design ──────────────────┐
                          │
/launch ──────────────────┼──cmux──►   /build
         │                │              Phase 0: init harness
         │  poll status.json◄────────        ▼
         │  send feedback.md────────►   Phase 4: implement
         │                │                  ▼
         │  ready-review ◄──────────   Phase 5-8: review + test
         │                │                  ▼
         │                │            Phase 9-12: PR + merge
         │                │
/wrap ◄───────────────────┘
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
                              ◄──────────────────── Write eval-response-2.md
       │
       ▼
  All PASS ──► Phase 9: cleanup + publish

  Max 5 rounds, then escalate to human
```

---

## .dev-workflow/ Artifact Layout

```
.dev-workflow/                          ← gitignored, per-workspace
├── progress-<change-id>.md             ← checkbox tracking
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
