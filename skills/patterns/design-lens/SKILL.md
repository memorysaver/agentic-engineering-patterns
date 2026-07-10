---
name: aep-design-lens
description: |-
  Theory-grounded design guidance and heuristic health-check for any product's UI/UX. This is a utility skill — an extensible catalog of established HCI/design theory in seven families (Nielsen/Norman/Shneiderman heuristics, cognitive & perceptual laws, information seeking, data-viz encoding, Human-AI & agent UX, accessibility, evaluation methods) plus a method that classifies the user's tasks + data (not a fixed product type) to select the lenses that apply, then emits design suggestions, a design guideline to build against, and a Nielsen 0–4 severity-scored health-check. Two depths: quick check (Baseline Ten) or deep audit. Use on "design review", "design guideline", "usability check", "heuristic evaluation", "design health check", "is this good UX", "accessibility check", "apply Nielsen/Norman", "audit my dashboard/agent UI", or when planning or auditing a UI. Cross-linked from /aep-model, /aep-calibrate, /aep-validate. NOT for capturing human taste (/aep-calibrate) or object/screen IA (/aep-model).
---

# Design Lens

A reusable pattern for grounding a product's UI/UX in **established HCI and design
theory** instead of gut feel. You point it at a product (an idea, a spec, or a
running UI); it works out which theoretical lenses apply, then produces concrete
design suggestions, a **design guideline** to build against, and a severity-scored
**health-check table** that audits whether the current design meets human
expectations.

> "Overview first, zoom and filter, then details-on-demand."
> — Ben Shneiderman, ["The Eyes Have It: A Task by Data Type Taxonomy for
> Information Visualizations"](https://www.cs.umd.edu/~ben/papers/Shneiderman1996eyes.pdf) (1996)

**This skill is both a utility library and a standalone skill:**

- **As a library:** other skills read `references/theory-catalog.md` to pull the
  right lens for a design decision (e.g. `/aep-model` reaching for the information-
  seeking and graphical-perception lenses when shaping a data-heavy screen).
- **As a standalone skill:** invoke directly to generate a design guideline for a
  new product, or to run a heuristic health-check on an existing one.

---

## Why This Pattern Earns Its Place

AEP's third design principle: _every harness component earns its place — each
exists because of a specific failure mode._ Grounding design in theory is a
structural fix for four failure modes that appear when agents design and build UI:

| Failure mode                     | What it looks like                                                                                                | How this pattern prevents it                                                                                                     |
| -------------------------------- | ----------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------- |
| **Ships to spec, not to humans** | The agent builds exactly what the story specifies — and it still violates basic usability (no feedback, no undo). | A design guideline injects evidence-based principles _before_ build, so the spec is checked against 40 years of HCI research.    |
| **"Looks done" ≠ usable**        | A UI renders and runs, so it's called complete — but fails Nielsen/Norman/cognitive-load tests a user would hit.  | A severity-scored heuristic health-check makes the gap between "works" and "right" explicit and rankable.                        |
| **Reinventing design knowledge** | Each product re-derives (or silently ignores) the same well-known theory.                                         | One extensible catalog makes the knowledge reusable and consistent across products — the same reason patterns exist at all.      |
| **Wrong theory for the product** | Applying data-viz encoding rules to a landing page, or ignoring them on a dashboard.                              | A task/data abstraction step selects _only_ the lenses that fire for this product, so the guidance is relevant, not boilerplate. |

The mechanism in every case is the same: **a shared theory catalog + a selection
method** instead of per-product guesswork.

### Design goals

Every output this skill produces is held to five goals — use them to judge whether
a run did its job:

1. **Relevant, not boilerplate** — lenses are selected by the product's task+data
   profile; families that didn't fire don't appear in the output.
2. **Traceable** — every suggestion and health-check row carries a `(→ lens id)`
   back to a citable source, so the guidance is defensible, not vibes.
3. **Honest** — Nielsen 0–4 severity forces triage; passing designs get short
   tables, and findings cite observed evidence, not assumptions.
4. **Actionable** — ranked by severity, closed with a cross-family "Top 5", and
   phrased specifically to the characterized product.
5. **Cheap to invoke** — a quick check (the Baseline Ten) answers "is this OK?"
   in minutes; the full audit is reserved for when depth is warranted.

---

## The Method (summary)

Product-agnostic, because it classifies **tasks and data**, not a product type.
Full step detail, the lens-selection rules, and the output templates are in
[`references/method-and-templates.md`](references/method-and-templates.md).

**Two depths.** A **quick check** scores just the **Baseline Ten** — a distilled
cross-family checklist — and answers "is this OK?" in minutes; any major/catastrophe
finding escalates to the full audit. A **deep audit** runs all seven steps below.
Default to quick check when the user asks a lightweight question ("is this good
UX?"), deep audit when they ask for a guideline or a review.

1. **Characterize** — what is the product, who uses it, what are they trying to do?
2. **Abstract** — classify the primary user **tasks** (monitor / compare / look-up /
   diagnose / explore / decide / create / reconstruct-provenance / converse) and the
   **data/content** type. (Munzner's domain → abstraction move.)
3. **Select lenses** — pick the applicable theory families from the catalog based on
   the task+data profile (rules below).
4. **Synthesize** — per selected lens, generate concrete design suggestions.
5. **Emit a design guideline** — prescriptive, grouped by family: "the product should…".
6. **Emit a health-check table** — each row traces to a theory, is checkable against
   the current design, and carries a Nielsen 0–4 severity, producing a scored
   "meets human expectations?" audit.
7. **(Opt-in) persist** — on request, write the report to a standalone markdown file.

**Lens-selection rules:**

| Product profile                                        | Add these families                                                |
| ------------------------------------------------------ | ----------------------------------------------------------------- |
| **Any interactive UI** (baseline)                      | A (usability heuristics) + B (cognitive & perceptual laws)        |
| **Large/complex info space, exploration**              | + C (info-seeking mantra, Information Foraging, Focus+Context)    |
| **Data-dense** (charts, dashboards, BI, observability) | + D (Munzner, Graphical Perception, Heer & Shneiderman, Tufte)    |
| **LLM / agent / chat / prompt / workflow composer**    | + E (Gulf of Envisioning, Human-AI guidelines, trust calibration) |
| **Always**                                             | + F (evaluation method + Nielsen 0–4 severity scoring)            |
| **Always** (any human-perceived UI, incl. terminal)    | + G (accessibility & inclusive design — WCAG POUR)                |

---

## The Theory Catalog (summary)

Grouped into families so lenses can be selected per product. Full entries — each
with _core question · when it applies · how to apply · what to check · source_ —
are in [`references/theory-catalog.md`](references/theory-catalog.md), which is
**append-only and extensible** (it documents how to add a theory).

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

## When to Use — and When NOT To

**Reach for this skill when:**

- Starting a UI-facing product and you want a **design guideline** grounded in
  theory before agents build (feeds `/aep-model` and `/aep-calibrate`).
- You have a running or drafted UI and want a **usability / heuristic health-check**
  with ranked severities.
- A design decision would benefit from the **relevant established theory** (which
  chart encoding, how to structure exploration, how to make an agent's actions legible).

**Do NOT reach for this skill when:**

- You need to **capture a human's taste decision** (palette, voice, journey feel) —
  that is [`/aep-calibrate`](../../product-context/calibrate/SKILL.md). This skill
  supplies the theory a calibration can lean on; it does not decide taste.
- You need the **object/screen IA structure** (which objects, what fields, which
  screens) — that is [`/aep-model`](../../product-context/model/SKILL.md).
- It's a **backend-only** surface with no human-perceived UI — though a CLI that
  wraps an agent still benefits from family E (Gulf of Envisioning), and any terminal
  output is still a human-perceived UI for family G (color-alone signals, contrast).
- You want it to **replace real user testing.** Heuristics predict where users will
  struggle; they do not observe actual users. Treat the health-check as evidence, not proof.

---

## How This Fits AEP (touchpoints)

| AEP touchpoint                                                         | Lens family it draws                 | Relationship                                                                                                                                                                                |
| ---------------------------------------------------------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [`/aep-model`](../../product-context/model/SKILL.md)                   | C (seeking/navigation), D (encoding) | Model decides _what_ objects/screens exist; this skill supplies the exploration/data-viz theory that informs how those screens should behave.                                               |
| [`/aep-calibrate`](../../product-context/calibrate/SKILL.md) `ux-flow` | A, B, C, E                           | A design-lens report can seed the ux-flow brief and pre-audit the flow before the human calibrates taste. Calibrate captures the decision; this skill supplies the evidence.                |
| [`/aep-validate`](../../product-context/validate/SKILL.md)             | F (evaluation)                       | Validate checks factual correctness of artifacts; the heuristic health-check is the _design-quality_ complement it points to for subjective/usability review (advisory, not a schema mode). |

### Cross-skill reference path

After sync with the `aep-` prefix, the catalog is at:

```
.claude/skills/aep-design-lens/references/theory-catalog.md
.claude/skills/aep-design-lens/references/method-and-templates.md
```

---

## Standalone Usage

1. **Describe the product.** A sentence or two is enough: what it is, who uses it,
   what they do with it. If a running UI or spec exists, point to it.
2. **Pick the depth.** Quick question ("is this good UX?") → **quick check**: score
   the Baseline Ten (`references/method-and-templates.md#the-baseline-ten-quick-check`)
   and stop unless something major surfaces. Guideline or review request → **deep
   audit**: continue with the full method.
3. **Run the method** (`references/method-and-templates.md`): characterize → abstract
   the tasks + data → select the lens families that fire.
4. **Produce the guideline** — the prescriptive "the product should…" statements,
   grouped by family, using the guideline template.
5. **Produce the health-check table** — one row per checkable expectation, each traced
   to a theory and scored 0–4 (Nielsen severity). Sum to an overall read on whether the
   design meets human expectations.
6. **Present the report** in the conversation (default). **Opt-in persist:** if the
   user asks ("save this", "write it to a file"), write a standalone markdown file
   (default `docs/design-review/<scope>-<date>.md`). Never write to `product-context.yaml`
   or any AEP schema file — this skill is advisory and adds no taxonomy.

Read [`references/method-and-templates.md`](references/method-and-templates.md) for
the templates and the worked selection rules.

---

## Design Decisions

**Why a product-agnostic method, not a product-type table.** A fixed
"dashboard → theories X" table drifts the moment a product is a hybrid, and it never
covers the long tail. Classifying **tasks + data** (Munzner's abstraction layer)
selects the right lenses for _any_ product, and the reusable value moves into the
theory catalog + the selection rules — which are stable — rather than a brittle
lookup that would need constant maintenance.

**Why it sits beside `/aep-model` and `/aep-calibrate`, not inside them.** They own
different jobs: `/aep-model` decides the object/screen **structure** (the _what_);
`/aep-calibrate` captures a human's **taste** decision (the _chosen_ look/voice/flow).
Neither owns the **external, evidence-based theory** — the _why_. Folding theory into
either would blur a clean separation and hide the health-check. This skill supplies
the theory both can lean on and points back to them for their jobs.

**Why hybrid output (report by default, opt-in file).** Most runs are a quick guide
or audit inside a conversation; forcing a persisted artifact every time adds
ceremony. But a design guideline is worth keeping, so persistence is one ask away —
as a standalone markdown deliverable that touches no schema, keeping this a zero-drift
leaf skill.

**Why the health-check uses Nielsen's 0–4 severity.** A checklist without severity
invites "everything is minor." Nielsen's scale (0 = not a problem … 4 = usability
catastrophe) forces triage and makes "does it meet human expectations?" answerable
rather than vibes-based.

**Why two depths (quick check vs deep audit).** Tesler's Law applied to this skill
itself: forcing the full 7-step audit on every "is this OK?" question pushes the
method's cost onto the user, so the cheap path would get skipped. The Baseline Ten
gives a minutes-scale sweep with an honest escalation rule — any major/catastrophe
finding upgrades to a deep audit — so speed never silently trades away rigor.

**Why accessibility always fires (family G).** The other families assume the user can
perceive and operate the UI at all; accessibility is that precondition, not a
nice-to-have. WCAG criteria are checkable and severity-scorable exactly like heuristic
failures, and impairments are situational as often as permanent (glare, one hand,
noise), so G applies to every human-perceived surface — including terminal output.

---

## Next Step

After a design-lens pass, control returns to the calling context:

- Guideline for a new UI product → feed it into
  [`/aep-model`](../../product-context/model/SKILL.md) (structure) and
  [`/aep-calibrate`](../../product-context/calibrate/SKILL.md) `ux-flow` / `visual-design`
  (taste), which turn the theory into concrete IA and captured decisions.
- Health-check on a built UI → route findings through
  [`/aep-reflect`](../../product-context/reflect/SKILL.md) (a "works vs right" gap is a
  calibration or refinement item), or fix directly if it's a clear defect.
- Recurring audit → pair with `/loop` to re-run the health-check each layer.
