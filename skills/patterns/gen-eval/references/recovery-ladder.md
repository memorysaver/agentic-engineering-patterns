# Change-Strategy Recovery Ladder

When the Phase 5 gen/eval loop FAILs, the default behavior is for the **same generator to retry the same way** — fix the FAIL items, re-request evaluation, repeat. After `max_rounds` (default 5) it escalates to a human. The failure mode this guards against is **strategy stagnation**: the generator keeps applying the approach that already failed, burning rounds without exploring a genuinely different path.

This reference defines an escalating recovery ladder. Each rung tries something **structurally different** from the last, so the system exhausts real strategy changes **before** a human gate — not five copies of the same attempt.

> The evaluator never climbs this ladder. Generator≠evaluator separation still holds: the evaluator scores; the generator (or a fresh generator) is the only role that "tries a new approach." A re-grounded read, a fresh generator, and a decomposition are all generator-side moves.

---

## Table of Contents

1. [The Ladder](#the-ladder)
2. [When to Skip the Ladder](#when-to-skip-the-ladder)
3. [State Tracking](#state-tracking)
4. [Spawning a Fresh Generator (Rung 4)](#spawning-a-fresh-generator-rung-4)
5. [Cross-References](#cross-references)

---

## The Ladder

Round numbers are tunable per project; the **shape** is what matters — each rung is a strictly larger change of strategy than the one below it.

| Eval round | Rung                   | Strategy                                                                                                                                |
| ---------- | ---------------------- | --------------------------------------------------------------------------------------------------------------------------------------- |
| 1–2        | **Same fix**           | Same generator fixes the FAIL items normally. Current default behavior.                                                                 |
| 3          | **Re-ground**          | Same generator re-reads the FULL spec + design + contracts **from scratch** and re-attempts.                                            |
| 4          | **Different approach** | Spawn a **fresh generator** told "the previous approach failed on X; take a different design path." Not anchored on the stuck solution. |
| 5          | **Decompose**          | Split the story into smaller sub-stories / sub-tasks; attempt the **smallest viable slice**. Surface the proposed split.                |
| after 5    | **Human gate**         | Ladder exhausted → escalate with type `eval_not_converging`.                                                                            |

### Round 1–2 — Same fix (current behavior)

The generator reads the latest `eval-response-<N>.md`, fixes the FAIL items in place, updates `eval-request.md`, and re-requests evaluation. This is the cheapest rung and resolves most failures (typical convergence is 2–3 rounds). No strategy change is warranted yet — the first couple of FAILs are usually ordinary bugs, not a stuck approach.

### Round 3 — Re-ground

Context may have rotted: the generator has been editing for several rounds and its working memory of the spec has drifted. Before fixing again, the generator **re-reads the full source of truth from scratch** — the spec, the design doc, and the contracts — rather than reasoning from its in-context summary. It then re-attempts the FAIL items against that fresh reading. This catches the common case where the FAIL persists because the generator has been solving the wrong problem.

### Round 4 — Different approach (fresh generator)

Re-grounding didn't converge, which suggests the generator is **anchored** on a design path that cannot satisfy the spec. The stuck generator cannot reliably unstick itself — it will keep returning to the same solution. So spawn a **fresh generator** that has none of the prior context except an explicit framing:

> The previous approach failed on **X** (cite the persistent FAIL findings). Do **not** continue that approach. Re-read the spec/design/contracts and take a **different design path**.

The fresh generator works in the **existing worktree** (the prior commits remain; it can revert or rework them). See [Spawning a Fresh Generator](#spawning-a-fresh-generator-rung-4) for the host-agnostic spawn contract.

### Round 5 — Decompose

If even a fresh approach FAILs, the story is likely **too large to land as one unit**. The generator (fresh or original) proposes a split into smaller sub-stories / sub-tasks and attempts the **smallest viable slice** — the thinnest piece that can PASS on its own. The proposed split is **surfaced**, not silently applied: write it to `eval-request.md` and the human-gate record so the human (and the autopilot) can see the story has been re-shaped. Landing one slice and deferring the rest is a legitimate outcome of this rung.

### After Round 5 — Human gate

Only once every rung has been tried does the loop escalate. This is the `eval_not_converging` escalation (`needs-human.md` + `blocked_on: human` in `status.json`; see `eval-protocol.md` → needs-human gate record). The escalation should record the **ladder history** — which rungs were attempted and why each failed — so the human inherits a genuinely-explored problem, not five identical attempts.

---

## When to Skip the Ladder

The ladder is for **convergence** failures — the generator can't get the work to PASS. Some FAILs are not convergence problems and **escalate immediately**, skipping all rungs:

- **Hard-failure / security FAIL that needs human judgment** — e.g. an auth-model gap, a data-exposure risk, or any finding whose fix requires a product/security decision the agent is not authorized to make. Trying "a different approach" on a security boundary is worse than asking. Escalate on the first such FAIL.
- **Spec contradiction** — the FAIL is caused by the spec itself being internally inconsistent or wrong. No generator strategy can fix a contradictory spec; this needs a human to amend the spec.
- **Missing external dependency / access** — the work cannot proceed without something outside the worktree (a credential, an unbuilt upstream service). Decomposing won't help.

In these cases, escalate with the appropriate type immediately and note that the ladder was deliberately skipped.

---

## State Tracking

Which rung we're on is **derived**, not free-standing — it follows the eval round count plus an explicit marker so a recovering agent (after a context reset) lands on the right rung:

- **`eval_round`** in `.dev-workflow/signals/status.json` is the primary driver (round 3 ⇒ re-ground, round 4 ⇒ fresh generator, etc.).
- **`recovery_rung`** in `status.json` records the rung explicitly — one of `same_fix` | `reground` | `fresh_generator` | `decompose` — so the rung is unambiguous even if rounds and rungs are re-tuned, and so the autopilot can read intent without re-deriving it. A fresh generator (rung 4) reads `recovery_rung` to learn it must take a different path rather than resume the stuck one.

```json
{
  "phase": 5,
  "eval_round": 4,
  "recovery_rung": "fresh_generator",
  "eval_result": "fail",
  "blocked_on": null,
  "updated_at": "2026-06-16T12:00:00Z"
}
```

The workspace owns this state and advances its own rung — the autopilot only observes it and nudges (see [Cross-References](#cross-references)). The autopilot does **not** climb the ladder on the workspace's behalf.

---

## Spawning a Fresh Generator (Rung 4)

The v1.8.0 spawn contract for the fresh generator (host-agnostic; same rules as any executor spawn):

1. **Mode:** `native-bg-subagent` — spawned via the **Agent tool** with `run_in_background: true`, **no team**. It runs as an in-process background subagent.
2. **Worktree:** it inherits the **EXISTING** worktree (`.feature-workspaces/<name>`). The prior generator's commits are present; the fresh generator may revert, rework, or build on them — but its prompt forbids resuming the stuck approach.
3. **Liveness:** it MUST pass the post-spawn liveness probe — `skills/patterns/executor/scripts/spawn-liveness-probe.sh <ws> <agent_id>`. A spawn call returning is **not** evidence the worker started; the probe confirms worktree activity, and the caller separately confirms the subagent process exists (`TaskList` shows `<agent_id>`). If the probe fails, tear down and re-spawn.
4. **Gate-and-park:** like any generator, the fresh generator **gates and parks for human input** when it hits a decision it can't resolve — it does not invent product/security answers.

The fresh generator is still a generator: the evaluator role is untouched, and the generator≠evaluator boundary is preserved across the swap.

---

## Cross-References

| Where                                                     | What it covers                                                                                                                                                                                                                                 |
| --------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `/aep-build` Phase 5                                      | Runs the multi-round gen/eval loop; this ladder governs what the generator does on each FAIL round.                                                                                                                                            |
| `eval-protocol.md` → Convergence Rules / needs-human gate | `max_rounds`, the escalation format, and the `needs-human.md` + `blocked_on` gate record the ladder feeds into.                                                                                                                                |
| `aep-autopilot` tick-protocol Step ④                      | The orchestrator observes `eval_round` / `recovery_rung`, nudges a stalled workspace, and emits the `eval_not_converging` escalation once the ladder is exhausted. It only nudges — the workspace runs its own loop and climbs its own ladder. |
| `aep-executor` `scripts/spawn-liveness-probe.sh`          | Post-spawn liveness probe the rung-4 fresh generator MUST pass.                                                                                                                                                                                |
