# Evaluator Setup (launch-specific)

A separate **evaluator** agent independently reviews the generator's work. Generation and
evaluation are split so the evaluator can be calibrated toward skepticism (agents asked to
grade their own work rate it positively even when it is mediocre). The generator spawns the
evaluator at **Phase 5** (after implementation is complete), via `executor.spawn_evaluator()`
— you do **not** spawn it at launch. Per-mode evaluator spawn recipes live in the
/aep-executor references; the eval loop, scoring dimensions, and contracts are canonical in
/aep-gen-eval (`scoring-framework.md`, `agent-contracts.md`, `eval-protocol.md`). This file
holds only the launch-time setup.

Source: [Harness Design for Long-Running Application Development](https://www.anthropic.com/engineering/harness-design-long-running-apps)
— evaluation is sequential (build first, then evaluate), which is why the evaluator is a
Phase-5 spawn, not a launch-time one.

## 1. Assemble the criteria from the verification recipe (at launch time)

The criteria file is **recipe-derived, then optionally ratcheted up**. Before the generator
starts, read the dispatch brief's provisional verification tier and dimension preset (/aep-gen-eval `references/verification-economics.md` → The Verification
Recipe; presets + floors in `scoring-framework.md`) and assemble
`.dev-workflow/evaluator-criteria.md` (per-workspace) so it is ready when the generator reaches
Phase 5:

- **`light`** → stop: write no criteria file (Phase 5 self-reviews).
- **`standard`** → the derived preset's dimensions, weights, and hard floors, with scale
  definitions tailored to this feature.
- **`deep`** → the derived preset with nothing de-weighted, plus the cross-family judge
  preference. The effort hint is the pinned `high` for every evaluator tier.

**Interactive customization only ratchets up.** With the user (when one is present), you may
**add** dimensions (Originality, Accessibility, API Design, Performance, Data Integrity, …),
**raise** thresholds, or weight dimensions the model tends to fall short on — never drop a
derived preset's hard-floor dimensions or lower a derived floor. Useful prompts:

1. Which dimensions matter most for this specific feature?
2. What does "good" look like — any concrete quality bars?
3. Where have you seen mediocre output from the model before on similar work?
4. Any hard failure conditions beyond the derived ones?

Autonomous launches (no user at the prompt) write the derived criteria as-is.

## 2. Per-mode spawn (pointer)

The generator picks the matching evaluator spawn at Phase 5 via `executor.spawn_evaluator()`:
a foreground Task subagent on Claude Code (`native-bg-subagent`/`claude-bg`), a
`codex exec --cd <abs worktree>` with the `aep-evaluator` role on Codex, or a `tmux split-window`
pane on `legacy`. Full recipes live in the /aep-executor references.

## 3. Evaluator prompt (canonical elsewhere)

The evaluator prompt the generator sends at Phase 5 is canonical in /aep-gen-eval
`references/agent-contracts.md` → Evaluator Prompt (Code Quality); build Phase 5 composes it
with the workspace paths. Launch owns one input to it: the brief is machine-assembled, so the
diff base the evaluator reviews (`git diff "$BASE"...HEAD`) is the launch run's already-resolved
integration branch, interpolated into the prompt — the evaluator receives a literal ref, never a
name to resolve.

## The loop (canonical elsewhere)

The eval loop — generator writes `eval-request.md` → evaluator writes
`eval-response-<N>.md` → generator fixes → repeat up to the **tier-derived round cap**
(`light` 0 / `standard` 2 / `deep` 2, from `verification-recipe.json`; 2 with no recipe, and
round 2 is bought only by a `blocking` finding) — is canonical in /aep-gen-eval
`eval-protocol.md` and realized in /aep-build Phase 5. It does not run here.
