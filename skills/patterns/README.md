# Patterns

Reusable design patterns extracted from the AEP workflow. These are utility skills — both directly invokable and referenceable by other skills.

| Pattern                         | What it does                                                                     | Used by                                                  |
| ------------------------------- | -------------------------------------------------------------------------------- | -------------------------------------------------------- |
| [gen-eval](gen-eval/SKILL.md)   | Generator/evaluator separation for honest artifact validation                    | `/build` Phase 5, `/launch` evaluator setup, `/validate` |
| [autopilot](autopilot/SKILL.md) | Tick-based autonomous orchestration of dispatch-launch-monitor-review-wrap cycle | `/loop`, orchestrates `/dispatch`, `/launch`, `/wrap`    |

## Why Patterns Exist

Some capabilities are shared across multiple workflow phases:

- Scoring dimensions and failure thresholds are used by build, launch, and validate
- Agent prompt templates are used by build and validate
- The eval request/response protocol is used by build and validate
- Findings consolidation is used by validate and gen-eval standalone
- Autonomous orchestration logic is shared between dispatch and the full lifecycle loop

Duplicating this logic across skills creates drift. Extracting it into a shared pattern ensures consistency and makes updates propagate to all consumers.

## How Skills Reference Patterns

After sync with `aep-` prefix, pattern files are at:

```
.claude/skills/aep-gen-eval/references/scoring-framework.md
.claude/skills/aep-gen-eval/references/agent-contracts.md
.claude/skills/aep-gen-eval/references/eval-protocol.md
.claude/skills/aep-gen-eval/references/findings-format.md

.claude/skills/aep-autopilot/references/tick-protocol.md
.claude/skills/aep-autopilot/references/state-schema.md
.claude/skills/aep-autopilot/references/review-trigger.md
.claude/skills/aep-autopilot/references/orchestration-learning.md
```

Skills reference them via these paths in their instructions.
