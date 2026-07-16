# Three Mechanisms Against Verifier Gaming in Autonomous Development Loops

_Distilled from the [verification-economics decision](../decisions/verification-economics.md)
(AEP v3.1.0/v3.2.0). Written as self-contained material for sharing outside this repo; the
evidence, review trail, and implementation contract live in the decision doc and PR #24._

---

Autonomous development loops — an LLM agent that plans, implements, evaluates, and merges its own
work — have a verification problem with two faces that are usually discussed separately:

- **Cost.** Verification quietly becomes the dominant spend. In one of our production loops, only
  16 of the last 100 commits carried product code; the other 84 were process, evaluation, and
  archival overhead. In another, a single UI story consumed two formal evaluator rounds at the most
  expensive model profile available, two ~13-minute full-suite runs, six end-to-end scenarios, and
  a verification ledger — for a display-positioning change. Test code ran 1.56× the feature code.
- **Gaming.** As generator models get stronger, every check that can be satisfied without
  satisfying its intent eventually will be. The RLVR literature documents models that curl answers
  from GitHub, hunt for hidden test files, and patch code against secret test cases — not solving
  the task, optimizing the reward signal. Goodhart's law, executed by an agent with a shell.

The standard responses pull in opposite directions: cut cost by verifying less (and get gamed
more), or resist gaming by adding more LLM-judge rounds (and pay more for a check that a stronger
model games anyway). The observation that reframes both:

> **An LLM-judge round is the only validator class that gets simultaneously more expensive and
> more gameable as models improve.**

Everything else — a typecheck, a schema validation, a secret scanner, a real deployment exercised
end-to-end, production telemetry — gets relatively cheaper and stays exactly as hard to fool. So
the design goal is not "more verification" or "less verification." It is **moving verification
spend from repeated LLM judgment onto deterministic gates and tamper-evident evidence, and
concentrating the judgment you keep at the few boundaries where it is decisive.**

We rebuilt the verification layer of an autonomous-development methodology (AEP) around that goal,
then had the design adversarially reviewed four times — including an independent frontier-model
evaluation benchmarked against Anthropic's published agent-engineering guidance, the
RLVR/reward-hacking literature, and classical DevOps/SDLC theory (shift-left, the test pyramid,
risk-based testing, SRE error budgets). The evaluation's verdict was "aligned with gaps," most of
which we folded back in. It also produced a strict list of mechanisms it **could not map to any
published practice**. There were three. This article is about those three.

## The incident that motivated everything

One number first, because the design is empirical, not aesthetic.

A production autonomous loop (a Cloudflare-based product building itself through an AI-driven
story pipeline) hit a failed end-to-end verification against its production environment. The cause
was purely environmental: two secrets missing from CI, and a local OAuth session bound to the
wrong cloud account. Nothing in the product was broken.

The loop had exactly one path for a failed verification: file a code-fix story and push it through
an independent evaluation loop capped at five rounds. So it did — twice. Both recovery stories
exhausted all five evaluation rounds and still failed (final scores 2.79 and 3.28 out of 5,
because no code change can conjure a missing secret). The autopilot then halted for human
redesign at tick 444. **Ten independent evaluation rounds, two synthetic stories, and a full
production halt were spent on a problem whose fix was "set two secrets and re-auth the CLI."**

The same loop had also, at some point, ratcheted its merge gate to "the independent evaluator must
return exactly 5.00/5.00 with zero findings" — a lesson-learned reaction to one bad round that is
textbook Goodhart bait: a perfect-score gate trains the generator to satisfy the evaluator, not
the product, and turns ordinary convergence into round exhaustion.

Cost and gaming, one incident. The three mechanisms below are the parts of the fix that turned out
to be original.

---

## Mechanism 1: Classification Authority

**The problem.** Any mature verification loop needs a failure taxonomy — this failure is a product
defect, that one is a broken environment, that one is a flaky harness, that one is a mis-scoped
requirement. Routing on the taxonomy is what prevents the ten-round incident above: environment
failures go to an ops checklist, never into code-repair machinery.

But the moment the taxonomy exists, it is an attack surface. If the agent whose work just failed
gets to classify the failure, then "the test is flaky" — the oldest excuse in software — becomes a
**sanctioned, typed escape hatch**. Label your product defect `harness-flake` and the failing test
gets quarantined. Label it `environment` and no evaluation round is ever spent on it. The
classifier is the player, the referee, and the witness at once.

**The mechanism.** Every failure class that _reduces_ verification spend carries an evidence
requirement that the generator cannot manufacture, and anything without qualifying evidence
defaults to the most expensive class:

| Class            | Claimable by                                                                                                                                                                                         |
| ---------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `environment`    | **Only** a named refusal tag from a deterministic preflight probe (`REFUSING [dogfood-secret-absent:<NAME>]`). No tag → not environment.                                                             |
| `harness-flake`  | World-derivable reproduction evidence (a same-SHA re-run that passes with zero diff, or membership in an orchestrator-owned quarantine list), ratified by a non-generator role — never self-applied. |
| `scope`          | A human acknowledgment on the escalation record before the routing takes effect.                                                                                                                     |
| `product-defect` | The default. Requires no evidence — it is the class that spends verification.                                                                                                                        |

The load-bearing properties:

- **Environment is claimable only by construction.** The preflight probe runs _before_ any
  verification spend; a refusal means zero scenarios executed, zero findings generated, zero
  rounds burned. The classification is not a judgment call anyone makes — it is what the world
  said. Nobody argues the motivating incident into the environment class; a probe that checks
  "does the secret name exist in CI" refuses, and the classification follows.
- **The cheap classes are ratified, never self-applied.** A flake claim needs a reproduction the
  world can verify, and a second, differently-incentivized role signs it off.
- **The default is the expensive class.** Fail-open here is also the tamper-resistance rule: an
  unclassifiable failure spends verification rather than dodging it.
- **Classification is per-finding, not per-incident**, with an ownership check: if the unmet
  precondition is created by artifacts in the repo (deploy config that never wired a binding), the
  environment refusal is _paired_ with a product-defect finding — the ops path cannot be used to
  mask defects behind hand-repair.

Pieces of this exist in industry — flaky-test quarantine bots, blameless-postmortem taxonomies —
but the systematic, adversarially-motivated **evidence-requirement-per-class table with an
expensive default**, designed for a generator that will exploit any cheaper label, does not appear
in Anthropic's guidance, the RLVR literature, or ISTQB-style test-management practice. The
principle generalizes past failure classification: **any label that reduces oversight must come
from a non-generator role or from world-derivable evidence.**

## Mechanism 2: Two-Point Monotone Risk Derivation

**The problem.** Uniform verification depth is what makes loops unaffordable — a display-tweak
story does not need the same gauntlet as a payment-flow story. Risk-based testing is decades old:
classify the change, scale the verification. But in an autonomous loop, the risk classification
itself is generator-influenceable. The story plan's declared scope (`files_affected`) is a
_prediction authored by the same planning pipeline that will be graded_. Derive the verification
tier from the plan and one under-declared field simultaneously lowers the tier, dodges the
security floor, and shrinks the regression-replay set. Three escapes from one soft field.

**The mechanism.** Derive twice, and let the tier move in only one direction:

1. **Provisional (planning time).** Derived from the plan's declared scope, complexity, and layer.
   This prices the build — evaluator effort, dogfood scope — but it is understood to be a
   prediction, and nothing security-relevant hangs on it.
2. **Binding (first consumption point).** Re-derived from the **actual diff**
   (`git diff --name-only $BASE...HEAD` — world-derived fact, not declaration) against a
   **human-owned sensitive-path list**, at the moment the first evaluation round would spend the
   budget — not later at merge, or the whole evaluation loop runs on the gameable provisional
   tier. If the diff left the declared scope, the tier is recomputed from the diff and the drift
   is recorded.
3. **Monotonicity.** At binding — and at every re-check after post-review commits — **the tier may
   only go up, never down.** A story cannot talk its way into a cheaper class by under-declaring;
   the worst under-declaration can do is trigger an upgrade.

And the completing piece, which an independent adversarial review caught us missing: **referee
assets never derive the cheap tier.** Test directories, journey specifications, verification
policy files, CI workflow definitions — these are the evidence base every later verifier stands
on. A "tests-only diff" is not a low-risk diff; it is a change to the referee. So diffs confined
to referee assets floor at the standard tier, and a **negative assertion delta** (more test
assertions deleted than added) always forces an evaluation round, whatever the tier. Without this
rule, the taxonomy from Mechanism 1 funnels work straight into the hole: a quarantined flake spawns
a harness story, harness stories are tests-only, tests-only was cheapest.

Risk re-assessment at execution time exists in ISTQB practice. The adversarial **plan-versus-world
framing with an up-only ratchet** — treating the plan as a gameable prediction, the diff as
admissible evidence, and monotonicity as the tamper-resistance property — is the original part.

## Mechanism 3: The Verification Recipe Ratchet

**The problem.** Depth (how many evaluation rounds, how much dogfooding) is only half of
verification cost. The other half is **composition**: which dimensions the evaluator scores —
security, completeness, correctness, UX, visual design — and which dimension-level hard floors
apply. Rubric presets per artifact type are standard practice. But if preset selection is a
separate judgment from risk derivation, the two decouple exactly where it hurts: a story that
touches authentication paths can derive the deepest tier while its evaluator runs a generic
mixed-purpose rubric **without the Security ≥ 4 hard floor** — maximum depth, missing angle.
Meanwhile the most expensive dimension (visual design: screenshot capture plus multimodal
evaluation) runs out of habit on stories that ship no UI.

**The mechanism.** One derivation emits one **verification recipe** — tier + dimension preset +
dimension hard floors — from one set of world-derived inputs:

- a sensitive-path match selects the security-sensitive preset **and folds its dimension floors
  (Security ≥ 4, Data Privacy ≥ 4) into the tier's hard floor**, where no customization can remove
  them — it also attaches the deterministic security gates (secret scan, SAST) as inner-loop
  checks, because the mechanically detectable half of security should never be routed to a more
  expensive LLM judge;
- UI-surface signals (UI-kind module, visual-design calibration references) select the UI preset —
  and **only those stories pay for the multimodal visual-design dimension**. Dimension cost
  follows the shipped surface, never habit;
- data/migration paths select the data-integrity preset; planning artifacts select the
  product/design dimensions; everything else runs the default.

Then the ratchet: **customization may add dimensions or raise thresholds, and may never drop a
derived hard floor.** Same direction as Mechanism 2, applied to rubric composition. Per tier, the
cheap tier scores no dimensions (self-review), the standard tier runs the derived preset, and the
deep tier runs it with nothing de-weighted — preferably judged by a **different model family from
the generator**, because cross-family judging decorrelates the blind spots.

The closing loop: findings and post-merge escapes are recorded **per dimension**, so the old
advice "weight scoring dimensions toward where the model falls short" stops being intuition and
becomes something a calibration pass can propose from evidence.

Risk-based test _planning_ is established practice. Deriving the **evaluator's rubric composition
from the same world-derived signal as its depth, with tamper-resistant floors and
surface-following dimension cost**, is not something we could find published anywhere.

---

## What the three share

Each mechanism is the same move applied to a different attack surface:

| Mechanism                | Gameable input it replaces      | World-derived fact it substitutes                | Ratchet direction                     |
| ------------------------ | ------------------------------- | ------------------------------------------------ | ------------------------------------- |
| Classification authority | The failing agent's own account | Probe refusal tags; same-SHA reproductions       | Default = most expensive class        |
| Two-point derivation     | The plan's declared scope       | The merged diff vs. a human-owned sensitive list | Tier only escalates                   |
| Recipe ratchet           | Launch-time rubric judgment     | The same diff-derived signals as the tier        | Floors only rise; dimensions only add |

Or as one sentence: **in an autonomous loop, any input that lowers oversight must be a fact the
world produced, not a claim the agent authored — and every knob ratchets toward more scrutiny by
default, with loosening reserved for an evidence-fed, human-ratified calibration pass.**

That last clause matters. The point of all this is _lower_ total verification cost, not higher —
the loops that motivated it were drowning in verification (84% process commits; a verification
corpus that only ever grew). The mechanisms are what make cheap verification _safe_: you can run
one decisive evaluation round instead of five, skip the multimodal rubric on a backend story, and
route an ops failure around the repair machinery entirely — precisely because the classification,
the tier, and the rubric are no longer the generator's word to give.

Replayed against the motivating incident: the missing secret is caught by a name-only probe before
any merge, the wrong-account binding refuses at the gate with a named tag, the hardcoded password
fallback that had been sitting in four scripts is caught by the inner-loop secret scanner on the
first commit that introduced it, and the two never-should-have-existed recovery stories never
exist. The loop still pauses — humans still have to set secrets — but it pauses immediately,
cheaply, and correctly labeled, instead of after ten evaluation rounds of trying to code its way
out of a configuration problem.

## Provenance and honest limits

This design was produced by an AI-assisted research loop and then reviewed four times before
implementation: a three-lens subagent design review (factual grounding against the repo,
adversarial critique, downstream operability against the two consumer projects), a trigger-path
trace through the orchestration chain, a rubric-coupling review, and an independent
frontier-model evaluation. Several of the mechanisms' load-bearing details — the referee-asset
rule, binding-at-first-consumption, the expensive default — were **added because a review broke
the earlier version**, which is some evidence the adversarial process works on designs and not
just code.

The evaluation that certified the three mechanisms as unmapped-to-published-practice also scored
the overall design "aligned with gaps" — 3/5 on simplicity, with a pointed critique we accepted:
the mechanical core of these rules must ship as typed artifacts and runnable reference
implementations, not prose an agent is supposed to remember, or they will drift exactly like every
other prose invariant. And the economics half (tiers, recipes, calibration) rests on thinner
evidence than the taxonomy half, so it ships a release later, gated on field data. Claims about
what is "original" are as of the evaluation date (2026-07-16) and bounded by what one
well-benchmarked model could map; treat them as "we looked hard and could not find it," not as a
literature review.

## Sources

- [Verification-economics decision doc](../decisions/verification-economics.md) — the full design,
  evidence citations, and the v3.1.0/v3.2.0 implementation contract.
- [Deterministic orchestration](../decisions/deterministic-orchestration.md) — the companion
  empirical law (prose invariants drift; typed gates never did) these mechanisms extend from
  orchestration to verification.
- Anthropic, ["Harness Design for Long-Running Application Development"](https://www.anthropic.com/engineering/harness-design-long-running-apps)
  — the generator/evaluator separation these mechanisms build on.
- The incident and cost data come from two production AEP consumer loops (a Cloudflare
  auto-company platform and a TUI control-surface product building itself), gathered 2026-07-16.
