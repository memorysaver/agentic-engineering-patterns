---
name: aep-workflow
description: >-
  Authors a deterministic multi-agent harness for one large, uncertain, or
  verification-heavy task. Use for "dynamic workflow", "ultracode", or
  "orchestrate subagents"; not process feedback (/aep-workflow-feedback).
---

# Dynamic Workflow Pattern

A reusable pattern for **building a custom harness for a single task**. Instead of
running the task in the one default coding harness, Claude writes a small
deterministic JavaScript program — a **Claude Code Workflow** — that spawns and
coordinates subagents, each with its own clean context window, tuned to the task.
Invoke it directly to decide whether a task warrants a workflow and to author one;
it also serves as the sub-pattern library that `/aep-gen-eval` and `/aep-executor`
read.

> "Claude can now write its own harness on the fly, custom-built for the task at
> hand." — Thariq Shihipar & Sid Bidasaria, Anthropic, ["A harness for every task:
> dynamic workflows in Claude Code"](https://claude.com/blog/a-harness-for-every-task-dynamic-workflows-in-claude-code)

> **Not `/aep-workflow-feedback`.** That skill captures lessons about the AEP
> _process_; this one authors an orchestration script for a task.

---

## When to reach for a workflow

The core judgment: a workflow trades **significantly more tokens** for isolated
context windows + focused goals + a deterministic orchestrator instead of one long,
drifting transcript. That trade is worth it — and only worth it — when a single
context would go lazy, biased, or off-goal (the three failure modes it counters are
tabulated in [`references/pattern-catalog.md`](references/pattern-catalog.md)).

**Reach for a workflow when the task:**

- **spans many independent items** — sort 1000+ rows, audit every claim, refactor
  every callsite — where quality degrades if crammed into one prompt;
- **needs adversarial verification** by something other than its author (security
  review, fact-check, eval);
- **is taste-based** and benefits from competing attempts (naming, design direction);
- **needs routing** of different inputs to different handling or model tiers.

Otherwise **stay single-context**: an ordinary coding task that fits one window, or
a verification need a single `/aep-gen-eval` loop already covers, does not need a
panel of agents. Use workflows "creatively to push Claude Code in ways you haven't
previously" — not as a default wrapper around routine work.

---

## Sub-patterns and their AEP touchpoints

Pick the shape that matches the task (they compose — a thorough review is
_fan-out → adversarial verify → loop-until-dry_). Full intent, the primitive to use
(`parallel` barrier vs `pipeline` no-barrier vs loop), skeletons, and architectural
levers are in [`references/pattern-catalog.md`](references/pattern-catalog.md).

| Sub-pattern                  | What it does                                                       | AEP touchpoint                                                                                                                                                                                       |
| ---------------------------- | ------------------------------------------------------------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Classify-and-route**       | A classifier agent decides task type / model tier, then routes.    | `/aep-dispatch` routes by `readiness_score` and offers the "…with workflow" opt-in; a full classifier / model-tier router is the workflow generalization.                                            |
| **Fan-out-and-synthesize**   | Split into many small steps → one agent each → a barrier merges.   | `/aep-executor` `workflow` mode runs one dispatched build wave this way (the narrow use); this skill is the general catalog + judgment.                                                              |
| **Adversarial verification** | For each output, a separate agent refutes it against a rubric.     | `/aep-gen-eval` (generator/evaluator) is the canonical N=1 instance; a workflow generalizes it to N verifiers per finding. `/aep-validate` runs a fixed trio today — a workflow gives one per claim. |
| **Generate-and-filter**      | Generate many candidates → dedupe → keep only rubric-passing ones. | Naming / design option generation before a calibration gate.                                                                                                                                         |
| **Tournament**               | N approaches compete; pairwise judging until a winner.             | Taste-based decisions (naming, design direction) feeding a `/aep-calibrate` gate.                                                                                                                    |
| **Loop-until-done**          | Keep spawning until a stop condition, not a fixed count.           | `/aep-autopilot`'s tick loop is the long-lived, OS-driven cousin.                                                                                                                                    |

---

## How to invoke

1. **Just ask.** "Set up a workflow to…" / "Use a workflow to verify every claim…".
2. **`ultracode` keyword.** Including `ultracode` forces Claude Code to author a
   workflow for the request.
3. **Within AEP.** "…with workflow" routes a dispatched build wave through
   `/aep-executor`'s `workflow` backend mode (one agent per locked story,
   hub-and-spoke gating) — see `/aep-executor` references (`backends.md`) →
   _Mode: workflow_.

**Pair with other tools:** `/loop` runs a repeatable workflow on an interval;
`/goal` sets a hard completion requirement so it can't quit early (counters
laziness); prompt with a token budget ("use 10k tokens") to cap spend; press `s` in
the workflow menu to save to `~/.claude/workflows` — treat a saved workflow as a
**template**, not a script to run verbatim.

---

## Standalone authoring

1. **Decide if it's worth it.** Apply the sizing rule above. If one context window
   suffices, stop here and just do the task.
2. **Pick a sub-pattern** from the catalog (or compose several).
3. **Choose the primitive:** `parallel` (barrier — you need all results together),
   `pipeline` (no barrier — items flow through stages independently, the default),
   or a loop (unknown amount of work).
4. **Add verification** — a separate agent per finding when correctness matters;
   default verifiers toward refuting, not rubber-stamping.
5. **Set guardrails** — per-agent model tier, worktree isolation for parallel file
   edits, a token budget, and (for triage of untrusted content) **quarantine**:
   agents that read untrusted public content must not take high-privilege actions.
6. **Author the script and launch it**, then return the synthesized result to the
   caller: a dispatched build wave to `/aep-executor` `workflow` mode; a
   verification / eval folded back into `/aep-validate` or `/aep-gen-eval`;
   standalone research / triage presented directly (pair with `/loop` if recurring).

Read [`references/pattern-catalog.md`](references/pattern-catalog.md) for skeletons.

Why this is a first-class pattern (not just `/aep-executor`'s `workflow` mode, and
why it sits beside `/aep-gen-eval` rather than inside it): rationale in
[`docs/decisions/workflow-rationale.md`](../../../docs/decisions/workflow-rationale.md).
