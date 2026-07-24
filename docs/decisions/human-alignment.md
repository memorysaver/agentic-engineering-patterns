# Human Alignment: A Project Brief Surface for AEP

> **Status:** Proposed. Decision doc only, per the decision-doc-first convention
> (PR #17, PR #24 precedent) — no schema, skill, or marketplace edits in this PR;
> implementation follows in a separate PR reviewed against this document.
>
> **Revision 2 (2026-07-24):** revised after a three-lens adversarial review
> (SIBYL-contract fidelity · guizang/archify integration · goal-achievement
> skepticism) and owner rulings on its findings. The review's load-bearing
> corrections: the attention-set and drift derivations are now framework-level
> specs (D7), fact derivation is a real script in v1 (D4), the plainness law's
> scope is split between content and canvas (D2/D3), and the delta baseline is a
> committed manifest rather than filename sort (D2).

## Problem

AEP's planning layer captures intent and state in `product-context.yaml` — stories,
layers, gates, architecture, cost, changelog — but has no human-facing rendering of
that state. The owner reads raw YAML, scrolls git history, or asks the agent, and
none of those answers the three questions a returning human actually has: _where are
we, what needs me, where did reality drift from intent?_ As agents do more of the
work, the human's cognitive model of the project decays fastest — and the planning
layer, which knows the answer, stays silent about it.

Two of those questions expose framework gaps, not just missing rendering: AEP has no
canonical definition of "needs a human" (the signals exist but are scattered across
six fields), and nothing in AEP computes intent-vs-reality divergence at all. D7
closes both as derived-view specs; this skill is their first consumer.

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
   as one graph is a machine artifact, not a human surface; **OBS-2** — a divergence
   detector that lists intentional deferrals is noise, and one that only compares
   status misses real structural divergence; **OBS-3** — hand-authored narrative rots
   when regeneration isn't wired to real state; **OBS-4** — a control plane can be
   semantically incoherent and still pass validation unless coherence is checked;
   **OBS-5** — the deterministically derived artifacts are the only ones the human
   learned to trust.
2. **guizang-ppt-skill** — the _authoring discipline_ for single-file HTML surfaces:
   `assets/template.html` is the only source of CSS class names (preflight before
   writing any section), references are split by concern (layouts / components /
   themes / checklist), and delivery requires passing a P0-graded self-check.
3. **archify** (tt-a1i/archify v2.12) — the _presentation ethic_ and the
   _verification architecture_: an "evidence console" where every rendered fact
   derives from authored or verified evidence; one spatial narrative first;
   progressive disclosure over permanent chrome; detail in cards rather than edges;
   typed JSON validated by standalone schema validators; machine-checkable layout
   gates; structured repair receipts (stable code · subject · evidence ·
   supportedFixes) with a bounded correction loop; delivery receipts (SHA-256).
   Notably, archify's rich interactivity (focus, semantic passport, guided views) is
   vanilla JS over inline SVG — no framework — and archify achieves humane layout
   through its _gates_, not through guidance prose; its DESIGN.md names "generic
   Mermaid beautifiers" as its first anti-reference. D3 confronts that tension
   explicitly.

The design was then hardened by a three-reviewer adversarial pass (2026-07-24)
whose confirmed findings are folded into D2–D7 below and into Alternatives.

## Decisions

### D1 — A standalone top-level skill: `skills/human-alignment/`

The skill lives at `skills/human-alignment/` — a peer of the four existing
categories, not nested inside one — and syncs as **`/aep-human-alignment`**.

- Human cognitive alignment is a functional theme of its own (owner direction), not
  a phase of the planning loop; the flat path also keeps the invocation name the
  owner specified without a `human-alignment/human-alignment/` duplicate.
- **v1 is manually invoked** (owner ruling: development-efficiency first; the skill
  stands alone before it integrates). The owner runs it when a fresh brief is
  wanted. Wiring regeneration into `/aep-wrap` postconditions or the autopilot tick
  is recorded in Horizon, not built now. Staleness stays visible cheaply: the
  filename, the page header, and the in-conversation summary all carry the
  generation commit, and the summary states how many commits HEAD has moved since.
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

One vertical-scroll HTML page per run — self-contained except the fonts and mermaid
CDNs, each with an offline degrade rung (D3) — written to
`docs/human-alignment/brief-<YYYY-MM-DD>T<HHMM>Z-<shorthash>.html`, e.g.
`brief-2026-07-24T0730Z-96a63f7.html`. The filename carries the generation time
(UTC, no colons, lexicographically sortable) and the git commit the brief was
generated at, so provenance is visible without opening the file (owner direction).
Briefs are committed under `docs/` and accumulate as a reviewable record; pruning
old briefs is the owner's choice. Delivery is an in-conversation notice (path +
delta summary) — the owner and newcomers open the file themselves, and the file
doubles as the thing to hand to anyone asking "what is this project?" (owner
ruling).

**The delta baseline is a committed ledger, not filename sort.** A small
`docs/human-alignment/manifest.json` records the latest generation: timestamp,
source commit, output filename, per-section gate stamps, and the artifact's content
SHA-256. Phase 0 reads this file; because it is branch-tracked, every branch carries
its own correct baseline, and pruning or rebasing brief files never corrupts the
delta. (Each brief also embeds its own manifest for self-description; the committed
`manifest.json` is the generation ledger.)

**Scope of the plainness law (owner ruling).** SIBYL's law — every element must
carry a meaning a human needs — governs the **content layer**: words, diagrams,
chips, cells, numbers, legends. The **canvas layer** (the WebGL wash, D3) is an
explicitly scoped aesthetic surface: data-free, subtle, degradable, and allowed to
be beautiful. This is a conscious adaptation of the SIBYL contract, not an
oversight; the one binding the wash does keep is tonal — it darkens over the record
section, so the canvas follows the live/record split rather than decorating at
random.

Six sections, ported from SIBYL's contract with `product-context.yaml` as the data
source (attention set and drift facts are defined in D7):

| Section       | Job                                                                | Derived from                                     | Altitude              | Regeneration gate             |
| ------------- | ------------------------------------------------------------------ | ------------------------------------------------ | --------------------- | ----------------------------- |
| **NOW**       | since-you-last-looked delta band · the one ask · quiet zone        | story-state diff vs. baseline; the attention set | daily                 | never (always fresh)          |
| **FRONTIER**  | every open story by layer, stage cells + action verb               | `stories`, `layer_gates`                         | daily · both          | never                         |
| **LEDGER**    | what moved + honest-state block (drift, deferrals, open questions) | `changelog`, `cost`, the drift facts             | daily                 | never (the only dark section) |
| **SHAPE**     | architecture in space: modules, contracts, one main path           | `architecture` (+ optional reality probe, D4)    | newcomer · reference  | structure change              |
| **LIFECYCLE** | the story state machine + the AEP workflow loop                    | the canonical vocabulary below                   | newcomer · vocabulary | structure change              |
| **PRIMER**    | what this product is, for whom, why                                | `opportunity`, `product`                         | newcomer              | era change                    |

**The canonical vocabulary (P4 as a mechanism, not a citation).** SIBYL's
architecture-as-vocabulary principle requires a _closed_ word set whose only
definitions are the two LIFECYCLE diagrams. AEP's set is:

- **Parts (8):** `ENVISION · MAP · DISPATCH · BUILD · WRAP · REFLECT` (the loop
  verbs), `CONTEXT` (`product-context.yaml`, the hub every verb reads or writes),
  `VIEW` (this brief, derived from CONTEXT and nothing else).
- **Stages (5):** `PENDING → READY → IN_PROGRESS → IN_REVIEW → COMPLETED` — the
  story state machine's happy path, which is exactly the five-cell stage grammar on
  every NOW/FRONTIER row. The exception states `failed · blocked · deferred` are
  markers on a cell (accent + word), not stages: a story is _at_ a stage and _in_
  an exception, and collapsing those loses information.

The vocabulary audit (D4 Phase 3) checks that no part or stage is named by any word
outside this set.

Carried over from the SIBYL contract, unchanged in meaning:

- **Three tenses, encoded in ink solidity** — IS is unmarked (fact is the default);
  `GOAL` is a solid-outline chip that must carry its binding (a story or layer id);
  `EXP` is a dotted chip that must carry the event that settles it. Accents keep
  their sole meanings (needs-you; drift); tense never uses a third color. The
  builder's rules transfer whole, including the two the honesty meter depends on:
  **never chip a fact** (the page-level read — how much of this screen is hollow —
  is the trust gauge, and dilution kills it; an IS chip is legal only to ground a
  fact inside an aspirational sentence) and **one chip governs one clause**, never
  a paragraph.
- **Delta-gate** — the manifest records per-section `gate` / `changed` / `stamp`.
  Gated sections that didn't change collapse to a one-line stamp in the owner read;
  the stamp joins NOW's quiet zone and stays reachable.
- **Dual reads** — default order serves the returning owner (NOW first);
  `?read=newcomer` reorders to PRIMER → SHAPE → LIFECYCLE first so the vocabulary is
  owned before the frontier uses it. The manifest's order arrays are the re-ordering
  feature.
- **Per-section self-legends** — every encoding (stage cells, tense chips, line
  styles) is defined where it is used; no section assumes memory of another.
- **Language** — one file in the repo's working language. One-file-per-language
  localization (SIBYL P3) is deferred until a consumer asks.

### D3 — Presentation stack (owner-directed)

- **WebGL fluid background** (guizang shader heritage) as the canvas layer defined
  in D2: most visible in the top NOW zone, subdued behind content, tonally bound to
  the live/record split (darkens over LEDGER), degrading to a CSS paper gradient
  when WebGL is unavailable. Visual beauty is the owner's stated requirement for
  this layer; the plainness law governs the content layer only.
- **Keyboard navigation on a free-scrolling page** — ↑/↓, PageUp/PageDown, j/k,
  Home/End jump smoothly between sections; a fixed section-dot rail and `#section`
  anchors make any position shareable. No CSS scroll-snap lock: the page must stay
  scannable as a whole (the reason deck paging was rejected), so keys _jump_, they
  do not _page_. A slim sticky state rail (current layer · open count · needs-you
  count) keeps the glance overview present at any scroll depth.
- **Diagrams: mermaid via CDN, with honest boundaries.** Mermaid is the owner's
  chosen renderer. What `references/presentation.md` provides is **author-side
  guidance, not archify's machine validation** — mermaid's auto-layout owns
  geometry, so edge crossings and edge-through-node cannot be mechanically
  prevented, only bounded and inspected:
  - hard size bounds per rendered graph — ≤ 12 nodes, and ≤ 8 nodes in any single
    subgraph; a graph that wants more is decomposed into multiple diagrams (the
    OBS-1 guard, applied per rendered graph so subgraph grouping cannot silently
    rebuild a hairball);
  - labels only on cross-boundary or non-obvious edges; detail goes to cards;
  - a human **glance gate** in the checklist: no edge crossing on the main path, no
    edge passing through an unrelated node — a soft check, named as such;
  - SIBYL's proven theming recipe (palette-bound theme variables, dashed/dotted
    edge classes for GOAL/EXP, the `run()` ordering and label-escaping fixes);
  - fallback when the CDN is blocked: the diagram section renders its pre-formatted
    source text with a caption naming the degradation — legible, honestly labeled,
    but not claimed to be a human surface;
  - **upgrade path**: if glance-gate failures recur, SHAPE/LIFECYCLE rendering
    moves to cross-invoking archify (validated inline SVG, native focus/guided
    views); recorded so the trigger is observable, not aspirational.
- **Typography** — three roles, stated: serif for section heads and counts; sans
  for body; mono for kickers, chips, ids, and feet; identifiers are mono, never
  italic. This adopts SIBYL's editorial/instrument split and consciously deviates
  from archify's mono-only One Voice rule (the brief is half narrative, half
  instrument).
- **Palette** — the template's `:root` block is the single palette source: paper +
  ink, two accents with fixed meanings (sienna = needs-you, olive = drift), and the
  wash colors derived from the same variables. No per-run or per-repo palette
  variation (the guizang preset-lock, narrowed to one canonical palette; archify's
  rule that saturated color is meaning, never decoration).
- **No React.** archify demonstrates that focus, passport-style detail, and guided
  reading are achievable in vanilla JS over inline SVG/DOM; a framework adds a build
  step and breaks single-file delivery without adding capability we need.
- **three.js admitted only on a named trigger** — a real 3D presentation need (e.g.,
  a layer/module topology that a 2D diagram measurably fails to carry), not by
  default. Recorded so the door is neither open by default nor welded shut.
- **Degrade ladder**: WebGL → CSS gradient; mermaid CDN blocked → labeled source
  text (see above); font CDN blocked → system serif/mono/sans. Every rung keeps the
  page legible offline; the diagram rung is legible but degraded, and says so.

### D4 — Hybrid honesty model: facts derived by code, narrative authored by agent, both labeled

The generation pipeline (each phase ends in a checkable postcondition, per the
deterministic-orchestration standard):

| Phase         | Action                                                                                                                                                                                                                                                                                                                                               | Postcondition                                                                              |
| ------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------ |
| 0 · Preflight | `product-context.yaml` exists (else point to `/aep-envision`); read `docs/human-alignment/manifest.json` for the delta baseline                                                                                                                                                                                                                      | baseline commit known, or first-run declared                                               |
| 1 · Derive    | run `scripts/derive.mjs`: extract **facts JSON** from `product-context.yaml` + git — story counts by state and layer, the attention set (D7), the drift facts (D7), changelog entries since baseline, layer-gate status, cost roll-up — and validate it against `facts.schema.json`                                                                  | facts JSON exists and validates; every fact names its YAML path                            |
| 1.5 · Probe   | _optional_ — architecture-reality probe: derive the actual import graph (madge/ts-morph/LSP-class tooling, per repo toolchain) and diff it against declared `architecture`; results join the drift facts, revision-pinned to HEAD                                                                                                                    | probe ran and SHAPE is marked `code-verified`, or probe skipped and SHAPE is `unverified`  |
| 2 · Author    | fill `assets/template.html`: numbers and states come only from facts JSON; narrative (PRIMER, why-lines, LEDGER prose) is written fresh, tense-chipped, stamped with authored-at + source commit; narrative obeys the evidence-language rule below                                                                                                   | every section rendered or stamped                                                          |
| 3 · Audit     | run `scripts/audit.mjs` for the mechanical checks (number-provenance: every number on the page ∈ facts JSON; class preflight; chip-grammar: every non-fact chipped, no fact chipped, one chip per clause) plus the judgment checks from `references/checklist.md` (vocabulary audit against the D2 closed set, evidence-language audit, glance gate) | audit passes; failures emit structured receipts; at most two correction rounds             |
| 4 · Deliver   | write `docs/human-alignment/brief-<date>T<time>Z-<shorthash>.html`; update `manifest.json` (generation record + content SHA-256); report the delta summary + path in-conversation                                                                                                                                                                    | new file exists; its name's hash equals repo HEAD; `manifest.json` digest matches the file |

- **Why the derive script is v1, not deferred** (owner ruling: faithful
  representation): OBS-5's trust came from _code-derived_ artifacts, and AEP's own
  verification-economics work (v3.1.0) requires evaluator independence — an agent
  auditing numbers it extracted itself is neither. The countable facts are
  trivially scriptable; the word "deterministic" is used only where a script did
  the work.
- **Structured repair receipts** (archify's contract): an audit failure names a
  stable `code`, the `subject` (element/selector), the measured `evidence`, and the
  `supportedFixes`; the authoring agent applies a listed fix and re-runs, never
  guesses, and never exceeds **two** correction rounds — a third failure is
  reported, not silently retried.
- **Evidence-language rule** (archify's discipline, applied to prose): narrative
  may not assert causality or impact — _blocks, breaks, guarantees, unblocks,
  proves_ — without citing a fact id from facts JSON. Tense chips say _when_ a
  claim holds; this rule says _whether it may be claimed at all_.
- Why hybrid: OBS-5 (deterministic derivation is what humans learn to trust) plus
  OBS-3 (unlabeled narrative rots). Rot is contained three ways: regeneration on
  every invocation, authored-at stamps that make staleness visible, and the rule
  that the agent never authors a number.

### D5 — Skill anatomy (lean-standard compliant)

```
skills/human-alignment/
├── SKILL.md              # steps + postconditions; target ≤ ~150 lines (R7 budget applies)
├── references/
│   ├── guideline.md      # the media-neutral contract: six sections, tenses (full builder's
│   │                     # rules), delta-gate, plainness law + its content/canvas scope,
│   │                     # canonical vocabulary, typography roles
│   ├── presentation.md   # author-side layout guidance (bounds, decomposition, labels,
│   │                     # cards) + mermaid theming recipe + glance-gate definition
│   ├── derivation.md     # facts field map: YAML path → fact; delta computation; probe notes
│   └── checklist.md      # the P0-graded audits (guizang discipline) + receipt format
├── scripts/
│   ├── derive.mjs        # deterministic facts extraction (yaml + git) → facts JSON
│   ├── audit.mjs         # independent mechanical audit (provenance, classes, chip grammar)
│   └── facts.schema.json # the typed contract between derive, author, and audit
└── assets/
    └── template.html     # seed file; the only source of CSS classes AND the palette
                          # (:root); manifest scaffold; nav + WebGL + degrade JS prebuilt
```

- The **attention-set and drift derivation specs live in
  `skills/product-context/_shared/references/`** (D7) — they are framework
  vocabulary with multiple future consumers, not skill-private logic;
  `derive.mjs` implements them and `derivation.md` points to them.
- Description ≤ 300 characters; triggers around _project brief / project status
  one-pager / human alignment_; a routing-eval entry is added per R7's triggering
  check.
- Rationale lives in this document (R5); SKILL.md carries pointers only.
- The template is the class-name single source: authoring begins by reading its
  `<style>` block, never by inventing classes (the guizang preflight, promoted to a
  P0 checklist item).

### D6 — Release, acceptance, propagation

- **Implementation PR(s)** against this doc: scaffold the skill (including
  `scripts/` + schema), author the template, write the two `_shared` specs (D7),
  add the marketplace entry and skills-index rows, add the routing-eval entry.
  Additive change → minor bump (v3.3.0); no other skill's step semantics change
  (the `/aep-validate` coherence rule from D7 is a separate follow-up PR).
- **Acceptance** (layer-gate style, checkable):
  1. against a fixture `product-context.yaml` containing known attention signals
     (one `failed` story + one pending `amendment_log` entry), the brief passes
     every P0 checklist item **and NOW renders the top-priority signal as the one
     ask with its action verb** — NOW provably non-empty;
  2. re-running with an unchanged YAML produces a new timestamped file whose
     SHAPE/LIFECYCLE/PRIMER collapse to stamps (delta-gate proof);
  3. a story-state edit in the YAML surfaces in NOW's delta band on the next run;
  4. with WebGL, mermaid CDN, and font CDN all blocked, the page stays legible and
     the diagram sections show their labeled source-text fallback;
  5. the output filename's commit hash equals repo HEAD at generation, the
     filename's timestamp matches the manifest's `generated` field, and
     `manifest.json`'s content SHA-256 matches the delivered file;
  6. facts JSON validates against `facts.schema.json`, and `audit.mjs` passes when
     run standalone (independent of the authoring agent).
- **Propagation**: visible downstream after the tag is cut and each of the 6
  consumer repos re-pins via the skills CLI. SIBYL adoption is a follow-up in that
  repo: replace its hand-authored Brief generation with the skill and flip its local
  docs to the upstreamed state per the existing convergence convention; its
  guideline's TUI-specific material stays project-local.

### D7 — Framework specs: the attention set and the drift facts

The review's deepest finding: the brief cannot render what the framework does not
represent. Two derived-view specs close the gaps. Both are **specs plus a reference
implementation in `derive.mjs`** — no schema fields, no new story states.

**The attention set** — the canonical answer to "what needs a human", as a spec in
`skills/product-context/_shared/references/attention-set.md`:

- The signal predicates, each with its YAML path and its action verb:
  `stories[].status == failed` (→ `reset ▸`; only a human may run
  `failed → pending`), `stories[].status == in_review` where `skip_human_eval`
  does not waive it (→ `review ▸`), `architecture.amendment_log[].status ==
pending` (→ `approve ▸`), object-map `status == draft` (→ `/aep-model ▸`),
  pending `.5` calibration checkpoints (→ `/aep-calibrate ▸`),
  `product.open_questions[]` (→ `answer ▸`).
- A deterministic priority order for choosing **the one ask** (NOW shows one;
  the rest join the quiet zone): failed > amendment pending > in_review >
  calibration > draft object-map > open questions; ties break by layer order then
  id. Two runs at the same commit must choose the same ask.
- Why a derived view and not a stored field or new state: a story state describes
  lifecycle position, not human duty (a `failed` story is failed _and_ needs a
  human — one enum cannot carry both), half the signals are not stories at all,
  and a stored copy of a derivable truth is a second source that validation will
  not catch when it drifts (the OBS-4 shape, and AEP's #1 bug class).
- Consumers: this skill now; `/aep-autopilot` escalation and `/aep-watch` alerting
  re-point to the same spec in later PRs (recorded in Horizon; zero behavior change
  in this round).

**The drift facts** — the canonical answer to "where did reality drift", as a spec
in `skills/product-context/_shared/references/drift-facts.md`. Every LEDGER drift
row must cite a derived fact; **hand-authored drift is banned** (OBS-2: underived
drift claims are noise or misses, and both spend the surface's credibility — when
nothing derives, the row is silent, not fabricated). The v1 derivations:

1. **Intent without evidence** — `layer_gates[].coverage.uncovered`: declared
   acceptance criteria no evidence covers.
2. **Plan behind the architecture** — `architecture.amendment_log[]` pending:
   stories were mapped against a structure that has since been amended.
3. **Reality resisting intent** — `failure_logs`: a story that repeatedly fails is
   evidence the spec and the code disagree.
4. **Control-plane incoherence** — layer↔story-state disagreement (e.g., completed
   stories in an unopened layer): the OBS-4 class. Follow-up (separate PR): the
   same check joins `/aep-validate` so incoherence cannot silently pass again —
   SIBYL's own OBS-4 disposition, applied to AEP.
5. **Declared vs. actual architecture** (optional, Phase 1.5): the import-graph
   probe — the strongest drift form ("the YAML says A does not depend on B; the
   code says it does"), and the exact move that earned OBS-5's trust in SIBYL
   (its architecture graph was the one artifact derived from real imports).
   Toolchain-dependent, so it degrades honestly: no tool → SHAPE marked
   `unverified`, never blocked.

## Horizon (recorded, not built)

- **Workflow 2 — comprehension check** (owner request): a second skill workflow
  that tests the reader — can they answer the three questions from the brief within
  a time bound, faster than from raw YAML? Turns the acceptance suite from
  mechanism-proof into goal-proof.
- **Loop integration**: `/aep-wrap` postcondition regenerates the brief; autopilot
  tick keeps it fresh; staleness guard escalates instead of merely reporting.
- **Attention-set consumers**: `/aep-autopilot` escalation and `/aep-watch`
  alerting re-point to the D7 spec.
- **archify cross-invocation** for SHAPE/LIFECYCLE when the glance gate recurs
  (trigger defined in D3).
- **Full architecture-reality probe**: LSP-grade scanning across the 6 consumer
  repos' toolchains.
- **TUI summary render** of NOW (the terminal-native owner's glance surface);
  one-file-per-language localization; a stable `latest` pointer if a fixed URL is
  ever needed.

## Alternatives considered

- **Horizontal deck** (SIBYL Brief / guizang form) — rejected: a status surface is
  read at a glance and scanned by scroll; paging hides the whole. Deck-style
  presentation remains available by printing or presenting section-by-section.
- **CSS scroll-snap on the scroll page** — rejected for the same reason paging was:
  snap re-creates pages. Keys jump; the page scrolls free.
- **Single-canvas console** (archify's native form) — rejected for v1: narrative
  sections (PRIMER, LEDGER) have no natural home on a canvas. The scroll page
  embeds the canvas _ideas_ (one narrative, bounded diagrams, disclosure)
  per-section instead.
- **Pure deterministic generation** — rejected: PRIMER and why-narrative would be
  absent or wooden (OBS-3's stub problem, mirrored).
- **Agent-freeform snapshot** — rejected: numbers drift from the YAML the moment
  they are authored (OBS-3).
- **Agent-executed fact derivation in v1** (the original D4) — rejected in revision
  2: the extractor auditing its own extraction violates evaluator independence
  (verification-economics v3.1.0) and borrows OBS-5's credibility without earning
  it; the script is small and ships in v1.
- **A new `awaiting_human` story state / a stored `needs_attention` field** —
  rejected (D7): lifecycle states cannot carry cross-cutting duty, half the signals
  are not stories, and stored copies of derivable truth drift silently.
- **Hand-authored drift rows** — rejected (D7, OBS-2): underived drift is noise or
  a miss; silence over fabrication.
- **`skills/human-alignment/human-alignment/` category nesting** — rejected by the
  owner (duplicate directory); category grouping revisits when a second theory
  skill exists under the theme (see D1).
- **A single `brief.html` overwritten in place** — rejected by the owner: the
  filename must carry generation time and commit hash so a reader knows which
  commit a brief describes without opening it; overwriting also erases the record
  of past briefs. A stable `latest` pointer remains a Horizon option.
- **React / three.js now** — deferred with named triggers (D3).

## References

- SIBYL `docs/human-alignment/{guideline.md, observations.md, examples/sibyl-brief.html}`
  (2026-07-23 state) — the contract, the failure register, the working example.
- `guizang-ppt-skill` (dotfiles) — template-as-single-class-source, checklist
  discipline, single-file HTML deck substrate.
- [tt-a1i/archify](https://github.com/tt-a1i/archify) v2.12 — README, PRODUCT.md,
  DESIGN.md, `archify/SKILL.md`: evidence-console ethic, typed-IR validation,
  repair receipts, layout gates, delivery receipts, framework-free interactivity.
- [design-lens-rationale.md](design-lens-rationale.md) — the theory catalog this
  skill cross-references instead of duplicating.
- [skill-authoring-standard.md](skill-authoring-standard.md) — R1–R9; this skill is
  authored lean from birth.
- [deterministic-orchestration.md](deterministic-orchestration.md) — the
  postcondition style D4's pipeline follows.
- [verification-economics.md](verification-economics.md) — evaluator independence
  (why `audit.mjs` is not the authoring agent) and tamper-evident evidence (why
  `manifest.json` carries a content SHA-256).
- Affected on implementation: `skills/human-alignment/` (new),
  `skills/product-context/_shared/references/{attention-set.md, drift-facts.md}`
  (new), `.claude-plugin/marketplace.json`, root README skill catalog,
  `evals/skill-routing.json`; follow-up PR: the `/aep-validate` coherence rule
  (D7.4).
