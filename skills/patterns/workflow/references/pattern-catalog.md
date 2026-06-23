# Dynamic Workflow — Sub-Pattern Catalog

Detailed reference for the six dynamic-workflow sub-patterns. Each entry gives the
**intent**, the **Workflow primitive** to use, an **AEP example**, and a **skeleton**.
Read [`../SKILL.md`](../SKILL.md) first for the "should I use a workflow at all"
judgment and the failure modes these patterns counter.

## Primitives (recap)

The Claude Code Workflow tool exposes a few orchestration hooks inside the script:

- `agent(prompt, {schema, label, phase, model, isolation})` — spawn one
  context-isolated subagent; with a `schema` it returns validated structured output.
- `parallel(thunks)` — **barrier**: run all, wait for all, return the array. Use
  only when a later step genuinely needs _every_ prior result at once (dedupe,
  merge, early-exit on zero).
- `pipeline(items, stage1, stage2, …)` — **no barrier**: each item flows through
  all stages independently; item A can be in stage 3 while item B is still in stage
  1. **This is the default for multi-stage work.**
- `phase(title)` / `log(msg)` — progress grouping and narration.
- `budget` — the turn's token target; loop until `budget.remaining()` is low.

Rule: prefer `pipeline`. Reach for a `parallel` barrier only when stage N needs
cross-item context from _all_ of stage N−1.

---

## 1. Classify-and-route

**Intent.** Decide the _type_ of a task (or output) and send it to the right
handler or model tier. Either classify up front and dispatch, or classify at the
end to shape the final result.

**Primitive.** A single classifier `agent()` whose structured result drives a
branch / model choice; often the first stage of a `pipeline`.

**AEP example.** `/aep-dispatch` deciding story type, or a model-routing classifier
that sends simple stories to a cheaper tier and hard ones to Opus.

```js
const klass = await agent(`Classify this task: ${task}. Return {type, complexity}.`, {
  schema: {
    type: "object",
    properties: { type: { type: "string" }, complexity: { enum: ["low", "high"] } },
    required: ["type", "complexity"],
  },
});
const model = klass.complexity === "high" ? "opus" : "sonnet";
const result = await agent(handlerPromptFor(klass.type), { model });
```

---

## 2. Fan-out-and-synthesize

**Intent.** Split a big task into many small, independent steps; run one agent per
step; then **synthesize** the structured outputs into one result. Best when there
are many small steps that each benefit from a clean context.

**Primitive.** `parallel` for the fan-out **when** the synthesize step needs all
results together (it is a barrier — it waits for every fan-out agent, then merges).
If each item can be reduced independently, use `pipeline` instead.

**AEP example.** `/aep-executor`'s `workflow` mode: one build agent per locked
story in a dispatched wave, results collected for the main agent.

```js
const parts = await parallel(
  steps.map((s) => () => agent(`Do step: ${s.goal}`, { schema: PART, phase: "Fan-out" })),
);
const merged = await agent(
  `Synthesize these parts into one result: ${JSON.stringify(parts.filter(Boolean))}`,
  { schema: RESULT, phase: "Synthesize" },
);
```

---

## 3. Adversarial verification

**Intent.** For each produced output, run a **separate** agent that tries to
_refute_ it against a rubric or criteria — never let the author grade its own work.
Counters self-preferential bias directly.

**Primitive.** `pipeline`: stage 1 produces, stage 2 verifies. For higher
confidence, fan a small panel of independent refuters per finding and take a
majority.

**AEP example.** The generalized form of [`/aep-gen-eval`](../../gen-eval/SKILL.md)
(generator/evaluator) — reuse its scoring framework and findings format.

```js
const checked = await pipeline(
  findings,
  (f) =>
    agent(
      `Finding: ${f.claim}. Try to REFUTE it against the rubric. Default to refuted=true if uncertain.`,
      { schema: VERDICT, phase: "Verify" },
    ),
  (v, f) => ({ ...f, real: v && !v.refuted }),
);
const confirmed = checked.filter((x) => x.real);
```

---

## 4. Generate-and-filter

**Intent.** Generate many candidate ideas on a topic, then **filter** them by a
rubric or by verification, dedupe duplicates, and return only the highest-quality,
tested ones.

**Primitive.** `parallel` to generate, plain code to dedupe, then `parallel` (or a
verify stage) to filter. The dedupe is a genuine barrier — it needs all candidates.

**AEP example.** Brainstorming naming or design options before a calibration gate.

```js
const raw = (
  await parallel(
    angles.map((a) => () => agent(`Generate candidates from the ${a} angle.`, { schema: IDEAS })),
  )
)
  .filter(Boolean)
  .flatMap((r) => r.ideas);
const unique = dedupe(raw); // plain JS, not an agent
const kept = (
  await parallel(
    unique.map(
      (i) => () =>
        agent(`Score "${i}" against the rubric; keep only if it passes.`, { schema: SCORE }),
    ),
  )
)
  .filter(Boolean)
  .filter((s) => s.passes);
```

---

## 5. Tournament

**Intent.** When quality is subjective, have agents **compete**. Spawn N agents
that each attempt the same task with a _different_ approach, then judge results in
a **pairwise** fashion until a winner emerges. Comparative judgment is more
reliable than absolute scoring.

**Primitive.** `parallel` to produce the bracket entrants; then pairwise judge
agents (a reduction, often a loop) until one winner remains.

**AEP example.** Taste-based decisions — naming, design direction — feeding a
`/aep-calibrate` gate with the top candidates.

```js
let bracket = await parallel(
  approaches.map((a) => () => agent(`Attempt the task using approach: ${a}.`, { schema: ENTRY })),
);
bracket = bracket.filter(Boolean);
while (bracket.length > 1) {
  const next = [];
  for (let i = 0; i < bracket.length; i += 2) {
    if (!bracket[i + 1]) {
      next.push(bracket[i]);
      continue;
    }
    const win = await agent(
      `Pick the better of A vs B for the goal.\nA:${JSON.stringify(bracket[i])}\nB:${JSON.stringify(bracket[i + 1])}`,
      {
        schema: {
          type: "object",
          properties: { winner: { enum: ["A", "B"] } },
          required: ["winner"],
        },
      },
    );
    next.push(win.winner === "A" ? bracket[i] : bracket[i + 1]);
  }
  bracket = next;
}
const champion = bracket[0];
```

---

## 6. Loop-until-done

**Intent.** For tasks with an **unknown** amount of work, keep spawning agents
until a _stop condition_ is met (no new findings, no more errors in the logs)
rather than a fixed number of passes. Counters laziness and premature "done".

**Primitive.** A `while` loop around `agent()`/`parallel()`, with a "dry rounds"
counter and a `seen` set so re-discovered items don't reset the loop.

**AEP example.** The short-lived cousin of `/aep-autopilot`'s tick loop —
e.g. reproduce a 1-in-50 flaky test, or mine a log until no new root causes appear.

```js
const seen = new Set();
let dry = 0;
while (dry < 2 && (!budget.total || budget.remaining() > 50_000)) {
  const found = (
    await agent("Find the next distinct issue; return [] if none.", { schema: ISSUES })
  ).issues;
  const fresh = found.filter((i) => !seen.has(key(i)));
  if (!fresh.length) {
    dry++;
    continue;
  }
  dry = 0;
  fresh.forEach((i) => seen.add(key(i)));
  log(`${seen.size} distinct issues so far`);
}
```

---

## Architectural levers

Independent of which sub-pattern you pick, a workflow can tune:

- **Per-agent model tier.** Cheap classifier / mechanical stages on a small model;
  hard judging or synthesis on the strongest. Pass `model` per `agent()`.
- **Worktree isolation.** Use `isolation: 'worktree'` only when agents mutate files
  in parallel and would otherwise conflict (it has real setup cost). In AEP, prefer
  AEP-created `.feature-workspaces/<ws>` worktrees so `monitor()` / `/aep-wrap` paths
  stay standard — see `../../executor/references/backends.md`.
- **Quarantine.** In triage workflows, agents that read **untrusted public content**
  must be barred from high-privilege actions — keep read-untrusted and act-with-
  privilege in separate agents.
- **Token budgets.** Scale fleet size or loop depth to `budget`; a hard ceiling
  prevents runaway spend.
- **Resumability.** If a workflow is interrupted, resuming the session lets it pick
  up where it left off; completed agents return from cache.
