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

## The Story Map

AEP organizes all work as a [Jeff Patton story map](https://www.jpattonassociates.com/user-story-mapping/). Read left-to-right for the user journey, top-to-bottom for enrichment. Every AEP term maps to a position on this structure:

```
                            ACTIVITY BACKBONE (extracted by /envision)
    ─────────────────────────────────────────────────────────────────────────────►
    "The user authenticates, then configures, then monitors, then reviews"

    ┌──────────────┐  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐
    │  Authenticate │  │  Configure   │  │   Monitor    │  │    Review    │
    │  (activity)   │  │  (activity)  │  │  (activity)  │  │  (activity)  │
    └──────┬───────┘  └──────┬───────┘  └──────┬───────┘  └──────┬───────┘
           │                 │                 │                 │
═══════════╪═════════════════╪═════════════════╪═════════════════╪══════════════
 Layer 0   │  WALKING SKELETON — thinnest end-to-end path       │
           │                 │                 │                 │
  Wave 1   │  ┌────────┐    │  ┌────────┐     │  ┌────────┐    │
           │  │ STORY  │    │  │ STORY  │     │  │ STORY  │    │
           │  │ db-    │    │  │ api-   │     │  │ web-   │    │
           │  │ setup  │    │  │ scaff  │     │  │ scaff  │    │
           │  │   S ◆  │    │  │   S    │     │  │   S    │    │
           │  └────────┘    │  └────────┘     │  └────────┘    │
           │                 │                 │                 │
  Wave 2   │  ┌────────┐    │  ┌────────┐     │                │  ┌────────┐
  (needs   │  │ STORY  │    │  │ STORY  │     │                │  │ STORY  │
   wave 1) │  │ auth-  │    │  │ config │     │                │  │ audit- │
           │  │ setup  │    │  │ basic  │     │                │  │ list   │
           │  │   M    │    │  │   S    │     │                │  │   M    │
           │  └────────┘    │  └────────┘     │                │  └────────┘
           │                 │                 │                 │
 ─ ─ ─ ─ ─│─ ─ LAYER GATE ─ "user can complete full journey" ─ │─ ─ ─ ─ ─ ─
           │                 │                 │                 │
═══════════╪═════════════════╪═════════════════╪═════════════════╪══════════════
 Layer 0.5 │  ALIGNMENT LAYER — human calibrates quality        │
           │                 │                 │                 │
  Wave 1   │  ┌────────┐    │                 │  ┌────────┐    │  ┌────────┐
  (visual- │  │ STORY  │    │                 │  │ STORY  │    │  │ STORY  │
   design) │  │ landing│    │                 │  │ dash-  │    │  │ auth-  │
           │  │ polish │    │                 │  │ board  │    │  │ pages  │
           │  │  M ✦   │    │                 │  │  M ✦   │    │  │  S ✦   │
           │  └────────┘    │                 │  └────────┘    │  └────────┘
           │                 │                 │                 │
           │  ✦ = calibration_type: visual-design               │
           │      dispatched with calibration/visual-design.yaml │
           │                 │                 │                 │
 ─ ─ ─ ─ ─│─ ─ RELEASE LINE ─ Layer 0 + 0.5 = first release ─ │─ ─ ─ ─ ─ ─
           │                 │                 │                 │
═══════════╪═════════════════╪═════════════════╪═════════════════╪══════════════
 Layer 1   │  CORE FEATURES — deeper capabilities               │
           │                 │                 │                 │
  Wave 1   │  ┌────────┐    │  ┌────────┐     │  ┌────────┐    │  ┌────────┐
           │  │ STORY  │    │  │ STORY  │     │  │ STORY  │    │  │ STORY  │
           │  │ oauth  │    │  │ guard- │     │  │ live-  │    │  │ audit- │
           │  │ provid │    │  │ rails  │     │  │ status │    │  │ detail │
           │  │   L    │    │  │   M    │     │  │   M    │    │  │   L    │
           │  └────────┘    │  └────────┘     │  └────────┘    │  └────────┘
           │                 │                 │                 │
  Wave 2   │                │  ┌────────┐     │  ┌────────┐    │
           │                │  │ STORY  │     │  │ STORY  │    │
           │                │  │ rule-  │     │  │ alert- │    │
           │                │  │ engine │     │  │ system │    │
           │                │  │   L    │     │  │   M    │    │
           │                │  └────────┘     │  └────────┘    │
           │                 │                 │                 │
 ─ ─ ─ ─ ─│─ ─ LAYER GATE ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ │─ ─ ─ ─ ─ ─
           │                 │                 │                 │
═══════════╪═════════════════╪═════════════════╪═════════════════╪══════════════
 Layer 1.5 │  ALIGNMENT LAYER — multiple calibration types      │
           │                 │                 │                 │
           │  ┌──────────────────────────────────────────────┐  │
           │  │ ✦ visual-design (extension — new patterns)   │  │
           │  │ ✦ copy-tone    (establishment — brand voice) │  │
           │  │ ✦ api-surface  (light — inline YAML update)  │  │
           │  └──────────────────────────────────────────────┘  │
           │                 │                 │                 │
 ─ ─ ─ ─ ─│─ ─ RELEASE LINE ─ Layer 1 + 1.5 = second release ─│─ ─ ─ ─ ─ ─
           │                 │                 │                 │
           ▼                 ▼                 ▼                 ▼
```

```
LEGEND

  STRUCTURE                           EXECUTION
  Activity    = column (user verb)    Wave      = parallel batch (← →)
  Layer       = row (enrichment)      Story     = atomic work unit (one PR)
  Layer Gate  = integration test      Dispatch  = pick + lock + launch

  ALIGNMENT                           SYMBOLS
  .5 Layer    = human checkpoint      ◆  critical path story
  Calibration = capture "right"       ✦  calibrated story
  Quality Dim = what to calibrate     S/M/L  complexity

  SKILLS                              READING ORDER
  /envision  → activities + layers    left → right  = user journey
  /map       → stories + waves        top → down    = enrichment
  /calibrate → alignment decisions    ═══           = layer boundary
  /dispatch  → scores + launches      ─ ─           = gate / release line
  /reflect   → feedback → right phase
```

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
 for whom, within               waves + slices"                     │
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
                    └─── .5 alignment layer ──► /calibrate → human aligns
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
         │                              init tracking, read tasks.md
/launch                                 implement each task linearly
  create git worktree                   (one git commit per task)
  on feat/<name> branch                 code review (+ evaluator loop)
  bootstrap agent             ◄────     create PR, handle review
  optional: spawn evaluator             merge (squash + delete branch)
         │                                     │
/wrap    ◄─────────────────────────────────────┘
  archive OpenSpec change
  update story status in YAML
  remove worktree + branch
  check layer gate
  suggest /reflect
```

**Why two sessions:** Design needs human judgment — you decide direction, scope, tradeoffs. Implementation is mechanical — the agent follows the spec, implements, tests, publishes. Separating them lets the agent work autonomously for hours while you do other things.

**Why git + worktree:** `git worktree add -b feat/<name>` gives each agent an isolated working tree on its own branch, sharing `.git/objects` so history isn't duplicated. Linear commits (one per `tasks.md` row) make the PR's commit list a readable table of contents. Squash-merge keeps `main`'s history clean. AEP previously used Jujutsu (jj); see [docs/decisions/migrate-from-jj-to-git.md](docs/decisions/migrate-from-jj-to-git.md) for why we switched.

### 3. Project Setup — the one-time foundation

Gets your machine and project ready. Run once.

```
/onboard                             /scaffold
    │                                    │
    ▼                                    ▼
Verify tools                         Scaffold monorepo
(bun, git, gh, claude,               (Better-T-Stack: frontend,
 openspec, tmux, cmux)                backend, database, auth,
                                      API layer, addons)
    │                                    │
    ▼                                    ▼
Install plugins                      Initialize OpenSpec
(superpowers, frontend-design,       (explore/propose/apply/archive
 mgrep; browser optional)            commands for spec-driven dev)
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
  "Polish" is now "Calibration" — covers visual design,
  UX flow, API surface, data model, copy/tone, scope,
  and performance quality dimensions.
  The product context evolves. The cycle continues.
```

### Human Alignment Layers

Agents build to spec, but specs are lossy compressions of human intent. After each implementation layer, optional `.5` alignment layers let the human recalibrate what "right" means across any quality dimension:

```
Layer 0 (walking skeleton)
  → /calibrate visual-design → human explores with design tools → capture
  → Layer 0.5 (alignment: implement with calibrated design context)
Layer 1 (core features)
  → /calibrate api-surface   → 30-min conversation → updates product-context.yaml
  → /calibrate copy-tone     → establish brand voice → calibration/copy-tone.yaml
  → Layer 1.5 (alignment: extend design system + apply voice)
```

The `/calibrate` skill supports 7 dimensions — **visual-design**, **ux-flow**, **api-surface**, **data-model**, **scope-direction**, **copy-tone**, **performance-quality** — split into two classes:

- **Heavy** (visual-design, ux-flow, copy-tone): external exploration, standalone YAML artifacts in `calibration/`
- **Light** (api-surface, data-model, scope-direction, performance-quality): 30-60 min conversation, updates `product-context.yaml` directly

Quality dimensions are declared during `/envision` and checked by `/reflect` after each layer.

### Institutional Memory

Workspace agents capture what they learn during builds — solutions discovered, errors encountered, missing docs — in `.dev-workflow/lessons.md`. When `/wrap` archives the workspace, substantive lessons are persisted to `lessons-learned/` at the repo root. `/launch` injects relevant prior lessons into bootstrap prompts, so the next agent building in the same module doesn't start from zero.

## Design Principles

These aren't rules we invented — they're patterns extracted from Anthropic's engineering research on long-running agent harnesses:

**Spec precision over implementation speed.** Time invested in unambiguous specs pays back exponentially across parallel agents. A 10-minute conversation in `/envision` saves hours of agent confusion.

**Walking skeleton first.** Build the thinnest end-to-end path (Layer 0) before going deep into any module. Validate the architecture at minimum cost. Going deep before proving the skeleton works is the most expensive mistake.

**Every harness component earns its place.** Sprint contracts, verification JSON, signal files, evaluator agents — each exists because of a specific failure mode observed in practice. As models improve, stress-test each component and remove what's no longer needed.

**Generator-evaluator separation.** Agents praise their own work even when it's mediocre. A separate evaluator agent, calibrated toward skepticism, catches problems the builder missed. This is the single most durable pattern from Anthropic's research.

## Getting Started

**Brand new to AEP?** Start with the [Orientation Guide](docs/orientation.md) for a 10-minute tour of the mental models, the 16 skills, and the four paths — then run `/onboard`.

**New to this plugin?**

```
/onboard
```

Installs prerequisites, verifies tools, configures recommended plugins, and walks you through the 5-minute mental-model orientation in Phase 0.

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

**Something feels off? Calibrate:**

```
/calibrate visual-design    → design brief → external tools → /calibrate capture
/calibrate api-surface      → conversation → updates product-context.yaml
/calibrate scope-direction  → conversation → updates product-context.yaml
```

Generate a dimension-specific brief, explore or discuss, capture decisions for agents to follow.

## All Skills

| Skill        | Plugin                       | Purpose                                                                          |
| ------------ | ---------------------------- | -------------------------------------------------------------------------------- |
| `/envision`  | product-context              | Opportunity brief + context document                                             |
| `/map`       | product-context              | System map + story graph + agent topology                                        |
| `/dispatch`  | product-context              | Pick next story + create OpenSpec change                                         |
| `/calibrate` | product-context              | Human alignment checkpoint for any quality dimension                             |
| `/reflect`   | product-context              | Classify feedback + update context                                               |
| `/onboard`   | project-setup                | Verify tools + install plugins                                                   |
| `/scaffold`  | project-setup                | Scaffold monorepo + initialize OpenSpec                                          |
| `/design`    | agentic-development-workflow | Explore + propose + review a feature                                             |
| `/launch`    | agentic-development-workflow | Spawn workspace (claude/codex; tmux/cmux/subagent/workflow) + optional evaluator |
| `/build`     | agentic-development-workflow | Implement → test → PR → merge                                                    |
| `/wrap`      | agentic-development-workflow | Archive + cleanup + suggest reflect                                              |
| `/git-ref`   | agentic-development-workflow | AEP git + worktree conventions (on-demand)                                       |
| `/gen-eval`  | patterns                     | Generator/evaluator separation for honest validation                             |
| `/executor`  | patterns                     | Host-agnostic backend for spawning/steering workspace agents                     |
| `/autopilot` | patterns                     | Autonomous dispatch-launch-monitor-wrap loop via `/loop`                         |

## Documentation

- [Orientation Guide](docs/orientation.md) — 10-minute first-hour tour of mental models, skills, and the four paths (start here if you're new)
- [Glossary — Ubiquitous Language](docs/glossary.md) — precise definitions for every AEP term
- [Skills Quick Reference](docs/skills-quick-reference.md) — when to use which skill, decision trees, common sequences
- [Autonomous Loop](docs/autonomous-loop.md) — how `/autopilot` orchestrates the full cycle
- [Generator/Evaluator Data Flow](docs/gen-eval-data-flow.md) — the three tracking systems and signal protocol
- [Release Line Adjustments](docs/release-line-adjustments.md) — when and how to re-slice layers
- [Design Calibration Workflow](docs/decisions/design-calibration-workflow.md) — the original visual-design `/calibrate` skill
- [Generalized Calibration Workflow](docs/decisions/generalized-calibration-workflow.md) — multi-dimension `/calibrate` and `.5` alignment layers
- [v2 Improvement Roadmap](docs/aep-v2-improvement-guideline.md) — capability maps, technical specs, dispatch enhancements

## Version History

Human-readable release notes for each version are in [CHANGELOG.md](CHANGELOG.md). The plugin version is the `metadata.version` field in `.claude-plugin/marketplace.json` and follows [Semantic Versioning](https://semver.org/).

## Installing Skills

AEP skills follow the open [Agent Skills](https://agentskills.io/) format, so any project — under
Claude Code, Codex, Cursor, OpenCode, and [70+ other agents](https://github.com/vercel-labs/skills#supported-agents) —
can install them with the [`skills`](https://github.com/vercel-labs/skills) CLI. No clone, no copied scripts.

### Quick start

> **Always name your agent with `-a`.** The CLI's auto-detect (and `--all` / `--agent '*'`)
> installs into the cross-agent `.agents/skills/` directory — which **Claude Code does not read**
> (Claude Code only loads `.claude/skills/`). Passing `-a claude-code` is what makes the install
> land where Claude Code will find it.

```bash
# Claude Code — installs all AEP skills into ./.claude/skills/ at project level
npx skills add memorysaver/agentic-engineering-patterns -a claude-code --skill '*'
```

Skills install with the `aep-` prefix (e.g. `aep-map`, `aep-build`) at **project level** — committed
with your repo and shared with your team — and each skill is **self-contained**, so its shared
templates and references travel with it.

### Multiple runtimes (run once per agent)

`-a` takes a single agent: a repeated `-a a -a b` keeps only the last, and a comma list
(`-a a,b`) installs nothing. To cover several runtimes, run the command once per agent:

```bash
npx skills add memorysaver/agentic-engineering-patterns -a claude-code --skill '*'   # → ./.claude/skills/
npx skills add memorysaver/agentic-engineering-patterns -a codex      --skill '*'    # → ./.agents/skills/
```

### Pinning a version

Append `@<git-ref>` (a release tag, branch, or commit) to lock what you install:

```bash
npx skills add memorysaver/agentic-engineering-patterns@v1.2.0 -a claude-code --skill '*'
```

One caveat worth knowing: `skills-lock.json` records each skill's **content hash**, not the git
tag. The lockfile alone therefore does **not** durably pin the version — `npx skills experimental_install`
restores from the source repo's **default branch**, which only matches the lock while that branch
still equals the locked content. To truly freeze a release, **commit the installed skill files**
(under `.claude/skills/` and/or `.agents/skills/`) together with `skills-lock.json`. The committed
bytes become the pin: teammates, CI, and Codex need no install step, and nothing drifts when
upstream moves on. Upgrade deliberately by re-running `add@<newtag>` in its own PR.

### Keep your formatter off the skills

Skill files are Markdown and JSON. If your repo auto-formats those on commit (Prettier, oxfmt,
Biome, dprint, a lefthook / husky hook…), it will rewrite the installed skills and **break the
content hashes in `skills-lock.json`**. Exclude the skill paths from your formatter —
`.claude/skills/**`, `.agents/skills/**`, and `skills-lock.json` — and make the install commit
with `--no-verify` so the pinned bytes stay byte-for-byte intact.

### Agent prompt

Prefer to delegate the install? Paste this to your coding agent — it covers the install, the
formatter fix, and wiring AEP into your `AGENTS.md` / `CLAUDE.md`:

```text
Install the Agentic Engineering Patterns (AEP) skills into this project, pinned to v1.2.0.

1. Install for each agent this repo uses (run the command once per agent):
     npx skills add memorysaver/agentic-engineering-patterns@v1.2.0 -a claude-code --skill '*' -y
     npx skills add memorysaver/agentic-engineering-patterns@v1.2.0 -a codex        --skill '*' -y
   This writes the skills under .claude/skills/ and/or .agents/skills/ plus a skills-lock.json manifest.

2. Commit the installed skill files together with skills-lock.json. The lockfile pins content
   hashes, not the git tag, so the committed files are what durably lock v1.2.0 — after that,
   teammates and CI need no install step.

3. If this repo auto-formats Markdown/JSON on commit (Prettier, oxfmt, Biome, dprint, a
   lefthook/husky hook): exclude .claude/skills/**, .agents/skills/**, and skills-lock.json from
   the formatter, then make the install commit with --no-verify. Otherwise reformatting rewrites
   the skill files and breaks the lockfile hashes.

4. Add a short section to AGENTS.md (and/or CLAUDE.md) so the workflow is discoverable:
     ## AEP Workflow
     This project uses the Agentic Engineering Patterns (AEP) skills — a spec-driven, multi-agent
     feature lifecycle in .claude/skills/ and/or .agents/skills/, pinned via skills-lock.json.
     The skills are self-describing; start with `aep-onboard`. Upgrade by re-running
     `npx skills add memorysaver/agentic-engineering-patterns@<newtag>` once per agent.

5. Verify with `npx skills list`. Restore from the lockfile later with `npx skills experimental_install`.
```

### Common commands

```bash
# List what's available before installing
npx skills add memorysaver/agentic-engineering-patterns --list

# Install specific skills (repeat --skill; use the aep- name)
npx skills add memorysaver/agentic-engineering-patterns -a claude-code --skill aep-map --skill aep-build

# Install globally (user-level, ~/.claude/skills) instead of project level
npx skills add memorysaver/agentic-engineering-patterns -a claude-code -g --skill '*'

# Copy files instead of symlinking (when symlinks aren't supported)
npx skills add memorysaver/agentic-engineering-patterns -a claude-code --skill '*' --copy

# Update / remove / list installed
npx skills update
npx skills remove aep-map
npx skills list

# Restore installed skills from skills-lock.json (e.g. after a fresh clone)
npx skills experimental_install
```

### Skill groups → skill names

The `skills` CLI selects by skill name (there's no "group" flag). The groups map to these `--skill` names:

| Group                                       | `--skill` names                                                                           |
| ------------------------------------------- | ----------------------------------------------------------------------------------------- |
| **Workflow** (agentic-development-workflow) | `aep-design`, `aep-launch`, `aep-build`, `aep-wrap`, `aep-git-ref`                        |
| **Product** (product-context)               | `aep-envision`, `aep-map`, `aep-dispatch`, `aep-validate`, `aep-calibrate`, `aep-reflect` |
| **Setup** (project-setup)                   | `aep-onboard`, `aep-scaffold`, `aep-testing-guide`                                        |
| **Patterns** (patterns)                     | `aep-gen-eval`, `aep-executor`, `aep-autopilot`, `aep-workflow-feedback`                  |

### Maintainer (legacy) workflow

The bespoke scripts remain for the maintainer's own multi-project workflow — they predate the
`skills` CLI and cover a few things it doesn't: **group-as-a-unit** installs, `--dry-run`,
exact orphan `--prune`, and **batch push to many registered projects** at once.

```bash
# Pull into one project (group filter, dry-run, prune all supported)
AEP_REPO=~/agentic-engineering-patterns bash scripts/sync.sh workflow --dry-run

# Push to every project registered in .aep/config.yaml (gitignored, machine-local)
bash scripts/sync-downstream.sh --init   # one-time: create the config
bash scripts/sync-downstream.sh          # push to all
bash scripts/sync-downstream.sh 91app    # push to one (name match)
```

For everyone else, `npx skills add` above is the supported path.

> **Releasing:** bumping `metadata.version` in `.claude-plugin/marketplace.json` must come with a matching [CHANGELOG.md](CHANGELOG.md) entry in the same PR (move the `[Unreleased]` notes under the new `[X.Y.Z] - DATE` heading), and a `vX.Y.Z` git tag on merge to `main`.

### Contributing skills (shared resources)

Skills are authored under `skills/<group>/<name>/SKILL.md` and must be **self-contained** so each
installs cleanly on its own. Resources shared across the product-context skills live once in
`skills/product-context/_shared/{references,templates}/`. A build step materializes them into each
skill that references them (those copies are marked with a `.aep-generated` file — don't edit them
by hand):

```bash
bun run skills:build    # edit _shared/, then regenerate the per-skill copies
bun run skills:check    # verify the copies are in sync (also runs in CI + pre-commit)
```

## Inspired By

- [Harness Design for Long-Running Application Development](https://www.anthropic.com/engineering/harness-design-long-running-apps) — Anthropic Engineering
- [Effective Harnesses for Long-Running Agents](https://www.anthropic.com/engineering/effective-harnesses-for-long-running-agents) — Anthropic Engineering
- [Effective Context Engineering for AI Agents](https://www.anthropic.com/engineering/effective-context-engineering-for-ai-agents) — Anthropic Engineering
- [Better-T-Stack](https://www.better-t-stack.dev) — Full-stack TypeScript scaffold engine
- [OpenSpec](https://openspec.dev) — Spec-driven development CLI
- User Story Mapping — Jeff Patton (walking skeleton, layered delivery)

## License

MIT
