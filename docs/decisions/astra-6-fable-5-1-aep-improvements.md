# AEP v4.1.0 review for Astra 6 and Fable 5.1z

AEP v4.1.0 limits evaluator work and reduces repeated instructions. The next step is to measure how its execution plane works on the requested models.

**Status:** Proposed. This document records review findings and suggested changes. It does not change a skill contract or select a new default model.

**Review date:** 2026-09-05. **Baseline:** [v4.1.0](https://github.com/memorysaver/agentic-engineering-patterns/releases/tag/v4.1.0), published 2026-09-03, at commit `acf03fc41fe25faf67905c9004451702b7986dd8`. The remote `main` branch, release tag, local checkout, and marketplace version agreed at review time.

**Recommendation:** Keep the story map, interface contracts, worktrees, human gates, and bounded evaluation. First close the gap between declared policy and emitted commands. Then use measured story runs to reduce instruction load and tune effort. A stronger model is a reason to test each harness rule.

## Model evidence and limits

“Astra 6” resolves to OpenAI's **GPT-6 Astra**, API ID `gpt-6-astra`. Its documented effort levels are `low`, `medium`, `high`, `xhigh`, and `max`. Its context window is 1,050,000 tokens. These are API properties; an installed coding host can expose a different set of controls. [OpenAI model page](https://developers.openai.com/api/docs/models/gpt-6-astra).

The exact name **Fable 5.1z** was searched. The official pages retrieved for this review list **Claude Fable 5.1**, API ID `claude-fable-5-1`. They do not establish a separate `5.1z` identity. The Fable recommendations below are conditional on that documented model. Before a run labeled `5.1z`, record its actual provider ID and any source for its differences. Do not silently record a Fable 5.1 run as 5.1z. [Anthropic model overview](https://platform.claude.com/docs/en/models/overview).

The following facts guide the proposals. The proposed AEP changes are inferences from these facts and the repository review, not provider claims about AEP.

| Target | Documented behavior relevant to AEP | Implication to test |
| --- | --- | --- |
| GPT-6 Astra | It can ask for clarification more often, respond strongly to skill instructions, delegate less than desired, and test small changes too broadly. | Test authorization conflicts, unnecessary stops, useful delegation, and verification scope. [Astra guidance](https://developers.openai.com/api/docs/guides/latest-model?model=gpt-6-astra) |
| Claude Fable 5.1, conditional reference for 5.1z | Anthropic recommends starting at `high` and testing other effort levels. It also documents sparse progress updates, serial tool calls, reduced retrieval at low effort, and loss of detail in client compaction summaries. | Test effort against outcomes, independent tool batching, source retrieval, progress visibility, and recovery. [Fable prompting guide](https://platform.claude.com/docs/en/build-with-claude/prompt-engineering/prompting-claude-fable-5-1) |
| Model plus host | Agent evaluation must inspect the final environment as well as the run record. Repeated trials and stable starting environments matter. | Grade completed stories and retained constraints. Static package checks cannot establish execution quality. [Anthropic agent evaluation guide](https://www.anthropic.com/engineering/demystifying-evals-for-ai-agents) |

No paired Astra/Fable benchmark was run for this review. This document makes no claim that one model is better or cheaper for AEP. Provider capability claims do not establish performance in a downstream project.

## What v4.1.0 already does well

The [Fable re-baseline decision](fable-5-1-behavioral-rebaseline.md) and the current [evaluation protocol](../../skills/patterns/gen-eval/references/eval-protocol.md) already cap `standard` and `deep` at two rounds. Only a blocking finding buys round 2. Material findings require a fix and evidence. Keep this limit while testing improvements.

The [bootstrap preamble](../../skills/agentic-development-workflow/launch/templates/bootstrap-preamble.md) already addresses completion, scope, test growth, and evidence for progress claims. The [instruction-file template](../../skills/project-setup/onboard/templates/AGENTS.md.tmpl) adds the general working agreement. Extend these canonical files only when a case shows a missing rule.

The [executor](../../skills/patterns/executor/SKILL.md) already separates host transport from story state. The [verification economics](../../skills/patterns/gen-eval/references/verification-economics.md) already uses risk tiers, real-environment evidence, and a preference for another judge family on `deep` work. Preserve those contracts. Measure whether the preference improves defect detection.

## Review findings

The review covered release metadata, all 24 skill entrypoint sizes, the instruction templates, the story execution path, executor recipes, evaluation contracts, and existing checks. It was a source review with static and fixture checks, not a runtime audit of every backend or the companion apps.

| Priority | Finding in v4.1.0 | Evidence and consequence |
| --- | --- | --- |
| P0 | The behavioral evidence does not cover the requested models. | [Routing observations](../../evals/skill-routing-observations.json) record 40 selections on one Sonnet alias, with no exact serving snapshot. [Behavior parity](../../evals/skill-behavior-parity.json) records reviewed scenario dry-runs. Neither measures a story completed by Astra or Fable. |
| P0 | Pinned evaluator effort is not explicit in the Codex command. | [Backend policy](../../skills/patterns/executor/references/backends.md) requires `high`, but permits a fixed profile to ignore the hint. The [Codex `spawn_evaluator` command](../../skills/patterns/executor/references/codex-native.md) supplies neither a model nor a reasoning-effort setting. This leaves a recipe reader to translate the policy. Actual runtime effort was not observed. |
| P1 | Typed state exists, but producer validation remains optional. | [State protocol](../../skills/patterns/autopilot/references/state-schema.md) explicitly describes validation as on demand. Its write example renames the candidate without validating it first. A valid schema alone cannot prevent publication of invalid state. |
| P1 | Material-finding closure is a prose contract. | [Eval protocol](../../skills/patterns/gen-eval/references/eval-protocol.md) requires evidence in an `eval-request.md` addendum. The [convergence gather](../../skills/agentic-development-workflow/wrap/references/convergence.md) copies evaluator responses but does not explicitly include that request/addendum in its standard copy list. The retained record can lose the evidence for closure. |
| P1 | Entry files can be short by line count and still costly to load. | [Build](../../skills/agentic-development-workflow/build/SKILL.md) is 343 lines and 34,551 bytes. [Autopilot](../../skills/patterns/autopilot/SKILL.md) is 349 lines and 19,944 bytes. Build still prescribes linear tasks and one commit per task. Neither bytes nor line counts show which instructions each run actually needs. |
| P2 | User documentation has drift that package checks do not cover. | [README](../../README.md) says “22 skills” in Getting Started although the catalog has 24. Three documentation links target old locations: `docs/autonomous-loop.md`, `docs/gen-eval-data-flow.md`, and `docs/release-line-adjustments.md`. The files now live under `docs/workflow/` or `docs/decisions/`. |

P0 means resolve before claiming model-specific support or savings. P1 means a focused change worth testing next. P2 means routine documentation maintenance.

## Suggested changes

### 1. Make the model and effective effort observable

First make the current `high` policy explicit in each supported evaluator recipe. Verify the actual host option before emitting it. Record the requested model, resolved model when available, requested effort, effective effort when exposed, host version, and AEP commit in the execution record. If the host does not expose a value, record it as unknown. A request is not evidence that the host honored it.

Next trial effort profiles for generator, evaluator, and CHECK work independently. Start with the existing evaluator policy. Sweep lower effort on routine stories. Use higher effort on a difficult case only when the run shows a benefit. A model switch, an effort change, and a prompt change should be separate comparisons.

For Fable at lower effort, include a case that requires current external evidence and a case answerable from repository files. Check that the first retrieves a source and the second avoids unnecessary browsing. Retain the user's exact model or tool name in the retrieval case. This turns the documented retrieval concern into an AEP test.

Keep these controls in the executor and project policy. Do not insert model names into each skill. Extend the existing `verification:` accounting block rather than create a second record. Any new policy fields need schema support, migration rules, and a resolved recipe. The current decision already identifies the missing effort-override field as follow-up work.

**Acceptance:** A fixture with a conflicting host default still emits the selected effort. Unsupported controls are reported. A fixed-profile host records the limitation. Existing `default` and `highest` aliases still resolve as documented. Runtime trial records distinguish requested settings from observed settings.

### 2. Test authorization and task continuity at the skill boundary

The template already permits reversible work and reserves material decisions for the user. Add focused cases before adding more prose. Cases should include a task that authorizes a PR, a task that authorizes analysis only, a previously approved design, and a later user correction.

The proposed rule is that a skill guide cannot add a new approval step to already authorized work. Platform restrictions still apply. When a real gate remains, the agent completes independent work first and states the missing decision and its source. A product choice with materially different outcomes remains a human gate. An explicit request to create a PR does not grant permission to merge it.

Persist changes in user scope through the existing feedback and recovery path. A side question should not erase the active story. An explicit cancellation should stop dependent work.

**Acceptance:** An approved story does not stop at “shall I continue?” An analysis-only request does not change source files. A PR-only request stops at the PR. A scope correction survives recovery. A real unresolved design gate is surfaced with the work needed for review.

**Change surface:** [AGENTS template](../../skills/project-setup/onboard/templates/AGENTS.md.tmpl), [bootstrap preamble](../../skills/agentic-development-workflow/launch/templates/bootstrap-preamble.md), and their existing migration path. Keep each rule in its canonical home.

### 3. Publish validated state and retain closure evidence

Add a small producer helper that validates a candidate state file before atomic replacement. On failure, retain the last valid file and report the validation error. Use the existing schemas and validators. Check legal transitions as well as field shape where the lifecycle already defines them.

Make material-finding evidence durable. Give each finding a stable identifier. Record the fix commit, focused check, result, and artifact path in its closure entry. Gather the request addendum, or its structured replacement, before worktree cleanup. A consumer can then verify that every material finding has a closure entry without buying another evaluator round.

A receipt proves that evidence was recorded. It does not prove that a semantic fix is correct. Retain the existing risk-tier checks, hard floors, and human gates. Use sampled independent review to test whether fix-and-attest misses defects. Missing or failed evidence prevents completion; it must not silently convert to a pass.

**Acceptance:** Invalid JSON, illegal transitions, missing closure entries, and failed checks cannot advance a story. A crash before replacement preserves the previous state. Archived material findings still resolve to their fix evidence. Valid older records remain readable under a documented migration.

**Change surface:** [State protocol](../../skills/patterns/autopilot/references/state-schema.md), [signal validator](../../skills/agentic-development-workflow/launch/scripts/validate-signal.mjs), [eval protocol](../../skills/patterns/gen-eval/references/eval-protocol.md), and [convergence gather](../../skills/agentic-development-workflow/wrap/references/convergence.md).

### 4. Trial a smaller build entrypoint against the full story contract

Start with `/aep-build`. Keep its inputs, outputs, worktree guard, verification gates, ownership rules, and stop conditions visible. Load phase-specific procedures when the agent reaches that phase. Measure loaded context, repeated reads, and completion quality. A lower file-size count is only an intermediate result.

In a separate experiment, permit the generator to group related task edits into coherent commits. Preserve task-to-commit evidence and dependency order. This tests whether one commit per task still earns its cost. It changes a public workflow contract, so it requires its own accepted decision before becoming a default.

Keep the current behavior as the comparison arm. The [context-engineering decision](claude-5-context-engineering.md) and [Fable decision](fable-5-1-behavioral-rebaseline.md) already require behavior evidence before removing phase prescriptions. Follow that requirement.

**Acceptance:** The smaller entrypoint completes the same acceptance criteria, emits the same required artifacts, and preserves human gates and recovery. It reduces measured context or completion time without a quality loss. A grouped-commit trial preserves a complete task-to-commit mapping.

**Change surface:** [Build](../../skills/agentic-development-workflow/build/SKILL.md), its references, [git-ref](../../skills/agentic-development-workflow/git-ref/SKILL.md), and existing behavior fixtures. Rebuild shared resources after moving references so that path-based generation does not remove a dependency.

### 5. Use concurrency and steering only where the host supports them

Astra's API supports asynchronous tools and mid-turn steering. Those capabilities belong to the API and its client implementation; they do not establish support in every Codex or Claude Code backend. [Async tool calling](https://developers.openai.com/api/docs/guides/async-tool-calling), [mid-turn steering](https://developers.openai.com/api/docs/guides/steering).

Extend backend detection with small capability tests for the operations AEP uses. Include launch, result delivery, steering, effort selection, and session lifetime. Preserve the existing driver compatibility checks. Keep file signals as the durable record.

For authorized parallel work, assign a bounded task, output, and ownership boundary. Let the lead agent continue independent work while results are pending. Keep writes to shared state ordered. Reserve capacity for evaluation instead of filling all worker slots with generators. Keep CHECK agents within the current signals-only boundary.

Test independent tool batching separately from subagent delegation. Two file reads need no extra agent. For Fable progress visibility, first check that the host displays the updates it receives. Use the existing communication rule to report the current finding and next action. Verify that the final response covers the whole task, even when intermediate tool output is hidden.

Exercise recovery while a tool or worker is pending. The resumed agent needs the current scope, worktree and branch, last verified commit, unresolved gates, pending work, and applied feedback. Reuse the existing state and recovery artifacts. Do not add a separate memory system.

**Acceptance:** Independent work overlaps in a trace. Dependent writes remain ordered. Restart does not launch a duplicate worker or merge twice. New user feedback is applied once. Unsupported transports follow a tested fallback or report that the required operation is unavailable.

**Change surface:** [Executor detection](../../skills/patterns/executor/scripts/detect-backend.sh), backend references, [autopilot](../../skills/patterns/autopilot/SKILL.md), and [build recovery](../../skills/agentic-development-workflow/build/references/harness-artifacts.md). AEP does not need a new provider API runtime to test these host contracts.

## Evaluation plan before changing defaults

Use a disposable downstream repository with installed, pinned AEP skills. The AEP source repository does not run the downstream story loop. Reuse the existing `evals/` records and `scripts/test-*.sh` fixture style; keep static fixtures distinct from live model runs.

Start with these six cases. Give each a clear completion condition and a known failing variant.

| Case | Outcome to verify |
| --- | --- |
| Small approved fix | The requested behavior works; no unrelated change or unnecessary approval pause. |
| Interface change with a consumer | Both sides satisfy the interface contract and the layer journey still works. |
| Sensitive-path change | The actual diff raises the verification floor and a seeded security defect blocks completion. |
| Material finding and blocking finding | Material closure retains evidence; blocking work uses at most two rounds and then gates if unresolved. |
| Interrupted story plus user correction | Resume preserves scope and completed work, applies the correction, and avoids duplicate actions. |
| Two independent stories | Workers overlap, stay within their worktrees, and integrate without conflicting shared writes. |

Also rerun the 40 routing prompts on each verified target. Add boundary cases for analysis-only requests, PR-only authorization, and an unavailable model ID. Preserve the existing observations as historical evidence. Store new observations with model, host, effort, corpus digest, date, and run method.

For the first screen, run each story case three times on each available target using unchanged v4.1.0. That is 36 baseline trials if both targets are verified and available, plus 36 trials for one candidate. A candidate uses the same tasks, host versions, settings, and reset state. Change one factor at a time. Keep evaluation artifacts and earlier solutions out of subsequent trial contexts. Label an unavailable target as not run.

Record completion rate, escaped blocking defects, unnecessary human gates, time to completion, tool calls, context and output usage when exposed, evaluator rounds, and total cost including failed attempts. Calculate cost per successful story from all attempted runs. Report unknown billing data as unknown. Measure factual and UI outcomes with deterministic checks where possible; use a calibrated judge for the remaining judgment.

Three trials per case are a screening budget, not proof of equivalence. Publish per-case outcomes and variability. A default change requires no new blocking escape or authorization violation, preserved gates, and a repeatable quality or efficiency gain. Increase repetitions where results are close. Then run a downstream canary for at least two layers before proposing any looser verification policy, consistent with the current [calibration rule](../../skills/patterns/gen-eval/references/verification-economics.md).

## Delivery order

| Order | Deliverable | Release condition |
| --- | --- | --- |
| 1 | Observable evaluator settings, explicit Codex effort, and the baseline run record | Recipe fixtures pass; actual model/host settings and evidence limits are recorded. |
| 2 | Authorization cases, validated state writes, and durable material closure | Boundary and failure fixtures pass; the current round cap and old record readers still work. |
| 3 | Smaller build entrypoint and effort experiments | Paired story runs show a repeatable gain. Commit grouping stays a separate decision. |
| 4 | Capability-tested concurrency and recovery | Both normal completion and interrupted runs pass on each supported host. |
| Maintenance | Correct the README skill count and three moved links | Local link and catalog checks pass. This does not depend on a model benchmark. |

This PR is the proposal only. Implementation PRs should identify which recommendation they implement and attach the relevant evidence. Apply the existing [release convention](../../project-convention/release.md): update the changelog, version, tag, and migration ledger when shipping a release. A change to instruction templates must include downstream migration instructions. Keep the previous installed pin available for rollback during the canary.

## Checks performed for this review

The following commands passed on the reviewed v4.1.0 checkout:

- `bun run skills:check` — generated resources agree with shared sources.
- `bun run skills:check-vocab` — 18 authored listings and 3 generated copies agree with 14 vocabularies.
- `bun run skills:check-steering` — 63 negation lines, zero hard imperatives; all 27 scanned files stay within their baselines.
- `bun run skills:package-check` — 24 valid installed skills, 3,369 advertised description characters, 40 recorded routing selections, and 23 recorded behavior dry-runs. The user-invoked-only metadata extension is tolerated by the check.
- `bash scripts/test-derive-recipe.sh` — tier, effort, round-cap, and scope-drift fixtures passed.
- `bash scripts/test-detect-backend.sh` — 12 backend-selection fixtures passed.

These checks support package and fixture correctness. They do not establish live backend behavior, story success rates, or model cost. Resolving the Fable 5.1z identity and running the paired story benchmark remain prerequisites for recommendations specific to that exact variant.
