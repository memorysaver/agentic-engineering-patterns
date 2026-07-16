# Verification Economics: Risk-Tiered Validators and Tamper-Evident Evidence

An empirical law from running two AEP consumer projects at scale: **verification whose cost is
unpriced and whose depth is uniform eventually displaces the product it protects — and every observed
loop failure was a verification-_placement_ error, not a verification-_volume_ error.** This document
records how AEP prices verification and where each validator class belongs: a typed failure taxonomy
evaluated before any repair spend, risk-derived verification depth per story, verification accounting
in the convergence record, regression replay moved from story to layer granularity, and a
tamper-evident-evidence requirement on every layer gate. Companion to
[deterministic-orchestration.md](deterministic-orchestration.md): that document put the WHEN and SHAPE
of _orchestration_ behind mechanism; this one applies the same mechanical/judgment split to
_verification itself_ — the routing, depth, and accounting of verification are mechanical; only the
judging content stays judgment.

> **Status:** Proposal (not yet implemented). This document records the design precisely so the
> implementation PRs (target: v3.1.0) can be made and reviewed against it. It changes how AEP works,
> so it lives in `decisions/` per the [docs routing guide](../README.md).

> **Sourcing note:** Sourced from the 2026-07-16 cross-repo research session over SIBYL (story
> SIBYL-189, `openspec/changes/archive/2026-07-15-SIBYL-189/`, and
> `skills/e2e-test/results/2026-07-16-s189-current-position.md`) and looplia (Layer 31,
> `.dev-workflow/autopilot-status.md`, `.dev-workflow/dogfood-l31-006.md`, lessons
> `l31-006`/`l31-014`), plus the framing that in the RLVR era stronger generator models make
> LLM-judged verification simultaneously more expensive and more gameable — so verification must
> scale through deterministic gates and real-environment evidence, not through more judge rounds.
> SIBYL's concrete machinery (its Terra evaluator profile, shell-use PTY driver, ledger-equality
> oracle) stays downstream; what upstreams here is the taxonomy, the tier model, the accounting
> schema, and the placement rules.

---

## The evidence (diagnosis)

Both v2.5.0 consumers now spend the majority of their loop on verification and process:

- **looplia:** of the last 100 commits, **16 are feature-bearing PR merges; 84 are
  dispatch/design/prove/complete/archive/lessons overhead** (≈1:5.3). Layer 31's 14 stories all
  merged within three days — then the layer halted at autopilot tick 444 with **zero further product
  advance**: the mandatory post-deploy production dogfood failed on a **pure environment
  precondition** (prod secrets absent from CI, Wrangler OAuth bound to the wrong Cloudflare account).
  The loop's only failure path routed this ops problem into **two code-fix recovery stories**, each of
  which exhausted **all five** independent evaluation rounds and still failed (final rounds 2.79 and
  3.28 of 5), pausing the loop for human redesign. Ten evaluation rounds were spent repairing a
  problem no code change could fix. Verification cost is uninstrumented there
  (`stats.total_cost_usd: 0` after 444 ticks).
- **looplia's merge gate is a perfect-score gate:** a lesson (`l31-014`) ratcheted the threshold to
  _"the independent evaluator must return exactly 5.00/5.00 with zero findings before PR/merge."_
  Upstream AEP requires only "no blocking findings remaining"
  (`gen-eval/SKILL.md` → Step 5). The perfect-score variant is what Goodhart's law predicts: it
  forces either endless repair rounds (observed) or evaluator gaming (the RLVR failure mode).
- **SIBYL:** story SIBYL-189 — a cockpit _display-positioning_ story — consumed **two formal
  evaluator rounds at the most expensive profile available (`gpt-5.6-terra @ xhigh`), two full-suite
  runs (~13 minutes each), six PTY journey scenarios, an actual-checkout dogfood, and a verification
  ledger**. Of its 28 commits, 8 are feature commits; test code (+2,461 lines) is **1.56×** its
  feature code (+1,581). Repo-wide, ~22% of the last 100 commits are feature commits, and the
  verification corpus (29 layer-gate evidence docs, 21 e2e result docs, 37 retrospectives, 114
  execution records) grows monotonically — nothing ever retires a verification asset.
- **Both repos sit on v2.5.0**, so neither consumes the v2.7.0 typed-gate/convergence improvements —
  SIBYL authored them — nor v3.0.0's −37.7% front-tier token footprint.

Four structural defects explain the numbers:

1. **Uniform depth.** Every story gets the maximal gauntlet regardless of blast radius. The only
   knobs today are Light mode and `policy.md`'s tiers/target/timing — nothing prices _depth_ per
   story.
2. **Missing failure taxonomy.** The dogfood report carries severity and category
   (`executor/references/dogfood-validation.md:160-199`) but not _failure class_ — so an environment
   failure, a harness flake, and a product defect all route into the same story-filing → gen-eval →
   recovery-ladder path, the most expensive repair machinery AEP owns. The recovery ladder's "When to
   Skip the Ladder" (`gen-eval/references/recovery-ladder.md:59-67`) already names "missing external
   dependency" — but as prose the agent must recall mid-FAIL, which is exactly the class of invariant
   [deterministic-orchestration.md](deterministic-orchestration.md) showed always drifts.
3. **The verification ratchet.** The lessons loop is one-directional: failures add verification
   (thresholds, scripts, evidence docs) and nothing prunes it. `/aep-build` Phase 8 replays **all**
   prior-layer journeys on **every story** (`build/SKILL.md:237`), making total journey executions
   O(layers × stories).
4. **Verification is unpriced.** `execution-record.yaml` records `cost_usd` and per-round gen-eval
   scores (`wrap/references/convergence.md:70-88`) but nothing separates verification spend from
   build spend, and no consumer of the record adjusts anything based on it. Topology templates carry
   token budgets per agent role, but no feedback loop connects actual verification spend to future
   depth decisions.

---

## The placement principle (cost × tamper-evidence)

Classify every validator on two axes: **cost per run** and **gameability** (can the generator — or a
reward-hacking future model — satisfy the check without satisfying the intent?). The placement rule:

> **Cheap and tamper-evident validators run in the inner loop, always. Expensive or gameable
> validators run at the outermost boundary that still catches their failure class, gated by risk.**

| DevOps position                  | Validators                                                                                              | Properties                                            | Frequency                 |
| -------------------------------- | ------------------------------------------------------------------------------------------------------- | ----------------------------------------------------- | ------------------------- |
| **Inner loop** (per task/commit) | Typed gates: lint, typecheck, focused Tier-1, schema validation, scope gates, **environment preflight** | Deterministic, named refusals, cannot be rationalized | Every commit              |
| **Merge boundary** (per story)   | **One** independent evaluator round (diff vs. acceptance contract) + CI + Tier-3 drivers                | Expensive, partially gameable → risk-tiered rounds    | Once per story by default |
| **Deploy boundary** (per layer)  | Tier-2 journey dogfood on the real target, **full regression replay**, coverage matrices, gate evidence | Expensive but tamper-evident (real execution)         | Once per layer            |
| **Operate** (post-deploy)        | Telemetry / production outcomes → `/aep-watch` → `/aep-reflect`                                         | The least gameable evidence that exists               | Continuous                |
| **Human gate**                   | Residual-risk acceptance at layer advance; taxonomy escalations                                         | Most expensive                                        | Per layer + exceptions    |

Two consequences worth naming:

- **Scaling verification means adding typed gates and real-environment evidence, not judge rounds.**
  An LLM-judge round is the only validator class that gets _simultaneously_ more expensive and more
  gameable as models improve. Long-horizon models make each round more thorough — which is precisely
  why rounds must be fewer and more decisive, not more numerous.
- **A verifier must not share incentives with the generator.** AEP already separates
  generator/evaluator (`gen-eval/SKILL.md`) and makes `/aep-wrap` execute-never-author journeys. This
  proposal extends the same separation to _evidence_: a gate flip requires at least one evidence
  class the generator cannot modify (§ Design 5).

---

## The design

### 1. Failure taxonomy — typed, evaluated before any repair spend

Every FAIL from any tier acquires a **`failure_class`**:

```
failure_class: product-defect | environment | harness-flake | scope
```

- **`product-defect`** — the built thing is wrong. Routes to the existing path: finding → story /
  fix commit → gen-eval.
- **`environment`** — a precondition outside the worktree is unmet (secrets, credentials, wrong
  account, target unreachable, quota exhausted). Routes to an **ops checklist surfaced to the
  human/orchestrator. Never files a code story; never enters the recovery ladder; never spends an
  evaluation round.** The gate stays refused with a named tag until the environment is repaired,
  then re-runs.
- **`harness-flake`** — the test machinery itself misbehaved (race, port collision, known-red
  baseline). Routes to quarantine + a harness story; the product gate re-runs after quarantine.
- **`scope`** — the acceptance criterion itself is wrong or the story is mis-sliced. Routes to
  `/aep-reflect` re-slicing, not to repair rounds.

Two mechanical carriers make the taxonomy typed rather than prose:

- **The unified dogfood report** (`executor/references/dogfood-validation.md` → Unified report
  format) gains a required `**Failure-Class:**` line per finding; the `dogfood_report` adapter and
  the `/aep-reflect` Step-2 classifier route on it.
- **The environment preflight gate**: before any Tier-2/Tier-3 execution (build Phase 6 Step B, wrap
  layer gate), a preflight probe checks the target's preconditions — required secrets present, auth
  identity matches the expected account, target reachable, fixtures seedable — and **refuses with
  named tags** (`REFUSING [dogfood-secret-absent:<NAME>]`, `REFUSING [auth-identity-mismatch:<got≠want>]`)
  _before_ any journey spend. A refused preflight is `environment` by construction: zero scenarios
  run, zero findings generated, zero rounds spent. The generated e2e skill's `policy.md` names the
  preflight probes next to the target it already owns.

The recovery ladder keeps its rungs, but the taxonomy check is promoted from a prose "When to Skip"
bullet to a **mandatory step at every FAIL, before choosing a rung**: only `product-defect` climbs
the ladder.

### 2. Risk-tiered verification depth

Each story gets a **`verification_tier`** — **derived at dispatch time from observable facts, not
hand-authored** (no new required schema field on stories; the tier is computed into the
machine-assembled brief and recorded in the execution record):

Inputs (all world-derivable at dispatch): number of architecture modules touched
(`files_affected` overlap), whether an API contract / schema / migration is in scope, whether the
diff touches security-sensitive paths (auth, payments, data deletion, secrets handling), novelty
(new module vs. modification of a covered one), and layer kind (walking skeleton vs. enrichment).

| Tier                   | Evaluator rounds (cap)             | Evaluator effort  | Dogfood scope (Phase 6B)           | Full-suite runs               |
| ---------------------- | ---------------------------------- | ----------------- | ---------------------------------- | ----------------------------- |
| **light**              | 0 (generator self-review, lenient) | —                 | render smoke / none                | focused only                  |
| **standard** (default) | **2**, taxonomy check each FAIL    | session default   | affected-surface journey scenarios | focused; full once at land    |
| **deep**               | up to 5 + full recovery ladder     | highest available | full journey + prior-layer replay  | full pre-eval **and** at land |

- **Hard floor:** stories touching auth, payments, irreversible data operations, or migrations are
  always `deep`, whatever the derivation says. The human can override any story's tier; overrides are
  recorded.
- **Threshold semantics (all tiers):** PASS means **zero blocking findings** against the
  hard-failure thresholds (`gen-eval/references/scoring-framework.md:107`). A perfect aggregate
  score is **never** a gate condition — perfect-score gates train evaluator-gaming and block
  convergence (looplia's 5.00/5.00 rule is the counterexample).
- The existing round protocol (`gen-eval/references/eval-protocol.md:116`, `max_rounds` default 5)
  becomes tier-derived; `/aep-build` Phase 5's "max 5" (`build/SKILL.md:126`) reads the tier from
  the brief.

### 3. Verification accounting — priced, recorded, calibrated

`execution-record.yaml` (`wrap/references/convergence.md:70-88`) gains a `verification:` block,
gathered best-effort like every other field:

```yaml
verification:
  tier: light | standard | deep # as dispatched (+ `override: human` when overridden)
  eval_rounds: <n>
  findings_by_round: [<n>, ...] # blocking+important findings per round
  suite_runs: <n>
  suite_seconds: <n> | null
  journey_scenarios_run: <n>
  preflight_refusals: [] # named tags, if any
  cost_usd: <n> | null # verification share when separable; else null
  escaped_defects: [] # filled retroactively by /aep-reflect (see below)
```

Consumers close the loop:

- **Escape-rate ingestion:** when `/aep-reflect` classifies a post-merge bug, it traces the bug to
  the story that introduced it and appends to that story's `escaped_defects`. Escape rate (defects
  that escaped ÷ verification rounds spent) is the calibration signal.
- **Bidirectional calibration:** layer distillation (already proposal-only,
  `wrap/references/convergence.md` §2) may propose tier-derivation adjustments **in both
  directions** — tighten where defects escape, and **loosen or retire** where `findings_by_round`
  shows round ≥2 catching nothing across a layer. Verification assets get a lifecycle: distillation
  may propose merging or retiring journeys that have replayed green for N layers with no coverage
  loss. Proposals only; a human applies them. This is what breaks the ratchet.
- **The layer budget box:** each layer gate's evidence doc records expected vs. actual verification
  spend (rounds, suite time, scenarios, `cost_usd` where known). When actuals overrun the box,
  `/aep-wrap` surfaces a **scope-vs-verification tradeoff to the human** at the layer-advance gate —
  the loop asks, it does not silently grind. (looplia's tick-444 halt is the degenerate form of this
  question, asked 10 rounds too late.)

### 4. Regression replay moves from story to layer granularity

- **Per story** (`/aep-build` Phase 8, `build/SKILL.md:237`): replay only **impacted** journeys —
  those whose `covers:` criteria or module mapping intersects the story's `files_affected`. The
  journey front-matter (`layer:`, `covers:`) already makes impact selection expressible.
- **Per layer** (`/aep-wrap` layer gate, `wrap/references/layer-advance.md`): the **full**
  prior-layer replay stays here, once per layer — this is where cumulative regression belongs, and
  it keeps the guarantee `/aep-dispatch` relies on (a layer cannot advance past a broken past).
- Total journey executions drop from O(layers × stories) to O(stories + layers).

### 5. Tamper-evident evidence classes on every gate

A layer gate may flip to `passed` only when its evidence includes **at least one class the
generator cannot modify**:

1. a CI run bound to the merged SHA;
2. journey execution performed by `/aep-wrap` (executes-never-authors — existing canon);
3. **read-only golden fixtures with a ledger-equality oracle** — fixture trees the generator's
   workspace cannot write, verified by before/after equality of durable state (SIBYL's
   fixture-repo + SHA-256 ledger comparison is the proven downstream shape; screenshots stay
   diagnostic, never a pass condition);
4. production telemetry ingested by `/aep-watch`.

Additionally:

- **Evaluator prompts are machine-assembled** (extending deterministic-orchestration's
  machine-assembled brief to evaluation): the orchestrating layer — not the generator — assembles
  the evaluator's context (criteria file, contract file, diff range), so the generator cannot curate
  what its judge sees.
- **`policy.md` gains a `live_policy` decision** for cost-bearing dogfoods (live model calls, quota-
  or fee-metered targets): `every_gate | milestone_gates | none`, with the milestone list named in
  the policy. Non-milestone gates use zero-cost render smokes / scripted tiers. This upstreams
  SIBYL's proven `live_policy: milestone_gates_only` pattern. The existing SKIP-not-FAIL degrade
  semantics stay; the taxonomy separates SKIP (tool absent), REFUSED (environment), and FAIL
  (product) so none masquerades as another.

---

## What upstream changes now (v3.1.0) vs future

**Now — one teaching reference plus prose edits across the verification-touching skills; no runtime,
no new required schema fields:**

1. **NEW `skills/patterns/gen-eval/references/verification-economics.md`** — the canonical reference
   carrying the placement matrix, the failure taxonomy + routing table, the tier presets +
   derivation inputs, the `verification:` accounting schema, the evidence-class catalog, and the
   worked examples. It lives under gen-eval because gen-eval already owns evaluation canon and is
   read by `/aep-build`, `/aep-validate`, and `/aep-launch`; build, wrap, dispatch, reflect, and the
   e2e scaffold cross-reference it **by name**.
2. **`skills/patterns/executor/references/dogfood-validation.md`** — Unified report format
   (`:160-199`): add the required `**Failure-Class:**` line and the four-way routing table
   (`environment` → ops checklist, never story-filing; `harness-flake` → quarantine; `scope` →
   re-slice; `product-defect` → existing adapter path).
3. **`skills/project-setup/e2e-skill-scaffolding/`** — `templates/policy.md.tmpl` + SKILL.md Phase 2
   confirmation questions gain the `live_policy` decision and the preflight-probe list;
   `references/layer-gate-loop.md` and `references/three-tier-model.md` add the environment
   preflight gate (named refusals before any Tier-2/3 spend), the evidence-class requirement on
   `passed`, and the replay-placement note (impacted per story, full per layer).
4. **`skills/agentic-development-workflow/build/SKILL.md`** — Phase 5 (`:126-145`): round cap and
   evaluator effort read from the brief's `verification_tier`; taxonomy check at every FAIL before
   any ladder rung. Phase 6 Step B (`:194-215`): run the preflight gate before executing; a refusal
   is `environment`, not a dogfood FAIL. Phase 8 (`:237`): impacted-only journey replay.
5. **`skills/patterns/gen-eval/references/`** — `eval-protocol.md` (`:116`): `max_rounds` becomes
   tier-derived. `recovery-ladder.md` (`:59-67`): "When to Skip the Ladder" reframed as the typed
   taxonomy step run at every FAIL. `scoring-framework.md`: state the zero-blocking-findings
   threshold semantics and name the perfect-score gate as an anti-pattern.
6. **`skills/agentic-development-workflow/wrap/references/`** — `convergence.md` (`:70-88`): the
   `verification:` block in `execution-record.yaml`. `layer-advance.md`: full replay owned here; the
   budget box; the evidence-class check before flipping `passed`.
7. **`skills/product-context/dispatch/SKILL.md`** — tier derivation at brief assembly (`:237`,
   machine-assembled; the observable inputs listed above), recorded in the brief and the dispatch
   note.
8. **`skills/product-context/reflect/SKILL.md`** (+ `watch` → `references/telemetry-ingestion.md`) —
   escape-rate ingestion (trace post-merge bugs to origin stories' `escaped_defects`); distillation
   may propose tier-derivation and journey-retirement changes, proposal-only.
9. **`docs/glossary.md`** — new entries: **Verification Tier**, **Failure Class (Failure
   Taxonomy)**, **Environment Preflight Gate**, **Verification Accounting**, **Escape Rate**,
   **Tamper-Evident Evidence**, **Verification Ratchet** (anti-pattern), **Perfect-Score Gate**
   (anti-pattern).
10. `CHANGELOG.md` `[3.1.0]`; `.claude-plugin/marketplace.json` + `package.json` → `3.1.0`.
    Product-context skills' edits (dispatch, reflect, watch) go through `_shared/` sources +
    `build-skills.sh` per the generated-dirs rule.

**Future — explicit non-goals, recorded as horizon:**

- A **held-out / adversarial case vault** (hidden tests the generator never sees, rotated per
  layer). Needs downstream runtime support for access separation; sliced only after the taxonomy and
  tiers prove out.
- Any AEP-shipped verifier runtime, verifier-model training loop, or cost-optimal scheduler. AEP
  stays prose; the probes and gates belong to the project.
- Rewriting `/aep-validate`'s protocol checker (its known prose drift is tracked separately in the
  [v2.5→v3 compatibility audit](../audits/2026-07-15-looplia-v2.5.0-to-v3.0.0-compatibility.md)).

---

## Forcing functions

| Skill           | New responsibility                                                                                              |
| --------------- | --------------------------------------------------------------------------------------------------------------- |
| `/aep-gen-eval` | Owns the verification-economics reference; tier-derived round caps; zero-blocking threshold semantics           |
| `/aep-build`    | Preflight before dogfood; taxonomy check before any ladder rung; impacted-only replay; tier read from the brief |
| `/aep-wrap`     | Full replay + budget box + evidence-class check at the layer gate; gathers the `verification:` block            |
| `/aep-dispatch` | Derives `verification_tier` from observable facts into the machine-assembled brief                              |
| `/aep-reflect`  | Routes on `failure_class`; ingests escape rate; proposes bidirectional calibration                              |
| e2e scaffold    | `live_policy` + preflight probes in `policy.md`; evidence classes and replay placement in the gate loop         |

## Migration

All changes are prose-additive and fail open: a story with no derived tier behaves as `deep`
(today's behavior); a report without `Failure-Class:` classifies as `product-defect` (today's
routing); an execution record without `verification:` stays valid (`story_id` remains the only
required field); a gate with no named evidence class records a warning, not a refusal, for one
release.

| Phase                       | Action                                                                                   | Breaking? |
| --------------------------- | ---------------------------------------------------------------------------------------- | --------- |
| **P0 — this decision doc**  | Record the evidence, the placement principle, the five design elements                   | No        |
| **P1 — canon reference**    | Ship `gen-eval/references/verification-economics.md` + glossary entries                  | No        |
| **P2 — taxonomy carriers**  | Dogfood report `Failure-Class:` + preflight gate (executor, e2e scaffold, build Phase 6) | No        |
| **P3 — tiers + accounting** | Tier derivation (dispatch), tier-derived rounds (build/gen-eval), `verification:` (wrap) | No        |
| **P4 — calibration + bump** | Reflect/distillation calibration, replay move, budget box; v3.1.0                        | No        |

**Exact change sites:** the numbered list under "What upstream changes now" is the implementation
PRs' review contract.

**Propagation discipline:** this proposal introduces net-new taxonomy — _failure class_,
_verification tier_, _environment preflight gate_, _tamper-evident evidence_, _escape rate_,
_verification ratchet_, _perfect-score gate_ — and two new enums (`failure_class` values;
`live_policy` values). Every skill that names one must say it identically; the glossary defines
each; audit with `grep -rn "failure_class\|verification_tier\|live_policy\|preflight" skills/ docs/`
after implementation. `live_policy` joins `applicable_tiers`/`dogfood_target`/`journey_timing` in
**every** policy listing (e2e templates, three-tier-model, layer-gate-loop, build Phase 6, wrap) —
half-applied enums are this repo's #1 historical bug class. Dispatch/reflect/watch edits are made in
`skills/product-context/_shared/` and regenerated via `scripts/build-skills.sh`; `skills:check` must
come back clean. Downstream consumers go live only after the v3.1.0 tag is cut and each consumer
re-pins via the skills CLI.

---

## Worked examples

**looplia L31, replayed under this design.** The post-deploy gate's preflight probes run before any
scenario: `REFUSING [dogfood-secret-absent:LOOPLIA_E2E_FIXTURE_SECRET]`,
`REFUSING [auth-identity-mismatch:account …d422b ≠ …88e6]`. Classification is `environment` by
construction — an ops checklist (set the two secrets, re-auth Wrangler to the prod account) surfaces
to the human; the gate stays `scripted_passed` with named blockers. **Zero journey scenarios spent,
zero code stories filed, zero evaluation rounds, no autopilot halt.** The actual history — two
recovery stories, ten exhausted evaluation rounds, a paused loop, and a layer stuck at 68/75
coverage on criteria the recovery itself created — is the cost of routing an environment failure
through product-defect machinery.

**SIBYL-189, replayed under `standard` tier.** Round 1 caught two real findings (armed-quit
priority, a matrix misclassification the new oracle exposed); the fixes landed; round 2 passed with
zero blocking findings — exactly the two rounds `standard` allows, so the catch quality is
unchanged. What the tier removes: the second full-suite run (~13 minutes; the land verify already
runs the suite on merged main) and the top-shelf evaluator profile for a display-positioning story —
the findings were caught by the focused matrix oracle and an ordinary evaluation round, not by
`xhigh` reasoning. The PTY journey and ledger-equality dogfood stay: they are the tamper-evident
half of the gate.

**SIBYL's `live_policy: milestone_gates_only`** (live-model dogfoods only at the L2/L4/L6 milestone
gates; zero-quota render smokes elsewhere) is the proven downstream shape of the `live_policy`
decision this proposal adds to `policy.md` — evidence that pricing the expensive half of dogfood
per-gate, instead of running it uniformly, holds up across 20+ layers of real use.

---

## Anti-patterns this prevents

- **Perfect-score gates.** "Exactly 5.00/5.00 with zero findings" trains the generator to satisfy
  the evaluator, not the product, and turns ordinary convergence into round exhaustion. PASS is zero
  blocking findings.
- **Environment failures fed to code-repair machinery.** A missing secret is not a 2.79/5.00 code
  problem; no evaluation round may be spent before the taxonomy step runs.
- **Per-story full-regression replay.** O(layers × stories) journey executions; full replay belongs
  to the layer gate.
- **Uninstrumented verification.** 444 ticks with `total_cost_usd: 0` recorded means the loop cannot
  know it is over-verifying; unpriced spend grows until it displaces the product.
- **The verification ratchet.** A lessons loop that only ever adds verification converges on a
  process that verifies instead of shipping; calibration must be able to loosen and retire.
- **Scaling verification with judge rounds.** More LLM-judge rounds is the only scaling direction
  that gets more expensive and more gameable at once; add typed gates and real-environment evidence
  instead.
- **Self-curated evidence.** A generator that assembles its own evaluator's context, or can write
  the fixtures it is graded against, is player, referee, and witness at once.

---

## References

- `SIBYL/openspec/changes/archive/2026-07-15-SIBYL-189/` (`tasks.md`, `execution/eval.yaml`) and
  `SIBYL/skills/e2e-test/results/2026-07-16-s189-current-position.md` — the S189 verification
  lifecycle and the ledger-equality oracle.
- `looplia/.dev-workflow/autopilot-status.md`, `looplia/.dev-workflow/dogfood-l31-006.md` and
  `…/dogfood-l31-006/02-auth-recovery-audit.md`, `looplia/lessons-learned/l31-014-…md` — the L31
  halt, the environment misrouting, and the perfect-score ratchet.
- `SIBYL/product-context.yaml` → `topology.routing.dogfood.live_policy: milestone_gates_only` — the
  downstream prior art for `live_policy`.
- [deterministic-orchestration.md](deterministic-orchestration.md) — the mechanical/judgment split
  and typed-gate pattern this document extends to verification routing, depth, and accounting.
- [build-convergence-pipeline.md](build-convergence-pipeline.md) — the execution-record gather this
  document's `verification:` block rides on.
- [2026-07-15 looplia compatibility audit](../audits/2026-07-15-looplia-v2.5.0-to-v3.0.0-compatibility.md)
  — the v2.5→v3 baseline both consumers migrate across before any of this lands downstream.
- Affected skills: `/aep-gen-eval`, `/aep-build`, `/aep-wrap`, `/aep-dispatch`, `/aep-reflect`,
  `/aep-watch`, `/aep-e2e-skill-scaffolding`, `/aep-executor`.
