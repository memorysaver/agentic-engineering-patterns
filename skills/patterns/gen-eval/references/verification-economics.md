# Verification Economics

The canonical reference for **pricing and placing verification**: where each validator class runs, how failures are classified before any repair spend, how verification depth is derived per story, and how verification spend is recorded and calibrated. Decision rationale and the motivating evidence live in `docs/decisions/verification-economics.md` (AEP repo); this file is the operative contract consumers execute.

Core law: **cheap and tamper-evident validators run in the inner loop, always; expensive or gameable validators run at the outermost boundary that still catches their failure class, gated by risk.** An LLM-judge round is the only validator class that gets _simultaneously_ more expensive and more gameable as models improve — so scaling verification means adding typed gates and real-environment evidence, never more judge rounds.

---

## Table of Contents

1. [The Placement Matrix](#the-placement-matrix)
2. [Failure Taxonomy](#failure-taxonomy)
3. [Classification Authority](#classification-authority)
4. [The Environment Preflight Gate](#the-environment-preflight-gate)
5. [Verification Tiers](#verification-tiers)
6. [The Two-Point Derivation](#the-two-point-derivation)
7. [The Verification Recipe](#the-verification-recipe)
8. [Verification Accounting](#verification-accounting)
9. [Tamper-Evident Evidence Classes](#tamper-evident-evidence-classes)
10. [Worked Examples](#worked-examples)
11. [Anti-Patterns](#anti-patterns)
12. [Cross-References](#cross-references)

---

## The Placement Matrix

Classify every validator on two axes — **cost per run** and **gameability** (can the generator, or a reward-hacking future model, satisfy the check without satisfying the intent?) — then place it:

| DevOps position                  | Validators                                                                                                                      | Properties                                            | Frequency                       |
| -------------------------------- | ------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------- | ------------------------------- |
| **Inner loop** (per task/commit) | Typed gates: lint, typecheck, focused Tier-1, schema validation, scope gates, **secret scan / SAST**, **environment preflight** | Deterministic, named refusals, cannot be rationalized | Every commit                    |
| **Merge boundary** (per story)   | One decisive evaluator cycle (diff vs. acceptance contract; tier-capped rounds) + CI + Tier-3 drivers                           | Expensive, partially gameable → risk-tiered rounds    | Once per story under `standard` |
| **Deploy boundary** (per layer)  | Tier-2 journey dogfood on the real target, **full regression replay**, coverage matrices, gate evidence                         | Expensive but tamper-evident (real execution)         | Once per layer                  |
| **Operate** (post-deploy)        | Telemetry / production outcomes → `/aep-watch` → `/aep-reflect`                                                                 | The least gameable evidence that exists               | Continuous                      |
| **Human gate**                   | Residual-risk acceptance at layer advance; taxonomy escalations                                                                 | Most expensive                                        | Per layer + exceptions          |

A verifier must not share incentives with the generator. AEP already separates generator/evaluator (`SKILL.md`) and makes `/aep-wrap` execute-never-author journeys; this reference extends the same separation to **evidence** (§9) and to **classification authority** (§3): any label that reduces verification spend must come from a non-generator role or from world-derivable evidence.

---

## Failure Taxonomy

Every FAIL from any tier acquires a **`failure_class`**, assigned **per finding** (one incident may carry findings of different classes):

```
failure_class: product-defect | environment | harness-flake | scope
```

| Class            | Meaning                                                                                                       | Routes to                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| ---------------- | ------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `product-defect` | The built thing is wrong                                                                                      | The existing path: finding → story / fix commit → gen-eval. **Security-critical product defects keep their existing immediate escalation** (recovery-ladder.md → When to Skip): they skip the ladder to a human on the first FAIL — the taxonomy adds routing, it never removes an escalation.                                                                                                                                                                                                                                     |
| `environment`    | A precondition outside the worktree is unmet (secrets, credentials, wrong account, unreachable target, quota) | An **ops checklist surfaced to the human/orchestrator. Never auto-files a code story; never enters the recovery ladder; never spends an evaluation round.** The gate stays refused with a named tag until the environment is repaired, then re-runs. The checklist may _recommend_ human-filed hardening stories through the normal path. **Exception — unmet in-repo dependencies** (an unmerged story, an unbuilt sibling module) are a sequencing problem: they route to `/aep-dispatch` re-ordering, not to the ops checklist. |
| `harness-flake`  | The test machinery itself misbehaved (race, port collision, known-red baseline)                               | Quarantine + a harness story; the product gate re-runs after quarantine.                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `scope`          | The acceptance criterion is wrong, the story is mis-sliced, or the spec is internally contradictory           | `/aep-reflect` re-slicing, not repair rounds. (The ladder's existing "spec contradiction" skip maps here.)                                                                                                                                                                                                                                                                                                                                                                                                                         |

**The taxonomy check is mandatory at every FAIL, before choosing a recovery rung.** Only `product-defect` climbs the ladder. The ladder's existing skip rules map onto classes: security → `product-defect` + immediate escalation; spec contradiction → `scope`; missing credential/access → `environment`; unbuilt in-repo dependency → dispatch re-ordering.

**Ownership check (dual-class incidents).** When a refused precondition is created or managed by artifacts _in this repo_ (deploy config, IaC, wrangler bindings, seed scripts), the ops checklist is paired with a `product-defect` finding for the wiring — otherwise the ops path becomes a way to mask product defects behind hand-repair.

### Typed carriers, per surface

A taxonomy that lives only in prose is the drift class `docs/decisions/deterministic-orchestration.md` documents. Each surface carries the class in a typed field:

- **Tier-2/3 dogfood and post-merge guard reports** — the unified report (`/aep-executor` → dogfood-validation.md) carries a required `**Failure-Class:**` line per finding; the `dogfood_report` adapter (telemetry-ingestion.md in `/aep-reflect` / `/aep-watch`) parses it and **never auto-files** `environment` / `harness-flake` / `scope` findings.
- **Phase 5 evaluation FAILs** — the **evaluator** (not the generator) writes `failure_class` into `eval-response-<N>.md` / `feature-verification.json`, consistent with the existing field-ownership rule (the generator cannot mark its own work as passing; eval-protocol.md → Field ownership).
- **Build/CI failures** — `status.json` failure logs carry `failure_class` alongside the existing `error_class` enum (`test_failure | timeout | context_overflow | merge_conflict`). `error_class` records execution mechanics and stays; `failure_class` is the routing layer above it.

### `error_class` → `failure_class` default mapping

The default is what routes when no evidence exists — and per Classification Authority (below), **no
execution mechanic reduces spend on its own**. `timeout` / `context_overflow` only _nominate_
`harness-flake`; they **route as `product-defect`** until wrap/`aep-reflect` ratifies the reproduction
evidence (same-SHA passing re-run or quarantine membership), and only that ratification flips the class:

| `error_class`      | Default `failure_class`                                      |
| ------------------ | ------------------------------------------------------------ |
| `test_failure`     | `product-defect`                                             |
| `timeout`          | `product-defect` (`harness-flake` **candidate** — see below) |
| `context_overflow` | `product-defect` (`harness-flake` **candidate** — see below) |
| `merge_conflict`   | sequencing → `/aep-dispatch`                                 |

---

## Classification Authority

The routing above is only safe if the FAILing party cannot label its own failure into a cheaper class ("the test is flaky" is the canonical reward hack). Each spend-reducing class has an **evidence requirement**, and anything without qualifying evidence defaults to `product-defect`:

| Class            | Claimable by                                                                                                                                                                                                            |
| ---------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `environment`    | **Only** a named preflight/probe refusal tag (`REFUSING [dogfood-secret-absent:<NAME>]`). No tag → not environment.                                                                                                     |
| `harness-flake`  | World-derivable reproduction evidence (a same-SHA re-run that passes with zero diff, or membership in an orchestrator-owned quarantine list), **ratified by wrap/`aep-reflect` — never self-applied by the generator**. |
| `scope`          | A human acknowledgment on the `needs-human.md` gate record before the routing takes effect.                                                                                                                             |
| `product-defect` | The default. Requires no evidence — it is the class that spends verification.                                                                                                                                           |

---

## The Environment Preflight Gate

Probes split into two sets, because half of the class of environment failures is knowable before any merge:

- **Deploy-independent probes** — required secret _names_ present in CI config, expected account fingerprint recorded vs. actual auth identity, env-var names wired in deploy config. These run **pre-merge in `/aep-build` Phase 6 Step B on every story, regardless of `journey_timing`** — they need no live target.
- **Target-bound probes** — target reachable, deployed bindings present, fixtures seedable. These run wherever journey _execution_ runs: Phase 6 Step B under `journey_timing: pre-merge`; the `/aep-wrap` layer gate (and the autopilot post-merge guard) under `post-deploy`. Under `post-deploy` an environment refusal therefore surfaces post-merge **by design** — as a named, zero-spend refusal, not evaluation rounds.

A refusal names every unmet precondition (`REFUSING [dogfood-secret-absent:<NAME>]`, `REFUSING [auth-identity-mismatch:<got≠want>]`) _before_ any journey spend: zero scenarios run, zero findings generated, zero rounds spent — `environment` by construction.

Probes are declared in the generated e2e skill's `policy.md` next to the target they guard, **derived per gate from `live_policy`** (`every_gate | milestone_gates_only | none` — canonical home: `policy.md`): a gate probes only the preconditions of criteria it _requires_. The optional live half of a non-milestone gate is not probed, and its absence stays **SKIP**. The three result semantics never blur:

- **REFUSED** — a required precondition is unmet; blocks what the gate requires; `environment` by construction.
- **SKIP** — an optional capability is absent; never blocks.
- **FAIL** — the product misbehaved; `product-defect` unless evidence says otherwise.

CLI/TUI projects use a non-vacuous probe vocabulary: provider auth present, provider reachable, PTY driver healthy, fixture repos creatable.

---

## Verification Tiers

Each story gets a **`verification_tier`** governing evaluator rounds, evaluator effort, and dogfood scope:

| Tier                   | Evaluator rounds (cap)                      | Evaluator effort  | Dogfood scope (Phase 6B)          | Full-suite runs               |
| ---------------------- | ------------------------------------------- | ----------------- | --------------------------------- | ----------------------------- |
| **light**              | 0 (generator self-review, lenient)          | —                 | render smoke / none               | focused only                  |
| **standard** (default) | **2** — one decisive fix-and-reverify cycle | session default   | impacted-surface journey + canary | focused; full once at land    |
| **deep**               | up to 5 + full recovery ladder              | highest available | full journey + prior-layer replay | full pre-eval **and** at land |

**The derivation function** (deterministic; ties resolve upward):

- **`deep`** iff any: the diff (or, provisionally, the plan) matches a **`sensitive_paths`** glob — a human-owned list in the generated e2e skill's `policy.md` covering auth, payments, irreversible data operations, migrations, deploy/CI config; **or** the layer is the walking skeleton (0/0.5); **or** a human override says so. This is the hard floor; it is never loosenable by calibration.
- **`light`** iff all: the change is **docs-only** by path (tests are _not_ docs — see the referee-asset rule), no contract obligations, no new non-doc files — confirmed by the binding diff, not just the plan.
- **`standard`** otherwise (the default; multi-module or `on_critical_path` stories are at least `standard`).

**Referee assets never derive `light`.** Test directories, journey specs (`skills/e2e-test/journeys/**`, including their `paths:` front-matter), the e2e skill's `policy.md`, and CI workflow definitions are the evidence base every later verifier stands on — a diff confined to them is a change to the _referee_, not a cheap change. **A diff touching referee assets floors at `standard`**, and a **negative assertion delta** (more test/`Verify` lines removed than added) always requires an evaluation round, whatever the tier. Journey `paths:` edits after first authoring trigger the same scope refusal as CI-workflow edits (§9).

**Cap exhaustion is defined, not silent:** when `standard` exhausts its 2 rounds on a genuine `product-defect`, the story **auto-escalates once to `deep`** (recorded as `tier_escalated: true`) and continues the recovery ladder from where it left off; only after `deep`'s ladder exhausts does it reach the human gate. Ladder rungs re-key to _position past the tier's cap_, not absolute round numbers (recovery-ladder.md).

**The layer gate's depth is owned by `/aep-wrap`, independent of any story's tier.** A `light` integration story does not weaken the gate: full replay, coverage matrices, and evidence classes run at the gate whatever the tiers of the stories inside the layer.

**Threshold semantics (all tiers):** PASS means **zero blocking findings** against the hard-failure thresholds (scoring-framework.md). A perfect aggregate score is **never** a gate condition — perfect-score gates train evaluator gaming and block convergence.

**Fail-open default:** a story with no derived tier behaves as `deep` (today's full-mode behavior). A design-time Light-mode run maps to `light`. Standalone projects (no `product-context.yaml`) never get derivation and run `deep` unless the human sets a tier by hand.

---

## The Two-Point Derivation

The tier is derived **twice**, because the honest inputs change between planning and merge:

1. **Provisional (dispatch time):** computed from the story's plan fields — declared `files_affected`, `module`, contract obligations, `complexity`, `on_critical_path`, layer kind — into the machine-assembled brief (`/aep-dispatch` → context-assembly.md). This prices the build (evaluator effort, dogfood scope) but is a _prediction_: `files_affected` is authored by the planning pipeline, not observed. Grouped changes (`compile_mode: grouped_change`) take the **max** of member tiers; Dynamic Workflow batches carry the tier in the STEP-0 brief and apply it to the workflow's verify stage.
2. **Binding (Phase 5 entry):** re-derived from the **actual diff** (`git diff --name-only "$BASE"...HEAD`) against the human-owned `sensitive_paths` list, **before the first evaluation round consumes the cap** — binding at the merge step would let the whole eval loop run on the gameable provisional tier. If the diff leaves the declared scope (`diff ∖ files_affected ≠ ∅`), the tier is recomputed from the diff and `scope_drift: true` is recorded.

**At binding, a tier may only go up, never down** — a story cannot talk its way into a cheaper class by under-declaring scope, and impacted-journey selection (build Phase 8) keys off the merged diff, never the declaration.

**Pre-merge re-check (Phase 12):** commits added after the last PASS (review fixes, human-eval fixes) re-run the derivation; a post-eval drift into `sensitive_paths` upgrades the tier and requires one fresh evaluation round at the new tier — this rides the existing stale-eval rule, it does not add a new mechanism.

---

## The Verification Recipe

The derivation outputs a **recipe, not just a depth**: tier + dimension preset + dimension hard floors, from one set of inputs. Depth and rubric must never decouple — a `sensitive_paths` story that derives `deep` while its evaluator runs the Mixed preset without the Security hard floor is a designed-in blind spot.

| Derivation input                                                                        | Dimension preset (scoring-framework.md)                                                                                                                                                                                                                                                                                                                                     |
| --------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `sensitive_paths` match                                                                 | **Security-sensitive** — its `Security ≥ 4` / `Data Privacy ≥ 4` floors join the tier hard floor (survive any customization); adds the **deterministic security gates** (secret scan, SAST) as inner-loop typed gates — the mechanically detectable half of security is never routed to a more expensive LLM judge, and their findings are `product-defect` by construction |
| `ui`-kind module / `calibration_type` in {visual-design, ux-flow} / `object_model_refs` | **UI-heavy** — only these stories pay for **Visual Design**, the most expensive dimension (screenshot capture + multimodal evaluation); dimension cost follows the shipped surface, never habit                                                                                                                                                                             |
| data/migration paths                                                                    | **Data pipeline** (Data Integrity floor)                                                                                                                                                                                                                                                                                                                                    |
| story-map / product-context artifacts (walking-skeleton design)                         | **Product & Design dimensions**                                                                                                                                                                                                                                                                                                                                             |
| otherwise                                                                               | **Mixed** with default thresholds                                                                                                                                                                                                                                                                                                                                           |

Per tier: `light` scores no dimensions (self-review); `standard` runs the derived preset; `deep` runs it with no de-weighted dimensions — and, where more than one model family is available, **`deep`-tier evaluation prefers a different model family from the generator** (cross-family judging reduces correlated generator/judge blind spots).

**Customization keeps the ratchet direction:** launch or the human may **add** dimensions or raise thresholds, never drop a derived preset's hard-floor dimensions — the same tamper-resistance rule as the tier binding.

### `verification-recipe.json` — the typed artifact

The binding derivation emits `.dev-workflow/verification-recipe.json`, and **build Phase 5 refuses to start without it** — a derivation function executed from prose recall is exactly the class of mechanical step that eventually gets skipped:

```json
{
  "story_id": "<id>",
  "derived_at": "<ISO 8601>",
  "base_sha": "<sha>",
  "tier": "light | standard | deep",
  "tier_provisional": "light | standard | deep",
  "tier_escalated": false,
  "scope_drift": false,
  "referee_assets_touched": false,
  "negative_assertion_delta": false,
  "dimension_preset": "ui-heavy | api-only | security-sensitive | data-pipeline | mixed | product-design",
  "hard_floors": { "Security": 4, "Data Privacy": 4 },
  "max_rounds": 2,
  "evaluator_effort": "default | highest",
  "inputs": {
    "sensitive_paths_matched": [],
    "files_in_diff": 0,
    "declared_files_affected": 0
  }
}
```

Downstream projects get a runnable reference implementation (derivation function + preflight probe stubs + recipe emission) shipped with `/aep-e2e-skill-scaffolding`; AEP itself ships no runtime.

### How the recipe travels the trigger path

Gen-eval is not self-triggering — it is configured down the chain **autopilot → dispatch → launch → build**, and the tier must ride that exact chain or it governs nothing:

- **Dispatch** computes the provisional tier into the machine-assembled brief.
- **Launch is the tier's first real consumer**: evaluator _existence_ is the criteria file it writes (`.dev-workflow/evaluator-criteria.md`), and that decision is recipe-derived and mechanical — `light` → no criteria file (build self-reviews); `standard` → criteria from the derived preset; `deep` → tailored criteria + the highest-available evaluator effort (the effort hint travels to `executor.spawn_evaluator()`). This also gives autonomous launches a deterministic criteria policy.
- **The workspace publishes the tier**: `status.json` carries `verification_tier`, `tier_escalated`, and the latest FAIL's `failure_class` (signals-spec.md is the schema owner) — that signal file is the **only** thing autopilot may read.
- **Autopilot's monitoring is tier-aware**: nudges match the workspace's published tier (a `light` workspace is nudged to self-review, not to spawn an evaluator), and `eval_not_converging` fires only after the published tier's cap **plus** the automatic `standard → deep` escalation have both been spent.

---

## Verification Accounting

`execution-record.yaml` (`/aep-wrap` → convergence.md) carries a `verification:` block. Fields split into a **mandatory file-derivable floor** — every field marked `MUST` is computable from artifacts the workflow already writes, and an implementation that leaves them null has not implemented this design — and best-effort fields that stay nullable. A calibration loop built on optional sensors starves.

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

Suite-level economics (`suite_runs`, `suite_seconds`) live in the **layer budget box** (the layer-gate evidence doc), where `/aep-wrap` actually runs the suites — not in the per-story record. **Per-round eval persistence** (keeping `eval-response-<N>.md` through wrap's gather) is an explicit requirement: `findings_by_round` is the loosening signal the calibration loop depends on.

### Closing the loop

- **Escape-rate ingestion:** when `/aep-reflect` classifies a post-merge bug, it traces the bug to the story that introduced it and appends to that story's `escaped_defects`. Escape rate is **per story per tier** (escapes ÷ stories at that tier — well-defined even for `light`'s zero rounds). Ambiguous multi-story escapes attribute to the **layer**, never to no one.
- **Bidirectional calibration, dampened.** Layer distillation (proposal-only, convergence.md §2) may propose tier-derivation adjustments in both directions — but loosening feels good immediately while escapes surface a layer later, the textbook oscillation setup. Dampening rules: **loosening proposals require ≥2 layers of `findings_by_round` evidence and zero unresolved escape attributions; loosen at most one notch per layer; the `sensitive_paths` hard floor is never loosenable.** Tightening has no damper. **Calibration proposals must condition on model version** (`generator_model` / `evaluator_model`): a model swap shifts escape rate and findings-per-round simultaneously, and a loop that cannot see the swap misattributes the shift to tier settings. Verification assets get the same lifecycle: distillation may propose merging or retiring journeys that have replayed green for N layers with no coverage loss. Proposals only; a human applies them.
- **The layer budget box:** each layer gate's evidence doc records expected vs. actual verification spend (rounds, suite runs/time, scenarios, `cost_usd` where known). **Expected values are human-owned** — set at layer planning or accepted from a distillation proposal, never authored by the loop graded against them. **Cold start:** the first instrumented layer runs with no box and only records; its observed actuals plus a human-chosen margin become the next layer's expected values. On overrun, `/aep-wrap` surfaces a **scope-vs-verification tradeoff to the human** at the layer-advance gate — the loop asks, it does not silently grind.

---

## Tamper-Evident Evidence Classes

A layer gate may flip to `passed` only when its evidence includes **at least one class the generator cannot modify**:

1. a CI run bound to the merged SHA — **valid as tamper-evident only when the workflow definitions are outside the story's diff scope** (a diff touching `.github/workflows/**` triggers a scope refusal or human review; a generator that edits its own referee is not evidence — the same refusal covers the rest of the referee-asset class: journey `paths:` re-scoping after first authoring, and `policy.md` edits outside a scaffold run);
2. journey execution performed by `/aep-wrap` (executes-never-authors — existing canon);
3. **read-only golden fixtures with a ledger-equality oracle** — fixture trees the generator's workspace cannot write, verified by before/after equality of durable state (SHA-256 ledger comparison; screenshots stay diagnostic, never a pass condition);
4. production telemetry ingested by `/aep-watch`.

Additionally:

- **Evaluator prompts are machine-assembled, and spawn authority leaves the player.** The orchestrating layer — not the generator — assembles the evaluator's context (criteria file, contract file, diff range), so the generator cannot curate what its judge sees. Where an orchestrating layer exists (autopilot; the main session in interactive runs), **it owns the evaluator spawn**; in genuinely standalone builds the spawn recipe stays generator-invoked, but in every mode the machine-assembled evaluator prompt marks `eval-request.md` as the **generator's untrusted claim** — data to verify, never framing to adopt.
- **`policy.md` owns the `live_policy` decision** for cost-bearing dogfoods (live model calls, quota- or fee-metered targets): `every_gate | milestone_gates_only | none`, with the milestone list named in the policy. Non-milestone gates use zero-cost render smokes / scripted tiers, and their preflight does not probe the optional live half, whose absence stays SKIP.

---

## Worked Examples

**An environment incident, replayed.** A mandatory post-deploy dogfood fails on a missing CI secret and a wrong cloud account. Under this reference: the deploy-independent preflight refuses pre-merge on the first story (`REFUSING [dogfood-secret-absent:<NAME>]` — the secret _name_ was checkable without any deploy); whatever slips through, the gate-time target-bound preflight refuses (`REFUSING [auth-identity-mismatch:<got≠want>]`) before any scenario spend. Classification is `environment` by construction; the ownership check pairs it with `product-defect` findings for any unwired deploy config — filed as ordinary stories, not spun into recovery loops. A tracked literal-credential fallback never reaches that path: the inner-loop secret scan catches it pre-merge as a typed gate. Zero journey scenarios wasted, zero evaluation rounds, no misclassified recovery stories. The loop still pauses on the human ops checklist — environment repair is genuinely human work — but immediately, cheaply, and correctly labeled, with the gate re-probing on each tick so repair auto-resumes.

**A display-positioning story, replayed.** Provisional: multi-module `files_affected`, contract obligations, not a walking-skeleton layer, no `sensitive_paths` match → `standard`. Binding: the merged diff stays inside declared scope → `standard` holds. Round 1 catches two real findings; the fixes land; round 2 passes with zero blocking findings — exactly `standard`'s one fix-and-reverify cycle. What the tier removes: a second full-suite run (the land verify already runs the suite on merged main) and the top-shelf evaluator profile for a cosmetic story. The journey and real-target dogfood stay: they are the tamper-evident half of the gate.

**A tests-only diff.** A quarantine story rewrites a flaky spec file. Path-wise it is "docs-like" — but it touches referee assets, so it floors at `standard`; and it removes more `Verify` lines than it adds, so a negative assertion delta forces one evaluation round regardless. The cheapest self-reviewed tier is unreachable for changes to the referee by construction.

---

## Anti-Patterns

- **Perfect-score gates.** "Exactly 5.00/5.00 with zero findings" trains the generator to satisfy the evaluator, not the product, and turns ordinary convergence into round exhaustion. PASS is zero blocking findings.
- **Self-classified failures.** A generator that labels its own FAIL `harness-flake` or `environment` is player, referee, and witness at once; spend-reducing classes require non-generator evidence, and the default is `product-defect`.
- **Environment failures fed to code-repair machinery.** A missing secret is not a 2.79/5.00 code problem; no evaluation round may be spent before the taxonomy step runs.
- **Per-story full-regression replay.** O(layers × stories) journey executions; full replay belongs to the layer gate and its checkpoints.
- **Uninstrumented verification.** A loop that records `cost_usd: 0` forever cannot know it is over-verifying; unpriced spend grows until it displaces the product.
- **The verification ratchet.** A lessons loop that only ever adds verification converges on a process that verifies instead of shipping; calibration must be able to loosen and retire — with dampers.
- **Scaling verification with judge rounds.** The only scaling direction that gets more expensive and more gameable at once; add typed gates and real-environment evidence instead.
- **Self-curated evidence.** A generator that assembles its own evaluator's context, writes the fixtures it is graded against, or edits the CI workflows that judge it, is not being verified.

---

## Cross-References

| Where                                                          | What it consumes from this reference                                                                                      |
| -------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------- |
| `/aep-build` Phase 5 / 6B / 8 / 12                             | Binding derivation + recipe refusal; taxonomy check at FAIL; preflight; impacted replay + canary; pre-merge tier re-check |
| `/aep-dispatch` context-assembly                               | Provisional tier derivation into the machine-assembled brief                                                              |
| `/aep-launch` SKILL.md + evaluator.md + signals-spec.md        | Recipe-derived evaluator existence/criteria/effort; `verification_tier` / `tier_escalated` / `failure_class` signals      |
| `/aep-wrap` convergence.md + layer-advance.md                  | The `verification:` gather block; full replay + checkpoints + budget box + evidence-class check                           |
| `/aep-autopilot` tick-protocol / state-schema / guard          | REFUSED ≠ FAIL; `environment_repair` escalation; tier-aware nudges                                                        |
| `/aep-executor` dogfood-validation.md + backends.md            | `Failure-Class:` report line; evaluator-effort hint on `spawn_evaluator()`                                                |
| `/aep-e2e-skill-scaffolding` policy + templates + references   | `sensitive_paths`, `live_policy`, probe sets, security gates, budget box, reference derivation script                     |
| `eval-protocol.md` / recovery-ladder.md / scoring-framework.md | Evaluator-authored `failure_class`; tier-keyed rungs; recipe-derived presets + zero-blocking semantics                    |
