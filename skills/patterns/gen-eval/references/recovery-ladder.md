# Change-Strategy Recovery Ladder

When the Phase 5 gen/eval loop returns a `blocking` finding, the generator fixes it and buys one more evaluator round. **The loop is two rounds for every tier that spawns an evaluator** (`light` 0 / `standard` 2 / `deep` 2, from `verification-recipe.json`; 2 with no recipe — v4.1.0, F1/F2 of `docs/decisions/fable-5-1-behavioral-rebaseline.md`). Round 2 exists to confirm the fix; a `blocking` finding still open after it is the human gate. The failure mode the old five-round ladder guarded against — **strategy stagnation**, the generator re-applying an approach that already failed — is now handled at the gate: the strategy changes are *proposed* there as options, not spent as automatic rounds. Field evidence for the change: SIBYL S179 ran seven evaluator rounds at $4.02 with only the final round durable; loops kept round-tripping on findings nobody had asked to weigh.

> **The taxonomy step comes first — at every FAIL, before choosing a rung.** The ladder is repair machinery for `product-defect` findings only. Classify each FAIL finding per `verification-economics.md` → Failure Taxonomy (evaluator-authored, evidence-gated): `environment` → ops checklist, zero rounds spent; `harness-flake` → quarantine + harness story; `scope` → `/aep-reflect` re-slicing; unbuilt in-repo dependency → `/aep-dispatch` re-ordering. Only `product-defect` climbs — and only at `Impact: blocking`. `material` findings are fix-and-attest (fixed, evidenced in the eval-request addendum, no round bought); `polish` is recorded and surfaced.

> The evaluator never climbs this ladder. Generator≠evaluator separation still holds: the evaluator scores; the generator (or a fresh generator) is the only role that "tries a new approach." A re-grounded read, a fresh generator, and a decomposition are all generator-side moves.

---

## Table of Contents

1. [The Ladder](#the-ladder)
2. [When to Skip the Ladder](#when-to-skip-the-ladder)
3. [State Tracking](#state-tracking)
4. [Spawning a Fresh Generator (Gate Option A)](#spawning-a-fresh-generator-gate-option-a)
5. [Cross-References](#cross-references)

---

## The Ladder

Two rungs run inside the loop; two strategy changes live at the gate. The **shape** is what matters: each step is a strictly larger change than the one before it, and the expensive ones need a decision, not a counter.

| When            | Rung                   | Strategy                                                                                                                                         |
| --------------- | ---------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------ |
| Round 1         | **Same fix**           | Same generator fixes the `blocking` findings in place.                                                                                           |
| Before round 2  | **Re-ground**          | Same generator re-reads the FULL spec + design + contracts **from scratch**, then fixes and re-requests. Always — not only when stuck.            |
| After round 2   | **Human gate**         | A `blocking` finding still open → escalate with type `eval_not_converging`. The record carries the two-round history and proposes the options below. |
| Gate option A   | **Different approach** | Spawn a **fresh generator** told "the previous approach failed on X; take a different design path." Picked by the human or the autopilot policy. |
| Gate option B   | **Decompose**          | Split the story; attempt the **smallest viable slice**; surface the split. Picked by the human or the autopilot policy.                           |

### Round 1 — Same fix

The generator reads `eval-response-1.md`, fixes the `blocking` findings in place, fixes and attests any `material` findings, updates `eval-request.md` with what changed and the evidence, and re-requests evaluation. Most blocking findings are ordinary bugs and resolve here.

### Before round 2 — Re-ground

Context may have rotted: the generator has been editing for a while and its working memory of the spec has drifted. Before the round-2 fix, the generator **re-reads the full source of truth from scratch** — the spec, the design doc, and the contracts — rather than reasoning from its in-context summary, then fixes against that fresh reading. This catches the common case where a finding persists because the generator has been solving the wrong problem. It costs a read, not a round, so it is unconditional.

### After round 2 — Human gate

A `blocking` finding still open after round 2 escalates with type `eval_not_converging` (`needs-human.md` + `blocked_on: human` in `status.json`; see `eval-protocol.md` → needs-human gate record). The record states which finding survived, what each round changed, and the two options below with the generator's recommendation. Under autopilot the story is parked and the tick continues with other work; the autopilot policy may pick an option where the project has authorized it, otherwise the human does.

### Gate option A — Different approach (fresh generator)

Two rounds on the same finding suggest the generator is **anchored** on a design path that cannot satisfy the spec. The stuck generator cannot reliably unstick itself — it will keep returning to the same solution. So spawn a **fresh generator** that has none of the prior context except an explicit framing:

> The previous approach failed on **X** (cite the surviving `blocking` finding). Do **not** continue that approach. Re-read the spec/design/contracts and take a **different design path**.

The fresh generator works in the **existing worktree** (the prior commits remain; it can revert or rework them) and gets its own two-round loop. See [Spawning a Fresh Generator](#spawning-a-fresh-generator-gate-option-a) for the host-agnostic spawn contract.

### Gate option B — Decompose

If the finding points at a story that is **too large to land as one unit**, the generator (fresh or original) proposes a split into smaller sub-stories / sub-tasks and attempts the **smallest viable slice** — the thinnest piece that can PASS on its own. The proposed split is **surfaced**, not silently applied: write it to `eval-request.md` and the human-gate record so the human (and the autopilot) can see the story has been re-shaped. Landing one slice and deferring the rest is a legitimate outcome.

---

## When to Skip the Ladder

This section is the **typed taxonomy step** (`verification-economics.md` → Failure Taxonomy), run **mandatorily at every FAIL before choosing a rung** — not a prose bullet to recall mid-FAIL. The ladder is for **convergence** failures on `product-defect` findings — the generator can't get the work to PASS. Every other class routes off the ladder immediately:

- **Hard-failure / security FAIL that needs human judgment** (`product-defect`, escalation preserved) — e.g. an auth-model gap, a data-exposure risk, or any finding whose fix requires a product/security decision the agent is not authorized to make. Trying "a different approach" on a security boundary is worse than asking. Escalate on the first such FAIL — the taxonomy adds routing, it never removes an escalation.
- **Spec contradiction** (`scope`) — the FAIL is caused by the spec itself being internally inconsistent or wrong. No generator strategy can fix a contradictory spec; routes to `/aep-reflect` re-slicing with a human acknowledgment on the gate record.
- **Missing external dependency / access** (`environment`) — the work cannot proceed without something outside the worktree (a credential, a wrong account, an unreachable target). Decomposing won't help; claimable **only** via a named preflight/probe refusal tag, and routed to the ops checklist — never a code story, never a rung, never an evaluation round.
- **Test machinery misbehaving** (`harness-flake`) — race, port collision, known-red baseline; claimable only with world-derivable reproduction evidence ratified by wrap/`aep-reflect`, then quarantined + a harness story. The product gate re-runs after quarantine.
- **Unbuilt in-repo dependency** — a sequencing problem, not an ops one: route to `/aep-dispatch` re-ordering.

In these cases, escalate/route with the appropriate type immediately and note that the ladder was deliberately skipped. **Without qualifying evidence, a FAIL is `product-defect` and climbs** — the generator never labels its own failure into a cheaper class.

---

## State Tracking

Which rung we're on is **derived**, not free-standing — it follows the eval round count plus an explicit marker so a recovering agent (after a context reset) lands on the right rung:

- **`eval_round`** in `.dev-workflow/signals/status.json` is the primary driver (round 1 ⇒ same fix; round 2 ⇒ the re-grounded fix is under evaluation; a `blocking` finding after round 2 ⇒ gate).
- **`recovery_rung`** in `status.json` records the rung explicitly — one of `same_fix` | `reground` | `fresh_generator` | `decompose`. The first two are the in-loop rungs; the last two are written only when the gate option was picked, so a fresh generator reads `recovery_rung: fresh_generator` and knows it must take a different path rather than resume the stuck one.

```json
{
  "phase": 5,
  "eval_round": 2,
  "recovery_rung": "reground",
  "eval_result": "fail",
  "blocked_on": null,
  "updated_at": "2026-06-16T12:00:00Z"
}
```

The workspace owns this state and advances its own rung — the autopilot only observes it and nudges (see [Cross-References](#cross-references)). The autopilot does **not** climb the ladder on the workspace's behalf, and it does not spend a gate option without the policy or the human picking it.

---

## Spawning a Fresh Generator (Gate Option A)

The spawn contract for the fresh generator (host-agnostic; same rules as any executor spawn):

1. **Mode:** `native-bg-subagent` — spawned via the **Agent tool** with `run_in_background: true`, **no team**. It runs as an in-process background subagent.
2. **Worktree:** it inherits the **EXISTING** worktree (`.feature-workspaces/<name>`). The prior generator's commits are present; the fresh generator may revert, rework, or build on them — but its prompt forbids resuming the stuck approach.
3. **Liveness:** it MUST pass `/aep-executor`'s `scripts/spawn-liveness-probe.sh <ws> <worker_handle>`. A spawn call returning is **not** evidence the worker started; the probe confirms worktree activity, and the caller separately confirms the worker with the backend-specific host tool. If the probe fails, tear down and re-spawn with the current host's fallback mode.
4. **Gate-and-park:** like any generator, the fresh generator **gates and parks for human input** when it hits a decision it can't resolve — it does not invent product/security answers.

The fresh generator is still a generator: the evaluator role is untouched, and the generator≠evaluator boundary is preserved across the swap.

---

## Cross-References

| Where                                                     | What it covers                                                                                                                                                                                                                       |
| --------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `/aep-build` Phase 5                                      | Runs the two-round gen/eval loop; this ladder governs what the generator does on a `blocking` finding and what the gate proposes after round 2.                                                                                      |
| `eval-protocol.md` → Convergence Rules / needs-human gate | `max_rounds`, the derived verdict (only `blocking` buys a round), the escalation format, and the `needs-human.md` + `blocked_on` gate record the ladder feeds into.                                                                   |
| `aep-autopilot` tick-protocol Step ④                      | The orchestrator observes `eval_round` / `recovery_rung`, nudges a stalled workspace to work its remaining round, and emits the `eval_not_converging` escalation once round 2 is spent with a `blocking` finding open. It only nudges. |
| `aep-executor` `scripts/spawn-liveness-probe.sh`          | Post-spawn liveness probe the gate-option-A fresh generator MUST pass.                                                                                                                                                               |
