# Fable 5.1 Behavioral Re-baseline: Two Rounds, Pinned Effort, Plain Register

> **Status:** Accepted — implemented on branch `release/v4.1.0` as **v4.1.0**, sequential commits on one branch reviewed as a single PR (the v4.0.0 convention). This document **amends** [claude-5-context-engineering.md](claude-5-context-engineering.md): C1–C6 remain in force; the explicit non-goal that excluded gen-eval's prompt templates is **lifted** here, and the target model generation is declared. It changes how AEP works, so it lives in `decisions/` per the [docs routing guide](../README.md).
>
> **Sourcing note:** Anthropic's [Prompting Claude Fable 5.1](https://platform.claude.com/docs/en/build-with-claude/prompt-engineering/prompting-claude-fable-5-1) guide (beta headers dated 2026-08-18 / 2026-08-21, so it post-dates v4.0.0's 2026-08-10 release) and the bundled `claude-api` prompt-audit checklist. Field evidence: SIBYL S179 (seven evaluator rounds, $4.02, only the final round durable) and S180 (two `xhigh` evaluators per uncapped round); looplia 16:84 feature:process; the 91app-agent-platform owner's observation that eval loops "keep round-tripping on fine-grained findings without asking impact" and that the downstream `AGENTS.md` steers the main agent into small-step local fixes.

## Target generation

AEP is written for the **Claude 5 generation** (Fable 5.1 is the reference model; Opus 5 / Sonnet 5 are peers), on any host that runs those models (Claude Code, Codex, and the other Agent Skills hosts). Model names appear in this corpus only where a host recipe needs a flag value. Everything below that names a behavior names a *documented* Fable 5.1 behavior, not a guess about older models.

The v4.0.0 audit of this corpus against the prompt-audit checklist is **clean on most signals**: no thinking scaffolds, no anti-formatting rules, no compile-check phrasing, no base64 tool output, no API-layer fossils, no numeric output caps. What remains is categorical, and it is all in the execution plane.

## Diagnosis

**1. The evaluator loop buys rounds on things that do not matter.** v3.2.0 set caps of 0 / 2 / 5 by tier and let `standard` auto-escalate to `deep` on cap exhaustion — so a `standard` story could spend five evaluator rounds. v4.0.0 added impact routing, but `material` findings still bought a round (`x-aep-stages.material`: "fixed in a round"), and an evaluator that finds one new `material` item each round keeps the loop alive. Downstream, no installed version has any of the v4.0.0 controls yet (91app-agent-platform's `AGENTS.md` still says v2.1.0), so the observed behavior is the uncapped one.

**2. Evaluator effort is pinned upward, on a model where the session default is already the ceiling.** The recipe's `deep → highest` and `standard → default` were decided against Opus 4.x. On Claude Code the session default is `xhigh`; the Fable 5.1 guide says `high` is the recommended start, `xhigh`/`max` only where a measured gain exists, and that at `xhigh`/`max` a long deliverable (an eval-response is one) is drafted once in thinking and again as output. S180's doubled `xhigh` evaluator is that sentence, observed.

**3. The evaluator prompt is the corpus's loudest text and escaped the steering ratchet.** `gen-eval/references/agent-contracts.md` carries 19 all-caps MUST/NEVER/CRITICAL lines and `CRITICAL: Score honestly.` It *is* the evaluator spawn prompt (`executor/references/claude-native.md`), yet `scripts/check-steering.mjs` scans only `SKILL.md`. The prompt-audit checklist's rule for tool and role contracts: contract and mechanics stay, volume goes — an anxious prompt produces a hedging model.

**4. The workspace agent is never told the one sentence the guide calls load-bearing.** Build Phase 12 encodes "PR ready is not a stop condition" and the six legitimate stops, but the bootstrap `/aep-launch` composes says nothing about autonomy, scope, test sprawl, targeted edits, or auditing progress claims against tool results. Those are the four prompt-tunable Fable 5.1 shifts that apply to an unattended coding agent.

**5. Fossils.** `agentic-development-workflow/README.md` draws a "Sonnet 4.5 era / Opus 4.6 era" capability diagram; `workflow/references/pattern-catalog.md` routes stories to `opus` vs `sonnet` by complexity (the guide: measure low-effort Fable before building a model cascade); `/aep-workflow`'s sizing rule assumes a single context "goes lazy, biased, or off-goal" at a length the current generation sustains.

## The standard (normative, amends C1–C6)

- **F1 — Two rounds, every tier.** The evaluator round cap is **2** for `standard` and `deep`; `light` stays 0. Tiers differ in *scope* (dogfood surface, full-suite timing), *judge family* (`deep` prefers a different model family from the generator), and *pinned effort* — never in round count. There is no tier escalation as a rounds mechanism: `tier_escalated` remains in the signal contract as a deprecated, always-`false` field so v4.0.0 autopilot consumers keep parsing.
- **F2 — Round 2 is bought only by `blocking`.** `material` findings are **fix-and-attest**: the generator fixes them, records the fix with its evidence (commit, test, or probe) in an addendum to `eval-request.md`, and the derived verdict is **PASS with notes**. `polish` buys nothing (unchanged). A round-1 response with no `blocking` finding ends the loop. Round 2 exists to confirm a blocking fix, and a blocking finding still open after round 2 is the human gate (`eval_not_converging`), where the escalation *proposes* the former ladder rungs — fresh generator, decompose — as options for the human or the autopilot policy to pick. Re-grounding (re-read spec + design + contracts from scratch) is no longer a rung; it is the generator's first step before the round-2 fix.
- **F3 — Pinned evaluator effort.** Every evaluator spawn pins the mode's `high` effort. `evaluator_effort` gains the value `high`; `default` and `highest` stay in the enum as **deprecated aliases** that validators and spawn recipes read as `high`, so a recipe written by a v4.0.0 derive script still validates (this is what keeps the change a minor bump under the repo's SemVer rule). `xhigh`/`max` are never selected by the recipe; a project that has *measured* a gain may raise it in its own `policy.md`, and that measurement belongs in the layer budget box.
- **F4 — Prompt payloads are steered text.** `check-steering.mjs` scans an allowlist of reference files that are spawned as prompts (gen-eval's `agent-contracts.md`, `eval-protocol.md`, `scoring-framework.md`) with their own baseline entries. The register diet keeps every contract line and drops the volume: role tables keep their reasons, MUST lists become plain requirements, pressure sentences with no check behind them go.
- **F5 — The bootstrap carries what only the build run knows.** `/aep-launch` prepends a fixed preamble (`launch/templates/bootstrap-preamble.md`) to every workspace bootstrap: the run is unattended and the stop list is `merge-decision-cases.md`; progress claims are audited against tool results before they are reported; changes and committed tests stay within what the story asks; edits are surgical where the result is the same. The generic working agreement is *not* repeated here — the worktree inherits the project's `AGENTS.md` (see [project-convention-and-upgrade-path.md](project-convention-and-upgrade-path.md)).
- **F6 — No model-era fossils.** Capability diagrams and routing examples describe the current generation without naming retired models; the one routing example routes by *effort* on one model, not by model family.

## Amendment (2026-09-03): the corpus pass — F7 and F8

The owner asked for two things after F1–F6 landed: every skill reviewed against the Fable 5.1 guideline, and the corpus reviewed for token efficiency from the angle that the story loop re-reads the same skills on every cycle. Two rules were added and applied across all 24 skills in one pass (five groups, one editing agent each, one integration review):

- **F7 — Archaeology stays out of skill text.** Skill prose states the current rule as if it were the only rule that ever existed. Version numbers, PR numbers, incident IDs, downstream project names, "drift bug #N", "no longer", "previously" belong in `CHANGELOG.md` and `docs/decisions/`; a skill may carry a pointer to the rationale, not the narrative. Exceptions: the install pin in `/aep-onboard` Phase 1, `references/migrations.md` (a ledger by design), the pinned release in `AGENTS.md.tmpl`, `legacy` as a mode name, and a schema `description` that names a value as a deprecated alias (without the version).
- **F8 — One canonical home per protocol; every other mention is pointer-only.** Where a protocol is canonical in one file (tiers and taxonomy in gen-eval's `verification-economics.md`, the eval loop in `eval-protocol.md`, the `verification:` block in wrap's `convergence.md`, signals in launch's `signals-spec.md`, stop conditions in build's `merge-decision-cases.md`, `$BASE` in git-ref, journey authoring in e2e's `layer-gate-loop.md`, executor modes in `backends.md`, the tick in `tick-protocol.md`), every other file carries at most the one or two lines its reader must act on plus the pointer. Rendered templates stay self-contained. When two files disagree, the canonical home wins and the other is fixed.

**Measured (authored prose, generated copies excluded):**

| Measure | Before | After |
| --- | ---: | ---: |
| Authored skill prose (md + tmpl) | 1,049,687 B | 1,022,920 B (−2.6%) |
| Story-loop path (dispatch, launch, build, wrap, autopilot, gen-eval, executor) | 469,111 B | 447,497 B (−4.6%) |
| `SKILL.md` total lines | 5,261 | 5,179 |
| Archaeology phrases in skill text | 61 | 12 (all ordinary English: "previously impossible", "no longer appears") |
| All-caps MUST / NEVER / ALWAYS / CRITICAL / IMPORTANT | 51 | 15 (nudge payloads delivered verbatim, bash `echo` text, table column labels) |
| Steering baseline: negation lines / hard imperatives | 75 / 6 | 63 / 0 |

**Contradictions the pass surfaced and fixed toward canon (eleven):** eval-protocol's round-budget table and escalation example still said five rounds; `review-trigger.md` escalated after "5 rounds" and "3 consecutive rounds"; launch's `evaluator.md` said `deep` = 5; dispatch's `workflow-mode.md` said `deep` runs "the full cap at top effort"; validate's `protocol-specs.md` listed a gate status `pending` that the vocabulary does not have and referenced an `estimated_size` field that exists nowhere; `drift-facts.md` said the coherence check "belongs in `/aep-validate`… until then" when validate Step 0 already runs it; `signals-spec.md` sent the eval-response format to the per-workspace criteria file instead of `eval-protocol.md`; `patterns/README.md` named `.agents/skills` the canonical install root against the per-agent install contract; autopilot's SKILL declared "exactly two prohibitions" and stated a third; `orchestration-learning.md` carried an example that "passed after 3 rounds".

**The byte reduction is modest by design.** The pass removed dated instructions, restatement, and contradiction; it did not remove phases, gates, schemas, exact commands, or examples that pin a format, and the phase structure of the five largest `SKILL.md` files stays (horizon, below). The token win that matters is in the loop: build's `SKILL.md` fell from 37.6 KB to 34.5 KB and now points at gen-eval, e2e, executor, and its own references instead of re-teaching them; gen-eval's `verification-economics.md` fell from 39.5 KB to 34.2 KB and no longer carries the `verification:` block, the worked incidents, or the anti-pattern list.

**Tooling fact the pass surfaced, recorded for editors:** `scripts/build-skills.sh` materializes a `_shared/` file into a skill only while that skill's `SKILL.md` mentions the literal path; trimming the mention un-declares the copy, `--check` reports the managed-file set as stale, and the next full build deletes the file — which breaks any sibling script that imports it (`validate-state.mjs` and `validate-signal.mjs` import `json-schema.mjs`). Two groups hit this and restored the mention. The rule is now in `project-convention/skills.md`.

**Follow-ups this pass did not take (each is a contract change):**

- `findings-format.md` grades consolidated findings `Blocking / Important / Minor` while every eval-response grades `blocking / material / polish` (`finding_impact`). Two near-synonym scales in one skill; reconciling them touches the `(aep-vocab)` listings and validate's consolidation format.
- `launch/references/evaluator.md` no longer carries a copy of the evaluator prompt (pointer to `agent-contracts.md`); the criteria-file assembly it still owns could move to the recipe emitter in a later release.
- `map/SKILL.md`'s `topology.routing.retry_routing` default (`same agent 2x → fresh agent 1x → human`) is a story-routing field, not the eval ladder, and was left; it reads like the old ladder and may confuse a reader.

## What is deliberately not done (horizon)

- **Phase de-prescription of the large SKILL.md files** (scaffold, autopilot, build, dispatch, git-ref). The guide says over-prescriptive skills reduce Fable 5.1 output quality *and* says to A/B before removing. v4.0.0's amendment records that no behavior eval exists (parity was a dry-run inventory). Removing phases without one would be the guide's own anti-pattern. Trigger: a behavior eval that runs one story end-to-end under both corpora.
- **PR #17 (walking skeleton as a living artifact).** Its forcing function (a currency reader for the skeleton) is not superseded by `drift-facts.md`'s five sources. It needs a rebase against the v4 vocabulary or an explicit close; that is the owner's call, not this release's.
- **Effort override in `policy.md`.** F3 names the escape hatch but ships no field; the first project that measures a gain defines it.

## Assumption stated for the record

The owner described the execution plane as "the main agent finishes the product-context update and writes the OpenSpec change, and only then dispatches." The corpus already does this: `/aep-dispatch` creates the change with its context package on the integration branch, `/aep-design` runs there, and `/aep-launch` is the handoff into a worktree. This release does not reorder dispatch and design; "dispatch" in the owner's sentence is read as the handoff.

## Forcing functions

| Surface                                                                                    | Change                                                                                                                                       |
| ------------------------------------------------------------------------------------------ | -------------------------------------------------------------------------------------------------------------------------------------------- |
| `_shared/references/aep-vocabulary.schema.json`                                            | `evaluator_effort` gains `high`; `default`/`highest` marked deprecated aliases; `finding_impact.material` reads "fix-and-attest, buys no round" |
| `gen-eval/references/verification-economics.md`, `eval-protocol.md`, `recovery-ladder.md` | Tier table (cap 2, judge family, pinned `high`); stop table; compressed ladder; human-gate options                                           |
| `gen-eval/references/verification-recipe.schema.json`                                      | `max_rounds` description; `evaluator_effort` enum                                                                                            |
| `e2e-skill-scaffolding/templates/derive-verification-recipe.sh.tmpl` + fixtures            | `MAX_ROUNDS` 2 for `standard`/`deep`; `EFFORT="high"`                                                                                        |
| `build/SKILL.md` Phase 5, `launch/SKILL.md` + `references/evaluator.md`, `executor/references/backends.md` | Loop cap, no auto-escalation, pinned effort hint                                                                              |
| `launch/templates/bootstrap-preamble.md` + `launch/SKILL.md` Step 3                        | F5 preamble                                                                                                                                  |
| `scripts/check-steering.mjs` + `evals/steering-baseline.json`                              | F4 allowlist; re-recorded after the diet                                                                                                     |
| `agentic-development-workflow/README.md`, `workflow/SKILL.md`, `workflow/references/pattern-catalog.md` | F6                                                                                                                              |

## Anti-patterns this prevents

- **Rounds as a mood.** A loop that continues because the evaluator found *something*, rather than because a named acceptance criterion is still violated.
- **Effort inherited from the terminal.** An evaluator that runs at whatever the operator's session happens to be set to.
- **Shouting at the judge.** A contract written in capitals because an older model under-triggered on it.
- **The silent bootstrap.** An unattended agent that stops to ask "shall I merge?" because nobody told it nobody is watching.

## References

- [Prompting Claude Fable 5.1](https://platform.claude.com/docs/en/build-with-claude/prompt-engineering/prompting-claude-fable-5-1) — Anthropic; sections *Finish the whole task*, *Keep changes and tests to what the task asks for*, *Consider all effort levels*, *Leave room for long outputs at xhigh and max effort*, *Prefer targeted edits over whole-file rewrites*.
- [claude-5-context-engineering.md](claude-5-context-engineering.md) — C1–C6, amended here; its gen-eval non-goal lifted.
- [verification-economics.md](verification-economics.md) — the tiers this document re-caps; its 2026-08-08 impact-routing amendment is the mechanism F2 tightens.
- [project-convention-and-upgrade-path.md](project-convention-and-upgrade-path.md) — the downstream `AGENTS.md` replacement that handles the main-agent half of the local-optimization complaint.
