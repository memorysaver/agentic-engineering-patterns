# Calibration Types

The 7 calibration dimensions supported by `/aep-calibrate`. Each addresses a specific gap between "works correctly" and "feels right" that agents cannot judge. Load this when picking a dimension, when running Phase 1 Step 2 (scan current state), or when resolving where a type writes its output.

Dimensions split into two classes by how the human explores and where the result lands.

---

## Heavy Calibrations

External exploration required. The human uses tools outside the agent workflow (design tools, wireframing, copy docs). Produces a standalone YAML artifact in `calibration/` and typically creates `.5` alignment-layer stories. Phase 1 hands off and stops; the human explores, then runs `/aep-calibrate capture`.

| Dimension     | Brief template                       | Exploration method                     | Time scale | Capture artifact                 |
| ------------- | ------------------------------------ | -------------------------------------- | ---------- | -------------------------------- |
| visual-design | `references/briefs/visual-design.md` | External tool (Stitch, Pencil.dev)     | Hours–days | `calibration/visual-design.yaml` |
| ux-flow       | `references/briefs/ux-flow.md`       | Conversation + optional wireframe tool | 30–60 min  | `calibration/ux-flow.yaml`       |
| copy-tone     | `references/briefs/copy-tone.md`     | Conversation + copy doc                | 1–2 hours  | `calibration/copy-tone.yaml`     |

**Theory before taste (ux-flow / visual-design).** A `/aep-design-lens` report can seed the brief and pre-audit the flow against established HCI theory with a severity-scored health-check. Design-lens supplies the _evidence_; calibrate captures the human's _decision_. Run design-lens first when you want the brief grounded in theory rather than a blank page.

### visual-design

Brand identity, color palette, typography, layout patterns, component styling. Nearly always needed for user-facing products. The most developed calibration type — includes a design brief with 3 spectrum-based directions, a vibe design tool workflow (Stitch, Pencil.dev), and `globals.css` integration.

**Artifact:** `calibration/visual-design.yaml` + updated `globals.css`

### ux-flow

User journey, information architecture, page transitions, navigation patterns. Needed when screens exist but the flow between them doesn't feel right — dead ends, wrong information density, confusing navigation, or mismatched transition pacing.

**Artifact:** `calibration/ux-flow.yaml`

### copy-tone

Brand voice, error messages, button labels, empty states, heading style, technical jargon policy. Needed when the product reads wrong — too formal, too casual, inconsistent terminology, or generic AI-generated text.

**Artifact:** `calibration/copy-tone.yaml`

---

## Light Calibrations

Conversational. The human reviews current state and makes decisions through structured Q&A. Updates existing sections of the product context directly. May or may not create `.5` layer stories. Phase 1 presents the brief and proceeds straight to Phase 2 — no external exploration.

| Dimension           | Brief template                             | Exploration method           | Time scale | Sections updated                                          |
| ------------------- | ------------------------------------------ | ---------------------------- | ---------- | --------------------------------------------------------- |
| api-surface         | `references/briefs/api-surface.md`         | Conversation + code review   | 30–60 min  | `architecture.interfaces`                                 |
| data-model          | `references/briefs/data-model.md`          | Conversation + schema review | 30–60 min  | `architecture.domain_model`                               |
| scope-direction     | `references/briefs/scope-direction.md`     | Conversation                 | 30–60 min  | `product.goals`, `product.mvp_boundary`, `product.layers` |
| performance-quality | `references/briefs/performance-quality.md` | Conversation + benchmarks    | 30–60 min  | `product.success_criteria`, `product.failure_model`       |

### api-surface

Endpoint naming, grouping, error contracts, versioning, pagination conventions. Needed when the API works but naming doesn't match domain language, grouping feels arbitrary, or error responses are inconsistent.

**Updates:** `architecture.interfaces` in `product-context.yaml`

### data-model

Entity naming, field semantics, relationships, invariants, normalization rules. Needed when the schema is functional but doesn't match the domain language the team actually uses in conversation.

**Updates:** `architecture.domain_model` in `product-context.yaml`

### scope-direction

Mid-build intent correction. Needed when what was built doesn't match what the PM/developer imagined — either missing features (scope gap) or wrong features (direction gap). Most common when PM and builder are different people.

**Updates:** `product.goals`, `product.mvp_boundary`, `product.layers`

### performance-quality

Latency thresholds, retry behavior, caching strategy, degradation behavior, cost ceilings. Needed when the system works but tolerances haven't been explicitly decided — agents default to generic retry/timeout patterns that may not match actual requirements.

**Updates:** `product.success_criteria.non_functional`, `product.failure_model`

---

## Scan Targets (Phase 1 Step 2)

What to inspect when framing the brief, per dimension:

| Type                | Scan targets                                                                                   |
| ------------------- | ---------------------------------------------------------------------------------------------- |
| visual-design       | `globals.css` (theme tokens), `components/` (available components), `routes/` (existing pages) |
| ux-flow             | `routes/` (existing pages), `product.activities` (journey backbone), stories (what was built)  |
| api-surface         | `architecture.interfaces` (contracts), existing API/route handler files, endpoint patterns     |
| data-model          | `architecture.domain_model` (entities), schema/migration files, ORM models                     |
| scope-direction     | `product.goals`, `product.mvp_boundary`, stories by layer (built vs planned)                   |
| copy-tone           | UI components with text content, `product.persona`, brand-related product context              |
| performance-quality | `product.success_criteria.non_functional`, error logs, monitoring data if available            |

---

## Write Targets (Phase 2 Step 2)

Where each class lands its output. Every type also appends `calibration.history` + `changelog` in `product-context.yaml` (per the product-context schema).

- **Heavy** (visual-design, ux-flow, copy-tone): write `calibration/<type>.yaml` (a standalone artifact). visual-design additionally writes the captured palette into `globals.css`.
- **Light — architecture** (api-surface, data-model): update `architecture.interfaces` or `architecture.domain_model` in `product-context.yaml`.
- **Light — product intent** (scope-direction, performance-quality): update `product.goals`, `product.mvp_boundary`, `product.layers`, `product.success_criteria`, or `product.failure_model` — in `product/index.yaml` (split mode) or `product-context.yaml` (v1 mode); see `references/file-resolution.md`.
