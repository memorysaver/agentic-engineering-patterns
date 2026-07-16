# Workflow Modes — full vs light

**Canonical home for the full/light workflow-mode criteria.** `/aep-design` selects the mode, and
`/aep-launch` and `/aep-build` honor it (they point here). Pick the mode before designing.

## Full mode (default)

All phases run, plus a **separate evaluator agent** that independently reviews the generator's work.
Phase 3 (Design Review) runs. Use full mode when the work is at the edge of what the model does
reliably on its own.

## Light mode

Simplified flow: **no evaluator**, and Phase 3 (Design Review) is skipped. Use light mode for small,
low-risk changes the model handles reliably in one pass.

## Selection criteria

| Signal          | Full mode                       | Light mode                          |
| --------------- | ------------------------------- | ----------------------------------- |
| Task count      | 3+ tasks in `tasks.md`          | 1–2 tasks                           |
| Surface         | UI-heavy or security-sensitive  | simple CRUD, config change          |
| Risk / novelty  | at the edge of model capability | well-worn, low-risk (small bug fix) |
| Evaluator agent | yes (separate reviewer)         | no (self-review)                    |
| Phase 3 review  | runs                            | skipped                             |

When the signals split, prefer full mode — the cost of an unneeded review is lower than the cost of
an unreviewed complex change.

## Recording the choice so launch and build honor it

Record the mode in the OpenSpec change's `design.md` — e.g. a `**Workflow mode:** light` line under
key decisions. The change is committed to `$BASE` (Commit step), so the `/aep-launch` and `/aep-build`
sessions created from `$BASE` read the mode from `design.md` and act on it: full mode sets up the
evaluator and runs every phase; light mode skips the evaluator and the review/dogfood phases.

## Eval-loop behavior is subsumed by the verification tier

Light mode's **eval-loop half is no longer its own toggle**: in product-cycle mode, Light mode
selects **`verification_tier: light`** (`/aep-gen-eval` → `references/verification-economics.md`,
glossary: **Verification Tier**) — the tier then governs the Phase 5 loop (0 rounds, self-review),
dogfood scope, and the launch criteria file through the one derivation everything else uses. The
selection-criteria table above (task count, surface, risk) likewise no longer acts as an
independently-consulted depth switch at launch — those signals are **inputs to the dispatch-time
tier derivation**. What Light mode still owns outright is the design-time half: skipping Phase 3
(Design Review) and the build-phase skips (`/aep-build` Phase 6B/7/8 Light-mode skips). The
binding re-derivation at Phase 5 entry can still **upgrade** a light story whose actual diff
touches `sensitive_paths` or referee assets — mode is a plan, the diff is the fact. Standalone
mode (no `product-context.yaml`, no derivation): this file's full/light choice keeps its original
eval-loop meaning (full → run the loop; light → skip it), which maps to `deep`/`light` fail-open
defaults.

## Tuning principle — re-evaluate with each model upgrade

> "Every component in a harness encodes an assumption about what the model can't do on its own. Those
> assumptions deserve stress-testing."
> — Anthropic, ["Harness Design for Long-Running Application Development"](https://www.anthropic.com/engineering/harness-design-long-running-apps)

With each model upgrade, re-evaluate which phases still earn their place. A capability that once
justified full mode may become something the model does reliably on its own, shifting more work into
light mode.
