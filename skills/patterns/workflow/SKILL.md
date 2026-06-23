---
name: aep-workflow
description: |-
  Reusable pattern for authoring a dynamic workflow — a custom multi-agent harness Claude writes on the fly for one task, run as a Claude Code Workflow (a deterministic JS script that spawns and coordinates context-isolated subagents). This is a utility skill — it provides the sub-pattern catalog (classify-and-route, fan-out-and-synthesize, adversarial verification, generate-and-filter, tournament, loop-until-done) and the "when to reach for a workflow" judgment. Cross-linked from /aep-gen-eval and /aep-executor; relevant when /aep-dispatch or /aep-validate work would benefit from a per-task harness. Use directly when a task is large, uncertain, needs adversarial verification, or runs at scale and the default single-context harness would be lazy, biased, or drift off-goal. Triggers on "dynamic workflow", "ultracode", "write a harness", "harness for this task", "…with workflow", "orchestrate subagents". NOT for capturing process feedback — that is /aep-workflow-feedback.
---

# Dynamic Workflow Pattern

A reusable pattern for **building a custom harness for a single task**. Instead of
running the task in the one default coding harness, Claude writes a small
deterministic JavaScript program — a **Claude Code Workflow** — that spawns and
coordinates subagents, each with its own clean context window, tuned to the task
at hand.

> "Claude can now write its own harness on the fly, custom-built for the task at
> hand. While the default Claude Code harness is built for coding, … Claude is now
> intelligent enough to write a custom harness tailor-made for your use case."
> — Thariq Shihipar & Sid Bidasaria, Anthropic, ["A harness for every task:
> dynamic workflows in Claude Code"](https://claude.com/blog/a-harness-for-every-task-dynamic-workflows-in-claude-code)

> **Not `/aep-workflow-feedback`.** That skill captures lessons about the AEP
> _process_. This skill is about authoring a multi-agent _orchestration script_ for
> a task. Different jobs, similar prefix.

**This skill is both a utility library and a standalone skill:**

- **As a library:** other skills read `references/pattern-catalog.md` to pick the
  right sub-pattern and shape its script.
- **As a standalone skill:** invoke directly to decide whether a task warrants a
  workflow and to author one.

---

## Why This Pattern Earns Its Place

AEP's third design principle: _every harness component earns its place — each
exists because of a specific failure mode._ Dynamic workflows are a structural fix
for three failure modes that appear when a complex task runs inside one context
window:

| Failure mode               | What it looks like                                                                                                           | How a workflow prevents it                                                                                                                   |
| -------------------------- | ---------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------- |
| **Agentic laziness**       | Stops after partial progress and declares done (e.g. 35 of 50 security-review items).                                        | Fan-out gives each item its own agent; a `loop-until-done` stop condition replaces a fixed pass count.                                       |
| **Self-preferential bias** | Praises / passes its own work when asked to verify it against a rubric.                                                      | A _separate_ verifier agent (no authorship attachment) judges each output — the generalized form of [`/aep-gen-eval`](../gen-eval/SKILL.md). |
| **Goal drift**             | Fidelity to the original objective decays across many turns, especially after compaction; "don't do X" constraints get lost. | Each subagent gets a short, focused goal in a fresh context, so the objective never has to survive a long lossy history.                     |

The mechanism in every case is the same: **isolated context windows + focused
goals + a deterministic orchestrator** instead of one long, drifting transcript.

---

## The Sub-Pattern Catalog (summary)

Pick the shape that matches the task. Full intent, the Workflow primitive to use
(`parallel` barrier vs `pipeline` no-barrier vs loop), AEP examples, and skeletons
are in [`references/pattern-catalog.md`](references/pattern-catalog.md).

| Sub-pattern                  | One-liner                                                                              | AEP instance                                                      |
| ---------------------------- | -------------------------------------------------------------------------------------- | ----------------------------------------------------------------- |
| **Classify-and-route**       | A classifier agent decides the task type / model tier, then routes.                    | `/aep-dispatch` readiness-score routing + "…with workflow" opt-in |
| **Fan-out-and-synthesize**   | Split into many small steps → one agent each → a barrier merges results.               | `/aep-executor` `workflow` mode (one agent per story)             |
| **Adversarial verification** | For each output, a separate agent tries to refute it against a rubric.                 | `/aep-gen-eval` (generator/evaluator) generalized to N verifiers  |
| **Generate-and-filter**      | Generate many candidates → dedupe → keep only rubric-passing ones.                     | naming / design option generation                                 |
| **Tournament**               | N approaches compete; pairwise judging until a winner (comparative > absolute).        | taste-based decisions (naming, design direction)                  |
| **Loop-until-done**          | Keep spawning until a stop condition (no new findings / no errors), not a fixed count. | `/aep-autopilot` tick loop is the long-lived cousin               |

These compose: a thorough review is _fan-out → adversarial verify → loop-until-dry_.

---

## When to Use a Workflow — and When NOT To

Workflows are powerful but **cost significantly more tokens**. They are not needed
for every task.

**Reach for a workflow when:**

- The work is **large or unbounded** (sort 1000+ rows, audit every claim, refactor
  every callsite) — quality degrades if crammed into one prompt.
- The output **must be verified** by something other than its author (security
  review, fact-check, eval).
- The task is **taste-based** and benefits from competing attempts (naming,
  design direction).
- You want to **route** different inputs to different handling or model tiers.

**Do NOT reach for a workflow when:**

- It's an ordinary coding task that fits one context. _Most traditional coding
  tasks do not need a panel of 5 reviewers._ Ask: **does this really need more
  compute?** If not, just do it.
- A single `/aep-gen-eval` loop already covers the verification need.

> Rule of thumb from the article: _"use workflows creatively to push Claude Code in
> ways you haven't previously"_ — not as a default wrapper around routine work.

---

## How to Invoke

1. **Just ask.** "Set up a workflow to…" / "Use a workflow to verify every claim…".
2. **`ultracode` keyword.** Including `ultracode` forces Claude Code to author a
   workflow for the request.
3. **Within AEP.** "…with workflow" routes a dispatched build wave through
   [`/aep-executor`](../executor/SKILL.md)'s `workflow` backend mode (one agent per
   locked story, hub-and-spoke gating). See
   [`../executor/references/backends.md`](../executor/references/backends.md) → _Mode: workflow_.

**Pair with other tools:**

- **`/loop`** — run a repeatable workflow (triage, research, verification) on an
  interval.
- **`/goal`** — set a hard completion requirement so the workflow can't quit early
  (counters laziness).
- **Token budgets** — prompt with a budget ("use 10k tokens") to cap spend.
- **Save & share** — press `s` in the workflow menu to save to
  `~/.claude/workflows`, or distribute via a skill. Treat a saved workflow as a
  **template**, not a script to run verbatim.

---

## How This Fits AEP (touchpoints)

| AEP touchpoint                                          | Sub-pattern it uses                | Relationship                                                                                                                                                                                                                                                    |
| ------------------------------------------------------- | ---------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [`/aep-executor`](../executor/SKILL.md) `workflow` mode | fan-out / pipeline                 | Runs one dispatched build wave as a Workflow script — the **narrow** use. This skill is the **general** catalog and the "should I even use a workflow" judgment.                                                                                                |
| [`/aep-gen-eval`](../gen-eval/SKILL.md)                 | adversarial verification           | Generator/evaluator separation is the canonical instance; workflows generalize it to N independent verifiers/refuters per finding.                                                                                                                              |
| `/aep-validate`                                         | gen/eval today; fan-out as upgrade | Validate runs a **fixed** Generator/Evaluator(/Protocol Checker) trio and checks claims **inside** the evaluator. When there are many independent claims, a workflow upgrades that to **one verifier per claim** — a generalization validate does not do today. |
| `/aep-dispatch`                                         | classify-and-route (score-based)   | Dispatch routes by `readiness_score` and offers the "…with workflow" batch opt-in. A classifier agent / model-tier routing is the **workflow** generalization, not what dispatch does today.                                                                    |
| [`/aep-autopilot`](../autopilot/SKILL.md)               | loop-until-done                    | The tick loop is the long-lived, OS-driven cousin of an in-workflow loop-until-dry.                                                                                                                                                                             |

### Cross-skill reference path

After sync with the `aep-` prefix, the catalog is at:

```
.claude/skills/aep-workflow/references/pattern-catalog.md
```

---

## Standalone Usage

1. **Decide if it's worth it.** Apply the "when to use / when NOT" test above. If
   one context window suffices, stop here and just do the task.
2. **Pick a sub-pattern** from the catalog (or compose several).
3. **Choose the primitive:** `parallel` (barrier — you need all results together,
   e.g. before a dedupe/synthesize), `pipeline` (no barrier — each item flows
   through stages independently, the default), or a loop (unknown amount of work).
4. **Add verification** — a separate agent per finding when correctness matters;
   default verifiers toward refuting, not rubber-stamping.
5. **Set guardrails** — per-agent model tier, worktree isolation for parallel file
   edits, a token budget, and (for triage of untrusted content) **quarantine**:
   agents that read untrusted public content must not take high-privilege actions.
6. **Author or ask for the script**, then run it.

Read [`references/pattern-catalog.md`](references/pattern-catalog.md) for skeletons.

---

## Design Decisions

**Why a pattern, not just the executor mode.** `/aep-executor` already has a
`workflow` backend — but it is narrow: run one dispatched _build_ wave as a
fan-out. The article's idea is broader (verification, tournaments, research,
triage, evals, sorting at scale) and the most valuable judgment is _when a task
warrants a workflow at all_. That judgment belongs in a first-class pattern, not
buried in one executor mode.

**Why it sits next to gen-eval, not inside it.** Generator/evaluator is _one_
sub-pattern (adversarial verification with N=1 evaluator). `/aep-gen-eval` stays
the canonical, reusable implementation; this skill points to it rather than
re-specifying scoring. Likewise `/aep-autopilot` owns the long-lived loop; this
skill only describes loop-until-done in the short-lived workflow sense.

**Why "when NOT to" is load-bearing.** Workflows multiply token cost. Without an
explicit guardrail, the pattern degrades into a reflex wrapper around routine
coding — exactly what the article warns against.

---

## Next Step

After deciding on / authoring a workflow, control returns to the calling context:

- Dispatched build wave → [`/aep-executor`](../executor/SKILL.md) `workflow` mode runs it.
- Verification / eval of an artifact → fold results back into `/aep-validate` or `/aep-gen-eval`.
- Standalone research / triage → present the synthesized result; pair with `/loop` if recurring.
