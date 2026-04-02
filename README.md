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
│        ▲                │              │         │              │
│        └────────────────┼──────────────┘         │              │
│                         │  feedback loop         │              │
│                         ▼                        │              │
│                  ┌────────────┐                   │              │
│                  │ /dispatch  │  picks stories    │              │
│                  │            │  from the map,    │              │
│                  │ what to    │  creates OpenSpec │              │
│                  │ work on    │  changes          │              │
│                  │ next       │                   │              │
│                  └─────┬──────┘                   │              │
│                        │                         │              │
└────────────────────────┼─────────────────────────┼──────────────┘
                         │                         │
          story specs    │    status + cost flow up │
          flow down      │                         │
                         ▼                         │
┌──────────────────────────────────────────────────┼──────────────┐
│                                                  │              │
│   EXECUTION PLANE  (agents build it)             │              │
│                                                                 │
│   Agents receive precise specs, work in isolation,              │
│   produce PRs. They don't decide what to build.                 │
│                                                                 │
│   ┌──────────┐    ┌──────────┐    ┌──────────┐    ┌─────────┐  │
│   │ /design  │───►│ /launch  │───►│  /build  │───►│  /wrap  │  │
│   │          │    │          │    │          │    │         │  │
│   │ refine   │    │ spawn    │    │ implement│    │ archive │  │
│   │ the spec │    │ agent    │    │ + test   │    │ + update│  │
│   │          │    │          │    │ + PR     │    │ status  │  │
│   └──────────┘    └──────────┘    └──────────┘    └─────────┘  │
│                                                                 │
│   (repeat per story — multiple stories run in parallel)         │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

**Agents don't talk to each other.** They communicate through structured artifacts — context documents, story specs, interface contracts, signal files. The harness coordinates everything. This is a production system design, not a chatroom-style agent swarm.

## The Plugins

Each plugin implements one layer of the mental model.

### 1. Product Context — the persistent map

Captures the "what and why" of the entire product in a single `product-context.yaml` — committed to git, versioned, and machine-parseable.

```
/envision                        /map                            /reflect
    │                               │                               │
    ▼                               ▼                               ▼
Opportunity Brief               System Map                      Classify feedback:
"should we build this?"         "modules + interfaces"          bug → fix story
    │                               │                           refinement → next layer
    ▼                               ▼                           discovery → update map
Context Document                Story Graph                     shift → re-envision
"what exactly to build,         "layered work items,                │
 for whom, within               execution slices"                   │
 what constraints"                  │                               │
    │                               ▼                               │
    │                           Agent Topology                      │
    │                           "roles + contracts"                 │
    │                               │                               │
    └───────────────┬───────────────┘                               │
                    │                                               │
                    ▼                                               │
               /dispatch                                            │
               "pick next story,          ◄─────────────────────────┘
                create OpenSpec change,     (new stories feed back
                route to /design"            into the dispatch queue)
                    │
                    ├─── integer layer ──► /design → /launch → /build → /wrap
                    │
                    └─── .5 polish layer ──► /calibrate → human designs
                                               → /calibrate capture
                                               → /dispatch → /launch → /build → /wrap
```

All sections live in one `product-context.yaml` file — opportunity, product, architecture, stories (with state machine), topology, layer gates, cost tracking, and a semantic changelog.

**Why this exists:** Without a product-level map, each feature is designed in isolation. Agents build incompatible pieces. Module boundaries are implicit. The YAML makes the whole system visible, machine-readable, and git-versioned before any code is written.

### 2. Feature Lifecycle — the execution cycle

Takes one story from the map and turns it into a merged PR. `/dispatch` picks the story; the two-session model executes it:

```
MAIN SESSION (you + AI)                WORKSPACE SESSION (agent alone)
━━━━━━━━━━━━━━━━━━━━━━                ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

/dispatch
  pick story from YAML
  create OpenSpec change
         │
/design
  refine the spec
  (or skip if well-specified) ────►   /build
         │                              init tracking + jj change stack
/launch                                 implement each task
  create jj workspace                   code review (+ evaluator loop)
  bootstrap agent             ◄────     create PR, handle review
  optional: spawn evaluator             merge
         │                                     │
/wrap    ◄─────────────────────────────────────┘
  archive OpenSpec change
  update story status in YAML
  check layer gate
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
     │  │  │  ┌─── │ ◄── polish                        │
     │  │  │  │     │      (.5 layer → /calibrate)      │
     │  │  │  │     │                                  │
     │  │  │  │  ┌─ │ ◄── bug                           │
     │  │  │  │  │  │      (fix story, back to /design) │
     │  │  │  │  │  │                                  │
     │  │  │  │  │  │ ◄── process                       │
     │  │  │  │  │  │      (workflow improvement)       │
     │  │  │  │  │  │                                  │
     │  │  │  │  │  └──────────────────────────────────┘
     │  │  │  │  │           /reflect
     ▼  ▼  ▼  ▼  ▼
  Each feedback type routes to the right phase.
  The product context evolves. The cycle continues.
```

### UI Polish Layers

Agents build **functional** UI but cannot judge **visual design quality**. After each implementation layer, an optional `.5` polish layer addresses this structural gap:

```
Layer 0 (walking skeleton)
  → Layer 0.5 (UI polish: /calibrate → design brief → human explores → capture)
Layer 1 (core features)
  → Layer 1.5 (UI polish: extend design system for new patterns)
```

The `/calibrate` skill generates a design brief from `product-context.yaml`, the human explores with vibe design tools (Stitch, Pencil.dev), then `/calibrate capture` records decisions into `design-context.yaml` — the authoritative visual reference that agents follow when implementing `.5` layer stories.

### Institutional Memory

Workspace agents capture what they learn during builds — solutions discovered, errors encountered, missing docs — in `.dev-workflow/lessons.md`. When `/wrap` archives the workspace, substantive lessons are persisted to `lessons-learned/` at the repo root. `/launch` injects relevant prior lessons into bootstrap prompts, so the next agent building in the same module doesn't start from zero.

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

**Want hands-free autonomous mode?**

```
/autopilot
```

One command. Autopilot dispatches, launches, monitors, reviews, merges, and wraps — pausing only when human design input is needed.

**Shipped something? Close the loop:**

```
/reflect
```

Classify feedback, update the product context, plan the next iteration.

**UI looks generic? Calibrate the design:**

```
/calibrate  →  human explores  →  /calibrate capture
```

Generate a design brief, explore with vibe design tools, capture decisions into `design-context.yaml`.

## All Skills

| Skill        | Plugin                       | Purpose                                                  |
| ------------ | ---------------------------- | -------------------------------------------------------- |
| `/envision`  | product-context              | Opportunity brief + context document                     |
| `/map`       | product-context              | System map + story graph + agent topology                |
| `/dispatch`  | product-context              | Pick next story + create OpenSpec change                 |
| `/calibrate` | product-context              | Design brief + capture decisions for `.5` polish layers  |
| `/reflect`   | product-context              | Classify feedback + update context                       |
| `/onboard`   | project-setup                | Verify tools + install plugins                           |
| `/scaffold`  | project-setup                | Scaffold monorepo + initialize OpenSpec                  |
| `/design`    | agentic-development-workflow | Explore + propose + review a feature                     |
| `/launch`    | agentic-development-workflow | Spawn workspace + optional evaluator                     |
| `/build`     | agentic-development-workflow | Implement → test → PR → merge                            |
| `/wrap`      | agentic-development-workflow | Archive + cleanup + suggest reflect                      |
| `/jj-ref`    | agentic-development-workflow | jj command reference (on-demand)                         |
| `/gen-eval`  | patterns                     | Generator/evaluator separation for honest validation     |
| `/autopilot` | patterns                     | Autonomous dispatch-launch-monitor-wrap loop via `/loop` |

## Documentation

- [Glossary — Ubiquitous Language](docs/glossary.md) — precise definitions for every AEP term
- [Skills Quick Reference](docs/skills-quick-reference.md) — when to use which skill, decision trees, common sequences
- [Autonomous Loop](docs/autonomous-loop.md) — how `/autopilot` orchestrates the full cycle
- [Generator/Evaluator Data Flow](docs/gen-eval-data-flow.md) — the three tracking systems and signal protocol
- [Release Line Adjustments](docs/release-line-adjustments.md) — when and how to re-slice layers
- [Design Calibration Workflow](docs/design-calibration-workflow.md) — the `/calibrate` skill and `.5` polish layer pattern
- [AEP v2 Lesson Learning](docs/aep-v2-lesson-learning.md) — structural improvements from Layer 0 post-mortem

## Syncing Skills to Your Project

A sync script is included to copy AEP skills into any project's `.claude/skills/` directory with the `aep-` prefix.

### Setup

1. Copy `scripts/sync.sh` to your project's `scripts/` directory
2. Set `AEP_REPO` to point to your local clone of this repo

### Usage

```bash
# Sync all skills
AEP_REPO=~/agentic-engineering-patterns bash scripts/sync.sh

# Preview changes without modifying files
bash scripts/sync.sh --dry-run

# Sync only one group (workflow, product, setup, patterns)
bash scripts/sync.sh workflow

# Override target directory
TARGET_DIR=./my-skills bash scripts/sync.sh
```

The script flattens the nested skill directories and prefixes each with `aep-` (e.g., `skills/product-context/envision/` becomes `.claude/skills/aep-envision/`). Run it whenever you want to pull the latest skill versions.

### Push Mode (sync-downstream)

Push skills from the AEP repo to all registered downstream projects at once.

```bash
# One-time setup: create the config
bash scripts/sync-downstream.sh --init

# Edit .aep/config.yaml with your project paths
# Then push to all projects:
bash scripts/sync-downstream.sh

# Preview changes:
bash scripts/sync-downstream.sh --dry-run

# Push to one project (name match):
bash scripts/sync-downstream.sh 91app
```

The config file (`.aep/config.yaml`) is gitignored — paths are machine-local. Each entry specifies the project path and optionally which skill groups to sync.

## Inspired By

- [Harness Design for Long-Running Application Development](https://www.anthropic.com/engineering/harness-design-long-running-apps) — Anthropic Engineering
- [Effective Harnesses for Long-Running Agents](https://www.anthropic.com/engineering/effective-harnesses-for-long-running-agents) — Anthropic Engineering
- [Effective Context Engineering for AI Agents](https://www.anthropic.com/engineering/effective-context-engineering-for-ai-agents) — Anthropic Engineering
- [Better-T-Stack](https://www.better-t-stack.dev) — Full-stack TypeScript scaffold engine
- [OpenSpec](https://openspec.dev) — Spec-driven development CLI
- User Story Mapping — Jeff Patton (walking skeleton, layered delivery)

## License

MIT
