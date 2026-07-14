# Alignment Layers (`.5` Layers)

Canonical definition of the `.5`-layer concept for the whole AEP corpus. `/aep-map` plans these layers during dependency resolution; `/aep-calibrate`, `/aep-reflect`, `/aep-envision`, `/aep-dispatch`, and `/aep-launch` all reference this concept — they point here rather than re-defining it.

## What a `.5` layer is

A `.5` alignment layer (0.5, 1.5, 2.5, …) is a point between integer implementation layers where the team **pauses agent execution to calibrate human intent** across one or more quality dimensions. It is a **human alignment layer, not just "UI polish"** — any dimension where agent output can diverge from human intent qualifies. Stories in a `.5` layer are tagged `calibration_type: <dimension>`.

## Planning `.5` layers during Map

After defining each implementation layer, review `calibration.plan` from `product-context.yaml` (operational file, both modes; populated by `/aep-envision`) or consider which quality dimensions may need human calibration:

- **UI-facing stories** → consider visual-design and/or copy-tone calibration
- **New API endpoints** → consider api-surface calibration
- **New domain entities** → consider data-model calibration
- **First user-testable layer** → consider scope-direction calibration

**Heavy dimensions** (visual-design, ux-flow, copy-tone): plan a `.5` alignment layer with stories tagged `calibration_type: <dimension>`. Run `/aep-calibrate <dimension>` before dispatching to generate a brief and capture decisions into `calibration/<type>.yaml`.

**Light dimensions** (api-surface, data-model, scope-direction, performance-quality): plan a `/aep-calibrate <dimension>` checkpoint BEFORE dispatching the relevant stories in the next integer layer. No `.5` layer needed — decisions update `product-context.yaml` directly.

## Layer progression

- **Layer 0.5** (first `.5` layer): typically establishes the visual design system. Run `/aep-calibrate visual-design` to create `calibration/visual-design.yaml`.
- **Layer 1.5, 2.5** (subsequent `.5` layers): extend calibration to new patterns. `/aep-calibrate` detects existing calibration artifacts and generates focused briefs covering only the delta.

## Opt-in, not automatic

The `/aep-reflect` step after each layer classifies calibration needs by dimension. The human decides which dimensions need attention — but the workflow makes the question unavoidable.

## Object Map feeds the heavy UI dimensions

Once `/aep-model` has approved an Object Map for a capability, the `visual-design` and `ux-flow` `.5`-layer briefs derive their "pages/screens to design" from the Object Map's screen plan (`product/maps/<cap>/object-map.yaml` → `screens`) instead of an ad-hoc `routes/` scan. Structure first (object-model), then taste (visual-design) and journey (ux-flow).
