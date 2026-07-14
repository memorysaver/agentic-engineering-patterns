---
name: aep-design-lens
description: >-
  Theory-grounded design guideline and 0–4 severity heuristic health-check for any
  product UI. Use on "design review", "usability check", "heuristic evaluation", or
  "accessibility check", and before building or auditing a UI. Not for taste capture
  (/aep-calibrate) or object/screen IA (/aep-model).
---

# Design Lens

Ground a product's UI/UX in **established HCI and design theory** instead of gut feel.
Point it at a product (an idea, a spec, or a running UI); it works out which theoretical
lenses apply, then produces concrete design suggestions, a **design guideline** to build
against, and a severity-scored **health-check table** that audits whether the current
design meets human expectations.

> "Overview first, zoom and filter, then details-on-demand."
> — Ben Shneiderman, ["The Eyes Have It: A Task by Data Type Taxonomy for Information
> Visualizations"](https://www.cs.umd.edu/~ben/papers/Shneiderman1996eyes.pdf) (1996)

**Two roles.** Invoke it directly to generate a guideline for a new product or run a
health-check on an existing one; or, as a library, other skills read
[`references/theory-catalog.md`](references/theory-catalog.md) to pull the right lens for
a design decision (e.g. `/aep-model` reaching for the information-seeking and
graphical-perception lenses when shaping a data-heavy screen).

---

## When to Use — and When NOT To

Reach for this skill to produce a **design guideline** before agents build, to run a
**usability / heuristic health-check** on a drafted or running UI, or when a design
decision would benefit from the **relevant established theory** (which chart encoding,
how to structure exploration, how to make an agent's actions legible).

Its boundary against neighboring skills — stated once, here:

- **Taste decisions** (palette, voice, journey feel) belong to `/aep-calibrate`. This
  skill supplies the theory a calibration leans on; it does not decide taste.
- **Object / screen IA** (which objects, what fields, which screens) belongs to
  `/aep-model`.
- **Factual correctness** of artifacts belongs to `/aep-validate`; the heuristic
  health-check is its advisory design-quality complement.
- It does **not** replace real user testing — heuristics predict where users will
  struggle; they do not observe actual users. Treat the health-check as evidence, not proof.

A backend-only surface with no human-perceived UI is out of scope — but a CLI that wraps
an agent still benefits from family E (Gulf of Envisioning), and any terminal output is a
human-perceived UI for family G (color-alone signals, contrast).

**Output goes to the conversation by default, or to a standalone markdown doc when the
user asks to save it — it never writes to `product-context.yaml` or any AEP schema file.**
This is an advisory leaf skill that adds no taxonomy.

---

## The Method

Product-agnostic, because it classifies **tasks and data**, not a product type. Full step
detail, the lens-selection rules, the worked examples, the live-UI evidence-gathering
protocol, and the output templates are in
[`references/method-and-templates.md`](references/method-and-templates.md).

**Two depths.** A **quick check** scores only the **Baseline Ten** — a distilled
cross-family checklist
([`method-and-templates.md#the-baseline-ten-quick-check`](references/method-and-templates.md#the-baseline-ten-quick-check)) —
and answers "is this OK?" in minutes; default to it for a lightweight question ("is this
good UX?"). A **deep audit** runs all seven steps below; default to it for a guideline or
review request. **Escalation rule:** any major/catastrophe finding (severity 3–4) upgrades
a quick check to a deep audit, so speed never trades away rigor.

1. **Characterize** — what is the product, who uses it, what are they trying to do? If a
   running UI or spec exists, point to it.
2. **Abstract** — classify the primary user **tasks** (monitor / compare / look-up /
   diagnose / explore / decide / create / reconstruct-provenance / converse) and the
   **data/content** type. (Munzner's domain → abstraction move.)
3. **Select lenses** — pick the applicable theory families from the catalog based on the
   task+data profile; families A, B, F, G always fire, C/D/E on triggers (selection rules
   in the reference). Note _why_ each family fired.
4. **Synthesize** — per selected lens, turn its **Apply** line into ≥1 concrete,
   product-specific suggestion, each tagged `(→ lens id)` back to its theory. _Done when
   every selected lens carries at least one traced suggestion._
5. **Emit a design guideline** — prescriptive, grouped by family: "the product should…".
6. **Emit a health-check table** — one row per checkable expectation, each traced to a
   theory and carrying a **Nielsen 0–4 severity**, producing a scored "meets human
   expectations?" audit.
7. **(Opt-in) persist** — on request, write the report to a standalone markdown file
   (default `docs/design-review/<scope>-<date>.md`).

---

## The Theory Catalog

Grouped into families so lenses can be selected per product. Full entries — each with
_core question · when it applies · how to apply · what to check · source_ — are in
[`references/theory-catalog.md`](references/theory-catalog.md), which is **append-only and
extensible** (it documents how to add a theory).

| Family                                    | Lenses (summary)                                                                                                                                           |
| ----------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **A. Usability & interaction heuristics** | Nielsen's 10 · Shneiderman's Eight Golden Rules · Norman's principles + Gulfs + Seven Stages · Jakob's / Tesler's / Postel's laws                          |
| **B. Cognitive & perceptual laws**        | Cognitive Load · Miller's · Hick's · Fitts's · Doherty Threshold · Gestalt · preattentive attributes · Aesthetic-Usability · memory/motivation biases      |
| **C. Information seeking & navigation**   | Shneiderman's Visual Information-Seeking Mantra · Information Foraging (scent) · Focus+Context / multiscale trio · IA / wayfinding                         |
| **D. Data visualization & encoding**      | Munzner's Nested Model + task typology · Graphical Perception · Grammar of Graphics / Bertin · Heer & Shneiderman · Tufte                                  |
| **E. Human-AI & agent-specific UX**       | Gulf of Envisioning · Microsoft's 18 HAI Guidelines · Google PAIR · trust calibration · observability · steerability/reversibility · Grice's maxims · CASA |
| **F. Evaluation & process methods**       | Heuristic Evaluation · Cognitive Walkthrough · Nielsen 0–4 severity scale · GOMS/KLM · Norman's stages-of-action as a lens                                 |
| **G. Accessibility & inclusive design**   | WCAG 2.2 POUR · keyboard & focus · contrast & legibility (never color alone) · inclusive design (permanent/temporary/situational impairments)              |

---

## How It Fits AEP

A design-lens pass feeds forward; control then returns to the calling context.

- **Guideline for a new UI** → `/aep-model` turns it into object/screen **structure**
  (drawing families C and D), and `/aep-calibrate` `ux-flow` / `visual-design` captures the
  human's **taste** decision (drawing A, B, C, E) — the report seeds the brief and
  pre-audits the flow before the human calibrates.
- **Health-check on a built UI** → route findings through `/aep-reflect` (a "works vs
  right" gap is a calibration or refinement item), or fix directly if it's a clear defect.
  `/aep-validate` points here for its advisory design-quality pass (family F).
- **Recurring audit** → pair with `/loop` to re-run the health-check each layer.

---

## Why it's shaped this way

A product-agnostic task+data method (not a product-type lookup), a leaf skill beside
`/aep-model` and `/aep-calibrate` rather than inside them, hybrid output, Nielsen 0–4
severity, two depths, and always-on accessibility — the reasoning is recorded in
[`docs/decisions/design-lens-rationale.md`](../../../docs/decisions/design-lens-rationale.md).
