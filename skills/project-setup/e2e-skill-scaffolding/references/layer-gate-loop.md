# The layer-gate loop — quality, layer by layer

AEP builds in **layers** (Layer 0 = walking skeleton, Layer N = enrichment). Each layer has a **layer
gate**: an integration checkpoint proving the layer's user journey actually works end-to-end, not just
that the individual stories merged. The journey library is the **manual half** of every gate.

## The loop

```
new capability / new layer
        │
        ▼
  add a journey      (copy journeys/README.md template; set front-matter `layer: N`, `target: …`)
        │
        ▼
  run the journey    (/aep-build Phase 6; tool resolved by tool-selection.md; verify state, not pixels)
        │
        ▼
  record evidence    →  docs/layer-gates/<layer>.md   (agent/thread ids, API JSON, screenshots, PASS/FAIL per Then)
        │
        ▼
  flip the gate      →  layer_gates[N].status = passed   in product-context.yaml
        │
        ▼
  /aep-dispatch      →  blocks the next layer until layer_gates[N-1].status == passed
```

So quality is **cumulative**: a later layer can't be dispatched until the prior layer's gate is green,
and the gate is only green once its journey passes with verified evidence. Each layer adds a journey;
the library grows into a full regression suite that the dogfood phase replays.

## `product-context.yaml` shape

```yaml
layer_gates:
  0:
    status: not_started # not_started | running | passed | failed
    journey: skills/e2e-test/journeys/00-walking-skeleton.md
    evidence: docs/layer-gates/0.md
    completed_at: null # ISO8601 once passed
```

## Where each AEP skill touches the gate

| Skill           | Touch point                                                                               |
| --------------- | ----------------------------------------------------------------------------------------- |
| `/aep-dispatch` | **Precheck** — refuses to start layer N stories while `layer_gates[N-1].status != passed` |
| `/aep-build`    | Phase 6 runs the layer's journey (the dogfood); Phase 7 codifies new scenarios            |
| `/aep-wrap`     | After merge, suggests running the gate; on pass, flips `layer_gates[N].status = passed`   |
| `/aep-reflect`  | Classifies journey findings — bug → high-priority story; "feels wrong" → calibration      |

## Standalone projects (no product-context.yaml)

The loop still works without the YAML state machine: journeys carry `layer: N` front-matter, evidence
still lands in `docs/layer-gates/<layer>.md`, and "flip the gate" becomes a manual checkbox in that doc.
The discipline — one journey per layer, verified evidence before advancing — is the same.

## Bugs found during a gate

A journey that finds a defect routes through `/aep-reflect`: a functional bug becomes a high-priority
story; a "works but feels wrong" observation becomes a calibration item. The unified dogfood report path
(`/aep-build` Phase 6 → `dogfood_report` adapter → classifier) auto-files when configured; otherwise the
finding is surfaced to the human.
