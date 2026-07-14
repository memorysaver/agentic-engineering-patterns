# Lean Skills: The Authoring Standard and the Single-Source Rule

AEP's 22 SKILL.md files total **8,209 lines** (avg 373, max 955) plus 12,528 lines of references, and their frontmatter descriptions put **~1,687 words of always-loaded context** into every session of every downstream repo. A full-corpus audit (2026-07-14, three parallel line-by-line reviews) shows the dominant cause is not content volume but structural indiscipline: the same meaning living in 2–21 places, reference material buried inside step sequences, and design rationale inlined into agent-facing files. This document adopts an explicit authoring standard — **a SKILL.md is lean steps ending in checkable postconditions, plus disclosed reference; every meaning has exactly one home** — and defines the refactor that brings the corpus to ~4,500 lines (-45%) while holding behavior parity, verified per R9. Duplication is not just token cost: it is the propagation surface behind this repo's #1 bug class (half-applied enum/taxonomy changes, three review rounds on PR #16).

> **Status:** Accepted — implemented on this branch as **v3.0.0** (a major bump: the corpus is restructured and downstream pointer shapes change), with phases P1–P3 collapsed into one implementation per the owner's direction; P4 remains horizon. This document is the contract the implementation is reviewed against. It changes how AEP works, so it lives in `decisions/` per the [docs routing guide](../README.md).

> **Sourcing note:** The lens comes from [mattpocock/skills](https://github.com/mattpocock/skills) — specifically `skills/productivity/writing-great-skills/` (SKILL.md + GLOSSARY.md) and `.agents/invocation.md`, whose 39-skill corpus totals 2,757 SKILL.md lines (7–140 each). Its vocabulary (steps/reference, progressive disclosure, no-op, negation, sediment, sprawl, leading word, context load) is adopted here as-is. Evidence line numbers below reference the corpus at commit `ededa12` (v2.7.0).

---

## Diagnosis

### The corpus, measured

|                                   | AEP (v2.7.0)                      | mattpocock/skills       |
| --------------------------------- | --------------------------------- | ----------------------- |
| Skills                            | 22                                | 39                      |
| SKILL.md lines                    | 8,209 (avg 373, max 955)          | 2,757 (avg 71, max 140) |
| Description words (always-loaded) | 1,687                             | ~25–40 per skill        |
| Rationale location                | inlined "Design Decisions" essays | ADRs, outside skills    |

The comparison is directional, not a target: that workflow is single-human and conversational where AEP is multi-agent and autonomous, so AEP legitimately needs more mechanical precision (bash probes, schemas, typed gates). What transfers is the structural discipline, not the absolute line counts.

### Failure-mode inventory (audit evidence)

**Duplication — the same meaning in 2–21 places.**

- The `$BASE` integration-branch resolver is inlined **~21 times across 13 skills, in two different spellings** (e.g. `build/SKILL.md:718-721` vs `:209`; `launch:52-55,108-111,179-182`; all 8 product-context skills) — most sites even carry a "see git-ref 'Integration Branch'" comment and then paste the block anyway. Two spellings of one meaning guarantees drift.
- Autopilot's 7-step tick block (`autopilot/SKILL.md:437-553`, ~116 lines) restates `references/tick-protocol.md` nearly in full; the ④b/④c nudge prompt texts exist **verbatim in three files** (SKILL, tick-protocol.md, review-trigger.md — the latter's line 5 admits it).
- `build/references/worktree-onboarding.md` is a near-complete second copy of build Phase 0, including the worktree-guard bash duplicated verbatim (`build:39-66` ≡ `worktree-onboarding:38-59`).
- The merge stop-condition list lives in both `build/SKILL.md:899-906` and `merge-decision-cases.md:68-77`; layer-gate tier logic in wrap (`:258-298`), build Phase 6, and build Phase 8; one-commit-per-task in build and git-ref; the orchestrator-boundary doctrine in five places; `full_auto` in three; the reflect classifier re-tabulated in watch (`:188-198`) directly under a sentence saying not to duplicate it.
- The File-Resolution SPLIT/V1 block repeats in 7 of the 8 product-context skills (all except watch, ~70 lines); the YAML-validation-plus-common-fixes prose repeats 4–7× while only 5 of the 8 carry `references/yaml-guardrails.md` (dispatch, envision, map, reflect, watch — model and validate have the prose but not the reference, because the build materializes the shared file only into skills whose SKILL.md names it); executor/gen-eval/workflow/design-lens each list their reference filenames **twice** (a prose path block and a Reference Files table).

**Reference buried inside steps.** `dispatch` (654 lines) carries ~130 lines of scoring formulas and ~120 lines of context-assembly recipe — material that changes ~never and is _consulted_, not _executed_. `launch` carries a 125-line Evaluator section that the skill itself says build handles automatically. `scaffold` (700) inlines four complete OpenSpec command-file bodies (~64 lines) and ~150 lines of audit/converge bash. `validate` inlines ~68 lines of agent-prompt templates that belong in `gen-eval/references/agent-contracts.md`, which it already cites.

**Rationale inlined into agent files.** "Design Decisions" essays sit inside `executor` (~60 lines), `design-lens` (~40), `gen-eval` (~22), `workflow` (~19) — the agent never needs "why not merge with /aep-validate" mid-run. AEP already has the correct home: `docs/decisions/`.

**Negation-dense steering.** 8 of build's 16 guardrails and five consecutive autopilot bullets steer by `NEVER …`, then repeat the same prohibitions in tables and again in Guardrails. Prohibitions name the elephant; the standard requires positive phrasing with prohibition reserved for hard guardrails.

**No-ops.** Lines the model does by default: "be extra critical", "Prune aggressively — irrelevant context degrades agent performance" (stated twice in dispatch), "One question at a time", "Don't rush".

**Sediment.** `progress-template.md:62-64` still lists a pre-rename "Phase 7: E2E Test Scripts"; the jj-migration history sits in git-ref's hot path.

**Cross-link inconsistency (live bug).** The same executor reference is addressed two ways — `../../patterns/executor/references/backends.md` (launch:250) and `.claude/skills/aep-executor/references/backends.md` (launch:18 et al.) — so one form breaks whenever install names differ, which the skills themselves warn about (launch:207).

### What already conforms (keep and generalize)

- **wrap's world-derived postconditions** ("archived = change dir gone AND `archive/*` exists", `wrap:229-239`) are exactly the checkable completion criteria the standard mandates — already generalized by [deterministic-orchestration.md](deterministic-orchestration.md).
- **git-ref as a reference skill** is correctly shaped; the defect is that other skills re-inline its content instead of invoking it.
- **envision and watch** are closest to target shape; every skill already has a `references/` dir and the `_shared/` build machinery, so progressive disclosure needs no new infrastructure.

---

## The standard (normative)

Every rule below is checkable in review; R7–R8 are checkable in CI; R9 is evidence attached to each implementation PR.

- **R1 — Information hierarchy.** A SKILL.md contains (a) the steps every run executes and (b) only the reference every branch needs. Reference that only some runs reach — formulas, mode recipes, edge-case recovery, worked examples, prompt templates — moves to `references/*.md` behind a pointer whose wording states _when_ to load it.
- **R2 — Single source of truth.** Each meaning has exactly one canonical home; every other site is a one-line pointer. Canonical-home table:

  | Meaning                                                                                      | Home                                                                                     |
  | -------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------- |
  | `$BASE` resolution, worktree lifecycle, branch naming, one-commit-per-task, PR/publish rules | `/aep-git-ref`                                                                           |
  | Backend selection, spawn/steer/liveness recipes                                              | `/aep-executor` references                                                               |
  | Tick state machine + nudge prompt texts                                                      | `autopilot/references/tick-protocol.md`                                                  |
  | YAML validation + common fixes                                                               | `_shared/references/yaml-guardrails.md`                                                  |
  | Gen/eval contracts, scoring, prompt templates                                                | `/aep-gen-eval` references                                                               |
  | Journey/tier/layer-gate theory                                                               | e2e-test skill (per project) + `e2e-skill-scaffolding` references                        |
  | File-Resolution SPLIT/V1 probe                                                               | one `_shared` reference, pointed at by the 7 product-context skills that carry the block |

- **R3 — Cross-skill dependencies are `/aep-x` prose invocations**, never relative file paths. Intra-skill `references/*.md` links stay relative. (Kills the dual-path bug class.)
- **R4 — Steps end in world-derived postconditions.** Every step's completion criterion is checkable from observable state (exit code, file existence, YAML field, `git` probe) — the [deterministic-orchestration](deterministic-orchestration.md) probe catalog applied to skill authoring. "Build shared understanding" is not a criterion; "proposal.md exists and the user approved it" is.
- **R5 — Rationale lives in `docs/decisions/`.** SKILL.md carries at most one sentence of why with a pointer. Existing inline "Design Decisions" sections migrate out.
- **R6 — Positive steering.** State the target behavior; a prohibition survives only as a hard guardrail that cannot be phrased positively (e.g. `--force-with-lease`, never-touch-main-checkout), stated **once**, paired with the positive action.
- **R7 — Budgets.** One corpus-wide budget for every SKILL.md: **warn > 400 lines, hard cap 500** (aligned with Anthropic's ~500-line skill guidance). The per-skill targets in the refactor table below are the binding review gate for each implementation PR; "utility/index skills aim for ≤ 150" is review guidance, not a separate CI tier. Descriptions: ≤ 50 words, one trigger per branch, no synonym stacking — and any description whose trigger set shrinks must pass a triggering check (probe prompts or a skill-creator-style eval) before merge, since dropped trigger terms can silently stop a skill firing.
- **R8 — CI enforcement** in `skills-check.yml`: (a) line check over `skills/**/SKILL.md` — warn > 400 from P1; the fail threshold (> 500) activates in the P3 PR once the corpus is under budget (activating earlier would fail the not-yet-refactored files); (b) the multi-line `$BASE` resolver block appears in exactly one **SKILL.md** (git-ref) — the `aep.integration-branch` literal legitimately survives in reference files that read the config key for their own bash (`executor/references/backends.md` probe, `autopilot/references/tick-protocol.md` nudge prompt), so those two are allowlisted and the check must not scan all of `skills/`; (c) existing `skills:check` stays green.
- **R9 — Behavior parity.** Every restructure PR (P1–P3) attaches parity evidence for each touched skill: at least one recorded scenario dry-run (or `/aep-validate` gen-eval pass) showing the leaned skill executes the same steps and loads its disclosed references when the scenario requires them. Line counts prove size, not behavior; a disclosure behind a weakly-worded pointer passes every other gate while silently regressing the skill — R9 is the gate that catches it.

New glossary entries (identical wording wherever used, per the propagation discipline): **Lean Skill**, **Progressive Disclosure**, **Single Source of Truth (skills)**, **No-Op Line**, **Negation Steering**, **Context Load**, **Leading Word**.

---

## The refactor (implementation contract)

Per-skill targets from the audit. "Lean" = steps kept inline, listed material moved to `references/` or replaced by pointers; no step is removed or reordered. The per-skill moves (what stays, what discloses where, what dedups to which canonical home) are expanded in [plans/2026-07-14-lean-skills-refactor.md](../plans/2026-07-14-lean-skills-refactor.md).

| Skill                 |       Now |     Target | Primary moves                                                                                                                                                                                   |
| --------------------- | --------: | ---------: | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| build                 |       955 |       ~325 | Phase 0 artifacts, Phase 5 eval machinery, Phase 6 dogfood theory, Phase 12 merge taxonomy → references; build Phase 0 stays canonical — worktree-onboarding.md shrinks to a thin pointer at it |
| autopilot             |       727 |       ~270 | delete inlined tick block (tick-protocol.md is canonical), drivers → `references/drivers.md`, boundary doctrine stated once                                                                     |
| scaffold              |       700 |       ~380 | split new-project spine / existing-project converge; opsx bodies + audit bash → template/script files                                                                                           |
| dispatch              |       654 |       ~300 | scoring → `references/scoring.md`, context assembly → `references/context-assembly.md`, workflow-mode → reference                                                                               |
| launch                |       495 |       ~160 | Evaluator section + per-mode spawn recipes + orphan recovery → references                                                                                                                       |
| validate              |       455 |       ~240 | prompt templates → gen-eval references; dimension/mode tables → reference                                                                                                                       |
| onboard               |       385 |       ~300 | settings.json block → template; integration-branch explainer → `/aep-git-ref`                                                                                                                   |
| map                   |       355 |       ~210 | telemetry binding, `.5`-layer treatise, object-map drafts → references                                                                                                                          |
| wrap                  |       352 |       ~150 | Reflect-and-Advance layer-gate/distillation → references (convergence.md pattern)                                                                                                               |
| watch                 |       325 |       ~200 | config schema + adapters → telemetry-ingestion.md; drop re-tabulated classifier                                                                                                                 |
| git-ref               |       319 |       ~260 | trim jj history; stays the canonical git home                                                                                                                                                   |
| reflect               |       297 |       ~180 | changelog/outcome YAML blocks → schema references                                                                                                                                               |
| calibrate             |       287 |       ~180 | type/scan tables → calibration-types.md (already materialized)                                                                                                                                  |
| e2e-skill-scaffolding |       265 |       ~210 | cli/none-target rules → three-tier-model.md                                                                                                                                                     |
| envision              |       247 |       ~150 | quality-dimensions catalog → reference (shared with calibrate)                                                                                                                                  |
| design-lens           |       243 |       ~145 | Design Decisions → `docs/decisions/`; earns-its-place table → `references/method-and-templates.md`                                                                                              |
| model                 |       233 |       ~150 | ORCA rounds thin to pointer (orca-process.md is canonical)                                                                                                                                      |
| executor              |       219 |       ~150 | Design Decisions essay → `docs/decisions/`; drop duplicate path block                                                                                                                           |
| design                |       187 |        ~95 | mode selection + prerequisites → references                                                                                                                                                     |
| workflow              |       187 |       ~135 | merge the two AEP-mapping tables; essays → references                                                                                                                                           |
| workflow-feedback     |       173 |       ~150 | fold DO/DO-NOT guardrails into steps                                                                                                                                                            |
| gen-eval              |       149 |       ~120 | drop duplicate path block; trim essays                                                                                                                                                          |
| **Total**             | **8,209** | **~4,455** | **-45%**                                                                                                                                                                                        |

Description diet runs across all 22 in the same pass: 1,687 → ~700 words, one trigger per branch. Pipeline-position prose ("The final step in the feature lifecycle") stays only where it aids chained invocation.

### Phases (each an independent PR against this document)

| Phase                                        | Action                                                                                                                                                                                                                                                                                                                                                       | Behavior change? |
| -------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ---------------- |
| **P0 — this decision doc**                   | Record the standard, the evidence, the targets                                                                                                                                                                                                                                                                                                               | No               |
| **P1 — mechanical dedup sweep**              | extend `build-skills.sh` to per-file materialization (prerequisite — see plan item 0); `$BASE` → git-ref one-liner everywhere; File-Resolution + YAML-guardrails → pointers; delete duplicate reference-path blocks; nudge prompts live only in tick-protocol.md; standardize cross-links to `/aep-x` prose (fixes dual-path bug); CI checks (R8, warn-only) | No               |
| **P2 — big-four restructure**                | build, autopilot, dispatch, scaffold to target shape (one PR each)                                                                                                                                                                                                                                                                                           | No               |
| **P3 — corpus pass**                         | remaining 18 skills to budget; Design Decisions essays → `docs/decisions/`; negation→positive + no-op pruning; description diet (with triggering checks per R7); glossary entries; activate the R8 fail threshold; v3.0.0 bump                                                                                                                               | No               |
| **P4 — invocation audit (future, optional)** | `disable-model-invocation: true` for run-once skills (onboard, scaffold, e2e-skill-scaffolding); upgrade `skills-quick-reference.md` into a router skill                                                                                                                                                                                                     | Invocation only  |

P4 is recorded as horizon, not committed: build/dispatch/launch/wrap/reflect/gen-eval/executor must stay model-invoked (autopilot and boot prompts invoke them), so the audit needs its own care.

### Propagation discipline

- Product-context skills' `references/` and `templates/` are **build-generated from `_shared/`** — every edit to those files, in any phase, goes to `_shared/` first, then `scripts/build-skills.sh`; authoring directly in a consumer dir gets silently wiped on commit. Today the wipe is dir-level (`rm -rf` on any dir carrying `.aep-generated`), which leaves **skill-owned disclosure files no valid home** inside a generated `references/` dir (and `_shared/` would fan them into all five generated consumers). P1 therefore first extends `build-skills.sh` to per-file materialization — the marker lists the managed basenames; only those are removed/copied — so skill-owned files (e.g. dispatch's `scoring.md`) coexist with generated ones, and shared files can also land in the three authored-dir skills (calibrate, model, validate) that the dir-level `continue` currently skips.
- The new glossary terms are net-new taxonomy: every skill that names one says it identically; audit with `grep -rn "single source of truth\|no-op\|progressive disclosure" skills/ docs/` after P3.
- Downstream consumers see nothing until the v3.0.0 tag is cut and each of the 6 repos re-pins via the skills CLI. P1–P3 must not change any step semantics, and each PR's R9 parity evidence is what makes that claim checkable; `skills:check` green + R9 evidence are the release gate.

---

## Anti-patterns this prevents

- **Paste-with-pointer.** "See git-ref 'Integration Branch'" followed by the pasted block — the pointer and the paste drift independently; keep only the pointer.
- **Second spelling.** Re-expressing an existing snippet "equivalently" creates a divergent source; there is one spelling, in one home.
- **Teaching mid-run.** Re-explaining tier theory / scoring math / backend selection inside a step sequence buries the steps; consult reference, don't re-teach it.
- **Rationale in the hot path.** "Why we chose X" essays in SKILL.md cost every run tokens to justify a decision no run revisits.
- **Guardrails-as-echo.** A Guardrails section that restates rules already stated inline doubles the maintenance surface; guardrails exist only for rules with no natural step home.
- **Synonym triggers.** Description trigger lists that restate one branch five ways pay permanent context load with no demonstrated recall gain — and trigger cuts ship only with a passing triggering check (R7), so the diet never silently un-wires a skill.

---

## References

- [mattpocock/skills](https://github.com/mattpocock/skills) — `skills/productivity/writing-great-skills/SKILL.md` + `GLOSSARY.md` (the adopted vocabulary), `.agents/invocation.md` (invocation axis), `skills/engineering/ask-matt/SKILL.md` (router shape for P4).
- Full-corpus audits, 2026-07-14: three parallel line-by-line reviews (agentic-development-workflow; patterns; product-context + project-setup) — line citations throughout this document are theirs, at commit `ededa12`.
- [deterministic-orchestration.md](deterministic-orchestration.md) — the probe catalog R4 applies to authoring; same mechanical/judgment logic, applied to prose instead of runtime.
- [aep-v2-lesson-learning.md](aep-v2-lesson-learning.md) and PR #16 review history — the half-applied-taxonomy bug class that shrinking the propagation surface addresses.
- Affected: all 22 skills, `docs/glossary.md`, `.github/workflows/skills-check.yml`, `_shared/` (via build).
