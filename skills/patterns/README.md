# Patterns

Reusable design patterns extracted from the AEP workflow. These are utility skills — both directly invokable and referenceable by other skills.

| Pattern                                         | What it does                                                                                                                                                                                                                  | Used by                                                                             |
| ----------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------- |
| [gen-eval](gen-eval/SKILL.md)                   | Generator/evaluator separation for honest artifact validation                                                                                                                                                                 | `/aep-build` Phase 5, `/aep-launch` evaluator setup, `/aep-validate`                |
| [executor](executor/SKILL.md)                   | Host-agnostic launch modes for spawning/steering workspace agents (Claude teams/bg sessions; Codex subagents/exec; tmux when pinned; workflow opt-in)                                                                         | `/aep-launch`, `/aep-build` Phase 5, `/aep-autopilot`, `/aep-dispatch`              |
| [autopilot](autopilot/SKILL.md)                 | Tick-based autonomous orchestration of dispatch-launch-monitor-review-wrap cycle                                                                                                                                              | `/loop`, orchestrates `/aep-dispatch`, `/aep-launch`, `/aep-wrap`                   |
| [workflow](workflow/SKILL.md)                   | Dynamic workflows — when to write a custom multi-agent harness for a task, and the sub-pattern catalog (fan-out, adversarial verify, tournament, loop-until-done, classify-route, generate-filter)                            | `/aep-executor` workflow mode, `/aep-gen-eval`, `/aep-validate`, `/aep-dispatch`    |
| [workflow-feedback](workflow-feedback/SKILL.md) | Capture process learnings about the AEP workflow itself (distinct from product feedback)                                                                                                                                      | `/aep-workflow-feedback capture` / `review`                                         |
| [design-lens](design-lens/SKILL.md)             | Theory-grounded design guidance + heuristic health-check — an extensible HCI/design-theory catalog (Nielsen, Norman, Shneiderman, Munzner, Human-AI guidelines…) + a task/data method that selects the lenses for any product | `/aep-model`, `/aep-calibrate` ux-flow, `/aep-validate` (design-quality complement) |

## Why Patterns Exist

Some capabilities are shared across multiple workflow phases:

- Scoring dimensions and failure thresholds are used by build, launch, and validate
- Agent prompt templates are used by build and validate
- The eval request/response protocol is used by build and validate
- Findings consolidation is used by validate and gen-eval standalone
- Autonomous orchestration logic is shared between dispatch and the full lifecycle loop
- Host detection + backend selection (executor) is shared by launch, build, autopilot, and dispatch
- Established HCI/design theory (design-lens) is applied by model (IA), calibrate (ux-flow), and validate (design-quality) — one catalog instead of each re-deriving 40 years of research

Duplicating this logic across skills creates drift. Extracting it into a shared pattern ensures consistency and makes updates propagate to all consumers.

## How Skills Reference Patterns

After sync with the `aep-` prefix, pattern files are below `<skills-root>`, where
the canonical cross-tool root is `.agents/skills` and a Claude-only install may
use `.claude/skills`:

```
<skills-root>/aep-gen-eval/references/scoring-framework.md
<skills-root>/aep-gen-eval/references/agent-contracts.md
<skills-root>/aep-gen-eval/references/eval-protocol.md
<skills-root>/aep-gen-eval/references/findings-format.md

<skills-root>/aep-autopilot/references/tick-protocol.md
<skills-root>/aep-autopilot/references/state-schema.md
<skills-root>/aep-autopilot/references/review-trigger.md
<skills-root>/aep-autopilot/references/orchestration-learning.md

<skills-root>/aep-executor/references/backends.md

<skills-root>/aep-workflow/references/pattern-catalog.md
```

Skills reference them via these paths in their instructions.
