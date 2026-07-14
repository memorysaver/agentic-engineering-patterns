# Dynamic Workflow as a First-Class Pattern — Rationale

> Decision record. Migrated out of `skills/patterns/workflow/SKILL.md` under the
> [skill-authoring standard](./skill-authoring-standard.md) (R5: rationale lives in
> `docs/decisions/`, not the agent hot path). Records why the dynamic-workflow
> pattern is its own invocable skill rather than a mode of another skill.

## Why a pattern, not just the executor mode

`/aep-executor` already has a `workflow` backend — but it is narrow: run one
dispatched _build_ wave as a fan-out. The dynamic-workflow idea is broader
(verification, tournaments, research, triage, evals, sorting at scale), and the
most valuable judgment is _when a task warrants a workflow at all_. That judgment
belongs in a first-class pattern, not buried in one executor mode.

## Why it sits next to gen-eval, not inside it

Generator/evaluator is _one_ sub-pattern (adversarial verification with N=1
evaluator). `/aep-gen-eval` stays the canonical, reusable implementation; this
skill points to it rather than re-specifying scoring. Likewise `/aep-autopilot`
owns the long-lived loop; this skill only describes loop-until-done in the
short-lived, in-workflow sense.
