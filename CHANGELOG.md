# Changelog

All notable changes to this skills plugin are recorded here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and the project adheres
to [Semantic Versioning](https://semver.org/spec/v2.0.0.html) on the
`metadata.version` field in [`.claude-plugin/marketplace.json`](.claude-plugin/marketplace.json).

**Maintenance convention:** every change that bumps the `marketplace.json`
version ships with a matching entry here, in the same PR. Add work in progress
under `[Unreleased]` as you go; when cutting a release, rename `[Unreleased]` to
`[X.Y.Z] - YYYY-MM-DD` and tag `vX.Y.Z` on merge to `main`. SemVer guide for this
repo: new skills / new backends / additive capability → **minor**; recipe and
bug fixes → **patch**; removing or breaking a skill contract → **major**.

> This is **release history for the plugin**. It is unrelated to the per-project
> `changelog:` array inside a downstream `product-context.yaml` (written by
> `/envision`, `/dispatch`, `/reflect`, …), which records product-state history
> for that project. See [`docs/glossary.md`](docs/glossary.md).

## [3.1.0] - 2026-07-16

The incident-proven half of
[`docs/decisions/verification-economics.md`](docs/decisions/verification-economics.md)
(P1 + P2 + P2.5): failure taxonomy, environment preflight, deterministic
security gates, regression-replay placement, evaluator-independence fixes, and
verification-accounting instrumentation. The economics half (tiers, recipes,
calibration) ships as v3.2.0.

### Added

- **Canonical reference** `gen-eval/references/verification-economics.md` —
  validator placement matrix, failure taxonomy + classification authority,
  environment preflight gate, verification tiers + two-point derivation,
  verification recipe, accounting schema, tamper-evident evidence classes,
  worked examples, anti-patterns. Registered in the gen-eval SKILL.md tables.
- **Failure taxonomy carriers** — `failure_class`
  (`product-defect | environment | harness-flake | scope`) with per-class
  evidence requirements and the `product-defect` default: a required
  `**Failure-Class:**` line in the unified dogfood report
  (`executor/references/dogfood-validation.md`, `tool-selection.md.tmpl`);
  evaluator-authored `Failure-Class` in `eval-response-<N>.md`
  (`eval-protocol.md`, `agent-contracts.md`); the `dogfood_report` adapter
  routes on class and **never auto-files** non-product classes
  (`_shared/references/telemetry-ingestion.md`); the recovery ladder's "When to
  Skip" is now the typed taxonomy step, mandatory at every FAIL
  (`recovery-ladder.md`, `build/SKILL.md` Phase 5).
- **Environment preflight gate** — deploy-independent probes pre-merge on every
  story; target-bound probes where journey execution runs; named refusals
  (`REFUSING [dogfood-secret-absent:<NAME>]`) with REFUSED ≠ SKIP ≠ FAIL
  semantics (`build/SKILL.md` Phase 6B, `wrap/references/layer-advance.md`,
  `autopilot/references/post-merge-guard.md`, `policy.md.tmpl`,
  `layer-gate-loop.md`, `three-tier-model.md`).
- **Autopilot REFUSED ≠ FAIL** — a refused gate pauses cheaply with an ops
  checklist via the new `environment_repair` escalation type, re-probes
  world-derived each tick (repair auto-resumes), and never files a code-fix
  recovery story (`tick-protocol.md`, `state-schema.md`).
- **Deterministic security gates** — secret-scan/SAST confirmed at scaffold as
  inner-loop typed gates (`policy.md.tmpl` `{{SECRET_SCAN_CMD}}`/`{{SAST_CMD}}`,
  e2e-skill-scaffolding SKILL.md Phase 2).
- **`live_policy` decision** (`every_gate | milestone_gates_only | none`)
  upstreamed into `policy.md` as its canonical home, propagated across the full
  enumerated site list (templates, build/wrap/executor/autopilot references,
  `product-context-schema.yaml` comments, `map/SKILL.md`,
  `scaffold/references/resulting-structure.md`).
- **`sensitive_paths`** — the human-owned deep-tier hard-floor list, seeded and
  confirmed at scaffold (`policy.md.tmpl`), with the referee-asset rule (test
  dirs / journeys / policy / CI never derive `light`).
- **Verification accounting instrumentation** — the `verification:` block in
  `execution-record.yaml` with a mandatory file-derivable sensor floor
  (tier/escalation/drift flags, generator/evaluator model ids, rounds,
  findings-by-round, scenarios, preflight refusals), per-round eval-response
  persistence, gather-source table (`wrap/references/convergence.md`), the
  human-owned layer budget box (record-only cold start;
  `layer-gate-evidence.md.tmpl`, `layer-advance.md`), and escape-rate ingestion
  via `/aep-reflect` (`telemetry-ingestion.md`).
- **Glossary** — new Verification Economics section: Verification Tier,
  Verification Recipe, Failure Class, Classification Authority, Environment
  Preflight Gate, Verification Accounting, Escape Rate, Tamper-Evident
  Evidence, Verification Ratchet, Perfect-Score Gate, Referee Asset, Recovery
  Ladder.

### Changed

- **Evaluator independence** — spawn authority leaves the generator where an
  orchestrating layer exists (the autopilot ④b nudge is the orchestrator-owned
  spawn); the machine-assembled evaluator prompt marks `eval-request.md` as the
  generator's **untrusted claim** (`build/SKILL.md` Phase 5,
  `agent-contracts.md`, `tick-protocol.md`).
- **Regression replay placement** — per-story replay (`build/SKILL.md` Phase 8)
  is impacted-only (selected against the merged diff, fail-open for journeys
  with no `paths:`) plus the walking-skeleton canary; **full** prior-layer
  replay moves to the `/aep-wrap` layer gate with a derived mid-layer
  checkpoint `k = min(5, ⌈N/3⌉)` (`layer-advance.md`, `layer-gate-loop.md`).
- **Layer gates require tamper-evident evidence** — `passed` needs at least one
  evidence class the generator cannot modify (SHA-bound CI with out-of-scope
  workflow definitions, wrap-executed journey, ledger-equality golden fixtures,
  telemetry); warning-not-refusal for one release (`layer-advance.md`,
  `three-tier-model.md`).
- **Zero-blocking PASS semantics** — a perfect aggregate score is never a gate
  condition; Perfect-Score Gate added to the scoring anti-patterns
  (`scoring-framework.md`).

## [3.0.0] - 2026-07-14

### Changed

- Skill routing metadata reduced from 6,711 to 4,697 characters (652 words,
  max 35 per skill), with a 5,000-character corpus budget and 36 recorded
  front-tier direct/boundary selections bound to the exact description digest.
- Selective-install documentation now states the `/aep-*` dependency-closure
  requirement; the product group list includes `aep-watch`.

### Fixed

- `/aep-scaffold` frontmatter is valid YAML and is again discoverable, taking
  the skills CLI from 21/22 to 22/22 installed skills.
- Scaffold converge now propagates write failures, repairs Claude-only and
  Codex-only layouts, collapses only content-and-mode-identical duplicates,
  rejects symlinked parent escapes, preserves divergent copies, resolves its
  scripts from the installed skill directory, and honors confirmed A/C/E categories.
- Scaffold audit safely handles arbitrary skill directory names, detects
  divergent AEP copies, requires exact gitignore entries, and prunes dependency
  and metadata trees from health-endpoint discovery.
- Generated-resource markers reject traversal, symlink, and directory targets
  before the first mutation; dedicated fixtures prove outside files survive.
- `/aep-onboard` ships its mandatory orientation, `/aep-build` resolves its
  progress template from the installed package, and launch/liveness preserves
  backend-specific handles and host-compatible fallbacks.
- CI now performs a real copied install, asserts the exact marketplace/source
  set, validates all installed units with the official Agent Skills validator,
  checks installed-link containment, enforces metadata/evidence budgets, and
  runs 11 converge, 4 audit, and 3 generated-resource regression fixtures.
- The upgrade guide no longer recommends deleting real Claude skill
  directories blindly; it routes normalization through scaffold's fail-closed
  converge.

### Lean Skills refactor

**Lean Skills** — the full corpus restructured to the authoring standard in
[`docs/decisions/skill-authoring-standard.md`](docs/decisions/skill-authoring-standard.md)
(per-skill moves in
[`docs/plans/2026-07-14-lean-skills-refactor.md`](docs/plans/2026-07-14-lean-skills-refactor.md)).
Major bump: every SKILL.md changed shape and several reference files moved or
became canonical. Signal files and schema fields are unchanged. One structural
exception is recorded: `/aep-reflect` Step 5.5 was folded into Step 2's Process
branch while retaining its actions. Final 22/22 scenario dry-runs and their
loaded-reference/postcondition evidence live in `evals/skill-behavior-parity.json`.

#### Corpus changes

- **All 22 SKILL.md files leaned**: 8,209 → 4,873 lines (-41%); largest file
  955 → 398; every file now under the 400-line budget. Steps stay inline and
  end in world-derived postconditions; branch-specific reference is disclosed
  to `references/*.md`; every duplicated meaning collapsed to a one-line
  pointer at its canonical home (the `$BASE` resolver alone was inlined ~21×
  in two spellings — it now lives only in `/aep-git-ref`).
- **Descriptions dieted**: 1,687 → 652 always-loaded words (-61%), one trigger
  per branch, every description ≤ 50 words.
- **Canonical homes established** (R2): git conventions → `/aep-git-ref`;
  gen/eval contracts → `/aep-gen-eval`; full/light mode criteria →
  `/aep-design` `references/workflow-modes.md`; tick machinery →
  `autopilot/references/tick-protocol.md`; merge taxonomy →
  `build/references/merge-decision-cases.md` (ownership flipped);
  `.5`-alignment-layer concept → `map/references/alignment-layers.md`;
  SPLIT/V1 file resolution → `_shared/references/file-resolution.md` (new).
- **`scripts/build-skills.sh` rewritten to per-file materialization**: the
  `.aep-generated` marker is now a managed-file manifest, so skill-owned
  disclosure files coexist with build-managed shared copies in the same
  `references/` dir, shared files materialize into authored dirs, and name
  collisions fail loudly. References are selected per-file (a consumer ships
  exactly the shared files its SKILL.md names).
- **Rationale left the hot path** (R5): inline "Design Decisions" essays moved
  to `docs/decisions/` (`gen-eval-rationale.md`, `workflow-rationale.md`,
  `design-lens-rationale.md` new; executor's essays folded into its existing
  decision docs).
- **Cross-skill links standardized** to `/aep-x` prose invocations (R3),
  killing the dual-path breakage class (`../../patterns/…` vs
  `.claude/skills/aep-…`).
- **CI**: `skills-check.yml` gains a SKILL.md line-budget check
  (warn > 400, fail > 500) and a `$BASE`-resolver single-home check with a
  three-file runtime-bash allowlist.
- **Glossary**: new "Skill Authoring (v3)" section (Lean Skill, Progressive
  Disclosure, Single Source of Truth (skills), No-Op Line, Negation Steering,
  Context Load, Leading Word).

### Removed

- `_shared/references/orchestration-patterns.md` (zero consumers; superseded
  by autopilot's tick-protocol / state-schema / deterministic-orchestration
  references).
- `build/references/worktree-onboarding.md` removed — the launch-assembled
  bootstrap prompt and `/aep-build` Phase 0 carry the onboarding; after the
  lean pass the file had no remaining consumers.
- `calibrate/references/design-brief-template.md` removed — orphaned since the
  per-dimension `references/briefs/<type>.md` templates superseded it (see
  `docs/decisions/generalized-calibration-workflow.md`).

### Fixed

- Dangling `references/evaluator-criteria.md` pointer in `/aep-validate`
  (file never existed) — repointed to `/aep-gen-eval` `scoring-framework.md`.
- Broken `specs/**/*.md` glob in the scaffolded OpenSpec `propose` command
  body (markdown-escape leak).
- Stale "Phase 7: E2E Test Scripts" name in `build/references/progress-template.md`.
- Watch's YAML validation pointed at `../reflect/…` instead of its own
  materialized `yaml-guardrails.md`.
- The evaluator bootstrap brief now receives the launch run's resolved `$BASE`
  instead of re-deriving it (machine-assembled-brief discipline).
- `/aep-design` added to git-ref's control-plane-commits enumeration; the
  integration-branch override set/unset commands documented at the canonical
  home.

Post-review hardening (adversarially verified review round, 16/16 findings
confirmed and fixed):

- `scripts/build-skills.sh` write mode no longer overwrites a skill-owned file
  on a `_shared` name conflict, nor adopts it into the manifest — the conflict
  stays in place and the error repeats until one side is renamed (previously
  the first pre-commit retry silently committed the clobber).
- Legacy dir-level marker migration no longer dies silently (`set -e` +
  `pipefail`) when the dir holds files that don't mirror `_shared/`.
- `_shared/references/` selection is recursive — nested shared files
  materialize instead of silently never shipping while `--check` stayed green.
- The pre-commit `skills-build` job now runs `--stage`, staging exactly the
  files the build changed (lefthook's `stage_fixed` cannot stage newly
  materialized copies or removals, so commits landed with a stale tree).
- The `$BASE` auto-detect bash surviving in `/aep-onboard` — a second resolver
  spelling that also skipped the config override, reporting the wrong branch
  under `aep.integration-branch` overrides — replaced with the `/aep-git-ref`
  pointer; the CI resolver check now also greps the develop-probe signature so
  literal-free copies can't evade it.
- Remaining cross-skill relative links in reference files (tick-protocol ×4,
  post-merge-guard ×2, backends, theory-catalog) converted to `/aep-x` prose
  invocations (R3 — they broke in flattened `.claude/skills/aep-*` installs).
- Dispatch scoring formulas, grouped-change rules, and routing bands
  single-homed in `/aep-dispatch` `references/scoring.md` (new § Routing
  Thresholds); autopilot tick Step ⑥ and validate `protocol-specs.md` point
  there, and CHECK prompts include the file verbatim at assembly time.
- Heavy/light calibration-dimension membership single-homed in `/aep-map`
  `references/alignment-layers.md`; reflect, calibrate, dispatch, and envision
  now point there (the file's consumer claim was previously false).
- `merge-decision-cases.md` no longer claims `worktree-onboarding.md` carries
  the autopilot-vs-interactive mode rule (`signals-spec.md` is the home).
- Watch's liveness probe is `/aep-executor`-qualified with a non-circular
  fallback (retry once, then inline for the tick — previously "fall back" named
  the mode that had just failed).
- YAML validation standardized on `npx js-yaml` everywhere (envision and model
  used PyYAML — a second spelling with YAML 1.1/1.2 edge-case divergence).
- Executor's Reference Files index now lists `references/dogfood-validation.md`
  and the `spawn-liveness-probe.sh` script; the CI allowlist comment corrected
  (three reference files, not two).
- R7 implementation note added to the decision doc recording the accepted
  per-skill target deviations (corpus 4,873 vs ~4,455; CI budget is the
  enforceable ceiling).

## [2.7.0] - 2026-07-10

**Build-convergence pipeline + deterministic orchestration** — one release
implementing both decision docs upstreamed from SIBYL's 21-layer run:
[`docs/decisions/build-convergence-pipeline.md`](docs/decisions/build-convergence-pipeline.md)
and
[`docs/decisions/deterministic-orchestration.md`](docs/decisions/deterministic-orchestration.md).
Build-time runtime signal (lessons, gen-eval rounds, review findings, cost/PR
identity) no longer dies at `git worktree remove`, and the mechanical/judgment
boundary of the orchestration loop is now named, with its invariants annotated
and a pattern reference for moving them behind project-owned typed gates.

### Added

- **Convergence Gather — `/aep-wrap` step 2.5** (`agentic-development-workflow/wrap/SKILL.md`):
  before `/opsx:archive`, converge the workspace's runtime signal into the
  pre-archive change dir (`openspec/changes/<name>/convergence/` +
  `execution-record.yaml`) so the archive move carries it in one commit. Every
  field best-effort → explicit `null`; the step 5.5 `lessons-learned/` copy
  stays additive. New guardrails: **gather before archive**, **distill is
  idempotent**, and world-derived postcondition annotations on the step chain.
- **Layer Distillation — `/aep-wrap` → Reflect and Advance** + the same trigger
  in `patterns/autopilot/references/tick-protocol.md` → Layer Completion
  (autonomous-path parity): when a layer completes and
  `lessons-learned/retrospectives/layer-<N>.md` doesn't exist (world-derived,
  idempotent — both sites share the rule verbatim), an isolated subagent writes
  the retrospective + a shape-validated, **proposal-only**
  `lessons-learned/distillations/layer-<N>.yaml` (`refinements` /
  `skill_amendments` / `weak_areas`). The distiller never edits skill files.
- **NEW `agentic-development-workflow/wrap/references/convergence.md`** — the
  producer contract: parameterized gather manifest, `execution-record.yaml` +
  `distillation.yaml` schemas, distiller subagent protocol, shape-validation
  checklist. (Authored in wrap — `build-skills.sh` materializes `_shared/` only
  into `product-context/*`.)
- **`distillation` source adapter**
  (`product-context/_shared/references/telemetry-ingestion.md`, propagated to
  the 5 generated copies): file glob `lessons-learned/distillations/*.yaml`,
  dedupe-only (`external_id = distillation:<layer>:<hash>`, no high-water
  mark), per-item `suggested_class` hints — `refinements` → refinement,
  `skill_amendments` → process (**never auto-filed, never auto-applied**),
  `weak_areas` → discovery/refinement. Wired into `/aep-reflect` Step 1 (new
  "Layer distillations" source) and `/aep-watch` (source listing + no-high-water
  exception + never-auto-file rule; schema comment under `watch.sources`).
- **NEW `patterns/autopilot/references/deterministic-orchestration.md`** — the
  typed-gate pattern reference: the empirical law (prose steps drift; typed
  gates hold), the mechanical/judgment split table, named refusals
  (`REFUSING [tag]` + exit-code contract), world-derived resumability
  (postcondition probe catalog; effectful steps skip, gates always re-run),
  machine-assembled dispatch briefs, and an incremental path for projects to
  grow their own verbs.
- **Glossary** — eight new entries: Convergence Pipeline, Execution Record,
  Layer Distillation, Typed Gate (Typed Verb), Named Refusal, World-Derived
  Resumability, Machine-Assembled Dispatch Brief, Mechanical/Judgment Split.

### Changed

- **`product-context/dispatch/SKILL.md`** — Dynamic Workflow mode: named
  **[stale-base]** hazard (host `isolation: "worktree"` bases on stale
  `origin/<base>`, missing the dispatch-lock commit) + machine-assembled STEP-0
  brief requirement; Dispatch Lock annotated as the **[lock-before-workspace]**
  mechanical ordering invariant.
- **`agentic-development-workflow/launch/SKILL.md`** — the unpushed-commit
  ABORT tied to the base-freshness invariant class; Step 3 bootstrap declared
  machine-assembled (worktree self-check, verbatim spec block,
  untrusted-output guard).
- **`patterns/executor/references/backends.md`** + **`patterns/workflow/references/pattern-catalog.md`** —
  the AEP-worktrees-vs-host-isolation notes now name **[stale-base]** and
  mandate the STEP-0 rebase line for host-managed worktrees.
- **`patterns/autopilot/SKILL.md`** guardrails +
  `references/tick-protocol.md` — tick ordering invariants named
  (**[wrap-before-dispatch]**, **[one-wrap-per-tick]**,
  **[one-launch-per-tick]**) and pointed at the pattern reference.
- `docs/skills-quick-reference.md` — `/aep-wrap` and `/aep-reflect` rows
  reflect gathered execution records and layer distillations.

## [2.6.0] - 2026-07-10

**Add `/aep-design-lens` — a `patterns/` skill that grounds UI/UX design in
established HCI/design theory.** AEP had skills for UI _structure_ (`/aep-model`,
object-first IA) and human _taste_ (`/aep-calibrate`, captured decisions), but nothing
owned the _why_ — the external, evidence-based design theory those two apply, or a
heuristic health-check of whether a design meets human expectations. This fills that
gap without touching any schema.

### Added

- **`skills/patterns/design-lens/`** — a utility pattern skill (directly invokable and
  referenceable). Given any product, it runs a **Munzner-style, product-agnostic method**
  (characterize → abstract the user's tasks + data → select lenses → synthesize) and emits
  three things: design suggestions, a **design guideline** to build against, and a
  **severity-scored (Nielsen 0–4) health-check table**. Two depths: a **quick check**
  (the Baseline Ten — ten distilled cross-family rows, with an escalation rule on any
  major finding) and the full **deep audit**. Output is **hybrid** — an advisory
  report by default, an opt-in standalone markdown file on request. It writes **no**
  `product-context.yaml` / schema fields (zero-drift leaf skill). SKILL.md states five
  explicit design goals: relevant (lens selection), traceable (`→ lens id` on every
  statement), honest (severity triage, observed evidence), actionable (Top 5), cheap
  to invoke (quick check).
- **`references/theory-catalog.md`** — an extensible catalog of ~40 established lenses in
  seven families: A usability heuristics (Nielsen, Shneiderman's golden rules, Norman) ·
  B cognitive/perceptual laws (Cognitive Load, Fitts, Hick, Gestalt, …) · C information
  seeking (Shneiderman's mantra, Information Foraging, Focus+Context) · D data
  visualization (Munzner, Graphical Perception, Heer & Shneiderman, Tufte) · E human-AI /
  agent UX (Gulf of Envisioning, Microsoft's 18 HAI guidelines, trust calibration,
  steerability) · F evaluation methods (heuristic evaluation, cognitive walkthrough,
  0–4 severity) · G accessibility & inclusive design (WCAG 2.2 POUR, keyboard & focus,
  contrast/color independence, the impairment spectrum — always fires, incl. terminal
  output). Documents how to add a lens.
- **`references/method-and-templates.md`** — the 7-step method, the family-selection rules,
  the Baseline Ten quick check, a live-UI evidence-gathering protocol (walk top tasks,
  evidence per finding, unhappy paths, cheap G probes; agent-browser for deployed UIs),
  worked selection examples, and the guideline / health-check / persisted-file templates.
- **Prose cross-links** (no schema/enum changes) from `/aep-model`, `/aep-calibrate`
  (ux-flow), and `/aep-validate` pointing to the new skill as the design-theory /
  design-quality complement. Glossary entries for **Design Lens** and **Heuristic
  Health-Check**.

### Fixed

- **Doc-listing drift (pre-existing):** README "All Skills" was missing `/aep-validate`,
  `/aep-watch`, and `/aep-workflow-feedback`; orientation and the quick reference were
  missing `/aep-executor`; stale skill counts ("17"/"20") corrected to 22. All listings
  now enumerate the same 22 skills.

## [2.5.0] - 2026-06-28

**Fix the autopilot "PR-ready stop" + "feat branch in the main checkout" failure
mode (latent since v1.6.0, surfaced on the Codex backend).** An autopilot worker
could build a story, open a CLEAN PR with no required checks, and **stop at "PR
ready"** instead of completing Phase 12 merge + wrap — and, upstream of that, could
build in the **main checkout** without ever creating a worktree. Root cause: the
worktree binding and the autopilot-mode decision were both unenforced soft
contracts inferred from cwd, `/aep-build` had no entry guard, and a global "always
confirm before merging" guardrail contradicted Phase 12's autopilot exception. On
Codex `codex-subagent` (`spawn_agent` has no cwd parameter) the failure was
near-deterministic.

### Added

- **Worktree entry guard** in `/aep-build` Phase 0 (and `worktree-onboarding.md`):
  verifies the worktree is exactly `.feature-workspaces/<name>` on `feat/<name>` and,
  if not, **enters the worktree `/aep-launch` already created** (anchored to the main
  repo root, so it works from any cwd). Worktree creation stays with `/aep-launch`;
  the guard never creates branches/worktrees and never mutates the main checkout — if
  no worktree exists it **escalates**. Being `<name>`-specific, it also catches a
  worker sitting in the wrong (sibling) feature worktree.
- **Explicit autonomy marker** `.dev-workflow/signals/mode` written by `/aep-launch`
  and read by Phase 12 (anchored to the worktree root) — a robust, backend-independent
  source of truth for the merge decision that does not depend on the worker's cwd.
  Documented in `signals-spec.md`.
- **`merge-decision-cases.md`** regression fixture pinning: clean PR + no required
  checks ⇒ worker merges (orchestrator wraps), not stop; worker outside its worktree
  ⇒ guard enters the launch-made worktree or escalates.

### Fixed

- **`/aep-build` self-contradiction:** the global "always confirm before merging"
  guardrail and the worker-facing onboarding Key Rule now carry the autopilot
  caveat (confirm only in **interactive** mode), no longer overriding Phase 12.
- **Phase 12 detection** uses the `mode` marker (read **anchored to the worktree
  root**) as the **sole authority** — cwd is never a mode signal (it's a soft contract
  under Codex `codex-subagent`, and a build phase may have `cd`'d into a subdir). A
  missing/ambiguous marker defaults to **interactive (ask)** — never auto-merge when unsure.
- **Readiness keys on `mergeStateStatus`**: CLEAN ⇒ proceed; UNKNOWN ⇒ re-read (the
  normal transient right after the rebase/force-push); UNSTABLE ⇒ proceed only if no
  check is **failing** (don't land red code even when the repo configured no _required_
  checks); BLOCKED/DIRTY ⇒ stop. Replaces the blunt "CI green" gate while still not
  parking a CLEAN, no-required-checks PR.
- **Asymmetric merge contract:** the orchestrator's "main NEVER merges" is now
  paired with an equally prominent positive worker obligation ("the worker MUST
  complete Phase 12; 'PR ready' is not a worker stop point"), so the main-only
  prohibition can't leak into a shared-session Codex worker. Merge nudges
  (`tick-protocol.md`, `autopilot/SKILL.md`) enumerate the full 6-item stop-condition
  list (including the human-approval gate and policy pause) — no half-applied subset.
- **Post-merge boundary:** the worker ends at merge + `status.json` `completed`; it
  must **not** run `/aep-wrap` itself (wrap's `/opsx:archive` runs on the integration
  branch — the human in interactive mode, the orchestrator's next tick in autopilot).
- **Codex `aep-builder` role** now verifies its worktree and points at the Phase 0
  guard as the backstop for the soft cd contract.
- **`.gitignore`:** the unanchored `build` pattern silently ignored new files inside
  the `build/` skill directory; replaced with root + per-workspace build-output
  anchors (`/build/`, `apps/*/build/`, `packages/*/build/`) that don't match the
  skill source dir.

## [2.4.0] - 2026-06-26

**Decouple journey AUTHORING from journey EXECUTION** so a layer's BDD journey is
a build-time deliverable, authored from acceptance criteria **before** dogfood and
committed pre-merge — never silently deferred. v2.3.0 coupled the two: `/aep-build`
Phase 6 ran the dogfood first and only "auto-remediated" coverage after, and Phase 7
"codified what Phase 6 exercised". Under `journey_timing: post-deploy` that deferred
**both** authoring and execution to the post-deploy gate, so a net-new layer could
reach its gate with **no journey file at all** — and with `full_auto: false` that
gate is a human handback, leaving the gap invisible. Now `journey_timing` governs
**only when the journey is EXECUTED** against the target; the journey FILE is always
authored pre-merge in the layer's build. Downstreams pick this up on their next
deliberate re-pin.

### Added

- **`cli` dogfood target — CLIs are now dogfoodable, not journey-less.** New
  `dogfood_target: cli` and journey `target: cli`, driven by a **bash** tool track in
  `tool-selection.md` / `e2e_tool()` — at the same level as choosing agent-browser or
  codex-native for web. A CLI journey runs the built binary as a user would and verifies
  **exit code / stdout / stderr / filesystem**; an agent invoking the CLI is the same as
  a human typing it. CLI/library projects move from `applicable_tiers: [1]` to `[1,2]`
  and get a real Tier-2 gate. `dogfood_target: none` now means **no runnable surface at
  all** (config / schema / docs repo) — a rare case, not the CLI default. Touches the
  canonical `e2e_tool()` (`patterns/executor/references/dogfood-validation.md`),
  `tool-selection.md` / `policy.md` / `three-tier-model` / `bdd-journeys` templates &
  references, and the `aep-e2e-skill-scaffolding` project-type decision.

### Changed

- **`/aep-build` Phase 6 is now journey-first.** Step A **authors/extends the journey
  from `stories[].acceptance_criteria`** (one scenario per criterion, each `Then` → a
  concrete `Verify`, tool-agnostic) **before any dogfood** and commits it pre-merge —
  unconditional and independent of `dogfood_target` / `journey_timing`. Step B then
  **executes** the authored journey per `dogfood_target` and confirms coverage. Under
  `journey_timing: post-deploy`, Phase 6 still authors + commits the journey now; only
  its execution against the deployed target is deferred to the `/aep-wrap` gate.
- **`/aep-build` Phase 7 renamed** "Codify the Journey" → **"Finalize the Journey"**:
  it refines the already-authored journey with reality discovered during execution
  (selector / route drift, extra `Verify` lines), then records the gate evidence.
- **`aep-map` records journeys as planned deliverables.** Each `layer_gates[N]`
  carries a new **`journeys:` list** (the pre-merge journey file(s) the build authors
  from that layer's acceptance criteria, one per capability area; an empty list when
  `dogfood_target == none`).
- **Docs aligned on "author pre-merge, execute per timing"** across `aep-map`,
  `aep-build` (Phase 6 & 7), `aep-wrap`, `aep-autopilot`, and the
  `aep-e2e-skill-scaffolding` `policy.md` template + `layer-gate-loop.md` /
  `three-tier-model.md` / `bdd-journeys.md` references.

### Fixed

- **`/aep-wrap` backstop:** a layer that reaches its gate with **no journey file** is
  now a **COVERAGE FAILURE** — `/aep-wrap` surfaces it and refuses to flip the gate to
  `passed` (it stays `scripted_passed` and routes back to build), instead of silently
  passing or assuming a journey that was never authored. The gate **executes, never
  authors**.

## [2.3.0] - 2026-06-25

Promote the downstream **BDD layer-gate E2E pattern** into AEP and make project
setup **idempotent**. The e2e-test skill a project gets is now a natural-language
**journey** library (Given/When/Then/**Verify**), tool-agnostic, with each journey
mapped to a **layer gate** so quality accrues layer by layer. Each gate is
**two-phase and coverage-checked** — `scripted_passed` (framework tests green) →
`passed` (all applicable tiers green _and_ every layer acceptance criterion proven by
a test); `/aep-build` **auto-authors** any missing scenario to close a coverage gap
and `/aep-wrap` asks the human before advancing. It ships in
**canonical cross-tool form** (real `skills/e2e-test/` + `.claude/skills` /
`.agents/skills` symlinks) so Claude Code, Codex, and Pi all see one copy. Which
browser/device tool drives the UI is resolved by a **separate `tool-selection.md`**
across four tracks (agent-browser/playwright/codex web set, webwright, agent-device
mobile, desktop/computer-use). Also retires `/aep-testing-guide` (its content is
redistributed, so no capability is lost) and the legacy `sync.sh` /
`sync-downstream.sh` / `migrate-downstream-layout.sh` scripts (canonical install is
the `skills` CLI). Downstreams pick this up on their next deliberate re-pin.

### Added

- **`/aep-e2e-skill-scaffolding` skill** (`project-setup`): generates/upgrades a
  project's `e2e-test` skill into the BDD layer-gate **three-tier** shape (scripted
  gates / journey dogfood / API drivers), in canonical cross-tool placement. Ships
  templates (`e2e-test.SKILL.md`, `journeys/README.md`, `00-walking-skeleton.md`,
  `tool-selection.md`, `seed.sh`) and references (`bdd-journeys.md`,
  `three-tier-model.md`, `layer-gate-loop.md`). Idempotent; migrates a legacy
  `.claude/skills/e2e-test` real dir into `skills/` and never overwrites hand-written
  journeys. Registered in `marketplace.json` `project-setup` plugin.
- **`scaffold/references/workspace-hook.md`**: the workspace-setup hook contract +
  template (salvaged from the removed testing-guide Part 1).
- **Coverage-checked layer gates**: the generated `e2e-test` skill ships a
  `layer-gate-evidence.template.md` (acceptance-traceability + scripted-coverage
  matrices + dogfood checklist + waivers), and journeys carry a `covers:` front-matter
  field tying each to the acceptance criteria it proves. "Coverage" is
  acceptance/requirements coverage — every layer criterion proven by ≥1 test — not a
  line/branch %.
- **Per-project E2E policy** (`policy.md` in the generated skill): single source of
  truth for _applicable tiers_, _dogfood target_ (`none` / `local` / `deployed:<url>`),
  and _journey timing_ (`pre-merge` / `post-deploy`). The generator proposes from the
  stack and **confirms with the user**; `/aep-build` and `/aep-wrap` read it — a
  CLI/`none` project gets no journey tier (and no `tool-selection.md`), while a
  pre-release app can dogfood post-deploy against a deployed (e.g. Cloudflare) URL.
  Skill-managed, never silently overwritten, and **no `AGENTS.md` copy** — the skill is
  canonical cross-tool, so every runtime reads the same file.

### Changed

- **`e2e_tool(target_type)`** (`patterns/executor/references/dogfood-validation.md`):
  generalizes the web-only `dogfood_method()` over **web / mobile / desktop** targets,
  adding **webwright** (web) and **agent-device** (mobile) plus per-target health
  probes and a `topology.routing.e2e.tool.*` pin. `dogfood_method()` is kept as a
  `:= e2e_tool('web')` wrapper, so `/aep-build` Phase 6 and the post-merge guard are
  unchanged. The generated `tool-selection.md` is a self-contained projection of it.
- **`/aep-scaffold`**: Phase 8 now **delegates** to `/aep-e2e-skill-scaffolding`; the
  existing-project path is an **idempotent audit → confirm → converge** flow that
  repairs drift (canonical skills layout, e2e-test shape, infra) and **recommends**
  (never auto-runs) the skills-CLI version re-pin.
- **`/aep-build`** Phases 6–8: reference BDD **journeys** + `e2e_tool(target_type)` and
  record the layer-gate evidence, instead of generating one-off bash `<feature>-e2e.sh`.
  Phase 6 now **computes the layer's coverage matrix and auto-authors missing tests** to
  close gaps; Phase 8 replays prior-layer journeys (regression) and checks coverage.
- **`layer_gates` schema** (`product-context-schema.yaml` + the generated gate loop):
  reconciled to one canonical list shape and enriched with a two-phase `status`
  (`scripted_passed` → `passed`), a `coverage` block (`criteria_total` /
  `criteria_covered` / `uncovered`), and structured `evidence` (`scripted` / `journeys`
  / `matrix`). Older gates without these keys still parse.
- **`/aep-dispatch` + `/aep-wrap`**: dispatch reports a `scripted_passed` gate as
  "machinery green, dogfood pending" (still blocks the next layer); wrap performs the
  **two-phase flip** (all applicable tiers green + coverage complete-or-waived +
  regression replay), then **asks the human before advancing** to the next layer.
- **`/aep-workflow-feedback`**: pushes skill improvements downstream via a deliberate
  skills-CLI **re-pin** (README upgrade flow) instead of the removed `sync-downstream.sh`.

### Removed

- **`/aep-testing-guide` skill** — content redistributed into
  `/aep-e2e-skill-scaffolding` references and `scaffold/references/workspace-hook.md`.
- **Legacy maintainer scripts** `scripts/sync.sh`, `scripts/sync-downstream.sh`,
  `scripts/migrate-downstream-layout.sh` — superseded by the `skills` CLI as the
  canonical install/upgrade mechanism.

## [2.2.0] - 2026-06-23

Add a **dynamic-workflow pattern** — `/aep-workflow` — that codifies "a harness for
every task": when (and when not) to have Claude write a custom multi-agent harness
for a task, plus the reusable sub-pattern catalog (classify-and-route,
fan-out-and-synthesize, adversarial verification, generate-and-filter, tournament,
loop-until-done). It frames workflows as a structural fix for three failure modes of
long single-context work — agentic laziness, self-preferential bias, and goal drift —
and cross-links the narrow `aep-executor` `workflow` backend, `/aep-gen-eval`
(adversarial verification), and `/aep-autopilot` (the long-lived loop) rather than
duplicating them. Source and rationale:
[`docs/research/dynamic-workflows.md`](docs/research/dynamic-workflows.md).

### Added

- **`/aep-workflow` skill** (`patterns`): dual library + standalone skill (same shape
  as `/aep-gen-eval`) covering the failure modes, the "should I even use a workflow"
  judgment, invocation (`ultracode`, "…with workflow", `/loop` + `/goal`, token
  budgets, save-as-template), and the AEP touchpoints. Registered in
  [`marketplace.json`](.claude-plugin/marketplace.json) `patterns` plugin.
- **Sub-pattern catalog** (`patterns/workflow/references/pattern-catalog.md`): the six
  sub-patterns with intent, the Workflow primitive to use (`parallel` barrier vs
  `pipeline` no-barrier vs loop), AEP examples, and skeletons; plus architectural
  levers (per-agent model, worktree isolation, quarantine, token budgets, resumability).
- **Research note** `docs/research/dynamic-workflows.md` (linked from the README
  Documentation list).

### Changed

- **`/aep-executor`** (SKILL.md + `references/backends.md`): the narrow "Mode:
  workflow" now points to `/aep-workflow` for the general pattern catalog and the
  "when to reach for a workflow" judgment.
- **`/aep-gen-eval`**: notes that adversarial verification in `/aep-workflow` is the
  generalized, N-verifier form of generator/evaluator separation.

## [2.1.0] - 2026-06-16

Add an **object-first design stage** — `/aep-model` — that turns the verb-first
story map into a noun-first **Object Map** (OOUX/ORCA) before UI is built, so build
agents stop inventing one-step-one-screen task-wizard UIs. The structural UI plan
(objects, attributes, relationships, CTAs, screens) is auto-drafted from artifacts
AEP already produces, human-approved at a short gate, then governs build — leaving
only taste (look/voice/journey) to `/aep-calibrate`. Background and the verb-first
vs noun-first analysis: [`docs/research/ooux-object-modeling.md`](docs/research/ooux-object-modeling.md).

### Added

- **`/aep-model` skill** (`product-context`): runs ORCA (Objects → Relationships →
  Calls-to-action → Attributes → screens) to draft an Object Map, takes a short
  human review gate (object boundaries, primary anchor, task-flow exceptions), and
  writes the approved noun-first blueprint. Sits between `/aep-map` and
  `/aep-dispatch` for UI-facing products. Registered in
  [`marketplace.json`](.claude-plugin/marketplace.json) `product-context` plugin.
- **Object Map artifacts + schemas**: `product/object-model.yaml` (cross-capability
  object ontology) and `product/maps/<capability>/object-map.yaml` (capability-scoped
  ORCA/IA projection), via `_shared/templates/object-model-schema.yaml` and
  `object-map-schema.yaml`. Object-first is the default; task-oriented flows are an
  opt-in escape hatch recorded with a reason.
- **ORCA reference** (`product-context/model/references/orca-process.md`):
  round-by-round derivation from AEP inputs + the object-first/task-oriented decision
  framework and the completeness checks.
- **Glossary terms**: Object Model, Object Map, ORCA, Call-to-Action (CTA), Nested
  Object Matrix, Object-First vs Task-Oriented.
- **Research note** `docs/research/ooux-object-modeling.md` and a `Research` category
  in [`docs/README.md`](docs/README.md).

### Changed

- **Schema** (`product-context-schema.yaml`): adds `stories[].object_model_refs`,
  `stories[].capability`, `architecture.modules[].kind`, and the `object-model`
  quality dimension. Object-map approvals are tracked as thin `calibration.history`
  references — the artifact bodies stay under `product/`, not inlined into
  `product-context.yaml`.
- **`/aep-envision`**: declares the `object-model` structural gate by default for
  UI-facing products.
- **`/aep-map`**: auto-drafts Object Maps after decomposition; sets `module.kind` +
  `story.capability`; flips an approved map to `stale` on re-decompose; routes Next
  Step to `/aep-model`.
- **`/aep-dispatch`**: injects the minimal Object Map slice into a story's context
  package and refuses UI-facing stories without an approved (non-stale) map.
- **`/aep-launch`**: aborts a UI-facing story when no approved Object Map covers it.
- **`/aep-build`**: UI implementation obeys the injected Object Map slice (object structure and CTA grammar; taste still from calibration).
- **`/aep-validate`**: Mode A gains Object Map completeness checks (coverage, object
  homes, anchors, task-flow justification, ref resolution).

All additions are backward-compatible — the object-model path only engages for
UI-facing products that opt in.

## [2.0.1] - 2026-06-16

Operationalize the **dogfood → reflect classifier → story** link so the G6
self-feeding loop fires for **every** dogfood trigger — not just the autopilot
post-merge guard. Previously "feed the report to the `/aep-reflect` classifier"
was prose-only: no adapter parsed the unified markdown report into the classifier's
normalized record, so a standalone / ad-hoc dogfood (and even the guard path) left
findings on disk with no auto-filing. A dogfood that surfaced real bugs would stop
and wait for a human to hand-author stories.

### Added

- **`dogfood_report` source adapter** (`product-context/_shared/references/telemetry-ingestion.md`):
  parses each `##` finding in `.dev-workflow/dogfood-*.md` (unified
  severity/category/repro format) into the normalized observation record the
  `/aep-reflect` Step 2 classifier consumes. Maps Severity → priority, Category →
  `suggested_class` hint, and assigns a deterministic
  `external_id = dogfood:<report>:<hash>` so re-running the same dogfood never
  duplicates stories. Self-describing file glob — not gated by `coverage_check`.
- **`dogfood_report` watch source** (`/aep-watch`): a standalone, local, or
  post-deploy dogfood report is now ingested headlessly — classified, deduped, and
  (under `full_auto` / `watch.auto_create`) auto-filed as a bug/refinement story
  that autopilot dispatches on its next tick. Calibration / discovery /
  opportunity-shift / process findings still surface to a human.

### Changed

- **`dogfood-validation.md`** "On issue" — replaced the prose assertion with the
  concrete adapter + watch ingestion path, and made the report-file path an
  explicit contract: findings left only in chat are a dead end.
- **`/aep-reflect` Step 1** — adds `.dev-workflow/dogfood-*.md` to the gathered
  feedback sources (was omitted), normalized via the same adapter `/aep-watch` uses.
- **`/aep-build` Phase 6** — write the dogfood report file even when findings are
  clean, since that path is the ingestion contract.

## [2.0.0] - 2026-06-16

The **autonomy loop** release. Closes the loop-engineering gaps identified in
`docs/research/loop-engineering-autonomy-gap.md` (G2–G7) and adds a `full_auto`
master switch. Every new capability defaults to **human-in-the-loop** — autonomy
is opt-in via `topology.routing` flags.

### Added

- **`/aep-watch` skill** (G6) — continuously ingests telemetry / error streams /
  bug trackers, classifies findings with the `/aep-reflect` classifier, and
  auto-files bug/refinement stories so reflect→dispatch becomes self-feeding.
- **Change-strategy recovery ladder** (G2) — `gen-eval/references/recovery-ladder.md`;
  on repeated eval FAIL the build climbs same-fix → re-ground → fresh
  `native-bg-subagent` generator → decompose **before** the `eval_not_converging`
  human gate.
- **Host-aware post-deploy dogfood** (G4b) — `executor/references/dogfood-validation.md`:
  `dogfood_method()` (Claude → agent-browser; Codex → native in-app browser /
  computer-use, or Playwright headless) + `target_url()` (config-first, CI fallback).
- **Post-merge guard** (G4a) — `autopilot/references/post-merge-guard.md` + tick
  Step ③.5: monitors merged stories' deploy health; dogfood issues → reflect story;
  hard regression → conservative `auto_revert` (default off, warn + escalate).
- **Telemetry-driven reflect** (G5) — `reflect/references/telemetry-ingestion.md`:
  automated source ingestion + quantitative outcome-contract auto-evaluation.
- **Telemetry source determination** — projects decide sources via a hybrid
  metric-driven rule: `/aep-scaffold`/`/aep-onboard` detect the observability stack
  (candidate sources); `/aep-map` binds each quantitative `success_metric` +
  `health_signal` to a source (`metric_map`); a shared `coverage_check()` lets
  `/aep-watch`, `/aep-reflect`, and the post-merge guard **block auto when the
  binding is incomplete** instead of silently no-op'ing.
- **Visual Design evaluator dimension** (G3) — vision-model scoring of screenshots
  against the design system, for both Claude and Codex (multimodal).
- **`full_auto` master switch** (A1) — `topology.routing.full_auto` (default false)
  gates the strategic human pauses (design escalation, qualitative outcome eval);
  implies `auto_design` + `auto_outcome_eval` + `watch.auto_create`. New config keys
  added to the product-context schema.

### Changed

- `/aep-build` Phase 5 climbs the recovery ladder; Phase 6 dogfood is host-aware
  (degrades instead of skipping when agent-browser is absent).
- `/aep-reflect` Step 1 supports automated ingestion; Step 2.75 auto-evaluates
  quantitative outcome contracts (qualitative still pauses unless `full_auto`).
- `/aep-autopilot` gains the post-merge guard step and `full_auto`-aware routing;
  loop hygiene unified on `--max-turns` (G7).

### Fixed

- Carries forward the v1.8.0 executor fix (claude-team removed; `native-bg-subagent`
  default + post-spawn liveness probe). Every new spawn path uses it.

## [1.8.0] - 2026-06-15

### Changed

- **Executor: removed the `claude-team` backend.** Its agent-teams spawn path
  fails silently on Claude Code ≥ 2.1.x — the launch command is truncated in a
  detached `claude-swarm` tmux pane and never submitted, so no worker starts,
  yet the team roster still reports the member "active". `native-bg-subagent`
  replaces it as the Claude Code default. The agent-teams env flag is no longer
  consulted and there is no "…with agent team" opt-in. See
  [`docs/decisions/remove-claude-team.md`](docs/decisions/remove-claude-team.md).
- Orphan/stuck detection now decides liveness by the **real-liveness probe**
  (process/agent exists AND worktree shows activity), never by roster/state
  membership.

### Added

- **`native-bg-subagent` executor backend** (Claude Code default): Agent tool
  with `run_in_background: true`, no team. Success signature is a bare-hex
  `agentId` + JSONL `output_file`; steered via `SendMessage(to: agentId)` +
  `feedback.md`; human gate is gate-and-park.
- **Mandatory post-spawn liveness probe** with auto-fall-back to
  `native-bg-subagent` on failure
  (`skills/patterns/executor/scripts/spawn-liveness-probe.sh`).
- Explicit **one launch = one worktree = one subagent = one story** invariant in
  `/aep-launch` (the `compile_mode: grouped_change` story group is the one
  documented exception).

### Fixed

- `claude-bg` is now gated on `BG_AVAILABLE`; documented that the `claude --bg`
  one-shot spawn flag was removed on Claude Code ≥ 2.1.x (so claude-bg is skipped
  there and native-bg-subagent is used).

## [1.7.0] - 2026-06-11

**Goal-driven autopilot driver**: `/aep-autopilot` now keeps itself ticking with
a **goal driver** by default — the host-native `/goal` primitive (Claude Code
v2.1.139+ and Codex's experimental `goals` feature) — which re-fires a tick when
there is work and **self-terminates** when the current layer is complete or a
human-judgment gate is hit. The fixed-interval `/loop` driver is retained as a
fallback (`--loop`). Only the _driver_ changes; the 7-step CHECK→ACT tick, the
delegated cheap CHECK, the signals protocol, and the orchestrator boundary are
unchanged. Decision record:
[`docs/decisions/goal-driven-autopilot.md`](docs/decisions/goal-driven-autopilot.md).

### Added

- **Goal driver (default)** — `/aep-autopilot` with no `--loop` flag builds a
  one-layer goal condition and drives it via `/goal`: "layer N complete (all
  stories merged + wrapped) OR autopilot paused". Scoped to **one layer per run**
  — it stops at the layer boundary so the human runs the layer gate / `/aep-reflect`
  and re-invokes for the next layer. Native and near-symmetric on both hosts
  (Claude Code Haiku-evaluator Stop hook; Codex persisted thread goal with
  `token_budget`).
- **Per-tick surface + wait tail (step ⑦, goal driver only)** — each tick
  surfaces a **signals-only** `AUTOPILOT …` status line for the goal evaluator to
  judge (boundary-safe: never workspace code), then waits a bounded **floor**
  (default `5m`, `--floor`) before ending the turn. The floor is the anti-hot-loop
  mechanism — CC uses `Monitor` with a hard timeout (a raw foreground `sleep` is
  blocked in a turn); Codex uses shell `sleep`.
- **Goal-driver flags** — `--floor <dur>` (per-tick wait floor) and
  `--max-turns <n>` (runaway backstop, default `200`); on Codex a `token_budget`
  is set as the hard wall (soft-stops to `budget_limited`).

### Changed

- **`/aep-autopilot` default behavior** — the default driver is now goal-driven
  and self-terminating per layer. The command surface is unchanged; `--loop
<interval>` selects the prior fixed-interval behavior exactly.
- **`/aep-autopilot stop`** — cancels whichever driver is active (`/goal clear`
  for the goal driver; `/loop` cancel or cron/launchd removal for the loop
  driver).
- **Driver × backend compatibility** (executor `backends.md`) — the long-lived
  session class now names two in-session variants, `/goal` (default) and `/loop`;
  the goal driver is in-session-only, so the cron/launchd row stays the
  `/loop` / `codex exec` path.

## [1.6.0] - 2026-06-10

**Native-first executor backends**: launch/dispatch/build/autopilot/wrap now
target each host's native parallel-agent machinery instead of tmux+cmux. The
B1–B4 ladder is replaced by named launch modes; every mode still runs its
worker in the AEP-created worktree at `.feature-workspaces/<ws>` with the
file-based signals protocol as the source of truth. Decision record:
[`docs/decisions/native-first-executor.md`](docs/decisions/native-first-executor.md).

### Added

- **`claude-team` mode** — Claude Code agent teams: one teammate per story in a
  **standing team** (TeamCreate once; spawn/shutdown teammates per tick), push
  steering via `SendMessage`, native display via `teammateMode`. Requires the
  experimental `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS` flag (project
  `.claude/settings.json` `env` block; `/aep-onboard` documents it).
- **`claude-bg` mode** — Claude Code native background sessions
  (`claude --bg` / `attach` / `logs` / `stop` / `respawn`): the GA fallback;
  OS-bound, cron-driver compatible, worktree-enforced by process cwd.
- **`codex-subagent` mode** — Codex multi_agent workers (desktop app + CLI)
  with shipped `aep-builder` / `aep-evaluator` roles (`.codex/agents/*.toml`),
  `send_input` steering, and the native approval overlay as a human gate.
- **`codex-exec` mode** — headless `codex exec --cd <worktree>` workers,
  steerable across sessions via `codex exec resume <id>`; the Codex mode for
  cron-driven autopilot and hard cwd isolation.
- **Human-gate protocol (hub-and-spoke)** — `.dev-workflow/signals/needs-human.md`
  - `status.json` `blocked_on: "human"`: host-agnostic record of decisions only
    the human can make. The **main agent is the canonical human console**: the
    question flows back to the orchestrator, the human answers there, and the
    answer is relayed per mode — by push on steerable modes
    (**block-in-place**), or by resuming a parked worker into its worktree on
    batch/pull modes (**gate-and-park**: the worker commits WIP, records the
    gate, and ends its run cleanly). Direct worker surfaces (teammate pane,
    `claude attach`, Codex thread, `tmux attach`) are optional conveniences.
    Gated workspaces count as waiting, not stuck.
- **Workflow mode is now a complete backend** — gate-and-park gives the
  dynamic-workflow fan-out a human-gate path: build agents return a structured
  `gated` result instead of guessing or stalling; the main agent collects
  gated stories after the run, asks the human, and resumes them into their
  existing worktrees (continuation via `resumeFromRunId` or re-launch). New
  "Mode: workflow" recipe in `backends.md`; the `/aep-dispatch … with
workflow` path now creates the `.feature-workspaces/<ws>` worktrees and
  announces gate behavior instead of "no mid-flight feedback".
- **Orphan re-adoption** — lead restarts no longer strand session-bound
  workers: autopilot re-spawns into the existing worktree with the recovery
  bootstrap instead of failing the story.
- New executor references: `claude-native.md`, `codex-native.md`,
  `tmux-session.md`; new `gate()` operation; driver × backend compatibility
  matrix (session-bound vs OS-bound worker lifetimes).
- Autopilot state: `backend`, `agent_id`, `last_liveness_hash` (generalizes
  `last_tmux_hash`), `human_gate` escalation type, `readopted` action.

### Changed

- **Behavior change:** Claude Code with tmux installed no longer auto-selects
  tmux — native modes win. Pin the old workflow with
  `git config aep.executor-backend tmux` (or "…with tmux"). tmux+cmux recipes
  are preserved verbatim as the **`legacy`** mode (generic-host fallback).
- Autopilot accepts any steerable, driver-compatible mode (was: B1/B2 only);
  Codex autopilot is now supported (in-thread ticks → codex-subagent; scheduled
  `codex exec` ticks → codex-exec).
- `/aep-build` Phase 5 evaluator spawn is mode-dispatched: foreground Task
  subagent (Claude native modes) or `codex exec --cd` with the aep-evaluator
  role (Codex modes); the evaluator prompt is delivered at spawn time — no
  sleep/poll/kill-pane on native modes.
- `/aep-wrap` teardown is mode-dispatched (teammate shutdown / `claude stop` /
  `close_agent` / no-op / `tmux kill-session`).
- **Command naming normalized to `/aep-*`** across all skills, README, and
  docs: every AEP skill is invoked by its canonical registered name
  (`/aep-autopilot`, `/aep-dispatch`, `/aep-launch`, …) — the unprefixed forms
  (`/autopilot`, `/launch`) no longer appear, eliminating the prefix confusion.
  Non-AEP commands (`/loop`, `/opsx:*`, Codex `/agent`, `/workflows`) are
  unchanged. (Changelog entries v1.5.0 and older keep their original wording
  as historical record.)

## [1.5.0] - 2026-06-09

Configurable **integration branch**: AEP no longer hardcodes `main`. The branch
that feature work is based off, rebased onto, PR'd into, and where control-plane
commits land is now resolved per-repo, with `main` as the default — so GitFlow
repos (`develop` = integration/staging, `main` = production) work out of the box.

### Added

- **Integration-branch resolution (`$BASE`).** Every git-touching skill resolves
  the integration branch in this order: explicit override
  `git config aep.integration-branch <name>` → auto-detect `develop` if it exists
  → `main`. `git config` is repo-local and shared across worktrees, so `$BASE`
  resolves identically in the main session and inside `.feature-workspaces/`.
- **Two modes, auto-detected.** _single-branch_ (no `develop`): integration =
  production = `main` (unchanged default behavior). _two-branch_ (`develop`
  exists): integration = `develop`; production `main` is promote-only and AEP
  never touches it — promotion `develop` → `main` stays the user's CI/CD or PR
  step, exactly like deployment.
- `aep-git-ref` gains an "Integration Branch" section documenting `$BASE`, the
  modes, the resolver, and the `aep.integration-branch` config key.
- `/onboard` Phase 5 detects the mode and persists `aep.integration-branch`;
  `/scaffold` sets it to `main` for fresh repos.

### Changed

- `/design`, `/launch`, `/build`, `/wrap`, `/dispatch`, `/reflect`, `/calibrate`,
  `/autopilot` (+ tick-protocol), and the executor backend recipe now substitute
  `$BASE`/`origin/$BASE` for the previously hardcoded `main`/`origin/main` in
  worktree creation, rebases, PR base (`--base "$BASE"` / `--target-branch`),
  and control-plane commits. Default-mode behavior is unchanged (`$BASE` = `main`).
- `.claude-plugin/marketplace.json` version `1.4.0` → `1.5.0`.

## [1.4.0] - 2026-06-05

Codex `/launch` now uses Codex-native, worktree-bound subagents for coding
launches by default, while Claude/generic executors keep the tmux session path.

### Changed

- `aep-executor` backend selection now chooses B3 for Codex hosts before checking
  cmux/tmux, so Codex coding launches no longer create tmux sessions just because
  tmux is installed.
- The B3 spawn recipe still creates the standard AEP worktree first
  (`.feature-workspaces/<name>` on `feat/<name>`), then starts the Codex
  worker/subagent with an explicit "operate only in this worktree" contract.
- `/launch`, `/dispatch`, `/build`, and `/autopilot` docs now distinguish Codex
  subagent launches from steerable tmux sessions. Autopilot remains B1/B2-only
  because it requires live `nudge()`/`liveness()`.
- `.claude-plugin/marketplace.json` version `1.3.2` → `1.4.0`.

## [1.3.2] - 2026-06-05

Fixes how `/launch` and the executor attach a cmux review surface (reported and
verified from a downstream session).

### Fixed

- **cmux detection no longer requires `$CMUX_SOCKET`.** A host where the `cmux` CLI
  is reachable but `$CMUX_SOCKET` is unset (Claude Code inside a cmux-managed tmux
  session) wrongly fell back to headless B2. Detection now keys on "cmux CLI
  reachable **and** a target pane resolves" (`cmux tree` `◀ here` / `$CMUX_PANE_ID`).
- **The review tab opens as a sibling of the orchestrator's own tab.** The old bare
  `cmux new-surface` defaulted to an unset `$CMUX_WORKSPACE_ID`; `/launch` now
  resolves the orchestrator's pane from `cmux tree` and opens the tab with
  `new-surface --workspace <ws> --pane <pane> --focus true`. `cmux new-workspace`
  (a separate top-level workspace) is kept out of the launch path.
- **Bootstrap is sent before the cmux surface attaches.** An attached cmux surface
  focuses the tmux composer and blocks external `send-keys`, so the bootstrap
  couldn't land. Reordered to: spawn tmux → wait ready → send bootstrap via
  `tmux send-keys` → then attach the cmux tab.
- `.claude-plugin/marketplace.json` version `1.3.1` → `1.3.2`.

## [1.3.1] - 2026-06-05

Onboarding refinements: OpenSpec is now a required install step, and the optional
memory supplement is wired as a thin layer over AEP's native lessons loop.

### Changed

- `/onboard` and the README "Agent prompt" now install the **OpenSpec CLI** as a
  REQUIRED step (`npm install -g @fission-ai/openspec@latest`, Node >= 20.19). AEP's
  skills shell out to `openspec`, but the delegated install never ensured it.
- The optional **memory supplement** (`project-memory` + `memory-forge`) now layers onto
  AEP's existing lessons loop instead of a parallel one. AGENTS.md gets a concise
  `## Memory & Learning Loop` section mapping recall → `/dispatch`, persist → `/wrap`, and
  distill → `/reflect` / pre-PR. AEP still captures via `/build` → `.dev-workflow/lessons.md`
  → `/wrap` → `lessons-learned/`; the supplement adds qmd semantic recall plus
  distillation-to-skills on top.
- `.claude-plugin/marketplace.json` version `1.3.0` → `1.3.1`.

### Fixed

- Corrected the OpenSpec install command in the `/onboard` and `/scaffold` tool tables:
  both listed `bun add -g openspec`, which installs the wrong npm package (OpenSpec's CLI
  is published as `@fission-ai/openspec`).

## [1.3.0] - 2026-06-04

Adds an optional supplement and modernizes onboarding to the `npx skills` installer.

### Added

- Optional **supplement** pointer to [`memorysaver/skills`](https://github.com/memorysaver/skills)
  in the README and `/onboard`: `project-behavior` (scaffold an `AGENTS.md` behavior pack),
  `project-memory` (git-committable memory system), and `memory-forge` (distill lessons into
  skills). The `/onboard` and README "Agent prompt" flows now **always ask** whether the user
  wants the `AGENTS.md` behavior config and the memory system before installing either.

### Changed

- `/onboard` (aep-onboard) Phase 1 install modernized from the legacy
  `/plugin marketplace add` and `/plugin install` commands to project-level
  `npx skills add memorysaver/agentic-engineering-patterns@<latest-tag>` (pin to the latest
  release, commit the installed files), matching the README install guide.
- `.claude-plugin/marketplace.json` version `1.2.0` → `1.3.0`.

## [1.2.0] - 2026-06-04

Host-agnostic executor: the feature lifecycle now runs under Claude Code **or**
Codex, with or without tmux/cmux, and (on explicit opt-in) as a Claude Code
dynamic workflow. `B1` (tmux + cmux) reproduces the prior behavior byte-for-byte,
so existing installs are unaffected. Decision record:
[`docs/decisions/host-agnostic-executor.md`](docs/decisions/host-agnostic-executor.md).

### Added

- New `aep-executor` utility skill (in the `patterns` group) with a uniform
  operation contract (`detect` / `spawn` / `spawn_evaluator` / `nudge` /
  `liveness` / `check` / `monitor` / `present` / `teardown`) and a recipe per
  backend.
- `executor.check(prompt, schema)` — run a read-only analysis prompt in a
  **cheap, context-isolated** agent (Claude Code Haiku subagent / Codex
  `codex exec` cheap one-shot) and return only its compact JSON result.
- Four execution backends, selected automatically from env markers
  (`$CLAUDECODE`, `$CODEX_*`, `$CMUX_SOCKET`, `$TMUX`) plus `command -v`:
  **B1** session in tmux + cmux tab, **B2** session in tmux (no cmux),
  **B3** native subagent (Desktop / no-tmux fallback), **B4** dynamic-workflow
  fan-out (explicit "…with workflow" opt-in, Claude Code only).
- Codex as a first-class executor alongside Claude Code.

### Changed

- `/launch`, `/build` (Phase 5 evaluator), `/autopilot`, `/dispatch`, `/onboard`,
  and `/wrap` now delegate spawning/steering/teardown to the executor abstraction.
- `/autopilot` ticks now split into **CHECK → ACT**: the read-heavy analysis
  (state, workspace signals, PR status) and the state write run in a cheap delegate
  via `executor.check()`, so the long-lived orchestrator session's context/token
  cost stays low; the orchestrator only executes the few emitted actions (nudge,
  wrap, launch, escalate). The default tick interval stays **5m**.
- Periodic driver is now documented per host: Claude Code `/loop` (in-session);
  Codex has no `/loop`, so schedule `codex exec "/autopilot tick"` via launchd / cron
  / a sleep-loop (each tick already runs isolated + cheap).
- `cmux` is now optional (auto-detected) rather than a hard dependency; `tmux` is
  recommended but no longer an abort-on-missing requirement in `/onboard`.
- `.claude-plugin/marketplace.json` version `1.1.0` → `1.2.0`.

### Fixed

- Removed the bogus `--rc` flag (not a real Claude Code parameter).
- Corrected the per-host CLI invocations, verified against Claude Code 2.1.161 and
  Codex 0.130.0: session backends use the interactive command
  (`claude --dangerously-skip-permissions` / `codex --dangerously-bypass-approvals-and-sandbox`),
  headless paths use `claude -p …` / `codex exec …`. Codex sessions no longer
  (incorrectly) used the non-interactive `codex exec`.
- Readiness probe is executor-aware (`❯` for claude, timed wait for codex).
- Multi-line prompts/nudges sent via `tmux send-keys -l` + a separate `Enter`
  (was submitting line-by-line).
- The gen/eval evaluator is always worktree-bound (eval-protocol Context labels
  name the spawn mechanism, not a read-only or CI use case).
- `/wrap` now kills the tmux session before removing the worktree, fixing an
  orphaned-session leak across autopilot runs.

## [1.1.0] - 2026-06-03

### Added

- Installable via the open [Agent Skills](https://agentskills.io/) format —
  `npx skills add memorysaver/agentic-engineering-patterns -a claude-code --skill '*'`.
- `aep-` prefix on all skill names for namespace isolation.
- Shared-resource materialization (`skills/product-context/_shared/` +
  `scripts/build-skills.sh`) keeping each skill self-contained, with a
  `skills:check` drift guard wired into the pre-commit hook.

### Changed

- README install docs refocused on `npx skills add` (maintainer `sync.sh` /
  `sync-downstream.sh` retained for the downstream-projects workflow).
- `.claude-plugin/marketplace.json` version `1.0.0` → `1.1.0`.

## [1.0.0] - 2026-04-29

First stable baseline after the Jujutsu → git migration. Decision record:
[`docs/decisions/migrate-from-jj-to-git.md`](docs/decisions/migrate-from-jj-to-git.md).

### Added

- Pure git + `git worktree` execution model (`.feature-workspaces/<name>` on a
  `feat/<name>` branch per parallel agent).
- Idempotent `/launch` pre-launch orphan-cleanup (orphan branch + worktree
  registration).
- `scripts/smoke-test-worktree.sh` for reproducible worktree-mechanics checks.
- `/git-ref` skill documenting the AEP git + worktree conventions and recovery.

### Changed

- Linear commit-per-task implementation: `tasks.md` rows map 1:1 to commits,
  squash-merged at PR time.

### Removed

- Jujutsu (one-shot migration, no dual-mode period) and the `/jj-ref` skill.

[3.0.0]: https://github.com/memorysaver/agentic-engineering-patterns/compare/v1.5.0...v3.0.0
[1.5.0]: https://github.com/memorysaver/agentic-engineering-patterns/compare/v1.4.0...v1.5.0
[1.4.0]: https://github.com/memorysaver/agentic-engineering-patterns/compare/v1.3.2...v1.4.0
[1.3.2]: https://github.com/memorysaver/agentic-engineering-patterns/compare/v1.3.1...v1.3.2
[1.3.1]: https://github.com/memorysaver/agentic-engineering-patterns/compare/v1.3.0...v1.3.1
[1.3.0]: https://github.com/memorysaver/agentic-engineering-patterns/compare/v1.2.0...v1.3.0
[1.2.0]: https://github.com/memorysaver/agentic-engineering-patterns/compare/v1.1.0...v1.2.0
[1.1.0]: https://github.com/memorysaver/agentic-engineering-patterns/compare/v1.0.0...v1.1.0
[1.0.0]: https://github.com/memorysaver/agentic-engineering-patterns/releases/tag/v1.0.0
