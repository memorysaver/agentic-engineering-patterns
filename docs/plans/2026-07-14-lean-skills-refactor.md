# Lean Skills Refactor — Per-Skill Plan

Companion to [decisions/skill-authoring-standard.md](../decisions/skill-authoring-standard.md), expanding its per-skill target table into concrete moves. Line numbers reference commit `ededa12` (v2.7.0). Vocabulary (spine, disclose, dedup, R1–R9) is the decision doc's. Every P1–P3 PR attaches R9 behavior-parity evidence for each touched skill.

Each skill entry uses four verbs:

- **Spine** — what stays inline in SKILL.md (the steps every run executes).
- **Disclose** — inline reference moved to a `references/` file (R1).
- **Dedup** — duplicated meaning replaced by a one-line pointer to its canonical home (R2/R3).
- **Rewrite** — negation→positive, no-op deletion, vague criterion → world-derived postcondition (R4/R6).

---

## P1 — cross-cutting sweep (applies corpus-wide; not repeated per skill below)

0. **Extend `build-skills.sh` to per-file materialization** (prerequisite for items 2–3 and the P2 dispatch/map disclosures). Today the `.aep-generated` marker is dir-level and the build does `rm -rf` + `cp -R`, so a skill-owned file inside a generated `references/` dir is wiped on commit, and the dir-level `continue` skips authored dirs entirely. Change the marker to list the managed basenames: the build removes/copies only those files, skill-owned files coexist beside them, and shared files can materialize into the three authored-dir skills (calibrate, model, validate) too. `--check` must diff per managed file.
1. **`$BASE` resolver** (21 SKILL.md sites, 13 skills, two spellings) → keep only in `git-ref` "Integration Branch"; every other SKILL.md site becomes one line: `Resolve $BASE per /aep-git-ref "Integration Branch".` The `aep.integration-branch` literal legitimately stays in `executor/references/backends.md` (probe) and `autopilot/references/tick-protocol.md` (nudge prompt) — those read the config key for their own bash and are not dedup targets.
2. **File-Resolution SPLIT/V1 block** (7 product-context skills — all except watch) → new `_shared/references/file-resolution.md`, one-line pointer at each site. Requires item 0: calibrate, model, and validate have authored `references/` dirs the current build never materializes into — without item 0 their pointers would dangle, and `--check` would not catch it.
3. **YAML validation + common-fixes prose** (4–7 sites) → pointer to `references/yaml-guardrails.md`. The file is materialized today in only 5 of 8 consumers (dispatch, envision, map, reflect, watch); for model, validate, and calibrate this step adds the pointer **and** runs `scripts/build-skills.sh` (verify with `--check`) so the file exists — it does not pre-exist there.
4. **Cross-skill links** → `/aep-x` prose invocations; delete the four "Cross-skill reference path" blocks (executor:66-73, gen-eval:31-38, workflow:133-137, design-lens:156-162); fixes the dual-path bug (`../../patterns/executor/…` vs `.claude/skills/aep-executor/…`).
5. **Nudge prompt texts** → live only in `autopilot/references/tick-protocol.md`; autopilot SKILL and review-trigger.md point at it.
6. **CI (R8)**: line check over `skills/**/SKILL.md` (warn > 400 from P1; the > 500 fail threshold activates in the P3 PR) + the multi-line `$BASE` resolver block appears in exactly one SKILL.md (git-ref), with `backends.md`/`tick-protocol.md` allowlisted for the raw literal.

---

## agentic-development-workflow

### /aep-build (955 → ~325) — P2

- **Spine**: Phase 0 as ~10 one-line steps; Phases 4–12 kept as numbered steps with their existing checkable gates (`coverage.criteria_covered == criteria_total`, `mergeStateStatus=CLEAN`).
- **Disclose**: Phase 0 artifact bodies (`:139-282` — contracts JSON, feature-verification.json, init.sh, status.json, lessons template) → `references/harness-artifacts.md`, consolidated with the existing `contract-template.md`/`progress-template.md` so each artifact has one template; Phase 5 evaluator machinery (`:375-511` — gen/eval table, recovery ladder, per-round spawn recipes) → keep the loop skeleton, point to `/aep-gen-eval` references (recovery-ladder.md already exists there); Phase 6 dogfood theory (`:554-646` — tier model, tool selection, target resolution) → keep author-journey/run-tiers steps, point to the project e2e-test skill's `policy.md` and `e2e-skill-scaffolding` references; Phase 12 taxonomy (`:857-916`) → `merge-decision-cases.md` becomes canonical (flip today's ownership note); SKILL keeps the CLEAN happy path + pointer; Story-Status section (`:285-324`, product-cycle only) → `references/story-status.md`.
- **Dedup**: worktree guard bash (`:39-66`) — SKILL is canonical; `worktree-onboarding.md` shrinks to a thin orientation that points to Phase 0 instead of copying it; one-commit-per-task (`:330-353`) and rebase/push/PR blocks (`:716-764`) → `/aep-git-ref` pointers; Guardrails (`:930-945`) keep only rules with no step home (drop the echoes of Phase 0/5/11 rules stated inline).
- **Rewrite**: the NEVER-stack in the Phase 0 guard and guardrails → positive phrasing, prohibition kept once for the two hard guardrails (main-checkout untouchable, `--force-with-lease`); fix `progress-template.md:62-64` stale "Phase 7: E2E Test Scripts" sediment.

### /aep-launch (495 → ~160) — P3

- **Spine**: Guardrails 1–2 (abort gates — already checkable), Steps 1–5, Present table.
- **Disclose**: Evaluator section (`:310-434`) → `references/evaluator.md` keeping only launch-specific setup (bootstrap template + spawn pointer); eval contracts/loop stay canonical in `/aep-gen-eval`; orphan-cleanup bash (`:102-141`) → `references/orphan-recovery.md`; per-mode spawn recipes (`:241-306`) → thin to `/aep-executor` reference pointers (recipes already live there); Guardrails 3–4 (calibration, Object Map — product-cycle branches) → conditional one-liners.
- **Dedup**: worktree create (`:177-197`) → `/aep-git-ref`; monitoring snippets (`:442-457`) → `references/signals-spec.md` pointer; "When to Use evaluator" table → design's workflow-modes reference (below).
- **Rewrite**: delete "This is the single most impactful improvement…" (no-op); Step 5 Present gets a postcondition (workspace row printed + signal file exists).

### /aep-design (187 → ~95) — P3

- **Spine**: Phases 1–3 + Commit + Next step.
- **Disclose**: Operating Mode product-cycle branch (`:23-39`) → `references/modes.md`; full/light workflow-mode selection (`:76-103`) → `references/workflow-modes.md` — **canonical home for mode criteria** (launch points here; build's full/light are eval-loop modes, a different axis, and stay with `/aep-gen-eval`); Prerequisites probing (`:41-73`) → two one-line checks + `/aep-onboard` pointer.
- **Dedup**: commit-to-integration pattern (`:162-173`) → `/aep-git-ref` "Control-Plane Commits".
- **Rewrite**: Phase 1–3 completion criteria become postconditions (Phase 1: requirements list written into the change dir; Phase 2: `proposal.md` exists; Phase 3: user approved — recorded in the proposal); delete "Build shared understanding" (vague) and "What NOT to review" (negation → "review scope: architecture, interfaces, task decomposition").

### /aep-wrap (352 → ~150) — P3

- **Spine**: Steps 1–6 with the ordering-invariant postcondition table (`:229-239`) — this table is the R4 exemplar and stays.
- **Disclose**: Reflect-and-Advance (`:243-336` — layer-gate tiers, distillation protocol) → `references/layer-advance.md`; step 2.5 re-embedded copy commands (`:65-99`) → `references/convergence.md` pointer (file exists; keep the authored prose there per the wrap-convergence memory).
- **Dedup**: layer-gate tier logic → points to the project e2e-test skill's `policy.md` + `e2e-skill-scaffolding` references (the R2 canonical home — not build Phase 6, which after its own refactor is itself only a pointer); worktree remove + ff-commit patterns (stated 3× internally) → `/aep-git-ref` pointers, once.
- **Rewrite**: guardrail Never-list → the ordering invariant already encodes them positively; keep archive-not-from-workspace once as hard guardrail.

### /aep-git-ref (319 → ~260) — P3

- **Role change**: becomes the enforced canonical home for `$BASE`, worktree lifecycle, branch naming, one-commit-per-task, publishing/PR, control-plane commits (R2 table). Heading anchors stay stable — every skill points at them by name.
- **Rewrite**: "Why Git, Not jj" (`:15-25`) → two lines + [migrate-from-jj-to-git.md](../decisions/migrate-from-jj-to-git.md) pointer; delete "If you've used git for ten minutes…" filler. Cheat Sheet stays (legitimate index for a reference skill).

---

## patterns

### /aep-autopilot (727 → ~270) — P2

- **Spine**: session/state intro, Start Protocol, one condensed STOP boundary statement, tick summary (~12 lines) + pointer, status/stop ops, Next Steps.
- **Disclose**: inlined 7-step tick block (`:437-553`) → delete; `tick-protocol.md` is canonical (already says so at `:429`); goal/loop driver recipes (`:311-400`) → `references/drivers.md` with a ~15-line selector inline; executor-transport table (`:65-112`) → `/aep-executor` pointer (~8 lines kept); Design Escalation (`:617-675`) → merge with the full_auto section, overflow to `tick-protocol.md` Step ⑥.
- **Dedup**: boundary doctrine stated once in SKILL (today: STOP + Guardrails + 3 reference files); `review-trigger.md` drops its copied prompts; ordering invariants named once, pointing to [deterministic-orchestration.md](../decisions/deterministic-orchestration.md); atomic-write instruction once (state-schema.md).
- **Rewrite**: five-NEVER stack + Forbidden table → one Allowed/Forbidden table, positively framed rows; delete "re-read the STOP section before every tick" (no-op); rationale → [goal-driven-autopilot.md](../decisions/goal-driven-autopilot.md).

### /aep-executor (219 → ~150) — P3

- **Spine**: operation-contract table, modes table, reference-files table, standalone steps (all legitimate for a utility index).
- **Disclose/Dedup**: Design Decisions essays (`:149-209`) → fold into [host-agnostic-executor.md](../decisions/host-agnostic-executor.md) / [native-first-executor.md](../decisions/native-first-executor.md) / [remove-claude-team.md](../decisions/remove-claude-team.md) (all exist; delete from SKILL); duplicate reference-path block (`:66-73`); claude-team removal note kept once (today ×4).

### /aep-gen-eval (149 → ~120) — P3

- **Spine**: intro, core principle, reference-files table, standalone steps 1–5. **Gains canonicity**: agent contracts, scoring, prompt templates for all gen/eval consumers (validate, launch, build point here).
- **Dedup**: duplicate path block (`:31-38`); dimension presets in step 3 → `scoring-framework.md` pointer; Design Decisions (`:119-140`) → one sentence + decisions pointer.
- **Rewrite**: Step 5 gets a postcondition (fixes applied AND re-scored, or findings file written).

### /aep-workflow (187 → ~135) — P3

- **Spine**: judgment framing (when a workflow earns its place), sub-pattern index (one line per pattern — details already in `references/pattern-catalog.md`), invoke steps.
- **Dedup**: merge the two AEP-mapping tables (`:51-68` catalog column ↔ `:121-137` touchpoints) into one; the "earns its place" failure-mode table is a shared _format_ with design-lens but skill-specific _content_ — each skill keeps its own instance, disclosed into its own references (workflow's → `references/pattern-catalog.md`); gen-eval relationship sentence once (today ×3).
- **Rewrite**: "Do NOT reach for a workflow when…" → positive sizing rule ("reach for a workflow when the task is ≥X independent items / needs adversarial verification; otherwise stay single-context"); Design Decisions → decisions pointer.

### /aep-workflow-feedback (173 → ~150) — P3

- Closest to target shape. **Dedup**: "never auto-edit skill files" once (today ×3); Relationship section merged into When-to-Use; re-pin recipe → README pointer. **Rewrite**: DO/DO-NOT guardrail pairs folded into the steps they echo.

### /aep-design-lens (243 → ~145) — P3

- **Spine**: the method (classify → select lenses → suggest → guideline → health-check), theory-catalog one-line index, standalone steps.
- **Disclose**: Design Decisions (`:190-229`) → new short `docs/decisions/design-lens-rationale.md` note (R5 — rationale lives in `docs/decisions/`, not a PR description); earns-its-place table + design-goals (`:29-61`) → `references/method-and-templates.md`; lens-selection rules (`:91-99`) → already owned by `method-and-templates.md`, keep pointer only.
- **Dedup**: model/calibrate/validate boundary stated once (today ×4); quick-check-vs-deep-audit rule stated once (today ×4); "never writes schema files" once as hard guardrail.
- **Description**: 138 → ~40 words (worst in corpus; the seven-family enumeration moves into the body).

---

## product-context

### /aep-dispatch (654 → ~300) — P2

- **Spine**: sync → cascade → score (as a call) → present → dispatch-lock (re-read/flip/commit — keep verbatim; it is a named mechanical invariant) → create change → push → route by readiness.
- **Disclose**: scoring formulas + worked example (`:164-256`) → `references/scoring.md`; context-package assembly (`:406-525`) → `references/context-assembly.md`; Dynamic-Workflow mode (`:312-363`, opt-in branch) → `references/workflow-mode.md` with 3-line pointer; present-queue ASCII example shrinks to one row. These three are **skill-owned files inside dispatch's build-generated `references/` dir — requires P1 item 0** (per-file materialization); without it the build hook wipes them on commit.
- **Dedup**: story state machine stated once (schema is canonical; map and dispatch point).
- **Rewrite**: Guardrails five-Never stack (`:649-654`) → the steps already encode them; keep "WIP limit respected" as a checkable gate; delete duplicated "irrelevant context degrades performance" (×2 no-op).

### /aep-validate (455 → ~240) — P3

- **Spine**: mode selection, spawn steps, consolidation with hard-failure thresholds (already checkable).
- **Disclose**: three agent-prompt templates (`:217-284`) → `/aep-gen-eval` `agent-contracts.md` (already cited at `:98` — canonical there); validation-dimension tables (`:362-394`) → `references/validation-dimensions.md`; mode B/C/D role tables (`:136-166`) → `references/modes.md`; customization (`:406-433`) → same.
- **Dedup**: changelog YAML → schema pointer.

### /aep-calibrate (287 → ~180) — P3

- **Disclose**: heavy/light tables (`:73-98`) + scan-targets (`:121-131`) → `references/calibration-types.md` (exists); visual-design reference-implementation (`:256-277`) → `references/briefs/visual-design.md` (exists).
- **Dedup**: calibration.history + changelog YAML → schema pointer. **Rewrite**: "Key Principles" 6 philosophy bullets → delete (body already enacts them); "Pick what feels right" → postcondition (chosen variant recorded in `calibration/<type>.yaml`).

### /aep-envision (247 → ~150) — P3

- Already the model citizen for schema delegation. **Disclose**: quality-dimensions catalog (`:94-104`) → `/aep-calibrate` pointer (calibration-types.md canonical); envision-vs-reflect boundary (`:209-226`) → one sentence each side; capability-maps section (`:154-163`) → reference. **Rewrite**: "Key Principles" no-op bullets (`:231-235`) → delete; Kill Point stays but gets an observable form (user answers three challenge questions; any "no" stops).

### /aep-map (355 → ~210) — P3

- **Disclose**: telemetry binding (`:137-158`) → `telemetry-ingestion.md` pointer (canonical, already cited); `.5`-alignment-layer treatise (`:171-194`) → `references/alignment-layers.md` — **canonical home for the layer concept** (calibrate/dispatch point here); object-map drafts (`:195-230`) → `references/object-map-drafts.md` — map owns drafting, `/aep-model` consumes; technical-spec trigger rules (`:52-68`) → reference. Map's `references/` dir is also build-generated, so these skill-owned files **require P1 item 0**.
- **Rewrite**: Anti-Patterns negations (`:327-331`) — walking-skeleton rule already stated twice inline; keep once, positively.

### /aep-model (233 → ~150) — P3

- **Dedup**: ORCA round-by-round (`:77-122`) → thin to the four round names + `references/orca-process.md` pointer (canonical, already cited); downstream-consumption section (`:197-209`) → delete (consuming skills own their gates); Anti-Patterns → fold into "the one rule" at top.

### /aep-reflect (297 → ~180) — P3

- **Spine**: classifier (5 categories — **canonical home**; watch points here), step chain with enum+changelog+YAML gates.
- **Disclose**: changelog/outcome/process YAML blocks (~55 lines) → schema pointers; auto-eval/telemetry paragraphs → `telemetry-ingestion.md` pointer. **Dedup**: Step 5.5 workflow-improvement merged into Step 2's process category.

### /aep-watch (325 → ~200) — P3

- **Disclose**: watch config YAML (`:75-95`) + finding record + adapters (`:143-179`) → `telemetry-ingestion.md` (canonical; already holds the material).
- **Dedup**: delete the re-tabulated classifier table (`:188-198`) — the `/aep-reflect` prose invocation above it is the correct form; Guardrails eight Never/Always bullets → the STOP box already covers them, keep the two with no step home.

---

## project-setup

### /aep-scaffold (700 → ~380) — P2

- **Shape decision**: stays **one skill, two branches** (new-project / existing-project) — splitting would spend a second description (context load) for no independent trigger gain; the mode-select step at top is the branch point, and each branch's detail is disclosed.
- **Disclose**: four OpenSpec command-file bodies (`:280-343`) → `templates/opsx/*.md` written programmatically; Phase-1 stack tables (`:53-122`) → `references/stack-guide.md` (exists, already cited at `:43` — delete the inline copies); existing-project audit/converge bash (`:516-673`, ~150 lines) → `scripts/audit.sh` + `scripts/converge.sh` invoked by name, flow prose → `references/converge-flow.md`; resulting-structure ASCII tree + compatibility table → reference.
- **Spine**: mode select, tool check, scaffold command, e2e delegate handoff (`/aep-e2e-skill-scaffolding`), commits — each already ends in a checkable gate (`test -d skills/e2e-test || exit 1` pattern; keep).

### /aep-onboard (385 → ~300) — P3

- **Disclose**: full `.claude/settings.json` block (`:148-193`) → `references/settings-template.json`; plugin reference table → reference.
- **Dedup**: integration-branch single/two-branch explainer (`:258-275`) → `/aep-git-ref`; native-launch-mode prose (`:277-301`) → `/aep-executor`; Phase-0 mental-model section → [docs/orientation.md](../orientation.md) pointer (canonical human-facing doc).

### /aep-e2e-skill-scaffolding (265 → ~210) — P3

- Leanest already. **Disclose**: `cli`-target adaptation (`:148-155`) and `none`-target rules (`:139-147`) → `references/three-tier-model.md` (cited). Keep the Phase-6 file-existence gates verbatim (R4 exemplars).

---

## Description diet (all 22, P3)

Rule: ≤50 words, one trigger per branch, leading word first. Any skill whose trigger set shrinks gets a **triggering check before merge** (a handful of probe prompts that previously fired the skill, or a skill-creator-style eval) — dropped trigger terms can silently stop a skill firing, and no other gate catches that. Worked examples of the pattern:

- **design-lens** (138 → ~40): "Theory-grounded design guideline and 0–4 heuristic health-check for any product UI. Use on 'design review', 'usability check', 'accessibility check', or before building/auditing a UI. Not for taste capture (/aep-calibrate) or object IA (/aep-model)."
- **workflow** (134 → ~40): "Author a dynamic workflow — a deterministic multi-agent harness for one task. Use when work is too large, uncertain, or verification-heavy for one context; triggers on 'dynamic workflow', 'ultracode', 'orchestrate subagents'. Not process feedback (/aep-workflow-feedback)."
- **executor** (109 → ~35): "Host-agnostic executor for spawning and steering workspace agents across Claude Code and Codex backends. Consulted by /aep-launch, /aep-build, /aep-autopilot; use directly for 'which backend', 'launch mode', 'host detection'."

Pipeline-position clauses ("Followed by /aep-launch") are kept — they aid chained invocation — but synonym stacks ("build", "implement", "execute implementation") collapse to one trigger per branch.

---

## Sequencing note

P1 item 0 (the `build-skills.sh` per-file extension) precedes every product-context disclosure in any phase. P2 order: **build → autopilot → dispatch → scaffold** (dependency: autopilot's tick pointers assume build/wrap phase names are stable; do build first). Every P2/P3 PR re-runs `bash scripts/build-skills.sh --check` and the R8 lints, and attaches R9 parity evidence per touched skill; product-context shared-reference moves route through `_shared/`. No step is renamed or renumbered without grepping all cross-references (`grep -rn "Phase <n>" skills/ docs/`) — phase names are load-bearing across autopilot/tick-protocol/wrap.
