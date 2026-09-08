# AEP v4.1 design review and modification guide for Astra 6

AEP v4.1 already defines autonomous work and bounded verification. Its next revision should make those rules consistent across skills and observable in executor behavior.

**Status:** Proposed; design only. **Reviewed:** 2026-09-08. **Baseline:** v4.1.0, commit `acf03fc41fe25faf67905c9004451702b7986dd8`.

This is a new review of the released source. OpenAI's Astra guidance is the primary model reference. [PR #33](https://github.com/memorysaver/agentic-engineering-patterns/pull/33), at `4914f68`, supplies secondary findings that were checked against the baseline. This proposal does not require its paired Fable review to proceed. No skill, model default, release version, or downstream installation changes in this PR.

**Architecture follow-up:** [Project rules and a simpler story workflow](project-rules-and-story-workflow-refactor.md) develops the requested scaffold/migration, verification, orchestration, and learning redesign. Its staged delivery order takes precedence for those areas; this review supplies the source findings and measurement constraints.

## Evidence and design boundary

The official guidance retrieved on the review date recommends tuning initiative, instruction conflicts, response style, delegation, and verification. It warns that skill instructions can cause avoidable pauses. It recommends preserving existing authorization, explaining skill-induced stops, and completing required checks without unnecessary repetition. These are model guidance, not evidence of AEP runtime failures. [Astra prompting guidance](https://developers.openai.com/api/docs/guides/latest-model?model=gpt-6-astra).

The verified model ID is `gpt-6-astra`; documented effort levels are `low`, `medium`, `high`, `xhigh`, and `max`. Treat another Astra family member as a separate target once its identity and host support are established. [Model reference](https://developers.openai.com/api/docs/models/gpt-6-astra).

The source pages are mutable. Retrieval on September 8 does not establish that their content changed after PR #33's September 5 review. This review inspected source contracts and examples; it did not run paid model trials or observe effective settings inside every host.

Keep the story map, interface contracts, isolated worktrees, generator/evaluator separation, sensitive-path verification floors, human gates, and the two-round evaluation ceiling. The proposed changes below are AEP design judgments. The official guide does not prescribe these mechanisms or justify removing them.

## Review findings

| ID | Priority | Verified baseline | Proposed result |
| --- | --- | --- | --- |
| A | P0 | The [AGENTS template](../../skills/project-setup/onboard/templates/AGENTS.md.tmpl) permits autonomous work within scope. [Scaffold](../../skills/project-setup/scaffold/SKILL.md) still requires category confirmation and makes version re-pin recommend-only. | A previously authorized update reaches its requested deliverable; an audit request remains read-only. |
| B | P0 | [Executor policy](../../skills/patterns/executor/references/backends.md) calls for evaluator effort `high`, then allows fixed profiles to ignore it. The [Codex evaluator example](../../skills/patterns/executor/references/codex-native.md) supplies neither model nor effort. | Requested, configured, and observed evaluator settings are distinguishable. |
| C | P1 | The template and [launch preamble](../../skills/agentic-development-workflow/launch/templates/bootstrap-preamble.md) already limit unnecessary tests. [Build](../../skills/agentic-development-workflow/build/SKILL.md) also carries per-task verification and later journey/preflight requirements. | Workers know which checks are required and when a completed check needs repeating. |
| D | P1 | [State writes](../../skills/patterns/autopilot/references/state-schema.md) rename an unvalidated candidate; validation is explicitly on demand. [Material closure](../../skills/patterns/gen-eval/references/eval-protocol.md) uses an eval-request addendum, absent from the standard [convergence gather](../../skills/agentic-development-workflow/wrap/references/convergence.md). | Invalid candidates cannot replace valid state; archived evidence supports material closure. |
| E | P1 | [Executor modes](../../skills/patterns/executor/references/backends.md) have different lifecycle and gate mechanisms. A model's API feature list does not verify those modes. | Delegation and recovery use tested host capabilities and preserve worktree ownership. |
| F | P2 | [Build](../../skills/agentic-development-workflow/build/SKILL.md) combines a long procedure with linear tasks and one commit per task. [Easy-explain](../../skills/patterns/easy-explain/SKILL.md) already owns the human-facing register. | Load only relevant phase detail; measure behavior before changing task or commit policy. |

The existing [routing observations](../../evals/skill-routing-observations.json) describe a Sonnet alias, and [behavior parity](../../evals/skill-behavior-parity.json) describes reviewed dry-runs. They remain useful historical checks. Neither establishes live Astra story performance.

## A. Resolve authorization at the task boundary

**Owners:** AGENTS template for the general rule; scaffold and its `references/converge-flow.md` for installation behavior; launch preamble for the worker's inherited scope.

Define the deliverable from the current request and relevant earlier authorization. An explicit update-and-create-PR request covers inspection, scoped repair, verification, and PR creation. It does not automatically cover merging that PR. A later merge request can authorize that final action. A status question during work should receive an answer while the original task continues. A cancellation or incompatible replacement changes that task.

Replace scaffold's unconditional confirmation with a scope check. Show the audit result, apply categories already covered by the request, and ask only about unresolved choices or destructive conflicts. Keep the in-place scaffold overwrite warning and permission boundary when existing project files would be replaced. Change recommend-only re-pin behavior only for an explicitly authorized upgrade; an audit alone still emits findings.

When a skill instruction causes a stop, report its exact file, the operative rule, and how that rule applies. Do not turn ordinary guidance into a new approval requirement. This proposal changes AEP guidance; system, developer, organization, and tool permission controls still apply.

**Acceptance:** transcript fixtures cover audit-only, authorized update, PR-only, later merge authorization, a destructive overwrite, a skill conflict, and a mid-task status question. Each has an expected terminal deliverable and an explicit list of permitted mutations. A worker must neither stop before the authorized deliverable nor exceed it. Use live Astra trials to check behavior; matching instruction text is insufficient.

## B. Make evaluator configuration reviewable

**Owners:** executor backend recipes, Codex examples, verification accounting in convergence, and [verification economics](../../skills/patterns/gen-eval/references/verification-economics.md).

Keep `high` as the existing evaluator policy while testing its implementation. For each mode, specify the supported configuration mechanism and its precedence over inherited defaults. Check the installed host's help/configuration before adding a CLI flag; API parameter names are not CLI syntax.

Record the requested model and effort, the configuration actually sent, host/mode/version, and observed model/effort when exposed. Record unavailable observations as unknown. A fixed-profile mode must name its profile and state that the requested effort was not configurable; it must not claim the hint was enforced. An unavailable requested model should produce a clear fallback decision, not a silently relabeled run.

Inspect approval and sandbox options alongside the model configuration. The Codex example currently includes a bypass flag. A revised recipe must select the host's permitted execution profile explicitly; an autonomy prompt is not authorization to bypass host controls.

**Acceptance:** command-generation fixtures verify effective precedence and unsupported settings. One smoke run per claimed supported mode verifies worktree cwd, evaluator artifact output, and exposed settings. Keep fixture results separate from runtime evidence. Preserve the cheap CHECK path and generator/evaluator separation. Do not switch every role to Astra or increase effort globally.

## C. Give verification a completion rule

**Owners:** build phase instructions and verification economics; retain the short generic principle in the AGENTS template.

At the start of verification, derive required checks from the actual diff, repository conventions, story acceptance criteria, and the recipe's hard floors. Record each check's purpose and applicable scope. Reuse a successful result only while its relevant inputs and environment remain unchanged. A new defect, changed dependency, or unresolved concern can require another run.

Distinguish task checks, story evaluation, and layer/deployment checks. A passing task test does not satisfy an outstanding journey requirement. Conversely, a completed layer suite does not need repeating merely because another prompt restates a generic testing instruction. Small documentation edits should use document checks unless repository policy requires more.

**Acceptance:** exercise a small approved edit, an interface change, a sensitive-path change, and a failure after an earlier pass. Assert that mandatory checks run, seeded defects block completion, and unchanged successful checks do not repeat without a recorded reason. Retain the existing round ceiling and material/polish classification. Lowering verification floors requires a separate measured proposal.

## D. Preserve state and closure evidence

**Owners:** autopilot state protocol/validator and wrap convergence gather.

Validate a complete candidate against the schema before replacing the state file. A failed validation leaves the prior bytes intact and reports the error. Retain atomic replacement on the same filesystem. Atomic rename does not itself solve concurrent-writer exclusion; exercise the tick-lock protocol separately before claiming that property.

Include the eval-request addendum in archived convergence evidence, or introduce a stable closure artifact with explicit finding references. Preserve the original evaluator responses. A reviewer after workspace teardown must be able to connect each material finding to its fix and verification evidence. Legacy records with no such evidence remain readable and are marked unknown, never inferred closed.

**Acceptance:** invalid enum and missing-required-field candidates cannot replace good state; interrupted writes preserve a readable prior state. A material-finding fixture survives wrap/archive/teardown with its evidence intact. A missing optional historical artifact keeps best-effort gather behavior without manufacturing proof of closure.

## E. Test delegation and steering at the host boundary

**Owners:** executor mode references, launch contracts, and autopilot recovery protocol.

Delegate only a bounded task that can run independently while the parent makes useful progress, when the active instructions permit delegation. Identify the worktree, file ownership, output artifact, and completion condition. Apply the host's available capacity. Shared integration writes stay with the integration owner. For a serial dependency, finish the prerequisite first.

On user steering or recovery, reconcile pending workers, completed artifacts, and external actions before resuming. An updated requirement must reach the affected worker. A status question should not restart the story. Replaying a continuation must not create another PR or repeat a merge already completed.

The Astra API documents async tools, mid-turn steering, and effort updates; tool calling requires Responses. These features still need application support. Migration guidance preserves effective effort where possible, mapping unsupported `none`/`minimal` to `low` for comparison. [Astra API guidance](https://developers.openai.com/api/docs/guides/latest-model?model=gpt-6-astra).

| Feature | AEP adoption gate |
| --- | --- |
| Async tools | Host exposes pending work and result identity; dependent operations wait for the right result. |
| Mid-turn steering | Host delivers updates and preserves completed work; pending external actions are reconciled. |
| `configuration_update` | Verify current API compatibility and host exposure before use; keep fixed effort as the comparison baseline. |

These are conditional integration experiments. This proposal does not add a Responses transport to AEP or assume that Codex native subagents and headless exec expose identical controls.

**Acceptance:** two independent stories overlap without shared-file conflicts; a serial pair waits correctly. Interrupt a worker, deliver a scope correction, and resume with exactly one terminal artifact/action. Unsupported modes report the limitation and use their documented fallback.

## F. Reduce loaded instructions through phase boundaries

**Owners:** build entrypoint and its references; easy-explain for response style.

Keep entry conditions, phase transitions, ownership, output locations, and gate rules in the entrypoint. Move detailed procedures into references linked from the phase that needs them. Audit conflicting copies before adding more prose. Preserve canonical names and machine-readable artifact shapes.

Use short progress reports that state the current result and next unresolved step. Keep evidence and remaining limits in the final report. Do not duplicate a long model-specific style block across every skill.

Measure loaded bytes/tokens, reference retrievals, missed requirements, elapsed time, and completion quality. A shorter file that causes repeated searching or lost requirements is not an improvement. Keep one commit per task during this experiment. Grouped commits change task-to-commit accounting and require a separate decision, compatibility plan, and test.

**Acceptance:** all current routing and behavior boundaries remain covered; paired Astra story trials preserve completion and required gates. No default change rests on line count alone.

## Evaluation and delivery sequence

1. Implement A and B as separate reviewable changes, with transcript and command fixtures. Establish a live Astra baseline before claiming behavioral improvement.
2. Implement C and D with verification and failure/recovery fixtures. Retain old artifact readers.
3. Test E per host. Try F only after behavior and settings are observable.

Use downstream fixture repositories for story runs; AEP itself authors the skills and does not run its consumer story loop. Start with six cases: a small approved fix, an interface plus consumer, a sensitive-path defect, material and blocking findings, an interrupted story with correction, and two independent stories. Add the authorization boundaries from A and an unavailable-model case. Rerun the existing 40 routing prompts on the exact verified Astra target.

For an initial story screen, use three trials per core case: 18 unchanged-v4.1 trials and 18 trials for one candidate. This is a proposed budget, not an executed benchmark or proof of equivalence. Keep model, effort, host version, tools, task corpus, and reset state fixed between prompt candidates. Test effort changes separately. Prevent previous solutions and judge artifacts from entering later trial contexts.

Record per-case completion, authorization violations, escaped blocking defects, unnecessary pauses, required-check coverage, repeat checks, evaluator rounds, time, tool calls, and usage/cost when available. Include failed attempts in cost per successful story; unknown billing stays unknown. Store corpus and skill digests with raw evidence and the run method. Broaden trials for close or variable results.

A candidate must preserve required gates and show no new authorization violation or blocking escape in the screen. Demonstrate repeatable benefit before selecting a new default. Any proposal to loosen verification must also meet the existing calibration rule: at least two layers of evidence, no unresolved escape attribution, and model-conditioned results. A successful screen alone does not satisfy that rule.

## Relationship to PR #33 and release

| PR #33 item | Treatment here |
| --- | --- |
| Effort observability and authorization fixtures | Retained and expanded into A/B, with explicit scope and host boundaries. |
| Candidate validation and material closure | Rechecked in source; retained in D. |
| Smaller entrypoint and grouped commits | Separate experiments; grouped commits deferred. |
| Concurrency and recovery | Reframed around host capabilities and steering in E. |
| Paired Astra/Fable benchmark | Astra is the first target; no dependency on resolving another model's identity. |
| README count and moved links | Separate documentation maintenance; not evidence for an Astra design change. |

Implementation PRs should name their finding ID, changed contract, compatibility effect, and checks. Follow the [release convention](../../project-convention/release.md) when shipping: classify fixes, additive capability, and breaking contracts correctly, then update version, changelog, tag, and migration ledger. Do not promise a minor release for a breaking change.

Template changes require an explicit downstream migration. Verify each installed runtime's committed skill files and lockfile, then update instruction markers and any already-rendered recipes. Re-pin alone does not rewrite those project files. Preserve local conventions and the previous pin for rollback. This design PR leaves PR #33 open for a separate disposition decision.
