---
name: calibrate
description: Human alignment checkpoint for any quality dimension during rapid agent development. Use when /reflect identifies a gap between "works" and "right", when the user says "calibrate", "align", "design brief", "capture", or before dispatching .5 alignment layers. Supports calibration types: visual-design, ux-flow, api-surface, data-model, scope-direction, copy-tone, performance-quality. Phase 1 generates a dimension-specific brief; Phase 2 captures decisions into the appropriate artifact.
---

# Calibrate

Human alignment checkpoint. Agents build to spec, but specs are lossy compressions of human intent. This skill pauses execution, lets the human inspect what was built, and captures what "right" actually means in a format agents can consume.

**Where this fits:**

```
/reflect (identified alignment gap)
  → /calibrate           (Phase 1: generate dimension-specific brief)
  → human explores       (method varies by type — external tools, conversation, code review)
  → /calibrate capture   (Phase 2: capture decisions into artifact)
  → /dispatch            (stories dispatched with calibration context)
```

**Session:** Main, interactive with user
**Input:** Calibration type + product definition from `product/index.yaml` (split mode) or `product-context.yaml` (v1 mode) + operational state from `product-context.yaml`
**Output:** Calibration artifact (standalone or inline) + updated `calibration.history`

---

## Type Detection

Check how the skill was invoked to determine the calibration dimension:

**Path A — Explicit:** User says `/calibrate visual-design` or `/calibrate api-surface`. Type is given directly.

**Path B — Routed from `/reflect`:** Reflection classified an observation as a calibration need with a specific dimension. The dimension and observation text are passed as context.

**Path C — Ambient:** User says `/calibrate` with no type. Determine the type:

1. Check `calibration.plan` in `product-context.yaml` (operational file, both modes) — which dimension is next for the current layer?
2. Check stories with `calibration_type` set in the current `.5` layer.
3. If neither applies, ask the user: "What feels off? (visual design / UX flow / API surface / data model / scope direction / copy tone / performance quality)"

---

## File Resolution

```bash
ls product/index.yaml 2>/dev/null && echo "SPLIT MODE" || echo "V1 MODE"
cat product-context.yaml
```

- **Split mode** (`product/index.yaml` exists): Read `quality_dimensions`, `layers`, `activities`, `constraints`, `success_criteria`, `failure_model` from `product/index.yaml`. Read `calibration.plan`, `calibration.history`, `stories`, `architecture` from `product-context.yaml`.
- **V1 mode**: Read everything from `product-context.yaml`.

**Write targets by calibration type:**

- **Heavy** (visual-design, ux-flow, copy-tone): Write `calibration/<type>.yaml`. Append `calibration.history` + `changelog` in `product-context.yaml`.
- **Light — architecture** (api-surface, data-model): Update `architecture.interfaces` or `architecture.domain_model` in `product-context.yaml`. Append `calibration.history` + `changelog`.
- **Light — product intent** (scope-direction, performance-quality): Update `product.goals`, `product.mvp_boundary`, `product.layers`, `product.success_criteria`, or `product.failure_model` in `product/index.yaml` (split mode) or `product-context.yaml` (v1 mode). Append `calibration.history` + `changelog` in `product-context.yaml`.

---

## Mode Detection

After type is determined, check for existing calibration:

- **Establishment mode**: No prior entry in `calibration.history` for this dimension → full brief, create artifact from scratch
- **Extension mode**: Prior entry exists → delta brief covering only NEW patterns/decisions not in existing calibration

---

## The Two Classes of Calibration

Calibration types split into two natural classes:

### Heavy Calibrations

External exploration required. The human uses tools outside the agent workflow (design tools, wireframing, copy docs). Produces standalone YAML artifacts in `calibration/` directory. Creates `.5` alignment layer stories.

| Dimension     | Brief Template                       | Exploration Method                     | Time Scale | Capture Artifact                 |
| ------------- | ------------------------------------ | -------------------------------------- | ---------- | -------------------------------- |
| visual-design | `references/briefs/visual-design.md` | External tool (Stitch, Pencil.dev)     | Hours–days | `calibration/visual-design.yaml` |
| ux-flow       | `references/briefs/ux-flow.md`       | Conversation + optional wireframe tool | 30–60 min  | `calibration/ux-flow.yaml`       |
| copy-tone     | `references/briefs/copy-tone.md`     | Conversation + copy doc                | 1–2 hours  | `calibration/copy-tone.yaml`     |

### Light Calibrations

Conversational. The human reviews current state and makes decisions through structured Q&A. Updates existing sections of `product-context.yaml` directly. May or may not create `.5` layer stories.

| Dimension           | Brief Template                             | Exploration Method           | Time Scale | Sections Updated                                          |
| ------------------- | ------------------------------------------ | ---------------------------- | ---------- | --------------------------------------------------------- |
| api-surface         | `references/briefs/api-surface.md`         | Conversation + code review   | 30–60 min  | `architecture.interfaces`                                 |
| data-model          | `references/briefs/data-model.md`          | Conversation + schema review | 30–60 min  | `architecture.domain_model`                               |
| scope-direction     | `references/briefs/scope-direction.md`     | Conversation                 | 30–60 min  | `product.goals`, `product.mvp_boundary`, `product.layers` |
| performance-quality | `references/briefs/performance-quality.md` | Conversation + benchmarks    | 30–60 min  | `product.success_criteria`, `product.failure_model`       |

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

Scan targets vary by calibration type:

| Type                | Scan Targets                                                                                   |
| ------------------- | ---------------------------------------------------------------------------------------------- |
| visual-design       | `globals.css` (theme tokens), `components/` (available components), `routes/` (existing pages) |
| ux-flow             | `routes/` (existing pages), `product.activities` (journey backbone), stories (what was built)  |
| api-surface         | `architecture.interfaces` (contracts), existing API/route handler files, endpoint patterns     |
| data-model          | `architecture.domain_model` (entities), schema/migration files, ORM models                     |
| scope-direction     | `product.goals`, `product.mvp_boundary`, stories by layer (built vs planned)                   |
| copy-tone           | UI components with text content, `product.persona`, brand-related product context              |
| performance-quality | `product.success_criteria.non_functional`, error logs, monitoring data if available            |

### Step 3: Generate Brief

Use the type-specific template from `references/briefs/<type>.md`.

**Establishment mode:** Generate the full brief template with all sections.

**Extension mode:** Read existing calibration artifact or product-context sections. Identify what's NEW in the current layer that isn't covered by prior calibration. Generate a focused brief: "Here's your current calibrated system. These new [pages/endpoints/entities/etc.] need decisions not yet covered: [list]."

Write the brief to `docs/calibration-brief.md` and output the full content to terminal.

### Step 4: Hand Off

**Heavy calibrations (visual-design, ux-flow, copy-tone):**

Output exploration instructions and **stop**. The human explores externally — this is explicitly out of the agent's hands.

```
Calibration brief written to: docs/calibration-brief.md

Next steps (you do these):

  1. [Type-specific exploration instructions]
  2. Explore variations. Pick what feels right.
  3. Save reference files to docs/calibration-references/ (if applicable)
  4. When ready, come back and run:
     /calibrate capture
```

For visual-design specifically, point to `references/vibe-design-tools.md` for tool guidance.

**Light calibrations (api-surface, data-model, scope-direction, performance-quality):**

Present the brief to the human, then proceed directly to Phase 2. No external exploration needed — the brief frames the conversation.

---

## Phase 2: Capture Decisions (`/calibrate capture`)

### Step 1: Interactive Q&A

Ask structured questions from `references/capture/<type>.md`, one at a time.

**Heavy calibration questions (examples):**

- **visual-design:** Direction chosen, palette, typography, components, layout, brand signals, reference files
- **ux-flow:** Journey decisions, transition feel (instant/guided/deliberate), page map, entry/exit points
- **copy-tone:** Voice personality, reference products, pattern decisions (headings, buttons, errors, empty states), glossary terms

**Light calibration questions (examples):**

- **api-surface:** Naming decisions, grouping, error contract shape, versioning approach
- **data-model:** Entity names, field names, relationships, invariants, normalization rules
- **scope-direction:** Gap assessment (what percentage is "right"?), one-thing-to-add, scope gap vs direction gap
- **performance-quality:** Latency thresholds per action, retry policy, caching strategy, degradation behavior

### Step 2: Produce Artifact

**Heavy calibrations:**

- **Establishment mode:** Create the artifact YAML from scratch using the schema at `references/schemas/<type>-schema.yaml`. Fill all sections from Q&A answers.
- **Extension mode:** Read existing `calibration/<type>.yaml`. Add new entries for newly covered patterns. Update `calibrated_at` and `calibrated_from_layer`. Do not replace existing values — extend them.

Write to `calibration/<type>.yaml`.

**Light calibrations:**

Update the relevant section(s) — see File Resolution above for which file to write to per calibration type:

| Type                | Section to Update                                                  |
| ------------------- | ------------------------------------------------------------------ |
| api-surface         | `architecture.interfaces` — naming, grouping, error contracts      |
| data-model          | `architecture.domain_model` — entity names, fields, relationships  |
| scope-direction     | `product.goals`, `product.mvp_boundary`, `product.layers`          |
| performance-quality | `product.success_criteria.non_functional`, `product.failure_model` |

**For visual-design specifically:** Also update `globals.css` with the captured palette. Read palette values, convert to oklch if provided in other color spaces, write as CSS custom properties under `:root` and `.dark` selectors.

- **Establishment mode:** Replace the full palette.
- **Extension mode:** Only add new custom properties if the palette expanded. Do not touch existing values.

### Step 3: Update Calibration History

Append to `calibration.history` in `product-context.yaml`:

```yaml
- dimension: <type>
  calibrated_at: "<ISO date>"
  calibrated_from_layer: <layer>
  mode: establishment # or extension
  artifact_path: "calibration/<type>.yaml" # null for light calibrations
  sections_updated: [] # e.g., ["architecture.interfaces"] for light calibrations
  summary: "<one-line summary of what was decided>"
```

Also append to `changelog`:

```yaml
- date: <ISO date>
  type: calibration
  author: human
  summary: "Calibrated <dimension> — <summary of decisions>"
  sections_changed:
    - calibration
    - <any other sections updated>
```

### Step 4: Commit

```bash
git pull --ff-only origin main
git add calibration/ product-context.yaml
git commit -m "feat: calibrate <dimension> — <brief summary>"
git push origin main
```

---

## Type-Specific Reference: visual-design

Visual design is the most fully developed calibration type and serves as the reference implementation for others.

### Phase 1 Specifics

- **Design brief template:** `references/briefs/visual-design.md`
- **Design directions:** Generate 3 directions spanning a spectrum from "maximum technical" to "maximum approachable". Name 2-3 reference products per direction as visual mood board anchors.
- **Vibe design tool guide:** `references/vibe-design-tools.md` — covers Google Stitch, Pencil.dev, and alternatives.

### Phase 2 Specifics

- **Schema:** `references/schemas/visual-design-schema.yaml`
- **Artifact:** `calibration/visual-design.yaml` — palette (oklch), typography, spacing, layout, components, brand signals, reference designs
- **Companion artifact:** Updated `globals.css` with CSS custom properties from captured palette
- **Three concerns, three artifacts:** `calibration/visual-design.yaml` documents decisions (why), `globals.css` enacts them (what), reference files in `docs/design-references/` show the visual target (how it looks).

### Mode Detection

- **Establishment mode:** `calibration/visual-design.yaml` does not exist → full calibration (palette, typography, layout, everything)
- **Extension mode:** File exists → focused brief covering only NEW UI patterns not in the existing design context

---

## Key Principles

- **Agents optimize for correctness against spec.** But specs are lossy compressions of human intent. Calibration corrects the loss.
- **The human decides.** The skill generates options, frames choices, and captures decisions. It does not make choices.
- **Heavy calibrations pause; light calibrations flow.** Visual design needs external tools and hours. API naming needs a 30-minute conversation. Both are calibration.
- **Extension mode covers only the delta.** No full redesign on subsequent calibrations — just new patterns that prior calibration didn't cover.
- **Machine-readable artifacts.** Agents query calibration artifacts for specific values (`palette.dark.primary`, `components.border_radius`, `voice.personality`). Prose descriptions are not queryable.
- **`.5` layers are human alignment layers.** Not just "UI polish." Any dimension where agent output diverges from human intent.
