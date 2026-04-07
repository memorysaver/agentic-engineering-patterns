# Calibration Types

Overview of the 7 calibration dimensions supported by `/calibrate`. Each dimension addresses a specific gap between "works correctly" and "feels right" that agents cannot judge.

---

## Heavy Calibrations

External exploration required. Produces standalone YAML artifacts in `calibration/` directory. Typically creates `.5` alignment layer stories.

### visual-design

Brand identity, color palette, typography, layout patterns, component styling. Nearly always needed for user-facing products. The most developed calibration type — includes design brief with 3 spectrum-based directions, vibe design tool workflow (Stitch, Pencil.dev), and `globals.css` integration.

**Artifact:** `calibration/visual-design.yaml` + updated `globals.css`

### ux-flow

User journey, information architecture, page transitions, navigation patterns. Needed when screens exist but the flow between them doesn't feel right — dead ends, wrong information density, confusing navigation, or mismatched transition pacing.

**Artifact:** `calibration/ux-flow.yaml`

### copy-tone

Brand voice, error messages, button labels, empty states, heading style, technical jargon policy. Needed when the product reads wrong — too formal, too casual, inconsistent terminology, or generic AI-generated text.

**Artifact:** `calibration/copy-tone.yaml`

---

## Light Calibrations

Conversational. The human reviews current state and makes decisions through structured Q&A. Updates `product-context.yaml` sections directly. May or may not create `.5` layer stories.

### api-surface

Endpoint naming, grouping, error contracts, versioning, pagination conventions. Needed when the API works but naming doesn't match domain language, grouping feels arbitrary, or error responses are inconsistent.

**Updates:** `architecture.interfaces` in `product-context.yaml`

### data-model

Entity naming, field semantics, relationships, invariants, normalization rules. Needed when the schema is functional but doesn't match the domain language the team actually uses in conversation.

**Updates:** `architecture.domain_model` in `product-context.yaml`

### scope-direction

Mid-build intent correction. Needed when what was built doesn't match what the PM/developer imagined — either missing features (scope gap) or wrong features (direction gap). Most common when PM and builder are different people.

**Updates:** `product.goals`, `product.mvp_boundary`, `product.layers` in `product-context.yaml`

### performance-quality

Latency thresholds, retry behavior, caching strategy, degradation behavior, cost ceilings. Needed when the system works but tolerances haven't been explicitly decided — agents default to generic retry/timeout patterns that may not match actual requirements.

**Updates:** `product.success_criteria.non_functional`, `product.failure_model` in `product-context.yaml`
