---
name: aep-calibrate
description: Human alignment checkpoint for a quality dimension agents can't judge. Use when /aep-reflect flags a works-vs-right gap, the user says "calibrate", "design brief", or "capture", or before dispatching a .5 alignment layer. Phase 1 writes a dimension brief; Phase 2 captures decisions into an artifact.
---

# Calibrate

Human alignment checkpoint. Agents build to spec, but specs are lossy compressions of human intent. This skill pauses execution, lets the human inspect what was built, and captures what "right" actually means in a format agents can consume.

**Where this fits:**

```
/aep-reflect (identified alignment gap)
  → /aep-calibrate           (Phase 1: generate dimension-specific brief)
  → human explores       (method varies by type — external tools, conversation, code review)
  → /aep-calibrate capture   (Phase 2: capture decisions into artifact)
  → /aep-dispatch            (stories dispatched with calibration context)
```

**Session:** Main, interactive with user
**Input:** Calibration type + product definition from `product/index.yaml` (split mode) or `product-context.yaml` (v1 mode) + operational state from `product-context.yaml`
**Output:** Calibration artifact (standalone or inline) + updated `calibration.history`

---

## Type Detection

Check how the skill was invoked to determine the calibration dimension:

**Path A — Explicit:** User says `/aep-calibrate visual-design` or `/aep-calibrate api-surface`. Type is given directly.

**Path B — Routed from `/aep-reflect`:** Reflection classified an observation as a calibration need with a specific dimension. The dimension and observation text are passed as context.

**Path C — Ambient:** User says `/aep-calibrate` with no type. Determine the type:

1. Check `calibration.plan` in `product-context.yaml` (operational file, both modes) — which dimension is next for the current layer?
2. Check stories with `calibration_type` set in the current `.5` layer.
3. If neither applies, ask the user: "What feels off? (visual design / UX flow / API surface / data model / scope direction / copy tone / performance quality)"

**Postcondition:** one of the 7 dimensions is selected.

---

## File Resolution

```bash
ls product/index.yaml 2>/dev/null && echo "SPLIT MODE" || echo "V1 MODE"
cat product-context.yaml
```

Mode semantics are canonical in `references/file-resolution.md`. Read the product definition (`quality_dimensions`, `layers`, `activities`, `constraints`, `success_criteria`, `failure_model`) from `product/index.yaml` in split mode or `product-context.yaml` in v1 mode; read operational state (`calibration.plan`, `calibration.history`, `stories`, `architecture`) always from `product-context.yaml`. Which file each type writes to is in `references/calibration-types.md` (Write Targets).

---

## Mode Detection

After type is determined, check for existing calibration:

- **Establishment mode**: no prior entry in `calibration.history` for this dimension → full brief, create artifact from scratch.
- **Extension mode**: prior entry exists → delta brief covering only NEW patterns/decisions not in the existing calibration.

---

## Calibration Types

Every dimension is either **heavy** or **light** (the split is canonical in `/aep-map` `references/alignment-layers.md`) — this drives the Phase 1 hand-off:

- **Heavy**: external exploration with tools outside the agent workflow, a standalone `calibration/<type>.yaml` artifact, and (usually) `.5` alignment-layer stories. Phase 1 stops and hands off.
- **Light**: conversational Q&A that updates the product context in place. Phase 1 flows straight into Phase 2.

Per-dimension brief templates, exploration methods, scan targets, and write targets are in `references/calibration-types.md` — load it when picking a dimension or scanning current state. For ux-flow / visual-design, running `/aep-design-lens` first seeds the brief with a theory-grounded, severity-scored health-check (evidence) that calibrate turns into the human's decision.

---

## Phase 1: Generate Brief

### Step 1: Read Product Context

```bash
cat product-context.yaml
```

Extract:

- `opportunity.bet` + `product.problem` → product identity
- `product.persona.description` + `product.persona.jtbd` → target user
- `opportunity.why_now` → urgency/positioning
- `stories` where `layer` matches the active layer → scope of what was built or will be built
- `product.constraints.required_stack` + `preferred_stack` → technical constraints
- `calibration.history` → prior calibration decisions for this dimension (extension mode)

### Step 2: Scan Current State

Inspect the current implementation against the scan targets for this type (table in `references/calibration-types.md` → Scan Targets) — e.g. `globals.css`/`components/`/`routes/` for visual-design, `architecture.interfaces` and handler files for api-surface.

### Step 3: Generate Brief

Use the type-specific template from `references/briefs/<type>.md`.

- **Establishment mode:** generate the full brief template with all sections.
- **Extension mode:** read the existing calibration artifact or product-context sections, then generate a focused brief covering only what's NEW in the current layer: "Here's your current calibrated system. These new [pages/endpoints/entities/etc.] need decisions not yet covered: [list]."

Write the brief to `docs/calibration-brief.md` and print the full content to the terminal.

**Postcondition:** `docs/calibration-brief.md` exists and its content was printed.

### Step 4: Hand Off

**Heavy calibrations (visual-design, ux-flow, copy-tone):** print exploration instructions and **stop** — the human explores externally, out of the agent's hands.

```
Calibration brief written to: docs/calibration-brief.md

Next steps (you do these):

  1. [Type-specific exploration instructions]
  2. Explore variations, then pick a direction.
  3. Save reference files to docs/design-references/ (if applicable)
  4. When ready, come back and run:
     /aep-calibrate capture
```

For visual-design, point the human to `references/vibe-design-tools.md` for tool guidance.

**Light calibrations (api-surface, data-model, scope-direction, performance-quality):** present the brief and proceed directly to Phase 2 — the brief frames the conversation, no external exploration needed.

---

## Phase 2: Capture Decisions (`/aep-calibrate capture`)

### Step 1: Interactive Q&A

Ask the structured questions from `references/capture/<type>.md`, one at a time. (Heavy examples: chosen direction, palette, journey decisions, voice. Light examples: naming decisions, entity/field names, scope-gap assessment, latency thresholds.)

**Postcondition:** every question in `references/capture/<type>.md` is answered.

### Step 2: Produce Artifact

**Heavy calibrations:**

- **Establishment mode:** create `calibration/<type>.yaml` from scratch using the schema at `references/schemas/<type>-schema.yaml`, filling all sections from the Q&A answers.
- **Extension mode:** read the existing `calibration/<type>.yaml`, add entries for newly covered patterns, and update `calibrated_at` / `calibrated_from_layer` — extend existing values, do not replace them.

For **visual-design** specifically, also update `globals.css` with the captured palette: convert values to oklch and write them as CSS custom properties under `:root` and `.dark`. Establishment replaces the full palette; extension adds only new custom properties.

**Light calibrations:** update the relevant section(s) in the file resolved for this type (see `references/calibration-types.md` → Write Targets) — `architecture.interfaces`, `architecture.domain_model`, `product.goals`/`mvp_boundary`/`layers`, or `product.success_criteria`/`failure_model`.

**Postcondition:** the chosen decision is recorded in existing schema fields — for heavy, `calibration/<type>.yaml` exists with the chosen direction in `direction.name`/`direction.description` and `calibrated_at` set; for light, the target section reflects the decisions.

### Step 3: Update Calibration History

Append one entry to `calibration.history` and one to `changelog` in `product-context.yaml`, per the product-context schema:

- `calibration.history` entry — `dimension`, `calibrated_at`, `calibrated_from_layer`, `mode` (establishment|extension), `artifact_path` (null for light calibrations), `sections_updated`, `summary`.
- `changelog` entry — `date`, `type: calibration`, `author: human`, `summary`, `sections_changed`.

### Step 4: Commit

Validate YAML before committing (see `references/yaml-guardrails.md` for common fixes):

```bash
npx js-yaml product-context.yaml > /dev/null && echo "YAML OK"
```

Fix any error before committing. Then commit and push to `$BASE` (resolve `$BASE` per `/aep-git-ref` "Resolving `$BASE`"):

```bash
git pull --ff-only origin "$BASE"
git add calibration/ product-context.yaml product/
git commit -m "feat: calibrate <dimension> — <brief summary>"
git push origin "$BASE"
```

**Postcondition:** `npx js-yaml` printed `YAML OK` and `git push` exited 0.
