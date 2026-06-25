# AEP Skills — Quick Reference

A cheat sheet for all 17 AEP skills. For precise term definitions, see the [Glossary](glossary.md). For a guided first-hour introduction to the mental models behind these skills, see the [Orientation Guide](orientation.md).

---

## Workflow at a Glance

```
CONTROL PLANE (human + AI)              EXECUTION PLANE (agents build)
━━━━━━━━━━━━━━━━━━━━━━━━━━              ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
/aep-envision → /aep-map → /aep-scaffold      →     /aep-dispatch → /aep-design → /aep-launch
                                                                 ↓
/aep-reflect  ← ← ← ← ← ← ← ← ←   ←     /aep-wrap    ← ← ← ←    /aep-build
```

---

## Skills by Phase

### Product Discovery (Control Plane)

| Skill           | When to use                                              | Input                                                          | Output                                                           | Session |
| --------------- | -------------------------------------------------------- | -------------------------------------------------------------- | ---------------------------------------------------------------- | ------- |
| `/aep-envision` | New product idea, revisit direction                      | Product idea (vague or refined)                                | `product-context.yaml` with `opportunity` + `product`            | Main    |
| `/aep-map`      | After `/aep-envision` — decompose into stories           | `product-context.yaml` with product section                    | Architecture, stories, topology, layer gates, cost added to YAML | Main    |
| `/aep-validate` | Check quality of any artifact before proceeding          | Any artifact (product context, design, code, document)         | Scoring dimensions + findings                                    | Main    |
| `/aep-dispatch` | Pick what to build next                                  | `product-context.yaml` with stories                            | OpenSpec change + story status updated + handoff                 | Main    |
| `/aep-reflect`  | After shipping — close the feedback loop                 | Observations from user testing, errors, cost data              | Classified feedback + updated YAML                               | Main    |
| `/aep-watch`    | Continuously ingest errors/telemetry → auto-file stories | Telemetry/bug-tracker/error sources (`topology.routing.watch`) | New bug/refinement stories in YAML → dispatch                    | Main    |

### Feature Execution (Execution Plane)

| Skill         | When to use                           | Input                                      | Output                                                                                                   | Session   |
| ------------- | ------------------------------------- | ------------------------------------------ | -------------------------------------------------------------------------------------------------------- | --------- |
| `/aep-design` | Refine an ambiguous story spec        | Dispatched story (OpenSpec change on main) | Refined OpenSpec artifacts (proposal, design, specs, tasks)                                              | Main      |
| `/aep-launch` | Spawn autonomous agent for a story    | Well-specified story with OpenSpec change  | git worktree + native worker (bg subagent/codex subagent; tmux when pinned) + running `/aep-build` agent | Main      |
| `/aep-build`  | Agent implements a feature end-to-end | OpenSpec artifacts on disk                 | Merged PR                                                                                                | Workspace |
| `/aep-wrap`   | Archive after PR merges               | Completed workspace with merged PR         | Archived OpenSpec, updated story status, cleaned workspace                                               | Main      |

### Patterns (Reusable)

| Skill                    | When to use                                                                 | Input                                                              | Output                                                              | Session |
| ------------------------ | --------------------------------------------------------------------------- | ------------------------------------------------------------------ | ------------------------------------------------------------------- | ------- |
| `/aep-gen-eval`          | Run honest evaluation on any artifact                                       | Artifact to evaluate + dimension selection                         | Scoring results + findings                                          | Both    |
| `/aep-autopilot`         | Hands-free autonomous orchestration                                         | `product-context.yaml` with ready stories                          | Continuous dispatch → build → wrap cycle                            | Main    |
| `/aep-workflow`          | Author a custom multi-agent harness for a large/uncertain/verify-heavy task | A task too big, drift-prone, or verification-heavy for one context | A dynamic workflow (fan-out, verify, tournament, loop) + its result | Main    |
| `/aep-workflow-feedback` | Capture or review process learnings                                         | Lessons from builds, downstream feedback files                     | Classified feedback in standardized format                          | Main    |

### Project Setup (Run Once)

| Skill                        | When to use                                     | Input             | Output                                                 | Session |
| ---------------------------- | ----------------------------------------------- | ----------------- | ------------------------------------------------------ | ------- |
| `/aep-onboard`               | First-time environment setup                    | (none)            | Tools verified, plugins installed                      | Main    |
| `/aep-scaffold`              | Create new project or onboard/converge existing | Stack preferences | Monorepo + OpenSpec + workspace hooks + BDD e2e skill  | Main    |
| `/aep-e2e-skill-scaffolding` | Generate/upgrade the e2e-test skill             | Project stack     | Canonical BDD layer-gate e2e-test skill (cross-tool)   | Main    |
| `/aep-git-ref`               | AEP git + worktree conventions                  | (none)            | Worktree lifecycle, branch naming, recovery procedures | Main    |

---

## Decision Tree

```
"I have an idea"                    → /aep-envision
"Break it down into stories"        → /aep-map
"Is the design good enough?"        → /aep-validate
"What should I build next?"         → /aep-dispatch
"I need to refine this spec"        → /aep-design
"Ready to start coding"             → /aep-launch
"Feature is done, PR merged"        → /aep-wrap
"What did we learn?"                → /aep-reflect
"Auto-file work from telemetry"     → /aep-watch
"Capture process learnings"         → /aep-workflow-feedback
"Pull learnings from downstreams"   → /aep-workflow-feedback
"I want hands-free mode"            → /aep-autopilot
"How do AEP worktrees work?"        → /aep-git-ref
"Set up my environment"             → /aep-onboard
"Create a new project"              → /aep-scaffold
"Set up testing infrastructure"     → /aep-e2e-skill-scaffolding
"Evaluate this honestly"            → /aep-gen-eval
"Write a custom harness / workflow" → /aep-workflow
"This is too big for one context"   → /aep-workflow
```

---

## Common Sequences

### First time setup

```
/aep-onboard → /aep-scaffold
```

### New product (idea → ready to build)

```
/aep-envision → /aep-map → /aep-validate (optional) → /aep-scaffold
```

### Feature cycle (one story)

```
/aep-dispatch → /aep-design (if ambiguous) → /aep-launch → /aep-build (autonomous) → /aep-wrap
```

### Feature cycle (batch)

```
/aep-dispatch --batch wave → /aep-launch (×N) → /aep-build (×N parallel) → /aep-wrap (×N)
```

### After shipping

```
/aep-reflect → new stories feed back into /aep-dispatch
```

### After reflecting on process

```
/aep-reflect → /aep-workflow-feedback capture → docs updated
```

### Pull upstream improvements

```
/aep-workflow-feedback review → approve items → tag release → re-pin downstreams
```

### Full autonomous mode

```
/aep-autopilot
```

Autopilot runs dispatch → launch → monitor → review → wrap → dispatch in a loop, pausing only for design escalations or layer gate failures.

---

## Skill Groups (Plugin Names)

Installed into projects via the [`skills` CLI](https://github.com/vercel-labs/skills) (once per agent:
`-a claude-code`, then `-a codex`), prefixed with `aep-`. See the README "Installing Skills" section for
the full install / pin / upgrade flow.

| Group                          | Skills                                                              |
| ------------------------------ | ------------------------------------------------------------------- |
| `product-context`              | envision, map, model, dispatch, validate, calibrate, reflect, watch |
| `agentic-development-workflow` | design, launch, build, wrap, git-ref                                |
| `patterns`                     | gen-eval, executor, autopilot, workflow, workflow-feedback          |
| `project-setup`                | onboard, scaffold, e2e-skill-scaffolding                            |

---

## v2 Enhancements

v2 improvements enhance existing skills — no new commands. See the [v2 Improvement Roadmap](aep-v2-improvement-guideline.md) for details on: capability maps for multi-journey products, enhanced dispatch formula, outcome contracts, and VCS abstraction.
