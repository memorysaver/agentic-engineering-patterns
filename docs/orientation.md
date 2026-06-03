# AEP Orientation Guide

**A 10-minute first-hour tour for new users.** Read this before (or right after) running `/onboard`. When you finish, you'll know what AEP is, the three mental models that drive every skill, what each of the 16 skills does, and which of four concrete paths matches your situation.

For precise definitions of every term used here, see the [Glossary](glossary.md). For a one-page decision tree, see the [Skills Quick Reference](skills-quick-reference.md).

---

## 1. Why AEP Exists

Traditional software development bottlenecks on human coding time. Process design optimizes "how to make people write code faster."

When agents can execute dozens of tasks in parallel, that bottleneck vanishes. A new one takes its place:

> **Agent execution capacity is near-infinite. Specification quality is not.**

Vague specs don't slow down a human — they ask a colleague and adjust. Vague specs paralyze agents — they guess, diverge, and produce incompatible code across parallel sessions. The cost of ambiguity scales with parallelism.

AEP inverts the design logic: **invest heavily in spec precision up front, then let agents execute in parallel.** A 10-minute conversation in `/envision` saves hours of agent confusion downstream.

Read the full argument in [README.md "Why This Exists"](../README.md#why-this-exists).

---

## 2. The Mental Model in 3 Concepts

These three concepts are the backbone of every AEP skill. If you internalize these, the rest of the system is mostly discoverable.

### 2.1 Control Plane vs Execution Plane

```
┌─────────────────────────────────────┐
│         CONTROL PLANE               │   ← you + AI decide WHAT to build
│   /envision  /map  /dispatch        │     goals, architecture, priorities
│   /calibrate /reflect               │     product-context.yaml lives here
├─────────────────────────────────────┤
│         EXECUTION PLANE             │   ← agents build it
│   /design  /launch  /build  /wrap   │     precise specs in, merged PRs out
│                                     │     agents never decide what to build
└─────────────────────────────────────┘
```

**The rule:** You work on the control plane. Agents work on the execution plane. They communicate only through structured artifacts — `product-context.yaml`, OpenSpec changes, and signal files — never by talking to each other.

**Why it matters:** When you're deciding _what_ to build, you should be on `main` with fresh context. When an agent is _building_, it should be in an isolated workspace with fresh implementation context. Mixing the two wastes tokens and corrupts reasoning.

More: [README.md "The Mental Model"](../README.md#the-mental-model).

### 2.2 The Story Map (Jeff Patton)

AEP organizes all work as a **[Jeff Patton story map](https://www.jpattonassociates.com/user-story-mapping/)** — a grid with four axes:

```
        Authenticate → Configure → Monitor → Review      ← Activities (columns)
        ───────────────────────────────────────────
Layer 0 │ [auth-v0]    [cfg-v0]   [mon-v0]  [rev-v0]    ← Walking Skeleton
        │              (Wave 1 — parallel stories)
        ─── Release Line ────────────────────────
Layer 1 │ [oauth]      [guards]   [alerts]  [audit]     ← Core features
        │
        ─── Release Line ────────────────────────
```

- **Activities** (columns, left→right): the ordered user journey. Extracted by `/envision`.
- **Layers** (rows, top→down): enrichment. Layer 0 = the **walking skeleton** — the thinnest end-to-end path that proves the architecture works.
- **Waves** (within a layer): parallel batches of stories with no mutual dependencies. `/dispatch` can launch a whole wave at once.
- **Release lines**: horizontal cuts saying "everything above is shippable".
- **`.5` alignment layers** (0.5, 1.5, ...): human calibration checkpoints between integer build layers.

**Why it matters:** `/envision` extracts activities, `/map` fills in stories and waves, `/dispatch` picks from the grid by score. The whole roadmap is visible, versioned, and machine-readable from day one.

Full ASCII diagram: [README.md "The Story Map"](../README.md#the-story-map).

### 2.3 Two-Session Model

```
MAIN SESSION                         WORKSPACE SESSION
(you + AI, on main branch)           (one agent, in a git worktree on feat/<name>)
─────────────────────────────        ────────────────────────────────
/envision → /map → /dispatch          /build
/design (refine spec)                   init harness
/launch  ──────────────────────►        implement each task
(keeps signal files in sync)            code review + eval
/wrap    ◄──────── merges PR  ◄─        create PR, handle review, merge
```

- **Main session**: interactive, on `main` branch. Where you + AI make high-leverage decisions. Runs `/envision`, `/map`, `/dispatch`, `/design`, `/wrap`, `/reflect`, `/calibrate`.
- **Workspace session**: autonomous, in an isolated git worktree on a `feat/<name>` branch. Where one agent implements a feature end-to-end. Spawned by `/launch`; runs `/build`.
- **Communication channel**: **signal files** only, in `.dev-workflow/signals/` (status.json, feedback.md, eval-request.md, ready-for-review.flag). The main session never reads workspace code directly.

**Why two sessions:** design needs fresh human-judgment context; implementation needs fresh spec-execution context. Separating them lets the agent work autonomously for hours while you do other things.

More: [README.md "The Feature Lifecycle"](../README.md) and [skills/agentic-development-workflow/README.md](../skills/agentic-development-workflow/README.md).

---

## 3. The 16 Skills at a Glance

| Skill                | Plugin                       | Session   | Purpose                                                         |
| -------------------- | ---------------------------- | --------- | --------------------------------------------------------------- |
| `/onboard`           | project-setup                | Main      | Install tools, verify env, configure plugins, orient new users  |
| `/scaffold`          | project-setup                | Main      | Scaffold a full-stack TypeScript monorepo + initialize OpenSpec |
| `/testing-guide`     | project-setup                | Main      | Reference for workspace-level quality infrastructure            |
| `/envision`          | product-context              | Main      | Opportunity brief + context document (what to build)            |
| `/map`               | product-context              | Main      | System map + story graph + agent topology (how to decompose)    |
| `/validate`          | product-context              | Main      | Generator/evaluator validation of any AEP artifact              |
| `/dispatch`          | product-context              | Main      | Pick next story + create OpenSpec change + hand off             |
| `/calibrate`         | product-context              | Main      | Human alignment checkpoint for any quality dimension            |
| `/reflect`           | product-context              | Main      | Classify feedback + update context (close the loop)             |
| `/design`            | agentic-development-workflow | Main      | Interactive feature design (explore + propose + review)         |
| `/launch`            | agentic-development-workflow | Main      | Spawn autonomous workspace + optional evaluator                 |
| `/build`             | agentic-development-workflow | Workspace | Implement → test → PR → merge (autonomous)                      |
| `/wrap`              | agentic-development-workflow | Main      | Post-merge archive + cleanup + update story status              |
| `/git-ref`           | agentic-development-workflow | Main      | AEP git + worktree conventions (on-demand)                      |
| `/autopilot`         | patterns                     | Main      | Hands-free dispatch → launch → monitor → wrap loop              |
| `/gen-eval`          | patterns                     | Both      | Reusable generator/evaluator pattern for honest evaluation      |
| `/workflow-feedback` | patterns                     | Main      | Capture + review process learnings from builds                  |

For when-to-use decisions, see [docs/skills-quick-reference.md](skills-quick-reference.md).

---

## 4. The Four Paths — Pick Your Situation

### Path A — New product from scratch

You have an idea and a fresh repo. Start here.

```
/envision  →  /map  →  /validate  →  /scaffold  →  /autopilot
```

- `/envision` validates the opportunity and extracts the activity backbone.
- `/map` decomposes the product into a system map, story graph, and agent topology.
- `/validate` runs generator/evaluator checks on the product context.
- `/scaffold` creates the full-stack monorepo and initializes OpenSpec.
- `/autopilot` (optional) takes over hands-free execution — or you can drive it manually with `/dispatch → /design → /launch → /build → /wrap`.

### Path B — Onboarding an existing project

You have an existing codebase and want to add AEP workflows to it.

```
/scaffold  →  /envision (optional)  →  /dispatch  →  /design  →  /launch  →  /build  →  /wrap
```

- `/scaffold` adds agentic infrastructure (OpenSpec, workspace hooks, test skeleton) to existing code.
- `/envision` is optional here — use it if you want to retrofit a product context for the codebase you already have.
- Then start a feature cycle with `/dispatch`.

### Path C — Single feature, no product context

You just want to ship one feature with AEP workflows but don't need the full product map.

```
/design  →  /launch  →  /build  →  /wrap
```

- `/design` explores, proposes, and produces an OpenSpec change on `main`.
- `/launch` spawns an isolated workspace and boots the agent.
- `/build` implements, tests, reviews, and merges.
- `/wrap` archives and cleans up.

### Path D — Hands-free autonomous mode

You have a validated product context and want to go grab coffee.

```
/autopilot
```

One command. Autopilot ticks every 5 minutes: sync signals → wrap completed → guide stuck workspaces → dispatch new stories → write state. It pauses only for design escalations, layer gate failures, or when it runs out of ready stories.

Deep dive: [docs/workflow/autonomous-loop.md](workflow/autonomous-loop.md).

---

## 5. v2 Split-Mode in One Minute

Recent AEP projects can store product context in **split mode** — two files instead of one. All skills auto-detect which mode a project uses; you don't pick manually.

| File                   | What it contains                                                     | How often it changes                   |
| ---------------------- | -------------------------------------------------------------------- | -------------------------------------- |
| `product/index.yaml`   | **Stable intent** — opportunity, personas, capabilities, constraints | Rarely (only when intent shifts)       |
| `product-context.yaml` | **Mutable state** — architecture, stories, topology, cost, changelog | Every `/dispatch`, `/wrap`, `/reflect` |

**When to use split-mode:** products with 2+ distinct user journeys (e.g., buyer flow + seller flow). Multi-journey products can also add per-capability map files under `product/maps/<capability-id>/{frame.yaml,map.yaml}`. Simple single-journey products can stay with a single `product-context.yaml` and skip capability maps entirely.

**How skills detect it:** every product-context skill runs a startup check along the lines of `[ -f product/index.yaml ] && echo "split-mode"`. Everything else is automatic.

Deep dive: [docs/aep-v2-improvement-guideline.md](aep-v2-improvement-guideline.md). This doc covers capability maps, enhanced dispatch scoring, readiness score routing, outcome contracts, technical specs, and grouped changes.

---

## 6. Critical Concepts to Know (Glossary Shortlist)

One-line pointers so you know what to look up when you hit an unfamiliar term. Full definitions are in [docs/glossary.md](glossary.md).

- **Walking skeleton (Layer 0)** — the thinnest end-to-end path through all activities. Build thin everywhere before going deep anywhere.
- **Layer gate** — an integration test that verifies a completed layer works as a whole. Automated, not a human approval.
- **`.5` alignment layer / Calibration** — optional human checkpoint between integer layers. `/calibrate` supports 7 quality dimensions (visual-design, ux-flow, api-surface, data-model, scope-direction, copy-tone, performance-quality).
- **Signal files & concurrency protocol** — workspace agents write `.dev-workflow/signals/status.json`; main session writes feedback. Only the main session writes `product-context.yaml`. A hook in `.claude/settings.json` enforces this.
- **Dispatch score** — the formula `/dispatch` uses to pick stories: `(business_value + unblock_potential + critical_path_urgency + reuse_leverage) / (complexity_cost + ambiguity_penalty + interface_risk)`.
- **Generator/evaluator separation** — agents praise their own work; a separate evaluator catches what they miss. Used by `/validate`, `/build` (evaluator loop), and `/gen-eval`. One of the most durable patterns from Anthropic's research.
- **Readiness score routing** (v2) — `/dispatch` checks each story's spec completeness. Score < 0.5 routes to `/design`; score >= 0.7 routes straight to `/launch`.
- **Outcome contracts** (v2) — layer-level success metrics with hypothesis, success_metric, and decision_rule. `/reflect` uses them to decide whether to advance or re-slice.
- **OpenSpec** — the spec-driven development CLI (`explore → propose → apply → archive`) that `/design`, `/dispatch`, and `/build` operate on.
- **Git worktrees** — `git worktree add -b feat/<name> .feature-workspaces/<name> main` gives each agent an isolated working tree on its own branch. Worktrees share `.git/objects` (no history duplication) but each adds one working-tree copy on disk. AEP migrated from jj to git+worktree in 2026-04 — see [decisions/migrate-from-jj-to-git.md](decisions/migrate-from-jj-to-git.md).

---

## 7. Your First-Hour Checklist

- [ ] Run `/onboard` (if not already done) — installs tools, verifies env, configures plugins.
- [ ] Read [README.md "Why This Exists"](../README.md#why-this-exists) and [README.md "The Mental Model"](../README.md#the-mental-model) (5–10 minutes).
- [ ] Skim [docs/glossary.md](glossary.md) — don't memorize, just notice the terms that exist so you know what to look up later.
- [ ] Pick your path from Section 4 above (A/B/C/D).
- [ ] Run the **first** command of your path — don't batch them, do one at a time and read the output.
- [ ] When stuck, check [docs/skills-quick-reference.md](skills-quick-reference.md) for the decision tree.
- [ ] If you need a refresher on AEP's git + worktree conventions, run `/git-ref`. For any other unfamiliar command, read the skill's own `SKILL.md` directly.

---

## 8. Where to Go Next

| I want to…                                       | Read                                                                                                            |
| ------------------------------------------------ | --------------------------------------------------------------------------------------------------------------- |
| Understand the full workflow philosophy          | [README.md](../README.md) (10 min)                                                                              |
| Look up a specific term                          | [docs/glossary.md](glossary.md)                                                                                 |
| Decide which skill to use                        | [docs/skills-quick-reference.md](skills-quick-reference.md)                                                     |
| Understand v2 upgrades                           | [docs/aep-v2-improvement-guideline.md](aep-v2-improvement-guideline.md)                                         |
| Run hands-free autonomous mode                   | [docs/workflow/autonomous-loop.md](workflow/autonomous-loop.md)                                                 |
| Understand the generator/evaluator pattern       | [docs/workflow/gen-eval-data-flow.md](workflow/gen-eval-data-flow.md)                                           |
| Learn AEP's git + worktree conventions           | [skills/agentic-development-workflow/git-ref/SKILL.md](../skills/agentic-development-workflow/git-ref/SKILL.md) |
| Understand how the product-context plugin thinks | [skills/product-context/README.md](../skills/product-context/README.md)                                         |
| Understand how the dev-workflow plugin thinks    | [skills/agentic-development-workflow/README.md](../skills/agentic-development-workflow/README.md)               |

---

**You're done with orientation.** The rest of AEP is discoverable from the three mental models, the 16-skill table, and the four paths. When in doubt, reach for the decision tree in the quick reference — it covers the common forks.
