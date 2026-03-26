# Product Context

A persistent product-level planning layer that captures the "what and why" of the entire product. Used for both initial product planning and ongoing iteration.

Three skills, one feedback loop:

```
/envision → /map → [ feature cycles ] → /reflect → loop
```

| Skill | What it does | When to use |
|-------|-------------|-------------|
| [/envision](envision/SKILL.md) | Opportunity brief + context document | Starting a new product, revisiting direction |
| [/map](map/SKILL.md) | System map + story graph + agent topology | Decomposing a product into executable work |
| [/reflect](reflect/SKILL.md) | Feedback classification + context update | After shipping, after user testing |

## How It Connects to the Feature Workflow

The product context layer sits above the feature workflow. It produces the "map" that individual features navigate.

```
                    ┌──────────────────────────────┐
                    │   Product Context Layer       │
                    │   (persistent, evolves)       │
                    │                               │
                    │   /envision → /map            │
                    │        ↑           │          │
                    │   /reflect         │ stories  │
                    │        ↑           ↓          │
                    └────────┼───────────┼──────────┘
                             │           │
                    ┌────────┼───────────┼──────────┐
                    │   Feature Workflow  │          │
                    │   (per-feature)     │          │
                    │                     ↓          │
                    │   /design → /launch → /build  │
                    │        → /wrap ─────────────→  │
                    └──────────────────────────────┘
```

- `/design` reads the Context Document and System Map for richer context
- `/wrap` suggests running `/reflect` after archiving
- `/reflect` routes feedback back to `/envision` or `/map` when needed

## Artifacts

Product context artifacts live in `product-context/` at the project root (committed to git, versioned):

```
product-context/
├── opportunity-brief.md    ← Phase 0 output
├── context-document.md     ← Phase 1 output (root context for all agents)
├── system-map.md           ← Phase 2 output (modules + interfaces)
├── story-graph.md          ← Phase 2 output (layered work items + slices)
├── agent-topology.md       ← Phase 3 output (roles + handoff contracts)
└── feedback-log.md         ← Phase 6 output (append-only)
```

## Core Architecture: Control Plane / Execution Plane

This plugin implements the **control plane** — goal-setting, decomposition, agent scheduling, policy, and replanning. The feature workflow (`/design → /build`) implements the **execution plane** — taking a spec, implementing it, testing it, producing a PR.

> "Agents do not talk to each other. They communicate through structured artifacts and state transitions on the work graph." — Design principle

## Inspired By

- [Harness Design for Long-Running Application Development](https://www.anthropic.com/engineering/harness-design-long-running-apps) — Anthropic Engineering
- [Effective Context Engineering for AI Agents](https://www.anthropic.com/engineering/effective-context-engineering-for-ai-agents) — Anthropic Engineering
- User Story Mapping — Jeff Patton (walking skeleton, layered delivery)

## Templates

- `templates/opportunity-brief.md` — Phase 0 output template
- `templates/context-document.md` — Phase 1 output template
- `templates/system-map.md` — Phase 2 System Map template
- `templates/story-spec.md` — Story specification template
- `templates/agent-topology.md` — Phase 3 agent role and handoff contract template

## References

- `references/orchestration-patterns.md` — Orchestrator design, state management, failure handling patterns
