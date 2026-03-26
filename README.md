# Agentic Engineering Patterns

A Claude Code plugin for building software products with AI agents — from raw idea to shipped MVP.

## Why This Exists

Traditional software development bottlenecks on human coding time. Process design optimizes "how to make people write code faster."

When agents can execute dozens of tasks in parallel, that bottleneck vanishes. A new one takes its place:

> **Agent execution capacity is near-infinite. Specification quality is not.**

Vague specs don't slow down a human — they ask a colleague and adjust. Vague specs paralyze agents — they guess, diverge, and produce incompatible code across parallel sessions. The cost of ambiguity scales with parallelism.

This inverts the entire design logic:

```
Traditional:    plan roughly → adjust as you go → ship
                (optimizes for human coding speed)

Agentic:        invest heavily in spec precision → parallel execution → ship
                (optimizes for agent execution quality)
```

Every skill in this plugin serves that logic. The time you spend in `/envision` and `/map` pays back exponentially when agents build in parallel without asking questions.

## The Mental Model

The workflow separates **thinking** from **doing**:

```
┌─────────────────────────────────────────────────────────────────┐
│                                                                 │
│   CONTROL PLANE  (human decides what to build)                  │
│                                                                 │
│   You + AI collaborate on high-leverage decisions:              │
│   goals, decomposition, architecture, priorities, feedback      │
│                                                                 │
│   ┌──────────┐    ┌──────────┐    ┌──────────┐                 │
│   │ /envision │───►│  /map    │───►│ /reflect │──┐              │
│   │          │    │          │    │          │  │              │
│   │ what to  │    │ how to   │    │ what we  │  │              │
│   │ build    │    │ break it │    │ learned  │  │              │
│   │          │    │ down     │    │          │  │              │
│   └──────────┘    └──────────┘    └──────────┘  │              │
│        ▲                               │         │              │
│        └───────────────────────────────┘         │              │
│                  feedback loop                   │              │
│                                                  │              │
└──────────────────────────────────────────────────┼──────────────┘
                                                   │
                        structured artifacts flow down
                        (context doc, system map, story specs)
                                                   │
┌──────────────────────────────────────────────────┼──────────────┐
│                                                  │              │
│   EXECUTION PLANE  (agents build it)             ▼              │
│                                                                 │
│   Agents receive precise specs, work in isolation,              │
│   produce PRs. They don't decide what to build.                 │
│                                                                 │
│   ┌──────────┐    ┌──────────┐    ┌──────────┐    ┌─────────┐  │
│   │ /design  │───►│ /launch  │───►│  /build  │───►│  /wrap  │  │
│   │          │    │          │    │          │    │         │  │
│   │ spec the │    │ spawn    │    │ implement│    │ archive │  │
│   │ feature  │    │ agent    │    │ + test   │    │ + clean │  │
│   │          │    │          │    │ + PR     │    │         │  │
│   └──────────┘    └──────────┘    └──────────┘    └─────────┘  │
│                                                                 │
│   (repeat per feature — multiple features run in parallel)      │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

**Agents don't talk to each other.** They communicate through structured artifacts — context documents, story specs, interface contracts, signal files. The harness coordinates everything. This is a production system design, not a chatroom-style agent swarm.

## The Three Plugins

Each plugin implements one layer of the mental model.

### 1. Product Context — the persistent map

Captures the "what and why" of the entire product. Lives in `product-context/` at the project root, committed to git, and evolves over time.

```
/envision                        /map                            /reflect
    │                               │                               │
    ▼                               ▼                               ▼
Opportunity Brief               System Map                      Feedback Log
"should we build this?"         "what are the modules?"         "what did we learn?"
    │                               │                               │
    ▼                               ▼                               │
Context Document                Story Graph                     Update context docs,
"what exactly to build,         "layered work items,            story graph, or
 for whom, within               execution slices,               architecture based
 what constraints"              dependencies"                   on what category
    │                               │                           of feedback
    │                               ▼
    │                           Agent Topology
    │                           "who does what,
    │                            handoff contracts,
    │                            routing rules"
    │                               │
    └───────────────┬───────────────┘
                    │
                    ▼
            feeds into /design
            (each story becomes a feature)
```

**Why this exists:** Without a product-level map, each feature is designed in isolation. Agents build incompatible pieces. Module boundaries are implicit. The product context makes the whole system visible before any code is written.

### 2. Feature Lifecycle — the execution cycle

Takes one story from the map and turns it into a merged PR. Runs in a two-session model:

```
MAIN SESSION (you + AI)                WORKSPACE SESSION (agent alone)
━━━━━━━━━━━━━━━━━━━━━━                ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

/design
  explore the problem
  propose a solution          ────►   /build
  review the design                     init tracking + jj change stack
         │                              implement each task
/launch                                 code review (+ evaluator loop)
  create jj workspace                   browser testing
  bootstrap agent             ◄────     create PR, handle review
  optional: spawn evaluator             merge
         │                                     │
/wrap    ◄─────────────────────────────────────┘
  archive OpenSpec change
  clean up workspace
  suggest /reflect
```

**Why two sessions:** Design needs human judgment — you decide direction, scope, tradeoffs. Implementation is mechanical — the agent follows the spec, implements, tests, publishes. Separating them lets the agent work autonomously for hours while you do other things.

**Why jj (not git):** Changes are mutable until published. No staging area. Auto-rebase when editing earlier changes. `jj workspace add` gives each agent an isolated working copy with no extra disk space. The agent generates rough code, then cleans up with `split`/`squash` — a natural post-generation step.

### 3. Project Setup — the one-time foundation

Gets your machine and project ready. Run once.

```
/onboard                             /scaffold
    │                                    │
    ▼                                    ▼
Verify tools                         Scaffold monorepo
(jj, bun, git, gh,                   (Better-T-Stack: frontend,
 claude, openspec,                    backend, database, auth,
 tmux, cmux)                         API layer, addons)
    │                                    │
    ▼                                    ▼
Install plugins                      Initialize OpenSpec
(superpowers, agent-browser,         (explore/propose/apply/archive
 frontend-design, mgrep)             commands for spec-driven dev)
```

## The Feedback Loop

The workflow is a loop, not a line. After shipping features, `/reflect` classifies what you learned:

```
                    ┌──────────────────────────────────┐
                    │                                  │
     ┌──────────── │ ◄── opportunity shift             │
     │              │      (back to /envision)          │
     │              │                                  │
     │  ┌───────── │ ◄── discovery                     │
     │  │           │      (update /envision or /map)   │
     │  │           │                                  │
     │  │  ┌────── │ ◄── refinement                    │
     │  │  │        │      (new story in next layer)    │
     │  │  │        │                                  │
     │  │  │  ┌─── │ ◄── bug                           │
     │  │  │  │     │      (fix story, back to /design) │
     │  │  │  │     │                                  │
     │  │  │  │     └──────────────────────────────────┘
     │  │  │  │              /reflect
     ▼  ▼  ▼  ▼
  Each feedback type routes to the right phase.
  The product context evolves. The cycle continues.
```

## Design Principles

These aren't rules we invented — they're patterns extracted from Anthropic's engineering research on long-running agent harnesses:

**Spec precision over implementation speed.** Time invested in unambiguous specs pays back exponentially across parallel agents. A 10-minute conversation in `/envision` saves hours of agent confusion.

**Walking skeleton first.** Build the thinnest end-to-end path (Layer 0) before going deep into any module. Validate the architecture at minimum cost. Going deep before proving the skeleton works is the most expensive mistake.

**Every harness component earns its place.** Sprint contracts, verification JSON, signal files, evaluator agents — each exists because of a specific failure mode observed in practice. As models improve, stress-test each component and remove what's no longer needed.

**Generator-evaluator separation.** Agents praise their own work even when it's mediocre. A separate evaluator agent, calibrated toward skepticism, catches problems the builder missed. This is the single most durable pattern from Anthropic's research.

## Getting Started

**New to this plugin?**
```
/onboard
```
Installs prerequisites, verifies tools, configures recommended plugins.

**Have a product idea?**
```
/envision  →  /map  →  /scaffold
```
Validate the opportunity, decompose into stories, scaffold the project.

**Ready to build a feature?**
```
/dispatch  →  /design  →  /launch  →  /build  →  /wrap
```
Pick a story from the map, spec it, spawn the agent, let it build, archive when merged.

**Shipped something? Close the loop:**
```
/reflect
```
Classify feedback, update the product context, plan the next iteration.

## All Skills

| Skill | Plugin | Purpose |
|-------|--------|---------|
| `/envision` | product-context | Opportunity brief + context document |
| `/map` | product-context | System map + story graph + agent topology |
| `/dispatch` | product-context | Pick next story + create OpenSpec change |
| `/reflect` | product-context | Classify feedback + update context |
| `/onboard` | project-setup | Verify tools + install plugins |
| `/scaffold` | project-setup | Scaffold monorepo + initialize OpenSpec |
| `/design` | agentic-development-workflow | Explore + propose + review a feature |
| `/launch` | agentic-development-workflow | Spawn workspace + optional evaluator |
| `/build` | agentic-development-workflow | Implement → test → PR → merge |
| `/wrap` | agentic-development-workflow | Archive + cleanup + suggest reflect |
| `/jj-ref` | agentic-development-workflow | jj command reference (on-demand) |

## Inspired By

- [Harness Design for Long-Running Application Development](https://www.anthropic.com/engineering/harness-design-long-running-apps) — Anthropic Engineering
- [Effective Harnesses for Long-Running Agents](https://www.anthropic.com/engineering/effective-harnesses-for-long-running-agents) — Anthropic Engineering
- [Effective Context Engineering for AI Agents](https://www.anthropic.com/engineering/effective-context-engineering-for-ai-agents) — Anthropic Engineering
- [Better-T-Stack](https://www.better-t-stack.dev) — Full-stack TypeScript scaffold engine
- [OpenSpec](https://openspec.dev) — Spec-driven development CLI
- User Story Mapping — Jeff Patton (walking skeleton, layered delivery)

## License

MIT
