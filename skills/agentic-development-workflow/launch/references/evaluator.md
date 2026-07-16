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

The criteria file is **recipe-derived, then optionally ratcheted up** — not a free-form
brainstorm. Before the generator starts, read the dispatch brief's provisional verification tier
and dimension preset (/aep-gen-eval `references/verification-economics.md` → The Verification
Recipe; presets + floors in `scoring-framework.md`) and assemble
`.dev-workflow/evaluator-criteria.md` (per-workspace) so it is ready when the generator reaches
Phase 5:

- **`light`** → stop: write no criteria file (Phase 5 self-reviews).
- **`standard`** → the derived preset's dimensions, weights, and hard floors, with scale
  definitions tailored to this feature.
- **`deep`** → the derived preset with nothing de-weighted, plus the top-effort hint.

**Interactive customization only ratchets up.** With the user (when one is present), you may
**add** dimensions (Originality, Accessibility, API Design, Performance, Data Integrity, …),
**raise** thresholds, or weight dimensions the model tends to fall short on — never drop a
derived preset's hard-floor dimensions or lower a derived floor. Useful prompts:

1. Which dimensions matter most for this specific feature?
2. What does "good" look like — any concrete quality bars?
3. Where have you seen mediocre output from the model before on similar work?
4. Any hard failure conditions beyond the derived ones?

Autonomous launches (no user at the prompt) write the derived criteria as-is — that is the
deterministic policy; the recipe needs no brainstorm to be valid.

## 2. Per-mode spawn (pointer)

The generator picks the matching evaluator spawn at Phase 5 via `executor.spawn_evaluator()`:
a foreground Task subagent on Claude Code (`native-bg-subagent`/`claude-bg`), a
`codex exec --cd <abs worktree>` with the `aep-evaluator` role on Codex, or a `tmux split-window`
pane on `legacy`. Full recipes live in the /aep-executor references — do not re-spawn manually.

## 3. Evaluator bootstrap prompt template

The generator sends this when spawning the evaluator (it handles this automatically at Phase 5
— shown here for reference):

```
You are an EVALUATOR agent. Begin evaluation immediately.

Read these files:
1. .dev-workflow/evaluator-criteria.md (scoring calibration)
2. .dev-workflow/signals/eval-request.md (what to evaluate)
3. All files in openspec/changes/<change-name>/
4. .dev-workflow/contracts.md (if exists)
5. .dev-workflow/feature-verification.json (if exists)

Then:
1. Review code changes via `git diff "$BASE"...HEAD` — `$BASE` is interpolated at assembly time from the launch run's already-resolved integration branch (machine-assembled brief: the evaluator receives the resolved value, it never re-derives it)
2. Test the running application if possible
3. Score each dimension per your criteria
4. Write structured feedback to .dev-workflow/signals/eval-response-<N>.md

CRITICAL: Score honestly. Do not rationalize problems away.
Apply hard failure thresholds strictly.
Never modify verification_steps in feature-verification.json.
```

## The loop (canonical elsewhere)

The eval loop — generator writes `eval-request.md` → evaluator writes
`eval-response-<N>.md` → generator fixes → repeat until pass, up to the **tier-derived round
cap** (`light` 0 / `standard` 2 / `deep` 5, from `verification-recipe.json`; 5 with no recipe) —
is canonical in /aep-gen-eval `eval-protocol.md` and realized in /aep-build Phase 5. You do not
run it here.
