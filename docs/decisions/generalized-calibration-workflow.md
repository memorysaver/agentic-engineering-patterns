# Generalized Calibration Workflow

> Extends the design calibration workflow (2026-04-03) to all quality dimensions.
> Supersedes the visual-design-only scope of `design-calibration-workflow.md`.
> Introduces multi-dimension `/calibrate`, quality dimensions in `/envision`, and calibration routing in `/reflect`.

---

## Problem

The original `/calibrate` skill (from `design-calibration-workflow.md`) solved one instance of a general problem: agents build to spec, but specs are lossy compressions of human intent. The gap between "works correctly" and "is what we actually want" was addressed only for visual design.

But the same gap exists across many dimensions:

| Dimension           | The gap                                                 |
| ------------------- | ------------------------------------------------------- |
| Visual design       | Looks generic, no brand identity                        |
| UX flow             | Screens exist but journey feels wrong                   |
| API surface         | Endpoints work but naming/grouping doesn't match domain |
| Data model          | Schema is functional but doesn't match team's language  |
| Scope/direction     | What was built isn't what the PM meant                  |
| Copy/tone           | Text is functional but reads wrong                      |
| Performance/quality | Works but tolerances haven't been decided               |

All share the same meta-pattern:

```
agent builds fast → human reviews → gap between "works" and "right"
→ human realigns → agent continues with corrected context
```

Without generalization, each new dimension would either be shoehorned into the visual-design workflow or require its own ad-hoc skill.

---

## Solution

### 1. Generalized `/calibrate`

The `/calibrate` skill now supports 7 calibration dimensions, each following the same two-phase pattern:

```
Phase 1: Generate dimension-specific brief → human explores/decides
Phase 2: Capture decisions into machine-readable artifact → feed to agents
```

Calibration types split into two natural classes:

| Class | Types                                                         | Exploration                        | Time       | Artifact                                 |
| ----- | ------------------------------------------------------------- | ---------------------------------- | ---------- | ---------------------------------------- |
| Heavy | visual-design, ux-flow, copy-tone                             | External tools or deep exploration | Hours–days | Standalone YAML in `calibration/`        |
| Light | api-surface, data-model, scope-direction, performance-quality | Conversation + code review         | 30–60 min  | Inline updates to `product-context.yaml` |

Heavy calibrations create `.5` alignment layer stories. Light calibrations update product context directly.

### 2. `/envision` Extension

`/envision` Phase 1 now includes a "Quality dimensions" inquiry — the team declares which dimensions will need human calibration during development. This produces:

- `product.quality_dimensions` — dimension, criticality, first calibration layer, rationale
- `calibration.plan` — maps layers to expected calibration checkpoints

`.5` layers are reframed as "human alignment layers" — not just "UI polish."

### 3. `/reflect` Integration

The "Polish" sub-type under Refinement becomes "Calibration" — a broader category that identifies gaps across all 7 dimensions. Classification questions guide the user to identify which dimension feels off. Routing splits by class:

- Heavy dimensions → `.5` alignment layer stories → `/calibrate <dimension>`
- Light dimensions → `/calibrate <dimension>` directly (inline)

### 4. Artifact Design

- **Heavy calibrations** produce standalone files in `calibration/` directory:
  - `calibration/visual-design.yaml` (renamed from `design-context.yaml`)
  - `calibration/ux-flow.yaml`
  - `calibration/copy-tone.yaml`
- **Light calibrations** update existing `product-context.yaml` sections directly
- All calibrations record entries in `calibration.history`

### 5. `/dispatch` and `/map` Integration

- `/dispatch` Step 6 context assembly checks `calibration_type` on stories and includes the appropriate artifact
- `/map` Step 3 replaces "UI Polish Layers" with "Alignment Layers" covering all dimensions

---

## Key Decisions

| Decision                                                | Reasoning                                                                                                                                                                                    | Alternatives                           |
| ------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------- |
| Two classes (heavy/light) rather than uniform treatment | Visual design needs external tools and hours. API naming needs a 30-min conversation. Forcing both through the same flow wastes time or loses quality.                                       | Single flow for all types              |
| Light calibrations update product-context.yaml directly | API naming, data model, scope, and performance decisions are already structured in the YAML. Creating separate files would duplicate information and require sync.                           | Separate files for all types           |
| Quality dimensions declared in /envision                | Declaring upfront prevents surprise — the team knows which dimensions will need calibration before agents start building. Undeclared dimensions can still be calibrated ad-hoc via /reflect. | Calibration only triggered by /reflect |
| .5 layers reframed as "human alignment layers"          | The original name "UI polish layers" was too narrow. Teams were confused about whether non-visual work belonged in .5 layers.                                                                | Keep "polish" terminology              |
| calibration_type field on stories                       | Explicit typing allows /dispatch to include the right artifact. Implicit detection (by layer number alone) can't distinguish which dimension a story addresses.                              | Detect from layer number + module      |
| Backward compat with design-context.yaml                | Existing projects may have design-context.yaml at repo root. The transition is non-breaking — /dispatch checks both paths.                                                                   | Hard migration                         |

---

## Migration

1. `design-context.yaml` → `calibration/visual-design.yaml` (symlink or check both during transition)
2. `design-brief-template.md` → `references/briefs/visual-design.md`
3. `design-context-schema.yaml` → `references/schemas/visual-design-schema.yaml`
4. Stories with `.5` layers but no `calibration_type` default to `visual-design`

---

## Appendix: The Calibration Pattern

The core insight: **specs are lossy compressions of human intent, and agents optimize against spec, not intent.**

```
Human Intent
    ↓ (lossy compression)
Product Spec (product-context.yaml)
    ↓ (faithful execution)
Agent Output
    ↓ (human reviews)
Gap Identified: "correct but not right"
    ↓ (/calibrate)
Corrected Intent Captured
    ↓ (machine-readable artifact)
Agent Continues with Aligned Context
```

The calibration skill makes this correction loop explicit and structured across all quality dimensions where human judgment is irreplaceable.
