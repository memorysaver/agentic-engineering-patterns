# Dynamic Workflows — "a harness for every task" × AEP

> **Status:** Research note + rationale for the `/aep-workflow` pattern (shipped).
> Records the source idea, the failure modes it targets, the sub-pattern catalog,
> and _why AEP adds it as a first-class pattern_ rather than relying on the
> `aep-executor` `workflow` backend mode alone.
> **Date:** 2026-06-23
> **Source:** Thariq Shihipar & Sid Bidasaria (Anthropic), _"A harness for every
> task: dynamic workflows in Claude Code."_ Original X post
> (`x.com/trq212/status/2061907337154367865`) links to the canonical article on
> claude.com. The X article body is login-gated, so the full text was recovered
> from the official post:
> <https://claude.com/blog/a-harness-for-every-task-dynamic-workflows-in-claude-code>
> (fetched via WebSearch + WebFetch — mgrep `--web` quota was exhausted this month).

---

## 0. TL;DR

- A **dynamic workflow** is a harness Claude **writes itself** for one task: a
  deterministic JS script (the Claude Code _Workflow_ tool) that spawns and
  coordinates context-isolated subagents, custom-built for the task — instead of
  running everything in the single default coding harness.
- It exists to counter three failure modes of long single-context work:
  **agentic laziness**, **self-preferential bias**, and **goal drift**.
- It is not a default wrapper. The article's own guardrail: _most traditional
  coding tasks do not need a panel of 5 reviewers_ — workflows cost significantly
  more tokens, so ask "does this really need more compute?"
- AEP already used dynamic workflows **narrowly** (one dispatched build wave =
  one Workflow script, in `aep-executor`'s `workflow` mode). The gap was a
  **general pattern** capturing the broader idea and the "when to reach for one"
  judgment. `/aep-workflow` fills it.

---

## 1. What a harness is, and what changed

A _harness_ is the scaffolding around the model — the part that decides how a task
is planned, divided, checked, and executed. Claude Code ships one default harness,
built for coding. With Opus 4.8 + dynamic workflows, Claude can author a **new
harness on the fly**, tailored to the task, and run it as a Workflow: a JS file
with a few special functions (`agent`, `parallel`, `pipeline`, `phase`, `log`,
`budget`) that spawn subagents and merge their structured outputs.

The leverage is **structural**: many small agents, each with a clean context and a
focused goal, coordinated by deterministic control flow — rather than one long
transcript that has to hold everything.

## 2. The three failure modes (why it earns its place)

AEP design principle #3: _every harness component earns its place — each exists
because of a specific failure mode._ The article names three:

1. **Agentic laziness** — Claude stops before finishing a complex multi-part task
   and declares it done after partial progress (e.g. 35 of 50 security-review
   items). → Fan-out (one agent per item) + a `loop-until-done` stop condition.
2. **Self-preferential bias** — Claude prefers / passes its own results when asked
   to verify or judge them against a rubric. → A _separate_ verifier agent. This is
   the same insight AEP already encodes in `/aep-gen-eval`; workflows generalize it
   to N independent verifiers/refuters.
3. **Goal drift** — gradual loss of fidelity to the original objective across many
   turns, especially after compaction; each summarization is lossy, and "don't do
   X" constraints get dropped. → Short, focused goals in fresh contexts that never
   have to survive a long lossy history.

## 3. The sub-pattern catalog

Codified in
[`skills/patterns/workflow/references/pattern-catalog.md`](../../skills/patterns/workflow/references/pattern-catalog.md):

1. **Classify-and-route** — classifier picks task type / model tier, then routes.
2. **Fan-out-and-synthesize** — split → agent per step → barrier merges results.
3. **Adversarial verification** — separate agent refutes each output vs a rubric.
4. **Generate-and-filter** — many candidates → dedupe → keep rubric-passers.
5. **Tournament** — N approaches compete; pairwise judging until a winner.
6. **Loop-until-done** — spawn until a stop condition, not a fixed pass count.

Plus architectural levers: per-agent model choice, worktree isolation, **quarantine**
(untrusted-content readers barred from high-privilege actions), token budgets, and
resumability. These compose — a thorough review is fan-out → adversarial verify →
loop-until-dry.

## 4. How it's invoked

- Ask Claude to "set up / use a workflow…".
- The **`ultracode`** keyword forces a workflow.
- Inside AEP, **"…with workflow"** routes a dispatched build wave through
  `aep-executor`'s `workflow` mode.
- Pair with **`/loop`** (recurring), **`/goal`** (hard completion requirement,
  counters laziness), and **token budgets** ("use 10k tokens"). Workflows are
  saveable (`s` in the menu → `~/.claude/workflows`) and shareable via a skill —
  best treated as a _template_, not a verbatim script.

## 5. Why a pattern in AEP, not just the executor mode

`aep-executor` already had a `workflow` backend, but it is scoped to one thing: run
a dispatched _build_ wave as a fan-out (see
[`skills/patterns/executor/references/backends.md`](../../skills/patterns/executor/references/backends.md)
→ _Mode: workflow_). The article's idea is much broader — verification, tournaments,
research, triage, evals, sorting at scale — and the most valuable thing to capture
is the **judgment of when a task warrants a workflow at all**. That belongs in a
first-class, discoverable pattern (`/aep-workflow`), cross-linked to the executor
mode and to `/aep-gen-eval` (the canonical adversarial-verification implementation)
and `/aep-autopilot` (the long-lived loop), rather than buried in one backend.

## 6. Decision in this change

- **Ship** `/aep-workflow` in the `patterns` group as a dual library + standalone
  skill, mirroring `/aep-gen-eval`'s shape.
- **Do not** duplicate gen-eval's scoring or autopilot's loop machinery — point to
  them. The new skill owns the catalog + the "when (not) to use" guardrail.
- **Do not** change the Workflow tool or executor selection logic — documentation
  cross-links only.

## Sources

- Thariq Shihipar & Sid Bidasaria, Anthropic — _A harness for every task: dynamic
  workflows in Claude Code_ —
  <https://claude.com/blog/a-harness-for-every-task-dynamic-workflows-in-claude-code>
- X post: <https://x.com/trq212/status/2061907337154367865> (login-gated body).
