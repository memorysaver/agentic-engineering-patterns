# Product Context

A persistent product-level planning layer that captures the "what and why" of the entire product in a single YAML file. Used for both initial product planning and ongoing iteration.

Four skills, one feedback loop:

```
/envision → /map → [ /dispatch → /design → /build → /wrap ] → /reflect → loop
```

| Skill | What it does | When to use |
|-------|-------------|-------------|
| [/envision](envision/SKILL.md) | Opportunity brief + context document | Starting a new product, revisiting direction |
| [/map](map/SKILL.md) | System map + story graph + agent topology | Decomposing a product into executable work |
| [/dispatch](dispatch/SKILL.md) | Pick next story, create OpenSpec change | Ready to start building a feature |
| [/reflect](reflect/SKILL.md) | Classify feedback + update context | After shipping, after user testing |

## Single Source of Truth: `product-context.yaml`

Everything lives in one structured YAML file at the project root:

```yaml
product-context.yaml
  ├── opportunity    # /envision Phase 0 — should we build this?
  ├── product        # /envision Phase 1 — what exactly to build
  ├── architecture   # /map Step 1 — modules + interface contracts
  ├── stories        # /map Steps 2-3 — layered work items with state machine
  ├── topology       # /map Step 4 — agent roles + handoff contracts
  ├── layer_gates    # integration tests per layer
  ├── cost           # accumulated cost data per story/layer/module
  └── changelog      # semantic history — why things changed, not just what
```

YAML was chosen because it is both human-readable AND machine-parseable. Agents can read specific sections, update fields atomically, and the file produces meaningful git diffs. Templates (`templates/*.md`) remain as conversation guides; the YAML is the structured output.

**Concurrency protocol:** Only the main session writes to this file. Workspace agents report status through `.dev-workflow/signals/`. The main session (via `/wrap`, `/dispatch`, `/reflect`) reads signals and updates the YAML.

## How It Connects to the Feature Workflow

```
┌──────────────────────────────────────┐
│   Product Context (control plane)    │
│                                      │
│   /envision → /map ───┐              │
│        ↑              │ stories      │
│   /reflect            ↓              │
│        ↑         /dispatch           │
│        │              │              │
└────────┼──────────────┼──────────────┘
         │              │
         │    ┌─────────┼─────────────┐
         │    │  Feature │ Workflow    │
         │    │  (execution plane)    │
         │    │         ↓             │
         │    │  /design → /launch    │
         │    │    → /build → /wrap ──┤
         │    └───────────────────────┘
         │                            │
         └────────────────────────────┘
```

- `/dispatch` picks stories from the YAML and creates OpenSpec changes
- `/design` reads story context from the YAML for richer specs
- `/build` updates story status and cost in the YAML
- `/wrap` checks layer completion and suggests `/reflect`
- `/reflect` classifies feedback and routes it back to the right phase

## Core Architecture: Control Plane / Execution Plane

This plugin implements the **control plane** — goal-setting, decomposition, agent scheduling, policy, and replanning. The feature workflow (`/design → /build`) implements the **execution plane** — taking a spec, implementing it, testing it, producing a PR.

> "Agents do not talk to each other. They communicate through structured artifacts and state transitions on the work graph." — Design principle

## Templates & References

**Templates** (conversation guides — used during /envision and /map to capture the right information):
- `templates/product-context-schema.yaml` — Complete YAML schema with field definitions
- `templates/opportunity-brief.md` — Phase 0 conversation guide
- `templates/context-document.md` — Phase 1 conversation guide
- `templates/system-map.md` — System Map conversation guide
- `templates/story-spec.md` — Story specification guide
- `templates/agent-topology.md` — Agent role and handoff contract guide

**References:**
- `references/orchestration-patterns.md` — Orchestrator design, state management, failure handling patterns

## Inspired By

- [Harness Design for Long-Running Application Development](https://www.anthropic.com/engineering/harness-design-long-running-apps) — Anthropic Engineering
- [Effective Context Engineering for AI Agents](https://www.anthropic.com/engineering/effective-context-engineering-for-ai-agents) — Anthropic Engineering
- User Story Mapping — Jeff Patton (walking skeleton, layered delivery)
