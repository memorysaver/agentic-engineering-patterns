# The layer-gate loop — quality, layer by layer

AEP builds in **layers** (Layer 0 = walking skeleton, Layer N = enrichment). Each layer has a **layer
gate**: an integration checkpoint proving the layer's user journey actually works end-to-end _and that
the layer is adequately covered_ — not just that the individual stories merged. The journey library is
the manual half of every gate; the project's scripted suite is the deterministic half.

## Two phases, because "one green journey" isn't coverage

A gate does **not** flip on a single passing journey. It advances through two phases, and "green" means
the layer is _covered_ across whichever test tiers apply to the project (see
[`three-tier-model.md`](three-tier-model.md) for which tiers gate which project type):

- **`scripted_passed`** — the Tier-1 scripted suite (the project's framework tests) for this layer is
  green. The machinery is proven; the live product is not yet.
- **`passed`** — `scripted_passed` **and** every applicable higher tier (journey dogfood, API drivers)
  is green **and coverage is complete**: each of the layer's acceptance criteria (aggregated from its
  stories' `acceptance_criteria`) maps to ≥1 proving test, **and** prior-layer journeys still replay
  green (regression).

**Coverage = acceptance/requirements coverage** — every behavior the layer promised is proven — **not** a
line/branch percentage (that stays a Tier-1 framework concern and never gates a layer).

**Which** tiers are applicable, **where** journeys dogfood (`none` / `local` / `deployed:<url>`), and
**when** (pre-merge / post-deploy) are not assumed — they come from the generated skill's
`skills/e2e-test/policy.md`, confirmed with the user at scaffold and read by `/aep-build` and `/aep-wrap`.
A `none`-target (CLI/library) project has no Tier-2 at all; its gate is Tier-1 (+ Tier-3) + coverage.

## The loop

```
new capability / new layer
        │
        ▼
  add a journey        (copy journeys/README.md template; front-matter layer: N, target: …, covers: […])
        │
        ▼
  run Tier-1 scripted  (the project's framework tests for the layer)  ──green──►  status: scripted_passed
        │
        ▼
  run the journey      (/aep-build Phase 6; tool resolved by tool-selection.md; verify state, not pixels)
        │
        ▼
  compute coverage     (map each layer acceptance criterion → a proving Verify / scripted case / API check)
        │
        ├─ uncovered? ─►  auto-author the missing scenario/case, re-run   (loop until covered, or WAIVER)
        ▼
  record evidence      →  docs/layer-gates/<layer>.md   (two matrices + checklist + PASS/FAIL + waivers)
        │
        ▼
  flip the gate        →  layer_gates[N].status = passed   (all applicable tiers green + coverage complete)
        │                 /aep-wrap then ASKS THE HUMAN before advancing to the next layer's design
        ▼
  /aep-dispatch        →  blocks the next layer until layer_gates[N-1].status == passed
```

So quality is **cumulative and covered**: a later layer can't be dispatched until the prior layer's gate
is `passed`, and a gate is only `passed` once its scripted suite _and_ its journey(s) pass with verified
evidence _and_ every acceptance criterion is proven. The agent **auto-closes** coverage gaps during the
build (it authors the missing scenario rather than asking); the human only decides _when to advance_ to
the next layer. Each layer adds journeys; the library grows into a regression suite the gate replays.

## `product-context.yaml` shape

This is the **canonical** `layer_gates` schema (the same one in
`skills/product-context/_shared/templates/product-context-schema.yaml`):

```yaml
layer_gates:
  - layer: 0
    status: not_started # not_started | running | scripted_passed | passed | failed | deferred
    test_definition: "End-to-end user journey from the Layer 0 MVP contract"
    coverage:
      criteria_total: 0 # acceptance criteria across this layer's stories
      criteria_covered: 0 # criteria with >=1 proving test
      uncovered: [] # [{ criterion: "...", plan: "scenario to add" | "WAIVER: <reason>" }]
    evidence:
      scripted: null # Tier-1 test file(s)
      journeys: [] # journey ids proving this layer, e.g. ["00-walking-skeleton"]
      matrix: docs/layer-gates/0.md # the traceability + scripted-coverage matrices
    completed_at: null # ISO8601 once passed
```

## Where each AEP skill touches the gate

| Skill           | Touch point                                                                                                                                                                    |
| --------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `/aep-dispatch` | **Precheck** — refuses to start layer N stories while `layer_gates[N-1].status != passed` (a `scripted_passed` gate still blocks — "machinery green, dogfood pending")         |
| `/aep-build`    | Phase 6 runs the journey, **computes coverage and auto-authors the missing scenarios/cases**; Phase 7 codifies them + writes the two evidence matrices                         |
| `/aep-wrap`     | **Two-phase flip:** `scripted_passed` (Tier-1 green) → `passed` (all applicable tiers green + coverage complete + regression replay); then **asks the human** before advancing |
| `/aep-reflect`  | Classifies journey findings — bug → high-priority story; "feels wrong" → calibration                                                                                           |

## Coverage evidence (the two matrices)

The evidence file `docs/layer-gates/<layer>.md` (template shipped in the generated skill) records, at
minimum, two tables so a reviewer can see the layer is _covered_, not just _touched_:

1. **Acceptance traceability** — `Criterion | Proving test (scenario/case) | Verify | PASS/FAIL`. One row
   per layer acceptance criterion; an empty "Proving test" cell is an uncovered criterion (fix it or
   `WAIVER:` it).
2. **Scripted-coverage** — `Case | Asserts | PASS/FAIL`. What the Tier-1 suite actually pins.

Plus the manual dogfood checklist, screenshots / API JSON, and a `WAIVER: <criterion> — <reason>` line
for any criterion deliberately deferred this layer.

## Standalone projects (no product-context.yaml)

The loop still works without the YAML state machine: journeys carry `layer: N` + `covers:` front-matter,
evidence still lands in `docs/layer-gates/<layer>.md` with the two matrices, and "flip the gate" becomes
a manual checkbox in that doc. The discipline — scripted half + journey half, every criterion proven,
verified evidence before advancing — is the same.

## Bugs found during a gate

A journey that finds a defect routes through `/aep-reflect`: a functional bug becomes a high-priority
story; a "works but feels wrong" observation becomes a calibration item. The unified dogfood report path
(`/aep-build` Phase 6 → `dogfood_report` adapter → classifier) auto-files when configured; otherwise the
finding is surfaced to the human.
