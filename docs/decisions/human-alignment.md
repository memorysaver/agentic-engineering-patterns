# Human Alignment: A Project Brief Surface for AEP

> **Status:** Proposed. Decision doc only, per the decision-doc-first convention
> (PR #17, PR #24 precedent) — no schema, skill, or marketplace edits in this PR;
> implementation follows in a separate PR reviewed against this document.

## Problem

AEP's planning layer captures intent and state in `product-context.yaml` — stories,
layers, gates, architecture, cost, changelog — but has no human-facing rendering of
that state. The owner reads raw YAML, scrolls git history, or asks the agent, and
none of those answers the three questions a returning human actually has: _where are
we, what needs me, where did reality drift from intent?_ As agents do more of the
work, the human's cognitive model of the project decays fastest — and the planning
layer, which knows the answer, stays silent about it.

SIBYL (a downstream consumer) built this surface project-locally as its L33 layer:
`docs/human-alignment/` holds a media-neutral design guideline, an HTML Brief, and a
register of measured problems. This decision upstreams and generalizes that work into
an AEP skill, following the established SIBYL→AEP convergence pattern (v2.7.0
precedent): the owner runs **`/aep-human-alignment`** in any AEP repo and gets a
one-pager HTML brief of the project's current state.

## Sources

Three references shaped the design, each contributing one layer:

1. **SIBYL `docs/human-alignment/`** — the _contract_. `guideline.md` fixes five
   principles (overview first; three tenses; one artifact per language; architecture
   as vocabulary; delta-gate) and a six-section Brief. `observations.md` records the
   measured failures this design must not repeat: **OBS-1** — a 220-node DAG rendered
   as one graph is a machine artifact, not a human surface; **OBS-3** — hand-authored
   narrative rots when regeneration isn't wired to real state; **OBS-5** — the
   deterministically derived artifacts are the only ones the human learned to trust.
2. **guizang-ppt-skill** — the _authoring discipline_ for single-file HTML surfaces:
   `assets/template.html` is the only source of CSS class names (preflight before
   writing any section), references are split by concern (layouts / components /
   themes / checklist), and delivery requires passing a P0-graded self-check.
3. **archify** (tt-a1i/archify v2.12) — the _presentation ethic_: an "evidence
   console" where every rendered fact derives from authored or verified evidence;
   one spatial narrative first; progressive disclosure over permanent chrome; ≤12
   nodes per diagram with detail in cards rather than edges; deterministic validation
   with repair receipts. Notably, archify's rich interactivity (focus, semantic
   passport, guided views) is vanilla JS over inline SVG — no framework.

## Decisions

### D1 — A standalone top-level skill: `skills/human-alignment/`

The skill lives at `skills/human-alignment/` — a peer of the four existing
categories, not nested inside one — and syncs as **`/aep-human-alignment`**.

- Human cognitive alignment is a functional theme of its own (owner direction), not
  a phase of the planning loop; the flat path also keeps the invocation name the
  owner specified without a `human-alignment/human-alignment/` duplicate.
- Tooling needs no change: CI's line budget uses `find skills -name SKILL.md`
  (depth-agnostic), `check-skills-package.sh` discovers skills by `SKILL.md` path,
  and `.claude-plugin/marketplace.json` enumerates explicit paths — implementation
  adds a fifth plugin entry (`"human-alignment"`, skills:
  `["./skills/human-alignment"]`).
- **design-lens does not move in this round.** Skills install per-directory
  (symlink or whole-dir copy), so a skill nested inside another skill's directory
  would ship inside every `/aep-human-alignment` install. Grouping design-lens under
  a human-alignment _category_ is recorded as open — worth doing when a second
  theory skill exists under the theme. Until then the new skill **cross-references**
  `/aep-design-lens` for HCI theory (R2: one canonical home; theory is not
  duplicated).

### D2 — The artifact: a timestamped, commit-stamped brief in `docs/human-alignment/`

One self-contained vertical-scroll HTML page per run, written to
`docs/human-alignment/brief-<YYYY-MM-DD>T<HHMM>Z-<shorthash>.html` — e.g.
`brief-2026-07-24T0730Z-96a63f7.html`. The filename carries the generation time
(UTC, no colons, lexicographically sortable) and the git commit the brief was
generated at, so provenance is visible without opening the file (owner direction).
Runs accumulate as a reviewable record; pruning old briefs is the owner's choice,
and the newest file by filename sort is the delta baseline for the next run. A
stable `latest` pointer is a recorded option if a fixed URL is ever needed (a
symlink breaks static hosting, so it would be a copy or a redirect stub).

Six sections, ported from SIBYL's contract with `product-context.yaml` as the data
source:

| Section       | Job                                                                | Derived from                                                    | Regeneration gate             |
| ------------- | ------------------------------------------------------------------ | --------------------------------------------------------------- | ----------------------------- |
| **NOW**       | since-you-last-looked delta band · the one ask · quiet zone        | story-state diff vs. baseline; stories in human-blocking states | never (always fresh)          |
| **FRONTIER**  | every open story by layer, stage cells + action verb               | `stories`, `layer_gates`                                        | never                         |
| **LEDGER**    | what moved + honest-state block (drift, deferrals, open questions) | `changelog`, `cost`                                             | never (the only dark section) |
| **SHAPE**     | architecture in space: modules, contracts, one main path           | `architecture`                                                  | structure change              |
| **LIFECYCLE** | the story state machine + the AEP workflow loop                    | fixed AEP vocabulary + story states                             | structure change              |
| **PRIMER**    | what this product is, for whom, why                                | `opportunity`, `product`                                        | era change                    |

Carried over from the SIBYL contract, unchanged in meaning:

- **Three tenses, encoded in ink solidity** — IS is unmarked (fact is the default);
  `GOAL` is a solid-outline chip that must carry its binding (a story or layer id);
  `EXP` is a dotted chip that must carry the event that settles it. Accents keep
  their sole meanings (needs-you; drift); tense never uses a third color.
- **Delta-gate** — a `<script type="application/json">` manifest records generated
  date, source commit, and per-section `gate` / `changed` / `stamp`. Gated sections
  that didn't change collapse to a one-line stamp in the owner read; the stamp joins
  NOW's quiet zone and stays reachable.
- **Dual reads** — default order serves the returning owner (NOW first);
  `?read=newcomer` reorders to PRIMER → SHAPE → LIFECYCLE first so the vocabulary is
  owned before the frontier uses it. The manifest's order arrays are the re-ordering
  feature.
- **Per-section self-legends** — every encoding (stage cells, tense chips, line
  styles) is defined where it is used; no section assumes memory of another.
- **Language** — one file in the repo's working language. One-file-per-language
  localization (SIBYL P3) is deferred until a consumer asks.

### D3 — Presentation stack (owner-directed)

- **WebGL fluid background** (guizang shader heritage), most visible in the top
  NOW/hero zone and subdued behind content sections, degrading to a CSS paper
  gradient when WebGL is unavailable. The SIBYL plainness concern is answered by
  keeping the wash data-free and subtle, not by removing it (owner direction:
  aesthetic quality is part of the surface's value).
- **Keyboard-driven navigation on a scroll page** — section-snap scrolling with
  ↑/↓, PageUp/PageDown, Home/End, and j/k; a fixed section-dot rail; `#section`
  anchors so any scroll position is shareable.
- **Diagrams: mermaid via CDN**, themed to the palette with dashed/dotted edge
  classes for GOAL/EXP (SIBYL's proven recipe, including the `run()` ordering and
  label-escaping fixes), with raw diagram text as the no-CDN fallback.
  `references/presentation.md` carries archify's layout law so mermaid output stays
  humane: one main path; ≤12 nodes or group into subgraphs; label only
  cross-boundary edges; detail goes to cards, not arrows (the OBS-1 guard).
- **No React.** archify demonstrates that focus, passport-style detail, and guided
  reading are achievable in vanilla JS over inline SVG/DOM; a framework adds a build
  step and breaks single-file delivery without adding capability we need.
- **three.js admitted only on a named trigger** — a real 3D presentation need (e.g.,
  a layer/module topology that a 2D diagram measurably fails to carry), not by
  default. Recorded so the door is neither open by default nor welded shut.
- **Degrade ladder** (every rung keeps the page readable offline): WebGL → CSS
  gradient; mermaid CDN blocked → raw text; font CDN blocked → system
  serif/mono/sans.

### D4 — Hybrid honesty model: facts derived, narrative authored, both labeled

The generation pipeline (each phase ends in a checkable postcondition, per the
deterministic-orchestration standard):

| Phase         | Action                                                                                                                                                                                                                                                   | Postcondition                                                              |
| ------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------- |
| 0 · Preflight | `product-context.yaml` exists (else point to `/aep-envision`); locate the newest existing brief by filename sort and read its manifest for the delta baseline                                                                                            | baseline commit known, or first-run declared                               |
| 1 · Derive    | extract **facts JSON** from `product-context.yaml` + git: story counts by state and layer, the needs-you list, changelog entries since baseline, layer-gate status, cost roll-up                                                                         | facts JSON embedded in the manifest; every fact names its YAML path        |
| 2 · Author    | fill `assets/template.html`: numbers and states come only from facts JSON; narrative (PRIMER, why-lines, LEDGER prose) is written fresh, tense-chipped, stamped with authored-at + source commit                                                         | every section rendered or stamped                                          |
| 3 · Audit     | run `references/checklist.md`: tense audit (every non-fact chipped), vocabulary audit (AEP canonical words only), class preflight (every class exists in the template), **number-provenance audit** (every number on the page ∈ facts JSON), glance test | all P0 checks pass                                                         |
| 4 · Deliver   | write `docs/human-alignment/brief-<date>T<time>Z-<shorthash>.html`; report the delta summary in-conversation                                                                                                                                             | new file exists; its name's hash equals repo HEAD; manifest stamps updated |

Why hybrid: OBS-5 (deterministic derivation is what humans learn to trust) plus
OBS-3 (unlabeled narrative rots). Rot is contained three ways: regeneration on every
invocation, authored-at stamps that make staleness visible, and the rule that the
agent never authors a number.

Derivation mechanics in v1 are agent-executed against an explicit field map
(`references/derivation.md` lists the exact YAML path for every fact), and the audit
re-verifies each number against the YAML. A bundled derive script is the recorded
upgrade path, adopted if drift between the field map and behavior is observed.

### D5 — Skill anatomy (lean-standard compliant)

```
skills/human-alignment/
├── SKILL.md              # steps + postconditions; target ≤ ~150 lines (R7 budget applies)
├── references/
│   ├── guideline.md      # the media-neutral contract: six sections, tenses, delta-gate, plainness law
│   ├── presentation.md   # layout & disclosure rules (archify-derived) + mermaid theming recipe
│   ├── derivation.md     # facts field map: YAML path → fact; delta computation
│   └── checklist.md      # the P0-graded audits (guizang discipline)
└── assets/
    └── template.html     # seed file; the only source of CSS classes; manifest scaffold;
                          # nav + WebGL + degrade JS prebuilt
```

- Description ≤ 300 characters; triggers around _project brief / project status
  one-pager / human alignment_; a routing-eval entry is added per R7's triggering
  check.
- Rationale lives in this document (R5); SKILL.md carries pointers only.
- The template is the class-name single source: authoring begins by reading its
  `<style>` block, never by inventing classes (the guizang preflight, promoted to a
  P0 checklist item).

### D6 — Release, acceptance, propagation

- **Implementation PR(s)** against this doc: scaffold the skill, author the
  template, add the marketplace entry and skills-index rows, add the routing-eval
  entry. Additive change → minor bump (v3.3.0); no other skill's step semantics
  change.
- **Acceptance** (layer-gate style, checkable):
  1. `/aep-human-alignment` against a fixture `product-context.yaml` produces a
     brief that passes every P0 checklist item;
  2. re-running with an unchanged YAML produces a new timestamped file whose
     SHAPE/LIFECYCLE/PRIMER collapse to stamps (delta-gate proof);
  3. a story-state edit in the YAML surfaces in NOW's delta band on the next run;
  4. the page stays readable with WebGL, mermaid CDN, and font CDN all blocked;
  5. the output filename's commit hash equals the repo HEAD at generation time, and
     the filename's timestamp matches the manifest's `generated` field.
- **Propagation**: visible downstream after the tag is cut and each of the 6
  consumer repos re-pins via the skills CLI. SIBYL adoption is a follow-up in that
  repo: replace its hand-authored Brief generation with the skill and flip its local
  docs to the upstreamed state per the existing convergence convention; its
  guideline's TUI-specific material stays project-local.

## Alternatives considered

- **Horizontal deck** (SIBYL Brief / guizang form) — rejected: a status surface is
  read at a glance and scanned by scroll; paging hides the whole. Deck-style
  presentation remains available by printing or presenting section-by-section.
- **Single-canvas console** (archify's native form) — rejected for v1: narrative
  sections (PRIMER, LEDGER) have no natural home on a canvas. The scroll page
  embeds the canvas _ideas_ (focus, one narrative, disclosure) per-section instead.
- **Pure deterministic generation** — rejected: PRIMER and why-narrative would be
  absent or wooden (OBS-3's stub problem, mirrored).
- **Agent-freeform snapshot** — rejected: numbers drift from the YAML the moment
  they are authored (OBS-3).
- **`skills/human-alignment/human-alignment/` category nesting** — rejected by the
  owner (duplicate directory); category grouping revisits when a second theory
  skill exists under the theme (see D1).
- **A single `brief.html` overwritten in place** — rejected by the owner: the
  filename must carry generation time and commit hash so a reader knows which
  commit a brief describes without opening it; overwriting also erases the record
  of past briefs. A stable `latest` pointer remains a recorded option (D2).
- **React / three.js now** — deferred with named triggers (D3).
- **Bundled derive script in v1** — deferred; adopt on observed field-map drift
  (D4).

## References

- SIBYL `docs/human-alignment/{guideline.md, observations.md, examples/sibyl-brief.html}`
  (2026-07-23 state) — the contract, the failure register, the working example.
- `guizang-ppt-skill` (dotfiles) — template-as-single-class-source, checklist
  discipline, single-file HTML deck substrate.
- [tt-a1i/archify](https://github.com/tt-a1i/archify) v2.12 — README, PRODUCT.md,
  DESIGN.md, `archify/SKILL.md`: evidence-console ethic, layout law, progressive
  disclosure, framework-free interactivity.
- [design-lens-rationale.md](design-lens-rationale.md) — the theory catalog this
  skill cross-references instead of duplicating.
- [skill-authoring-standard.md](skill-authoring-standard.md) — R1–R9; this skill is
  authored lean from birth.
- [deterministic-orchestration.md](deterministic-orchestration.md) — the
  postcondition style D4's pipeline follows.
- Affected on implementation: `skills/human-alignment/` (new),
  `.claude-plugin/marketplace.json`, root README skill catalog,
  `evals/skill-routing.json`.
