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
> implementation PRs can be made and reviewed against it — **across two releases**: v3.1.0 ships
> the incident-proven half (taxonomy, preflight, deterministic security gates, replay move,
> evaluator-independence fixes, accounting instrumentation); v3.2.0 ships the economics half
> (tiers, recipes, calibration), gated on field data from a v3.1.0 consumer. It changes how AEP
> works, so it lives in `decisions/` per the [docs routing guide](../README.md). Revised after a
> three-lens design review (factual grounding, adversarial critique, downstream operability) and an
> independent frontier-model best-practice evaluation; their blocking findings are folded in below
> — classification authority, two-point tier derivation, cap-exhaustion semantics, autopilot
> integration, the `live_policy`-aware preflight, the referee-asset rule, and the typed recipe
> artifact.

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
  advance**: the mandatory post-deploy production dogfood failed on an **environment
  precondition** (prod secrets absent from CI, Wrangler OAuth bound to the wrong Cloudflare
  account). The loop's only failure path routed this ops problem into **two code-fix recovery
  stories**, each of which exhausted **all five** independent evaluation rounds and still failed
  (final rounds 2.79 and 3.28 of 5), pausing the loop for human redesign. Ten evaluation rounds were
  spent repairing a problem no evaluation round could fix. (The incident also carried a genuine
  product component — a tracked literal-password fallback in four dogfood scripts, and deploy config
  that never wired the fixture binding — which is exactly why the taxonomy below is per-finding, not
  per-incident.) Verification cost is uninstrumented there (`stats.total_cost_usd: 0` after 444
  ticks).
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
   knobs today are the design-time Light mode and `policy.md`'s tiers/target/timing — nothing prices
   _depth_ per story.
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

| DevOps position                  | Validators                                                                                                                      | Properties                                            | Frequency                       |
| -------------------------------- | ------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------- | ------------------------------- |
| **Inner loop** (per task/commit) | Typed gates: lint, typecheck, focused Tier-1, schema validation, scope gates, **secret scan / SAST**, **environment preflight** | Deterministic, named refusals, cannot be rationalized | Every commit                    |
| **Merge boundary** (per story)   | One decisive evaluator cycle (diff vs. acceptance contract; tier-capped rounds) + CI + Tier-3 drivers                           | Expensive, partially gameable → risk-tiered rounds    | Once per story under `standard` |
| **Deploy boundary** (per layer)  | Tier-2 journey dogfood on the real target, **full regression replay**, coverage matrices, gate evidence                         | Expensive but tamper-evident (real execution)         | Once per layer                  |
| **Operate** (post-deploy)        | Telemetry / production outcomes → `/aep-watch` → `/aep-reflect`                                                                 | The least gameable evidence that exists               | Continuous                      |
| **Human gate**                   | Residual-risk acceptance at layer advance; taxonomy escalations                                                                 | Most expensive                                        | Per layer + exceptions          |

Two consequences worth naming:

- **Scaling verification means adding typed gates and real-environment evidence, not judge rounds.**
  An LLM-judge round is the only validator class that gets _simultaneously_ more expensive and more
  gameable as models improve. Long-horizon models make each round more thorough — which is precisely
  why rounds must be fewer and more decisive, not more numerous.
- **A verifier must not share incentives with the generator.** AEP already separates
  generator/evaluator (`gen-eval/SKILL.md`) and makes `/aep-wrap` execute-never-author journeys. This
  proposal extends the same separation to _evidence_ (§ Design 5) and to _classification authority_
  (§ Design 1): any label that reduces verification spend must come from a non-generator role or
  from world-derivable evidence.

---

## The design

### 1. Failure taxonomy — typed, evaluated before any repair spend

Every FAIL from any tier acquires a **`failure_class`**, assigned **per finding** (one incident may
carry findings of different classes):

```
failure_class: product-defect | environment | harness-flake | scope
```

- **`product-defect`** — the built thing is wrong. Routes to the existing path: finding → story /
  fix commit → gen-eval. **Security-critical product defects keep their existing immediate
  escalation** (`recovery-ladder.md:63`): they are `product-defect`, and they skip the ladder to a
  human on the first FAIL — the taxonomy adds routing, it never removes an escalation.
- **`environment`** — a precondition outside the worktree is unmet (secrets, credentials, wrong
  account, target unreachable, quota exhausted). Routes to an **ops checklist surfaced to the
  human/orchestrator. Never auto-files a code story; never enters the recovery ladder; never spends
  an evaluation round.** The gate stays refused with a named tag until the environment is repaired,
  then re-runs. The checklist may _recommend_ human-filed hardening stories through the normal path.
  **Exception — unmet in-repo dependencies** (an unmerged story, an unbuilt sibling module) are a
  sequencing problem: they route to `/aep-dispatch` re-ordering, not to the ops checklist.
- **`harness-flake`** — the test machinery itself misbehaved (race, port collision, known-red
  baseline). Routes to quarantine + a harness story; the product gate re-runs after quarantine.
- **`scope`** — the acceptance criterion itself is wrong, the story is mis-sliced, or the spec is
  internally contradictory (the ladder's existing "spec contradiction" skip maps here). Routes to
  `/aep-reflect` re-slicing, not to repair rounds.

**Classification authority — the tamper-resistance rule.** The routing above is only safe if the
FAILing party cannot label its own failure into a cheaper class ("the test is flaky" is the
canonical reward hack). Each spend-reducing class therefore has an **evidence requirement**, and
anything without qualifying evidence defaults to `product-defect`:

| Class            | Claimable by                                                                                                                                                                                                           |
| ---------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `environment`    | **Only** a named preflight/probe refusal tag (`REFUSING [dogfood-secret-absent:<NAME>]`). No tag → not environment.                                                                                                    |
| `harness-flake`  | World-derivable reproduction evidence (a same-SHA re-run that passes with zero diff, or membership in an orchestrator-owned quarantine list), **ratified by wrap//aep-reflect — never self-applied by the generator**. |
| `scope`          | A human acknowledgment on the `needs-human.md` gate record before the routing takes effect.                                                                                                                            |
| `product-defect` | The default. Requires no evidence — it is the class that spends verification.                                                                                                                                          |

**Ownership check (dual-class incidents).** When a refused precondition is created or managed by
artifacts _in this repo_ (deploy config, IaC, wrangler bindings, seed scripts), the ops checklist is
paired with a `product-defect` finding for the wiring — otherwise the ops path becomes a way to mask
product defects behind hand-repair. looplia's secret incident is the worked case: the missing GitHub
secret was `environment`; the deploy config that silently omitted the fixture binding and the
tracked literal-password fallback were `product-defect` findings discovered by the same probe.

**Typed carriers, per surface** (a taxonomy that lives only in prose is the drift class this repo
already documented):

- **Tier-2/3 dogfood and post-merge guard reports** — the unified report
  (`executor/references/dogfood-validation.md` → Unified report format) gains a required
  `**Failure-Class:**` line per finding; the `dogfood_report` adapter
  (`_shared/references/telemetry-ingestion.md`) parses it and **never auto-files** `environment` /
  `harness-flake` / `scope` findings.
- **Phase 5 evaluation FAILs** — the **evaluator** (not the generator) writes `failure_class` into
  `eval-response-<N>.md` / `feature-verification.json`, consistent with the existing field-ownership
  rule ("the generator cannot mark its own work as passing", `eval-protocol.md:265`).
- **Build/CI failures** — `status.json` failure logs gain `failure_class` alongside the existing
  `error_class` enum (`product-context-schema.yaml:348`: `test_failure | timeout | context_overflow
| merge_conflict`). `error_class` records execution mechanics and stays; `failure_class` is the
  routing layer above it. Default mapping (overridable with evidence): `test_failure` →
  `product-defect`; `timeout` / `context_overflow` → `harness-flake`; `merge_conflict` → sequencing
  → `/aep-dispatch`.

**The environment preflight gate.** Probes split into two sets, because half of looplia's L31
failure was knowable before any merge:

- **Deploy-independent probes** — required secret _names_ present in CI config, expected account
  fingerprint recorded vs. actual auth identity, env-var names wired in deploy config. These run
  **pre-merge in build Phase 6 Step B on every story, regardless of `journey_timing`** — they need
  no live target.
- **Target-bound probes** — target reachable, deployed bindings present, fixtures seedable. These
  run wherever journey _execution_ runs: Phase 6 Step B under `journey_timing: pre-merge`; the
  `/aep-wrap` layer gate (and the autopilot post-merge guard) under `post-deploy`. Under
  `post-deploy` an environment refusal therefore surfaces post-merge **by design** — the win over
  today is that it surfaces as a named, zero-spend refusal instead of ten evaluation rounds.

A refusal names every unmet precondition (`REFUSING [dogfood-secret-absent:<NAME>]`,
`REFUSING [auth-identity-mismatch:<got≠want>]`) _before_ any journey spend: zero scenarios run, zero
findings generated, zero rounds spent — `environment` by construction. Probes are declared in the
generated e2e skill's `policy.md` next to the target they guard, **derived per gate from
`live_policy`** (§ Design 5): a gate probes only the preconditions of criteria it _requires_ — the
optional live half of a non-milestone gate is not probed, and its absence stays **SKIP**, preserving
the existing degrade semantics (`REFUSED` blocks only what the gate requires; `SKIP` = optional
capability absent; `FAIL` = the product misbehaved). CLI/TUI projects get a non-vacuous probe
vocabulary: provider auth present, provider reachable, PTY driver healthy, fixture repos creatable.

The recovery ladder keeps its rungs, but the taxonomy check is promoted from a prose "When to Skip"
bullet to a **mandatory step at every FAIL, before choosing a rung**: only `product-defect` climbs
the ladder, and the existing skip rules map onto classes (security → `product-defect` +
immediate escalation; spec contradiction → `scope`; missing credential/access → `environment`;
unbuilt in-repo dependency → dispatch re-ordering).

### 2. Risk-tiered verification depth

Each story gets a **`verification_tier`** — derived **twice**, because the honest inputs change
between planning and merge:

- **Provisional (dispatch time):** computed from the story's plan fields — declared
  `files_affected`, `module`, contract obligations, `complexity`, `on_critical_path`, layer kind —
  into the machine-assembled brief. This prices the build (evaluator effort, dogfood scope) but is a
  _prediction_: `files_affected` is authored by the planning pipeline, not observed.
- **Binding (Phase 5 entry):** re-derived from the **actual diff** (`git diff --name-only
"$BASE"...HEAD` — genuinely world-derived, and it exists as soon as Phase 4 implementation
  commits land) against the human-owned sensitive-path list. The binding derivation must run
  **before the first evaluation round consumes the cap** — binding at the merge step would let the
  whole eval loop run on the gameable provisional tier. If the diff leaves the declared scope
  (`diff ∖ files_affected ≠ ∅`), the tier is recomputed from the diff and `scope_drift: true` is
  recorded. **At binding, a tier may only go up, never down** — a story cannot talk its way into a
  cheaper class by under-declaring scope, and impacted-journey selection (§ Design 4) keys off the
  merged diff, never the declaration. **Pre-merge re-check (Phase 12):** commits added after the
  last PASS (Phase 11 review fixes, Phase 11.5 human-eval fixes) re-run the derivation; a
  post-eval drift into `sensitive_paths` upgrades the tier and requires one fresh evaluation round
  at the new tier — this rides the existing stale-eval rule (`tick-protocol.md:237`, "code has
  changed since your last evaluation"), it does not add a new mechanism.

**The derivation function** (deterministic; ties resolve upward):

- **`deep`** iff any: the diff (or, provisionally, the plan) matches a **`sensitive_paths`** glob —
  a human-owned list in the generated e2e skill's `policy.md` covering auth, payments, irreversible
  data operations, migrations, deploy/CI config; **or** the layer is the walking skeleton (0/0.5);
  **or** a human override says so. This is the hard floor; it is never loosenable by calibration.
- **`light`** iff all: the change is **docs-only** by path (tests are _not_ docs — see the
  referee-asset rule below), no contract obligations, no new non-doc files — confirmed by the
  binding diff, not just the plan.
- **`standard`** otherwise (the default; multi-module or `on_critical_path` stories are at least
  `standard`).

**Referee assets never derive `light`.** Test directories, journey specs
(`skills/e2e-test/journeys/**`, including their `paths:` front-matter), the e2e skill's
`policy.md`, and CI workflow definitions are the evidence base every later verifier stands on — a
diff confined to them is a change to the _referee_, not a cheap change. The design already refuses
generator edits to `.github/workflows/**` ("a generator that edits its own referee is not
evidence", § Design 5); the same rule must cover the rest of the referee class, or the derivation
function hands the reward-hacking generator a designed-in channel: delete assertions or weaken
oracles in a tests-only diff and receive the cheapest, self-reviewed tier by construction — and the
taxonomy _funnels_ work into that hole, because a `harness-flake` quarantine story is typically
tests-only. Therefore: **a diff touching referee assets floors at `standard`**, and a **negative
assertion delta** (more test/`Verify` lines removed than added) always requires an evaluation
round, whatever the tier. Journey `paths:` edits after first authoring trigger the same scope
refusal as workflow edits.

**The recipe is a typed artifact, not a remembered rule.** The binding derivation emits
`.dev-workflow/verification-recipe.json` (tier, dimension preset, hard floors, referee-asset and
`scope_drift` flags), and **Phase 5 refuses to start without it** — by the companion doc's own law,
a derivation function executed from prose recall is exactly the class of mechanical step that
eventually gets skipped. Downstream projects get a runnable reference implementation (derivation
function + preflight probe stubs + recipe emission) shipped with the e2e scaffolding skill —
reference scripts are established precedent there (`scripts/audit.sh`, `scripts/converge.sh`);
AEP still ships no runtime.

| Tier                   | Evaluator rounds (cap)                      | Evaluator effort  | Dogfood scope (Phase 6B)          | Full-suite runs               |
| ---------------------- | ------------------------------------------- | ----------------- | --------------------------------- | ----------------------------- |
| **light**              | 0 (generator self-review, lenient)          | —                 | render smoke / none               | focused only                  |
| **standard** (default) | **2** — one decisive fix-and-reverify cycle | session default   | impacted-surface journey + canary | focused; full once at land    |
| **deep**               | up to 5 + full recovery ladder              | highest available | full journey + prior-layer replay | full pre-eval **and** at land |

**The derivation outputs a recipe, not just a depth.** The scoring framework already carries the
other half of verification cost: _which dimensions_ the evaluator scores
(`scoring-framework.md` → Dimension Presets — UI-heavy, API-only, Security-sensitive, Data
pipeline, Mixed) and _which dimension-level hard floors_ apply (e.g. the Security-sensitive
preset's `Security < 4` fail). Today preset selection is an interactive judgment at launch — made
from the **same signals the tier derives from**, but independently, so a `sensitive_paths` story
can derive `deep` while its evaluator runs the Mixed preset without the Security hard floor. The
derivation therefore emits one **verification recipe** — tier + dimension preset + dimension hard
floors — from one set of inputs:

- `sensitive_paths` match → **Security-sensitive preset**, and its `Security ≥ 4` /
  `Data Privacy ≥ 4` floors become part of the tier hard floor — they survive any customization;
  the recipe also adds the **deterministic security gates** (secret scan, SAST) as inner-loop typed
  gates — the mechanically detectable half of security must never be routed to a more expensive LLM
  judge, and their findings are `product-defect` by construction;
- `ui`-kind module / `calibration_type` in {visual-design, ux-flow} / `object_model_refs` →
  **UI-heavy preset**, and only these stories pay for **Visual Design** — the most expensive
  dimension (screenshot capture + multimodal evaluation). Dimension cost follows the shipped
  surface, never habit;
- data/migration paths → **Data pipeline preset** (Data Integrity floor);
- story-map / product-context artifacts (walking-skeleton design) → the **Product & Design
  dimensions**;
- otherwise → **Mixed** with default thresholds.

Customization keeps the ratchet direction: launch or the human may **add** dimensions or raise
thresholds, never drop a derived preset's hard-floor dimensions — the same
tamper-resistance rule as the tier binding. Per tier: `light` scores no dimensions (self-review);
`standard` runs the derived preset; `deep` runs it with no de-weighted dimensions — and, where more
than one model family is available, **`deep`-tier evaluation prefers a different model family from
the generator**: cross-family judging reduces correlated generator/judge blind spots (SIBYL's
`gpt-5.6-terra`-judging-Claude-output split is the downstream prior art). The framework's
existing advice — _"weight dimensions toward areas where the model falls short"_ — becomes
data-driven through the accounting below: findings and escapes are attributable per dimension, so
calibration can propose re-weighting from evidence instead of intuition.

**How the tier travels the trigger path.** Gen-eval is not self-triggering — it is configured down
the chain autopilot → dispatch → launch → build, and the tier must ride that exact chain or it
governs nothing:

- **Dispatch** computes the provisional tier into the machine-assembled brief (grouped changes —
  `compile_mode: grouped_change`, one workspace for N stories — take the **max** of member tiers;
  Dynamic Workflow batches, which bypass `/aep-launch` entirely, carry the tier in the STEP-0 brief
  and apply it to the workflow's verify stage, the evaluator of that mode).
- **Launch is the tier's first real consumer**: today evaluator _existence_ is launch's full/light
  call ("offer one when the change is full mode", `launch/SKILL.md:199-205`) and the criteria file
  it writes (`.dev-workflow/evaluator-criteria.md`) is what makes build Phase 5 spawn an evaluator
  at all. Under this design that decision becomes tier-derived and mechanical: `light` → no
  criteria file (build self-reviews); `standard` → criteria from the scoring-framework presets;
  `deep` → tailored criteria + the highest-available evaluator effort (the effort hint travels to
  `executor.spawn_evaluator()`'s recipes). This also fixes the autonomous gap — the criteria
  brainstorm is interactive today, and autopilot launches had no rule for it.
- **The workspace publishes the tier**: `status.json` gains `verification_tier`, `tier_escalated`,
  and the latest FAIL's `failure_class` (the signals spec is the schema owner), because that signal
  file is the **only** thing autopilot may read.
- **Autopilot's monitoring goes tier-aware**: the ④b nudge texts currently say "spawn the
  evaluator" unconditionally and the escalation rule waits for a 5-round ladder — under tiers, the
  nudge must match the workspace's published tier (a `light` workspace is nudged to self-review,
  not to spawn), and `eval_not_converging` fires only after the published tier's cap **plus** the
  automatic `standard → deep` escalation have both been spent.

- **Cap exhaustion is defined, not silent:** when `standard` exhausts its 2 rounds on a genuine
  `product-defect`, the story **auto-escalates once to `deep`** (recorded as `tier_escalated: true`
  in the execution record — which is itself calibration data) and continues the ladder from where it
  left off; only after `deep`'s ladder exhausts does it reach the human gate. Ladder rungs re-key to
  _position past the tier's cap_ rather than absolute round numbers 3–5.
- **The layer gate's depth is owned by `/aep-wrap`, independent of any story's tier.** A `light`
  integration story does not weaken the gate: full replay, coverage matrices, and evidence classes
  run at the gate whatever the tiers of the stories inside the layer.
- **Light-mode subsumption:** the design-time Light mode's eval-loop behavior
  (`design/references/workflow-modes.md`) is subsumed — Light mode selects `verification_tier:
light` instead of carrying its own eval-loop toggle, collapsing the repo's three near-identical
  "light" senses into one glossary-defined term. The same reconciliation absorbs launch's
  full/light evaluator-offer criteria ("3+ tasks, UI-heavy, or security-sensitive → full",
  `launch/SKILL.md:203`): those heuristics become **inputs to the provisional derivation** at
  dispatch, not a second, independently-consulted depth switch.
- **Threshold semantics (all tiers):** PASS means **zero blocking findings** against the
  hard-failure thresholds (`gen-eval/references/scoring-framework.md:107`). A perfect aggregate
  score is **never** a gate condition — perfect-score gates train evaluator-gaming and block
  convergence (looplia's 5.00/5.00 rule is the counterexample).
- The existing round protocol (`gen-eval/references/eval-protocol.md:116`, `max_rounds` default 5)
  becomes tier-derived; `/aep-build` Phase 5's "max 5" (`build/SKILL.md:126`) reads the tier from
  the brief.

### 3. Verification accounting — priced, recorded, calibrated

`execution-record.yaml` (`wrap/references/convergence.md:70-88`) gains a `verification:` block. The
fields split into a **mandatory file-derivable floor** — every field marked `MUST` below is
computable from artifacts the workflow already writes, and an implementation that leaves them null
has not implemented this design — and best-effort fields that stay nullable. The motivating
evidence is the warning: looplia ran 444 ticks with `total_cost_usd: 0`; a calibration loop built
on optional sensors starves.

```yaml
verification:
  tier: light | standard | deep # MUST — from verification-recipe.json
  tier_escalated: true | false # MUST — cap-exhaustion escalation fired
  scope_drift: true | false # MUST — binding diff left the declared files_affected
  generator_model: <id> | null # MUST when known — model swaps shift both escape rate and findings
  evaluator_model: <id> | null # MUST when an evaluator ran
  eval_rounds: <n> | null # MUST when an evaluator ran — from signals/eval-response-*.md count
  findings_by_round: [<n>, ...] | null # MUST when an evaluator ran — needs per-round persistence
  finding_dimensions: [<dimension>, ...] | null # dimensions breached across rounds; feeds re-weighting
  journey_scenarios_run: <n> | null # MUST when a journey ran — from the dogfood report
  preflight_refusals: [] # MUST — named tags; [] when preflight passed
  cost_usd: <n> | null # best-effort — verification share when separable
  escaped_defects: [] # filled retroactively by /aep-reflect
```

Suite-level economics (`suite_runs`, `suite_seconds`) live in the **layer budget box** (the
layer-gate evidence doc), where `/aep-wrap` actually runs the suites — not in the per-story record
where they proved ungatherable downstream. **Per-round eval persistence** (keeping
`eval-response-<N>.md` through wrap's gather) becomes an explicit requirement of the implementation:
`findings_by_round` is the loosening signal the calibration loop depends on, and today only one
consumer's workspaces retain it.

Consumers close the loop:

- **Escape-rate ingestion:** when `/aep-reflect` classifies a post-merge bug, it traces the bug to
  the story that introduced it and appends to that story's `escaped_defects`. Escape rate is defined
  **per story per tier** (escapes ÷ stories at that tier — well-defined even for `light`'s zero
  rounds). When attribution is ambiguous (multi-story interaction bugs), the escape attributes to
  the **layer**, not to no one — ambiguous escapes must count somewhere.
- **Bidirectional calibration, dampened.** Layer distillation (already proposal-only,
  `wrap/references/convergence.md` §2) may propose tier-derivation adjustments in both directions —
  but the two directions have asymmetric feedback latency (loosening feels good immediately;
  escapes surface a layer later), which is the textbook oscillation setup. Dampening rules:
  **loosening proposals require ≥2 layers of `findings_by_round` evidence and zero unresolved
  escape attributions; loosen at most one notch per layer; the `sensitive_paths` hard floor is
  never loosenable.** Tightening has no such damper. **Calibration proposals must condition on
  model version** (`generator_model` / `evaluator_model` from the accounting block): a model swap
  shifts escape rate and findings-per-round simultaneously, and a loop that cannot see the swap
  will misattribute the shift to tier settings. Verification assets get the same lifecycle:
  distillation may propose merging or retiring journeys that have replayed green for N layers with
  no coverage loss. Proposals only; a human applies them. This is what breaks the ratchet.
- **The layer budget box:** each layer gate's evidence doc records expected vs. actual verification
  spend (rounds, suite runs/time, scenarios, `cost_usd` where known). **The expected values are
  human-owned** — set at layer planning or accepted from a distillation proposal — never authored by
  the loop that is graded against them. **Cold start is defined, not improvised:** the first
  instrumented layer runs with no box and only records; its observed actuals plus a human-chosen
  margin become the next layer's expected values. When actuals overrun the box, `/aep-wrap`
  surfaces a **scope-vs-verification tradeoff to the human** at the layer-advance gate — the loop
  asks, it does not silently grind. (looplia's tick-444 halt is the degenerate form of this
  question, asked ten rounds too late.)

### 4. Regression replay moves from story to layer granularity

- **Per story** (`/aep-build` Phase 8, `build/SKILL.md:237`): replay the **impacted** journey set —
  selected against the **merged diff** (never the declared `files_affected`): a journey is impacted
  when its `covers:` criteria belong to the story, when its optional `paths:` front-matter globs
  intersect the diff, or when it declares no `paths:` at all (fail-open: undeclared journeys stay in
  the replay set). **Plus, always, the walking-skeleton journey as a canary** — one execution that
  catches the "the app no longer starts" class immediately. `deep`-tier stories replay the full
  prior-layer set per story (the one exception to impacted-only).
- **Per layer** (`/aep-wrap` layer gate, `wrap/references/layer-advance.md`): the **full**
  prior-layer replay stays here, once per layer. For large layers, a **mid-layer full-replay
  checkpoint every k stories** keeps gate-time bisects tractable — and k is **derived, not an
  unguided planning judgment**: default `k = min(5, ⌈N/3⌉)` for a layer of N stories, overridable
  by the human (looplia's L31 had 14 stories — the default gives k=5, bounding defect lifetime to
  5 merges).
- **The honest tradeoff:** executions drop from O(layers × stories) to O(stories + layers), but a
  cross-cutting regression a story's diff does not intersect now lives until the next checkpoint or
  the gate — worst-case defect lifetime is k merges, and gate-time attribution costs a bisect. The
  canary, the fail-open selection rule, and the checkpoint are the price paid to bound that
  latency; layers that cannot tolerate it (or stories at `deep`) keep full replay.

### 5. Tamper-evident evidence classes on every gate

A layer gate may flip to `passed` only when its evidence includes **at least one class the
generator cannot modify**:

1. a CI run bound to the merged SHA — **valid as tamper-evident only when the workflow definitions
   are outside the story's diff scope** (a diff touching `.github/workflows/**` triggers a scope
   refusal or human review; a generator that edits its own referee is not evidence — and the same
   refusal covers the rest of the referee-asset class from § Design 2: journey `paths:` re-scoping
   after first authoring, and `policy.md` edits outside a scaffold run);
2. journey execution performed by `/aep-wrap` (executes-never-authors — existing canon);
3. **read-only golden fixtures with a ledger-equality oracle** — fixture trees the generator's
   workspace cannot write, verified by before/after equality of durable state (SIBYL's
   fixture-repo + SHA-256 ledger comparison is the proven downstream shape; screenshots stay
   diagnostic, never a pass condition);
4. production telemetry ingested by `/aep-watch`.

Additionally:

- **Evaluator prompts are machine-assembled, and spawn authority leaves the player** (extending
  deterministic-orchestration's machine-assembled brief to evaluation): the orchestrating layer —
  not the generator — assembles the evaluator's context (criteria file, contract file, diff range),
  so the generator cannot curate what its judge sees. Today the generator also _spawns_ its own
  evaluator and authors `eval-request.md`, the narrative the judge reads first — two residual
  player-referee channels. Where an orchestrating layer exists (autopilot already nudges Phase 5;
  the main session in interactive runs), **it owns the evaluator spawn**; in genuinely standalone
  builds the spawn recipe stays generator-invoked, but in every mode the machine-assembled
  evaluator prompt marks `eval-request.md` as the **generator's untrusted claim** — data to verify,
  never framing to adopt — consistent with the untrusted-output guard the companion doc already
  mandates for subagent results.
- **`policy.md` gains a `live_policy` decision** for cost-bearing dogfoods (live model calls, quota-
  or fee-metered targets): `every_gate | milestone_gates_only | none`, with the milestone list named
  in the policy — the value set matches the proven downstream shape (SIBYL's
  `live_policy: milestone_gates_only`, which moves from `topology.routing.dogfood` to `policy.md` on
  re-pin; `policy.md` already canonically owns tiers/target/timing). Non-milestone gates use
  zero-cost render smokes / scripted tiers, and — per § Design 1 — their preflight does not probe
  the optional live half, whose absence stays SKIP.

---

## What upstream changes now (v3.1.0 / v3.2.0) vs future

**The two halves of this design carry unequal evidence and get unequal commitment.** The
taxonomy + preflight + replay half is proven by incident — it neutralizes looplia's L31 failure end
to end. The tiers + recipes + calibration half is extrapolated from one story (SIBYL-189) and
carries most of the prose surface. They therefore ship in two releases: **v3.1.0** = items marked
(3.1) below — taxonomy carriers, preflight, deterministic security gates, replay move,
evaluator-independence fixes, and the accounting **instrumentation** (record-only, so the field
data exists); **v3.2.0** = items marked (3.2) — the tier/recipe machinery and the calibration loop
that consume that data, gated on **≥2 layers of real accounting data from a re-pinned consumer**.
One teaching reference plus prose edits across the verification-touching skills; no runtime, no new
required schema fields:

1. **(3.1) NEW `skills/patterns/gen-eval/references/verification-economics.md`** — the canonical reference
   carrying the placement matrix, the failure taxonomy + classification-authority table + routing,
   the `error_class` → `failure_class` mapping, the tier derivation function + two-point protocol,
   the `verification:` accounting schema with gather sources, the evidence-class catalog, and the
   worked examples. It lives under gen-eval because gen-eval already owns evaluation canon and is
   read by `/aep-build`, `/aep-validate`, and `/aep-launch`; every other change site cross-references
   it **by name**.
2. **(3.1) `skills/patterns/executor/references/dogfood-validation.md`** — Unified report format
   (`:160-199`) gains the required `**Failure-Class:**` line + the four-way routing table; Config
   block (`:203-232`) gains a cross-reference stating `live_policy` lives in the e2e skill's
   `policy.md`, not in `topology.routing.dogfood`. **(3.2)** `backends.md`: the
   `executor.spawn_evaluator()` recipes accept an optional evaluator-effort hint derived from the
   tier (`deep` → highest available, preferring a different model family from the generator).
3. **(3.1) `skills/project-setup/e2e-skill-scaffolding/`** — `templates/policy.md.tmpl` + SKILL.md
   Phase 2 confirmation questions gain the `live_policy` decision, the `sensitive_paths` list, the
   **deterministic security gates** (secret-scan/SAST commands the project confirms at scaffold),
   and the preflight probe sets (deploy-independent vs target-bound, with web **and** cli/tui probe
   vocabularies); `templates/layer-gate-evidence.md.tmpl` gains the budget box (record-only until
   3.2); `templates/tool-selection.md.tmpl` notes REFUSED vs SKIP semantics;
   `references/layer-gate-loop.md` and `references/three-tier-model.md` add the preflight gate, the
   evidence-class requirement on `passed`, and the replay-placement note. **(3.2)** a **runnable
   reference implementation** in the generated skill's `scripts/` — derivation function + probe
   stubs + `verification-recipe.json` emission (precedent: `aep-scaffold`'s `audit.sh` /
   `converge.sh`; AEP still ships no runtime).
4. **`skills/agentic-development-workflow/build/SKILL.md`** — **(3.1)** Phase 5: the taxonomy check
   at every FAIL; evaluator spawn authority moves to the orchestrating layer where one exists, and
   `eval-request.md` is marked the generator's untrusted claim. Phase 6 Step B (`:192-215`):
   deploy-independent preflight on every story; target-bound preflight when execution runs here.
   Phase 8 (`:237`): impacted-only replay + canary. **(3.2)** Phase 5 (`:126-145`): the **binding
   tier re-derivation at Phase 5 entry** (before any round spends the cap) emitting
   `verification-recipe.json` (Phase 5 refuses to start without it), tier-capped rounds,
   cap-exhaustion auto-escalation, publishing `verification_tier` / `tier_escalated` to
   `status.json`, and the `deep` full-replay exception in Phase 8. Phase 12: the tier re-check on
   post-eval commits (sensitive-path drift ⇒ one fresh round at the upgraded tier).
5. **`skills/patterns/gen-eval/references/`** — **(3.1)** `eval-protocol.md`: evaluator-authored
   `failure_class` in the response format; per-round response persistence (the sensor data 3.2's
   calibration needs). `agent-contracts.md`: the machine-assembled evaluator prompt treats
   `eval-request.md` as the generator's untrusted claim. `recovery-ladder.md` (`:59-67`): "When to
   Skip the Ladder" reframed as the typed taxonomy step with the class mapping.
   `scoring-framework.md`: zero-blocking threshold semantics; perfect-score gates named as an
   anti-pattern. **(3.2)** `eval-protocol.md` (`:116`) + `recovery-ladder.md` (`:3`, `:25-31`):
   tier-derived `max_rounds`; rungs re-keyed relative to the tier cap. `scoring-framework.md`: the
   Customization Guide's preset-selection step becomes **recipe-derived** (the derivation-inputs →
   preset + hard-floor mapping from § Design 2), with the ratchet rule — customization adds
   dimensions or raises thresholds, never drops a derived hard floor.
6. **(3.2) `skills/agentic-development-workflow/launch/`** — `SKILL.md` "Optional: Evaluator (Full
   Mode)" (`:199-205`): evaluator existence, criteria, and effort become **recipe-derived from the
   dispatch brief** (light → no criteria file; standard → the derived dimension preset; deep →
   the derived preset with nothing de-weighted, at top effort) — this replaces the workflow-modes
   full/light heuristics as launch's decision rule and gives autonomous launches a deterministic
   criteria policy.
   `references/evaluator.md` (`:73-74`): the third `max_rounds` copy goes tier-derived; the
   criteria-brainstorm step assembles `.dev-workflow/evaluator-criteria.md` from the derived
   recipe, and interactive customization only ratchets up (add dimensions / raise thresholds,
   never drop derived hard floors).
   `references/signals-spec.md`: `status.json` gains `verification_tier`, `tier_escalated`, and
   latest-FAIL `failure_class` — the fields autopilot's tier-aware monitoring reads.
7. **(3.1) `skills/agentic-development-workflow/wrap/references/`** — `convergence.md` (`:70-88`):
   the `verification:` block with the mandatory file-derivable sensor floor (instrumentation ships
   early so 3.2 has data). `layer-advance.md`: full replay + the derived mid-layer checkpoint +
   evidence-class check before flipping `passed`; budget-box recording (cold-start: record-only on
   the first instrumented layer). **(3.2)** `layer-advance.md`: budget-box overrun surfacing at the
   layer-advance gate.
8. **(3.2) `skills/product-context/dispatch/`** — `references/context-assembly.md` (the assembly-time
   home): provisional tier derivation into the machine-assembled brief (grouped changes take the
   max of member tiers); `SKILL.md` (`:237`) notes the binding re-derivation contract (Phase 5
   entry, build-owned). `references/workflow-mode.md`: the STEP-0 brief carries the tier; the
   workflow verify stage — that mode's evaluator — honors the tier cap.
9. **(3.1) `skills/product-context/_shared/references/telemetry-ingestion.md`** (regenerated into
   `reflect/` and `watch/` via `build-skills.sh`) — the `dogfood_report` adapter parses
   `**Failure-Class:**`; `environment` / `harness-flake` / `scope` findings **never auto-file**
   stories; escape-rate ingestion appends `escaped_defects`.
10. **`skills/patterns/autopilot/`** — **(3.1)** `references/tick-protocol.md`: REFUSED ≠ FAIL at
    the gate (a refused gate pauses cheaply with the ops checklist and **re-probes world-derived on
    each tick**, so human repair auto-resumes; remaining in-layer stories may still dispatch); the
    ④b Phase-5 trigger nudge notes the orchestrator owns the evaluator spawn.
    `references/state-schema.md` (`:138`): a new `environment_repair` escalation type carrying the
    checklist. `references/post-merge-guard.md`: preflight before the guard's dogfood;
    `Failure-Class:` in the guard report; `live_policy` governs the guard's live half. **(3.2)**
    `tick-protocol.md`: "`eval_not_converging` after the ladder is exhausted" becomes tier-derived
    (fires only after the published cap **plus** the automatic `standard → deep` escalation are
    spent); the ④b nudge texts (`:212`, `:237`) go tier-aware — read `verification_tier` from
    `status.json`, nudge a `light` workspace to self-review rather than "spawn the evaluator", and
    extend the stale-eval nudge with the sensitive-path-drift upgrade.
11. **(3.2) `skills/agentic-development-workflow/design/references/workflow-modes.md`** — Light
    mode's eval-loop behavior subsumed into `verification_tier: light`.
12. **(3.1) `docs/glossary.md`** — new entries: **Verification Tier**, **Verification Recipe** (tier +
    dimension preset + dimension hard floors, one derivation), **Failure Class (Failure
    Taxonomy)**, **Classification Authority**, **Environment Preflight Gate**, **Verification
    Accounting**, **Escape Rate**, **Tamper-Evident Evidence**, **Verification Ratchet**
    (anti-pattern), **Perfect-Score Gate** (anti-pattern), **Referee Asset** — the Verification
    Tier entry disambiguates the three prior "light" senses.
13. `CHANGELOG.md` `[3.1.0]` and later `[3.2.0]`; `.claude-plugin/marketplace.json` bumps per
    release (the sole versioned field — root `package.json` carries none). If any skill description
    changes, re-record the front-tier description digest and update
    `docs/skills-quick-reference.md`. Among the sites above, only `telemetry-ingestion.md` is
    `_shared/`-managed; the SKILL.md and skill-owned reference edits are made in place.

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

| Skill            | New responsibility                                                                                                        |
| ---------------- | ------------------------------------------------------------------------------------------------------------------------- |
| `/aep-gen-eval`  | Owns the verification-economics reference; tier-derived caps; evaluator-authored `failure_class`; zero-blocking semantics |
| `/aep-build`     | Preflight before dogfood; taxonomy check before any ladder rung; impacted replay + canary; cap-exhaustion escalation      |
| `/aep-wrap`      | Full replay + checkpoints + budget box + evidence-class check at the gate; gathers the `verification:` block              |
| `/aep-dispatch`  | Provisional tier into the machine-assembled brief (grouped = max; workflow mode carries it in STEP-0)                     |
| `/aep-reflect`   | Routes on `failure_class`; ratifies `harness-flake`; ingests escape rate; proposes dampened calibration                   |
| `/aep-autopilot` | REFUSED ≠ FAIL ticks; tier-aware ④b nudges + escalation; `environment_repair`; preflight + `live_policy` on the guard     |
| `/aep-launch`    | Evaluator existence + criteria + effort derived from the tier; signals spec carries the tier fields                       |
| e2e scaffold     | `live_policy` + `sensitive_paths` + probe sets in `policy.md`; budget box in the gate evidence template                   |

## Migration

All changes are prose-additive and fail open — with the honesty note that fail-open means
**non-adopting consumers keep today's cost, not today's cost minus savings**: a story with no
derived tier behaves as `deep` (today's full-mode behavior; a design-time Light-mode run keeps its
existing skips and maps to `light`); a report without `Failure-Class:` classifies as
`product-defect` (today's routing — and also the tamper-resistance default); an execution record
without `verification:` stays valid (`story_id` remains the only required field); a gate with no
named evidence class records a warning, not a refusal, for one release. Standalone projects (no
`product-context.yaml`) never get derivation and therefore run `deep` permanently unless the human
sets a tier by hand.

**Consumer adoption step (mid-layer re-pins):** a consumer re-pinning with open failed/blocked gate
records runs a **one-time reclassification pass before any dispatch** — existing failure records are
re-labeled under the taxonomy (looplia's L31 `prod_dogfood: FAIL` becomes `environment` + ops
checklist), and acceptance criteria authored solely by misrouted recovery stories are re-sliced out
of the coverage denominator via `/aep-reflect` (`scope`), so a poisoned gate can reach `passed` once
the environment is actually repaired.

| Phase                                          | Action                                                                                                                                                                                                                                         | Breaking? |
| ---------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------- |
| **P0 — this decision doc**                     | Record the evidence, the placement principle, the five design elements                                                                                                                                                                         | No        |
| **P1 — canon reference (v3.1.0)**              | Ship `gen-eval/references/verification-economics.md` + glossary entries                                                                                                                                                                        | No        |
| **P2 — taxonomy carriers (v3.1.0)**            | `Failure-Class:` + classification authority + referee-asset rule + preflight + secret-scan/SAST + adapter routing + REFUSED≠FAIL/`environment_repair` + replay move/canary/checkpoints + evaluator spawn ownership/untrusted `eval-request.md` | No        |
| **P2.5 — accounting instrumentation (v3.1.0)** | `verification:` sensor floor + per-round persistence + model-version fields + budget-box recording (cold-start record-only); **tag v3.1.0**                                                                                                    | No        |
| **P3 — tiers + recipes (v3.2.0)**              | Two-point derivation + `verification-recipe.json` + reference script + tier caps (build/gen-eval/launch) + recipe-derived criteria + signals fields + tier-aware nudges — **gated on ≥2 layers of P2.5 field data from a re-pinned consumer**  | No        |
| **P4 — calibration (v3.2.0)**                  | Dampened calibration + escape-rate ingestion + journey retirement + budget-box overrun surfacing; **tag v3.2.0**                                                                                                                               | No        |

**Exact change sites:** the numbered list under "What upstream changes now" is the implementation
PRs' review contract.

**Propagation discipline:** this proposal introduces net-new taxonomy — _failure class_,
_classification authority_, _verification tier_, _environment preflight gate_, _tamper-evident
evidence_, _escape rate_, _verification ratchet_, _perfect-score gate_ — and two new enums
(`failure_class` values; `live_policy: every_gate | milestone_gates_only | none`). Every skill that
names one must say it identically; the glossary defines each; audit with
`grep -rn "failure_class\|verification_tier\|live_policy\|sensitive_paths\|preflight" skills/ docs/`
after implementation. The `live_policy` propagation set is enumerated (per the half-applied-enum
history): `templates/policy.md.tmpl`, `e2e-skill-scaffolding/SKILL.md`, `three-tier-model.md`,
`layer-gate-loop.md`, `build/SKILL.md` Phase 6, `wrap/references/layer-advance.md`,
`executor/references/dogfood-validation.md` (Config block cross-reference),
`_shared/templates/product-context-schema.yaml` (`:415-418` routing comment, `:451-459` layer-gate
comments), `map/SKILL.md` (`:116`, `:217`), `scaffold/references/resulting-structure.md`, and
`autopilot/references/post-merge-guard.md`. The existing `error_class` enum is **not** renamed; its
mapping into `failure_class` lives in the canon reference. `_shared/` edits regenerate via
`scripts/build-skills.sh`; `skills:check` must come back clean. Downstream consumers go live only
after each release tag (v3.1.0, then v3.2.0) is cut and the consumer re-pins via the skills CLI —
**after** first completing the v2.5.0 → v3.0.0 re-pin per the
[migration guide](../aep-v3.0.0-migration-guide.md), so routing changes, taxonomy changes, and
tier-economics changes are canaried separately, one release apart.

---

## Worked examples

**looplia L31, replayed under this design.** The deploy-independent preflight runs pre-merge on the
first L31 story: `REFUSING [dogfood-secret-absent:LOOPLIA_E2E_FIXTURE_SECRET]` (the GitHub secret
name was checkable without any deploy) — the ops checklist surfaces days before the gate. Whatever
slips through, the gate-time target-bound preflight refuses with
`REFUSING [auth-identity-mismatch:account …d422b ≠ …88e6]` before any scenario spend. Classification
is `environment` by construction; the ownership check pairs it with `product-defect` findings for
the unwired deploy binding — filed as an ordinary story, not spun into recovery loops. The tracked
literal-password fallback never even reaches that path: the inner-loop **secret scan** catches it
pre-merge as a typed gate, on the first story that committed it. **Zero journey scenarios wasted, zero evaluation rounds, no
misclassified recovery stories.** The loop still pauses on the human ops checklist — environment
repair is genuinely human work — but it pauses **immediately, cheaply, and correctly labeled**, with
the gate re-probing on each tick so repair auto-resumes; the actual history (two recovery stories,
ten exhausted rounds, a layer stuck at 68/75 on criteria the recovery itself authored) is the cost
of routing an environment failure through product-defect machinery.

**SIBYL-189, replayed under the derivation.** Provisional: multi-module `files_affected`, contract
obligations, not a walking-skeleton layer, no `sensitive_paths` match → `standard`. Binding: the
merged diff stays inside declared scope → `standard` holds. Round 1 caught two real findings; the
fixes landed; round 2 passed with zero blocking findings — exactly `standard`'s one
fix-and-reverify cycle, so the catch quality is unchanged. What the tier removes: the second
full-suite run (~13 minutes; the land verify already runs the suite on merged main) and the
top-shelf evaluator profile for a display-positioning story. The PTY journey and ledger-equality
dogfood stay: they are the tamper-evident half of the gate. (An integration-gate story like
looplia's L31-006 also derives `standard` — and that is safe **because the layer gate's depth is
wrap-owned**, not a property of the story's tier.)

**SIBYL's `live_policy: milestone_gates_only`** (live-model dogfoods only at the L2/L4/L6 milestone
gates; zero-quota render smokes elsewhere) is the proven downstream shape of the `live_policy`
decision this proposal adds to `policy.md` — evidence that pricing the expensive half of dogfood
per-gate, instead of running it uniformly, holds up across 20+ layers of real use. Its SKIP-not-FAIL
degrade for the optional live half is preserved verbatim by the `live_policy`-aware preflight rule.

---

## Anti-patterns this prevents

- **Perfect-score gates.** "Exactly 5.00/5.00 with zero findings" trains the generator to satisfy
  the evaluator, not the product, and turns ordinary convergence into round exhaustion. PASS is zero
  blocking findings.
- **Self-classified failures.** A generator that labels its own FAIL `harness-flake` or
  `environment` is player, referee, and witness at once; spend-reducing classes require
  non-generator evidence, and the default is `product-defect`.
- **Environment failures fed to code-repair machinery.** A missing secret is not a 2.79/5.00 code
  problem; no evaluation round may be spent before the taxonomy step runs.
- **Per-story full-regression replay.** O(layers × stories) journey executions; full replay belongs
  to the layer gate and its checkpoints.
- **Uninstrumented verification.** 444 ticks with `total_cost_usd: 0` recorded means the loop cannot
  know it is over-verifying; unpriced spend grows until it displaces the product.
- **The verification ratchet.** A lessons loop that only ever adds verification converges on a
  process that verifies instead of shipping; calibration must be able to loosen and retire — with
  dampers, because asymmetric feedback oscillates.
- **Scaling verification with judge rounds.** More LLM-judge rounds is the only scaling direction
  that gets more expensive and more gameable at once; add typed gates and real-environment evidence
  instead.
- **Self-curated evidence.** A generator that assembles its own evaluator's context, writes the
  fixtures it is graded against, or edits the CI workflows that judge it, is not being verified.

---

## References

- `SIBYL/openspec/changes/archive/2026-07-15-SIBYL-189/` (`tasks.md`, `execution/eval.yaml`) and
  `SIBYL/skills/e2e-test/results/2026-07-16-s189-current-position.md` — the S189 verification
  lifecycle and the ledger-equality oracle.
- `looplia/.dev-workflow/autopilot-status.md`, `looplia/.dev-workflow/dogfood-l31-006.md` and
  `…/dogfood-l31-006/02-auth-recovery-audit.md`, `looplia/lessons-learned/l31-014-…md` — the L31
  halt, the environment misrouting, the dual-class product findings, and the perfect-score ratchet.
- `SIBYL/product-context.yaml` → `topology.routing.dogfood.live_policy: milestone_gates_only` and
  `SIBYL/skills/e2e-test/tool-selection.md` (SKIP-not-FAIL degrade) — the downstream prior art for
  `live_policy` and the REFUSED/SKIP distinction.
- [deterministic-orchestration.md](deterministic-orchestration.md) — the mechanical/judgment split
  and typed-gate pattern this document extends to verification routing, depth, and accounting.
- [build-convergence-pipeline.md](build-convergence-pipeline.md) — the execution-record gather this
  document's `verification:` block rides on.
- [2026-07-15 looplia compatibility audit](../audits/2026-07-15-looplia-v2.5.0-to-v3.0.0-compatibility.md)
  and the [v3.0.0 migration guide](../aep-v3.0.0-migration-guide.md) — the v2.5→v3 baseline both
  consumers cross before any of this lands downstream.
- Affected skills: `/aep-gen-eval`, `/aep-build`, `/aep-wrap`, `/aep-dispatch`, `/aep-reflect`,
  `/aep-watch`, `/aep-autopilot`, `/aep-launch`, `/aep-design`, `/aep-e2e-skill-scaffolding`,
  `/aep-executor`.
