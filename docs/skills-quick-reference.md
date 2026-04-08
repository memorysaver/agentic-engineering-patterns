# AEP Skills — Quick Reference

A cheat sheet for all 16 AEP skills. For precise term definitions, see the [Glossary](glossary.md).

---

## Workflow at a Glance

```
CONTROL PLANE (human + AI)              EXECUTION PLANE (agents build)
━━━━━━━━━━━━━━━━━━━━━━━━━━              ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
/envision → /map → /scaffold      →     /dispatch → /design → /launch
                                                                 ↓
/reflect  ← ← ← ← ← ← ← ← ←   ←     /wrap    ← ← ← ←    /build
```

---

## Skills by Phase

### Product Discovery (Control Plane)

| Skill       | When to use                                     | Input                                                  | Output                                                           | Session |
| ----------- | ----------------------------------------------- | ------------------------------------------------------ | ---------------------------------------------------------------- | ------- |
| `/envision` | New product idea, revisit direction             | Product idea (vague or refined)                        | `product-context.yaml` with `opportunity` + `product`            | Main    |
| `/map`      | After `/envision` — decompose into stories      | `product-context.yaml` with product section            | Architecture, stories, topology, layer gates, cost added to YAML | Main    |
| `/validate` | Check quality of any artifact before proceeding | Any artifact (product context, design, code, document) | Scoring dimensions + findings                                    | Main    |
| `/dispatch` | Pick what to build next                         | `product-context.yaml` with stories                    | OpenSpec change + story status updated + handoff                 | Main    |
| `/reflect`  | After shipping — close the feedback loop        | Observations from user testing, errors, cost data      | Classified feedback + updated YAML                               | Main    |

### Feature Execution (Execution Plane)

| Skill     | When to use                           | Input                                      | Output                                                      | Session   |
| --------- | ------------------------------------- | ------------------------------------------ | ----------------------------------------------------------- | --------- |
| `/design` | Refine an ambiguous story spec        | Dispatched story (OpenSpec change on main) | Refined OpenSpec artifacts (proposal, design, specs, tasks) | Main      |
| `/launch` | Spawn autonomous agent for a story    | Well-specified story with OpenSpec change  | jj workspace + tmux session + running `/build` agent        | Main      |
| `/build`  | Agent implements a feature end-to-end | OpenSpec artifacts on disk                 | Merged PR                                                   | Workspace |
| `/wrap`   | Archive after PR merges               | Completed workspace with merged PR         | Archived OpenSpec, updated story status, cleaned workspace  | Main      |

### Patterns (Reusable)

| Skill                | When to use                           | Input                                          | Output                                     | Session |
| -------------------- | ------------------------------------- | ---------------------------------------------- | ------------------------------------------ | ------- |
| `/gen-eval`          | Run honest evaluation on any artifact | Artifact to evaluate + dimension selection     | Scoring results + findings                 | Both    |
| `/autopilot`         | Hands-free autonomous orchestration   | `product-context.yaml` with ready stories      | Continuous dispatch → build → wrap cycle   | Main    |
| `/workflow-feedback` | Capture or review process learnings   | Lessons from builds, downstream feedback files | Classified feedback in standardized format | Main    |

### Project Setup (Run Once)

| Skill            | When to use                                | Input             | Output                                               | Session |
| ---------------- | ------------------------------------------ | ----------------- | ---------------------------------------------------- | ------- |
| `/onboard`       | First-time environment setup               | (none)            | Tools verified, plugins installed, jj initialized    | Main    |
| `/scaffold`      | Create new project or onboard existing one | Stack preferences | Monorepo + OpenSpec + workspace hooks + E2E skeleton | Main    |
| `/testing-guide` | Set up quality infrastructure              | Project context   | Workspace setup hook + E2E test skill                | Main    |
| `/jj-ref`        | Need help with jj commands                 | (none)            | Command reference + concept mapping                  | Main    |

---

## Decision Tree

```
"I have an idea"                    → /envision
"Break it down into stories"        → /map
"Is the design good enough?"        → /validate
"What should I build next?"         → /dispatch
"I need to refine this spec"        → /design
"Ready to start coding"             → /launch
"Feature is done, PR merged"        → /wrap
"What did we learn?"                → /reflect
"Capture process learnings"         → /workflow-feedback
"Pull learnings from downstreams"   → /workflow-feedback
"I want hands-free mode"            → /autopilot
"How do I use jj?"                  → /jj-ref
"Set up my environment"             → /onboard
"Create a new project"              → /scaffold
"Set up testing infrastructure"     → /testing-guide
"Evaluate this honestly"            → /gen-eval
```

---

## Common Sequences

### First time setup

```
/onboard → /scaffold
```

### New product (idea → ready to build)

```
/envision → /map → /validate (optional) → /scaffold
```

### Feature cycle (one story)

```
/dispatch → /design (if ambiguous) → /launch → /build (autonomous) → /wrap
```

### Feature cycle (batch)

```
/dispatch --batch wave → /launch (×N) → /build (×N parallel) → /wrap (×N)
```

### After shipping

```
/reflect → new stories feed back into /dispatch
```

### After reflecting on process

```
/reflect → /workflow-feedback capture → docs updated
```

### Pull upstream improvements

```
/workflow-feedback review → approve items → sync-downstream.sh
```

### Full autonomous mode

```
/autopilot
```

Autopilot runs dispatch → launch → monitor → review → wrap → dispatch in a loop, pausing only for design escalations or layer gate failures.

---

## Skill Groups (Plugin Names)

When synced to downstream projects, skills are prefixed with `aep-`:

| Group                          | Skills                                     | Sync command       |
| ------------------------------ | ------------------------------------------ | ------------------ |
| `product-context`              | envision, map, dispatch, validate, reflect | `sync.sh product`  |
| `agentic-development-workflow` | design, launch, build, wrap, jj-ref        | `sync.sh workflow` |
| `patterns`                     | gen-eval, autopilot, workflow-feedback     | `sync.sh patterns` |
| `project-setup`                | onboard, scaffold, testing-guide           | `sync.sh setup`    |

---

## v2 Enhancements

v2 improvements enhance existing skills — no new commands. See the [v2 Improvement Roadmap](aep-v2-improvement-guideline.md) for details on: capability maps for multi-journey products, enhanced dispatch formula, outcome contracts, and VCS abstraction.
