# Generator/evaluator findings — this example does NOT pass

Two independent evaluation rounds have been run against the generated brief in
this directory, following `/aep-gen-eval` (separate evaluator agent, no access
to the generator's reasoning, ground truth = the consumer's raw plan file).

**Both rounds returned FAIL.** This artifact is committed as evidence, not as a
model to copy. Read this file before treating anything here as exemplary.

| Round | Verdict | Shape of the failure                                                                                                                             |
| ----- | ------- | ------------------------------------------------------------------------------------------------------------------------------------------------ |
| 1     | FAIL    | The prose needed facts the derivation had not produced, so the agent supplied them from diagram labels, ambient knowledge, or manual counting.   |
| 2     | FAIL    | The facts now exist and are cited — but several citations **resolve without bearing the claim**, plus scope errors and pipeline-hygiene defects. |

Round 1 produced decision-doc revision 9 (D9: census, claim-level audit,
predicates, coverage declaration). Those mechanisms work — the round-1 defects
are now unauthorable, and the round-1 prose is rejected by the current audit.
Round 2 is a different failure, on the far side of that fix.

## Round 2 — blocking

**B1 · A queued stage is described as unstarted while its only task shipped.**
"None has started; their tasks are written and waiting" is cited to
`by_status.pending by_status.deferred`, and the disclosure beneath it names the
dynamic source plane — layer 41, whose single story completed hours before
generation. The page states four sections apart both that the stage has not
started and that its task closed. The cited counts are real and say nothing
about layer 41.

**B2 · A temporal claim with no fact behind it.** "closed by your decision last
week" — the decision is dated the same day the brief was generated, and its own
anchor prints that date directly beneath the words "last week". None of the
three cited paths carries a date.

**B3 · The delta baseline was destroyed by hand, making "first brief" true.**
The committed manifest recorded a prior generation (`2026-07-25T0745Z` at
`65e359c6`). Regenerating by `rm` + `cp` rather than through `assemble.mjs`
bypassed the `history[]` append, so `delta.first_run` became true and the brief
says there is nothing to compare against. Checklist P2 — "the prior generation
moved into `history[]`" — fails.

**B4 · The delivered file fails the skill's own P0 audit; the diagrams
contradict the prose.** The audit was only ever run on the authored HTML, never
on the assembled file. Run against the delivered artifact it raises 34
`claims/uncited-assertion` receipts, all in injected diagram chrome — and three
of those captions carry numbers that disagree with the prose beside them:

| Diagram caption | Prose / facts      | Cause                                                                    |
| --------------- | ------------------ | ------------------------------------------------------------------------ |
| `18 units`      | `21 code units`    | the diagram counts after R1/R2 reduction; neither side says so           |
| `30 of 44`      | `31 of 44`         | the code graph is one commit behind the facts (`792208b0` vs `60f4d754`) |
| `8 net-new`     | `7 new subsystems` | same commit skew                                                         |

## Round 2 — important

- **The wrong fact is bound to a correct sentence.** "4 stages passed their
  scripted checks … the newsroom stage is not among them" binds
  `gates.by_status.scripted_passed` (which counts 4 _including_ the newsroom)
  where it means the 3 capabilities carrying `tense: EXP`. The right fact exists
  and was not used.
- **A scoped measurement promoted to an unscoped claim.** "7 subsystems inside a
  single existing unit, across 84 files" is measured _within_ `do-agent`.
  Unfiltered, those concepts touch 128 files across 3–8 units each, and 15 of 21
  pairs are disjoint rather than 17. The conclusion ("not tangled, divided
  cleanly") is stronger than repo-wide data supports.
- **A citation that resolves but does not bear its sentence.**
  `structure.options.0.cleanest_seam` names one module with no size field; the
  sentence built on it names two modules and compares their size. Both halves
  happen to be true, which is the failure mode exactly — it reads verified.
- **Option display order contradicts the stated ranking**, and every `Costs`
  cell is `data-authored` with no number, so "costs in measured terms" is unmet
  across the whole set.
- **A dependency is described backwards.** The supervisor repair is a declared
  dependency _of_ the canary repair, so it sits in front, not behind; the
  recommended reset-and-dispatch is not executable as written.

## What this says about the design

Mechanism 2 of D9 (claims bind, not numbers) did what it was built to do: no
assertion ships without a citation. Round 2 shows its limit, and the evaluator
put it exactly —

> A citation that resolves but does not support the claim is worse than no
> citation, because it looks verified.

Static checking can enforce that a path resolves. It cannot enforce that the
path _bears_ the sentence. That gap is the open design question; it is recorded
here rather than patched, because two rounds of iteration have now produced two
different failure modes and a third round of the same shape is not evidence of
convergence.

## Immediately actionable, independent of that question

1. Audit the **delivered** file, not only the authored one — Phase 3 currently
   runs before Phase 4 injects, and nothing re-checks after.
2. Derive the code graph and the facts at the **same commit**, and fail the run
   when they diverge.
3. Regenerate only through `assemble.mjs` so `history[]` survives; never `rm` a
   brief by hand.
4. Label diagram captions with their own scope (`18 of 21 units after reduction`).
