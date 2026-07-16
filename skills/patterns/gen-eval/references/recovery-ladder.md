# Change-Strategy Recovery Ladder

When the Phase 5 gen/eval loop FAILs, the default behavior is for the **same generator to retry the same way** — fix the FAIL items, re-request evaluation, repeat. After `max_rounds` (tier-derived — `light` 0 / `standard` 2 / `deep` 5, from `verification-recipe.json`; default 5 with no recipe) it escalates to a human. The failure mode this guards against is **strategy stagnation**: the generator keeps applying the approach that already failed, burning rounds without exploring a genuinely different path.

> **The taxonomy step comes first — at every FAIL, before choosing a rung.** The ladder is repair machinery for `product-defect` findings only. Classify each FAIL finding per `verification-economics.md` → Failure Taxonomy (evaluator-authored, evidence-gated): `environment` → ops checklist, zero rounds spent; `harness-flake` → quarantine + harness story; `scope` → `/aep-reflect` re-slicing; unbuilt in-repo dependency → `/aep-dispatch` re-ordering. Only `product-defect` climbs.

> **The taxonomy step comes first — at every FAIL, before choosing a rung.** The ladder is repair machinery for `product-defect` findings only. Classify each FAIL finding per `verification-economics.md` → Failure Taxonomy (evaluator-authored, evidence-gated): `environment` → ops checklist, zero rounds spent; `harness-flake` → quarantine + harness story; `scope` → `/aep-reflect` re-slicing; unbuilt in-repo dependency → `/aep-dispatch` re-ordering. Only `product-defect` climbs.

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

Round numbers are tunable per project; the **shape** is what matters — each rung is a strictly larger change of strategy than the one below it. **Rungs key to position relative to the tier's round cap, not absolute round numbers**: under `standard` (cap 2), exhausting the cap on a genuine `product-defect` auto-escalates the story once to `deep` (`tier_escalated: true`) and the ladder continues from where it left off — round 3 of the escalated story is the re-ground rung, exactly as below. The absolute numbers below are the `deep` / no-recipe (cap 5) rendering.

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
3. **Liveness:** it MUST pass `/aep-executor`'s `scripts/spawn-liveness-probe.sh <ws> <worker_handle>`. A spawn call returning is **not** evidence the worker started; the probe confirms worktree activity, and the caller separately confirms the worker with the backend-specific host tool. If the probe fails, tear down and re-spawn with the current host's fallback mode.
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
