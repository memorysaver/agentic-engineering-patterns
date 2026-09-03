# Human Alignment: A Project Brief Surface for AEP

> **Status:** Accepted — implemented in **v3.3.0** (PR #29; v3.3.1 gated the brief
> behind an explicit ask). Authored decision-doc-first (PR #28, no schema, skill, or
> marketplace edits); the implementation was reviewed against this document.
>
> **Revision 2 (2026-07-24):** revised after a three-lens adversarial review
> (SIBYL-contract fidelity · guizang/archify integration · goal-achievement
> skepticism) and owner rulings on its findings. The review's load-bearing
> corrections: the attention-set and drift derivations are now framework-level
> specs (D7), fact derivation is a real script in v1 (D4), the plainness law's
> scope is split between content and canvas (D2/D3), and the delta baseline is a
> committed manifest rather than filename sort (D2).
>
> **Revision 3 (2026-07-24):** after a full-pipeline simulation against a real
> consumer (looplia, 395 stories) and the owner's readability verdict on the
> result. Two changes: the **cold-reader contract** replaces the warm-context
> reader model — every surfaced item speaks plain language first with the system
> identifier demoted to a provenance anchor (D2, D4) — and five derivation-spec
> corrections the simulation surfaced are folded into D7. The simulation evidence
> is recorded at the end of D7.
>
> **Revision 4 (2026-07-24):** owner ruling on reader modes — the `?read=`
> mechanism is removed. One document, one fixed order: an **audience-depth
> pyramid** of four bands (everyone → product user → product manager → engineer),
> where role adaptation is scroll depth and disclosure, not a URL parameter (D2).
> Band 2 (the product-user view) is net-new content derived from passed layer
> gates and the changelog.
>
> **Revision 5 (2026-07-24):** owner ruling on the engineering band — D3's
> deferred archify upgrade path activates now. Band 4's architecture view renders
> through **archify cross-invocation** (typed IR → schema + layout gates + repair
> receipts → delivered companion artifact, embedded in the brief); mermaid remains
> only for the LIFECYCLE vocabulary mini-diagrams. Proven live in the looplia
> simulation (evidence in D7).
>
> **Revision 6 (2026-07-25):** three owner rulings, all proven in the simulation
> before writing. (a) Bands are named by **scope, not audience**: Overview ·
> Product · Project · Engineering. (b) The architecture view's topology comes from
> a **deterministic code pipeline** (workspace-graph scanner → auditable rules
> R1–R10 → mechanical receipt consumer → archify), rendered at three tiers
> (domain overview · full package graph · declared narrative); code is the source
> of truth for edges, the YAML for meaning, and their gap is a drift fact —
> `modules[].paths` is recommended as the schema binding that would let them
> reconcile. (c) The brief is **one file**: every archify artifact embeds via
> `srcdoc` (five diagrams — architecture ×3, workflow ×1, lifecycle ×1), and the
> mermaid CDN dependency is eliminated from the primary path.
>
> **Revision 7 (2026-07-25):** a review pass re-derived every simulation number
> from the looplia repo and the surviving artifacts, and found three derivation
> specs that name fields the real consumer never populates. Corrections: drift 1
> derives from the coverage counters, not `coverage.uncovered` (which is a
> `/aep-build` worklist, empty precisely when the gate is `not_started`); schema
> tolerance is **field-level**, not section-level (looplia's `amendment_log`
> entries carry no `status`, and `calibration.plan` has no `status` field
> upstream either, so that predicate is respecified as plan-minus-history); band
> 2 admits `passed` gates as fact and `scripted_passed` only under an EXP chip;
> the number-provenance audit requires declarative `data-fact` bindings so it is
> statically checkable; the architecture pipeline gets files in D5; retention is
> ruled (keep the latest 3); and the revision-6 rulings (scope names, mermaid off
> the primary path) are propagated to every section that still contradicted them.

> **Revision 8 (2026-07-25):** owner rulings after the implementation run, on
> what the Engineering band is _for_. (a) The band is **prospective and
> structural**, not a deeper cut of progress — an engineer needs the current
> structure and _what the next design does to it_, and "where work happened /
> what it cost / where it failed" is Project's depth, not Engineering's. Its
> spine is **Now · Concepts · Next · Options** (D8). (b) The ontology is no
> longer a deployment taxonomy: nodes are **concept modules measured against the
> code units that carry them**, because `stories[].module` × `files_affected`
> makes that binding derivable today. (c) Suggestions are **allowed and
> welcome**, but only as a **Design Option Set** with a derived trigger, ≥3
> options including "leave it as is", per-option cost/benefit in the project's
> own measured terms, a design sketch, and a stated ranking criterion — a bare
> one-liner recommendation is banned (D8). (d) Trigger thresholds are **fixed in
> the spec**, not configurable. (e) The mining principle: when deterministic
> signal looks insufficient, **dig further into what the project actually
> records** — do not concede the ground to agent judgment. Revision 7's own
> implementation violated this three times (D7).

> **Revision 9 (2026-07-25):** after an independent generator/evaluator pass on
> the generated brief returned **FAIL**. Nine content defects, and all but one
> share a single shape: **the prose needed a fact the derivation had not
> produced, so the authoring agent supplied it from a diagram label, from
> ambient knowledge, or by counting manually — and the audit could not see it,
> because its unit of check was the digit rather than the claim.** The root
> cause is that the facts plane was designed top-down (what should the brief
> show?) against a source carrying **483 populated key paths**, of which the
> derivation read about **thirty**. Revision 9 replaces hole-by-hole patching
> with four mechanisms that make the gap visible and the omission illegal (D9):
> a **source census**, **claims that bind** rather than numbers that bind,
> facts that carry **predicates** rather than raw fields, and tools that
> **declare their own coverage**. Owner rulings: the census classifies at
> **path-template level**, and an `ignored` entry must carry a **reason**, not
> a checkmark.

> **Revision 10 (2026-07-26):** after two independent reviews of the design
> itself and a re-measurement of the velocity premise revision 9 rested on.
>
> **Scope note, because revisions 7–9 blurred it.** This revision is about **the
> surface's own design**. The reference consumer's plan-file hygiene — stale
> calibration entries, gates never flipped, an unwired cost roll-up, undeclared
> modules — is that consumer's business and `/aep-validate`'s. Those findings are
> evidence about _what this surface does and does not make visible_; they are not
> requirements on it, and treating them as such is how a rendering skill grew a
> detector suite.
>
> Three rulings: the unit of delivery is the **clock, not the page** (D10); the
> skill **renders and does not detect** (D11); and it **never keeps a private
> store** (D12). Plus corrections to D7 and D9 that are the design's own, not the
> consumer's.

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

One vertical-scroll HTML page per run — self-contained except the font CDN, which
has an offline degrade rung (D3) — written to
`docs/human-alignment/brief-<YYYY-MM-DD>T<HHMM>Z-<shorthash>.html`, e.g.
`brief-2026-07-24T0730Z-96a63f7.html`. The filename carries the generation time
(UTC, no colons, lexicographically sortable) and the git commit the brief was
generated at, so provenance is visible without opening the file (owner direction).
Delivery is an in-conversation notice (path +
delta summary) — the owner and newcomers open the file themselves, and the file
doubles as the thing to hand to anyone asking "what is this project?" (owner
ruling).

**The delta baseline is a committed ledger, not filename sort.** A small
`docs/human-alignment/manifest.json` records the latest generation: timestamp,
source commit, output filename, per-section gate stamps, and the artifact's content
SHA-256 — plus an append-only `history[]` of the same record for every past
generation, so the ledger outlives the files it describes. Phase 0 reads this file;
because it is branch-tracked, every branch carries its own correct baseline, and
pruning or rebasing brief files never corrupts the delta. (Each brief also embeds
its own manifest for self-description; the committed `manifest.json` is the
generation ledger.)

**Retention: the latest three, pruned by the tool (owner ruling, revision 7).**
Briefs are committed under `docs/` — the file must be openable straight from the
repo for the hand-it-to-someone use above — but they are **not** left to
accumulate. Measured cost is ~3 MB per brief (D3: five embedded archify
runtimes), so Phase 4 deletes every brief beyond the newest **three** as part of
delivery; the count is an invocation parameter for owners who want a longer tail.
The record of pruned generations survives in `manifest.json`'s `history[]`
(timestamp · commit · filename · content SHA-256), which is what a reader
actually needs to answer "which brief described commit X" — the 3 MB body is not.
This resolves D3's open caveat: the prune policy is real and mechanical, so
briefs stay in git rather than moving behind `.gitignore`.

**Scope of the plainness law (owner ruling).** SIBYL's law — every element must
carry a meaning a human needs — governs the **content layer**: words, diagrams,
chips, cells, numbers, legends. The **canvas layer** (the WebGL wash, D3) is an
explicitly scoped aesthetic surface: data-free, subtle, degradable, and allowed to
be beautiful. This is a conscious adaptation of the SIBYL contract, not an
oversight; the one binding the wash does keep is tonal — it darkens over the record
section, so the canvas follows the live/record split rather than decorating at
random.

**The cold-reader contract (owner ruling, revision 3).** SIBYL's guideline wrote
for a warm reader — the owner who reads the surface a hundred times and owns its
vocabulary. The simulation proved that reader does not exist: the real owner runs
many repos, returns from days away, and meets every brief cold. So the reader
model is: **every read is a first read.** Two consequences:

- **Two channels, inverted hierarchy.** Every surfaced item is a plain-language
  sentence first — _what happened, to what, and why the reader cares_ — with the
  system identifier (story id, YAML path, SHA, PR number) demoted to a small mono
  **provenance anchor** under it. System names are citations, not prose. SIBYL's
  "use the system's own name" survives in the anchor channel; the prose channel
  translates.
- **A vocabulary budget.** The prose channel may use at most **seven** system
  words undefined (Miller's bound); each is defined in one clause at first use,
  not in a legend three screens away. Everything else — epoch, wave, SHA, PR
  numbers, module ids — lives only in anchors. The diagram/cell channel keeps the
  closed canonical set below; the budget governs prose.

**The audience-depth pyramid (owner ruling, revision 4; scope names, revision
6).** There is exactly one document in one fixed order; the `?read=` mechanism
and the manifest's order arrays are removed. Role adaptation is **scroll depth
plus disclosure**: everyone enters at the same top, and each band below is deeper
and more technical — the inverted pyramid. Stopping early is the feature: a
stakeholder who reads only the first two bands has read a complete, honest
surface, so no separate stakeholder mode is needed. Bands are named by **scope,
not audience** (owner ruling: role names make readers self-exclude; scope names
only classify depth): **Overview · Product · Project · Engineering**.

| Band                | Audience                 | Content (block re-homed from)                                                                                                                                                                                                                                                                                                                             | Derived from                                                                                                                                                                                         | Regeneration gate                                    |
| ------------------- | ------------------------ | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------- |
| **1 · Overview**    | every role, 30 seconds   | what this project is, one paragraph (PRIMER's essence) · health + progress in one sentence · the one ask + the while-you-were-away narrative (NOW)                                                                                                                                                                                                        | identity fields; the attention set; state diff vs. baseline                                                                                                                                          | identity: era · rest: never                          |
| **2 · Product**     | whoever uses the product | what works today (shipped, user-visible capabilities) · what recently changed for users · what the current layer will add, in user language                                                                                                                                                                                                               | **net-new derivation**: `passed` `layer_gates` + changelog, translated to user-visible outcomes (authored + anchored, D4 rules); `scripted_passed` gates admitted only under an EXP chip (see below) | capability list: gate reaches `passed` · rest: never |
| **3 · Project**     | plan and progress        | the full layer strip + current-layer story rows (FRONTIER) · queued layers, one sentence each + disclosure · what moved, where reality drifted, cost (LEDGER's plain layer)                                                                                                                                                                               | `stories`, `layer_gates`, `changelog`, the drift facts                                                                                                                                               | never (the LEDGER block stays the dark record)       |
| **4 · Engineering** | the system itself        | **Now** the real code structure · **Concepts** the module vocabulary measured onto it · **Next** the queued design projected onto both, and what it needs that does not exist · **Options** only when a D8 trigger fires · the loop + story state machine, vocabulary's sole definition (LIFECYCLE) · gate table, raw story lists, all provenance anchors | code scan → `stories[].module` × `files_affected` binding → typed IR → archify (`compare` for Next), the canonical vocabulary                                                                        | structure change                                     |

The six SIBYL jobs all survive — NOW and PRIMER's essence fuse into band 1,
FRONTIER and LEDGER's plain layer form band 3, SHAPE and LIFECYCLE anchor band 4 —
but the page's spine is reading depth, not section type. The navigation rail
labels the four bands by their **scope names** — Overview · Product · Project ·
Engineering, never by role — and keyboard jumps move between bands. (The
Audience column above documents who each band serves; it is design rationale,
not label text.)

**Band 2 honors the two-phase gate (revision 7).** AEP's gate vocabulary is
`scripted_passed → passed`: `scripted_passed` means the scripted run went green,
`passed` means coverage was checked and human acceptance ran. A capability list
derived from `scripted_passed` would assert shipped-ness the evidence does not
carry — the exact failure the simulation produced, where a `scripted_passed` L32
surfaced as "acceptance passed" in prose while its own anchor read
`scripted_passed (human acceptance pending)`. So: **`passed` gates yield facts**
(unchipped prose, the honesty meter's default), and a `scripted_passed` gate may
appear in band 2 **only as an `EXP` chip whose settling event is the named human
acceptance run**. Gates below `scripted_passed` do not reach band 2 at all. This
is the D4 evidence-language rule applied to the one place a reader most wants to
over-read.

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
outside this set — and, since revision 3, that the prose channel stays within the
cold-reader vocabulary budget. `layer / gate / story / needs-you` are the expected
budget residents (translated into the brief's output language, defined at first
use); `epoch / wave / SHA / PR` may appear only in provenance anchors.

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
- **Delta-gate** — the manifest records per-block `gate` / `changed` / `stamp`.
  Gated blocks that didn't change collapse to a one-line stamp; the stamp joins
  band 1's quiet zone and stays reachable. (Revision 4: gates apply per block
  within the bands — identity and the band-2 capability list are the slow blocks,
  alongside band 4's architecture and lifecycle.)
- **Per-section self-legends** — every encoding (stage cells, tense chips, line
  styles) is defined where it is used; no section assumes memory of another.
- **Language** — one file per run, in the **owner's language** (an invocation
  parameter, defaulting to the repo's working language); system identifiers stay
  untranslated in the anchor channel. Native-language prose is the single largest
  cognitive-load lever for a cold reader. One-file-per-language localization
  (SIBYL P3) stays deferred.

### D3 — Presentation stack (owner-directed)

- **WebGL fluid background** (guizang shader heritage) as the canvas layer defined
  in D2: most visible in the top NOW zone, subdued behind content, tonally bound to
  the live/record split (darkens over LEDGER), degrading to a CSS paper gradient
  when WebGL is unavailable. Visual beauty is the owner's stated requirement for
  this layer; the plainness law governs the content layer only.
- **Keyboard navigation on a free-scrolling page** — ↑/↓, PageUp/PageDown, j/k,
  Home/End jump smoothly between bands; a fixed rail labels the four bands by
  their scope names (D2) and `#band` anchors make any position shareable. No CSS
  scroll-snap lock:
  the page must stay scannable as a whole (the reason deck paging was rejected),
  so keys _jump_, they do not _page_. A slim sticky state rail (current layer ·
  open count · needs-you count) keeps the glance overview present at any scroll
  depth.
- **The architecture view is generated by a deterministic code pipeline and
  rendered by archify** (owner rulings, revisions 5–6, scope corrected in
  revision 8). The ruling was never "no agent anywhere in this view" — it is
  **ground truth comes from code tooling, and translating it into something a
  human understands is the agent's job**. A workspace-graph scanner reads the
  real package topology (package.json / turbo graph; dependency-cruiser is the
  later import-level rung; LSP is rejected for batch graph work); a rule table
  transforms it into typed archify IR — R1 exclude test packages · R2 fold
  ubiquitous deps (in-degree ≥ 60%) into cards · R3 layered/cascade layout ·
  R4 semantic-type map · R5 boundaries from directory structure · R6 transitive
  reduction · **R7 concept-module ↔ code-unit binding, measured (below)** ·
  R8 edge aggregation · R9 generated guided views · R10 permutation search for
  row order — and a **mechanical receipt consumer** applies archify's repair
  receipts within the two-round bound. Same commit in, byte-identical artifact
  out.
- **R7 is a measurement, not a naming convention** (revision 8). The first
  implementation grouped packages by splitting their names on the first hyphen.
  That is deterministic and worthless: `db` and `auth` being separate "domains"
  is an artifact of how packages were named, not a fact about the system, and
  the grouping threw away the 32 prose module descriptions the project already
  carries. It produced 14 domains from 19 packages, 11 of them singletons — no
  compression, no meaning.
  **The project already states its own ontology and binds it to code:**
  `stories[].module` (present on 396/396 stories in the reference consumer) says
  which concept each piece of work belongs to, and `stories[].files_affected`
  (also 396/396, 84% resolving to a real workspace package) says where that work
  landed. The concept→code binding is therefore **measured from the work record**,
  not declared and not guessed. No graph-clustering alternative can replace this:
  clustering yields groups with no names, and a domain's _name_ is a human
  concept that exists only in prose.
- **The Engineering band is prospective and structural** (owner ruling, revision
  8): **Now · Concepts · Next**, and the payload is the tension between them.
  - **Now** — the real code structure. Ground truth, tool-derived.
  - **Concepts** — the module vocabulary humans reason in, bound to Now by the
    R7 measurement. The gap here is "the boundary you believe in is not the
    boundary that exists".
  - **Next** — the queued design projected onto Now: which code units the open
    stories' modules land in, which concepts are net-new, and whether each has a
    home. The gap here is "what the next design needs that the structure has
    not got". This is the question no status surface answers and the one an
    engineer most needs; archify's `compare` (Before/Delta/After) renders it,
    pointed **forwards** rather than at the previous brief.

  Progress-shaped annotations — where work happened, what it cost, where it
  failed — belong to Project's depth and are **excluded** from this band. That
  separation is what keeps the pyramid honest: each band is the same three
  questions at higher resolution, not a different subject.

- **All diagrams are archify; mermaid leaves the primary path** (revision 6). The
  AEP loop renders as an archify `workflow` diagram and the story state machine as
  an archify `lifecycle` diagram, so the page has one diagram system, one visual
  language, and no mermaid CDN dependency. Degrade ladder when the archify CLI is
  absent: a bounded mermaid diagram under author-side guidance (≤ 12 nodes, human
  glance gate, named as degraded) → labeled source text.
- **Single-file assembly** (owner ruling, revision 6: one file, nothing else to
  open). The assembler embeds every delivered artifact into the brief as a
  JSON-encoded string rendered through a sandboxed `srcdoc` iframe — Engineering
  shows a tab strip (overview · deep-dive · narrative) over one frame, plus an
  open-in-window action via a Blob URL; a `localStorage` seed pins the embedded
  artifacts to the light theme. Full viewer interactivity (guided views, passport,
  Present, exports) survives embedding. Cost stated honestly and now measured:
  each embedded artifact carries archify's ~600 KB viewer runtime, so a
  five-diagram brief is ~3 MB (the looplia one-pager is 3,218,172 bytes). That
  cost is bounded by D2's retention ruling — Phase 4 keeps the newest three
  briefs and prunes the rest, so `docs/human-alignment/` stays under ~10 MB and
  briefs stay in git rather than behind `.gitignore`.
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
- **Degrade ladder** (revision 7: mermaid is no longer a primary-path dependency,
  so it is a rung, not a risk): WebGL → CSS gradient; **archify CLI absent** →
  bounded mermaid under author-side guidance (≤ 12 nodes, glance gate, named as
  degraded) → labeled source text; font CDN blocked → system serif/mono/sans. The
  only network dependency on the primary path is the font CDN. Every rung keeps
  the page legible offline; the diagram rung is legible but degraded, and says so.

### D4 — Hybrid honesty model: facts derived by code, narrative authored by agent, both labeled

The generation pipeline (each phase ends in a checkable postcondition, per the
deterministic-orchestration standard):

| Phase         | Action                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                | Postcondition                                                                                                                                                    |
| ------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 0 · Preflight | `product-context.yaml` exists (else point to `/aep-envision`); read `docs/human-alignment/manifest.json` for the delta baseline                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       | baseline commit known, or first-run declared                                                                                                                     |
| 1 · Derive    | run `scripts/census.mjs` first: enumerate every populated key path in the plan file and classify it against `scripts/source-census.json` (`derived` / `ignored` **with a reason** / `unhandled`), reporting unhandled paths that carry data. Then run `scripts/derive.mjs`: extract **facts JSON** from `product-context.yaml` + git — story counts by state and layer, the attention set (D7), the drift facts (D7), the shipped-capability inputs for band 2 (passed layer gates + their summaries), changelog entries since baseline, layer-gate status, cost roll-up — and validate it against `facts.schema.json`                                                                                                | census reported; facts JSON exists and validates; every fact names its YAML path                                                                                 |
| 1.5 · Scan    | run the deterministic architecture pipeline (D3), all three scripts named in D5: `scan-workspace.mjs` (workspace-graph scan) → `arch-rules.mjs` (R1–R10 → typed IR) → `receipt-consumer.mjs` (archify validate → apply receipts → deliver), producing the domain-overview and package-graph artifacts revision-pinned to HEAD; the declared-vs-actual gap joins the drift facts (import-level dependency-cruiser diff is the later rung)                                                                                                                                                                                                                                                                              | artifacts delivered `code-verified` (package level), or scanner unavailable and the view is marked degraded                                                      |
| 2 · Author    | fill `assets/template.html`: no number is ever typed into markup — every one is a `data-fact="<facts-JSON path>"` binding the template's renderer fills at load — and **every assertive block carries `data-claims="<fact paths>"`** naming the facts it rests on (D9). A block that can cite nothing is marked visibly as authored; narrative (PRIMER, translations, LEDGER prose) is written fresh, tense-chipped, stamped with authored-at + source commit; narrative obeys the evidence-language rule and the cold-reader authoring rules below                                                                                                                                                                   | every section rendered or stamped                                                                                                                                |
| 3 · Audit     | run `scripts/audit.mjs` for the mechanical checks (claim-provenance, statically: every `data-fact` and `data-claims` path resolves in facts JSON; no assertive block exists without citations; and **no digit _or number-word_** appears in authored markup outside a binding or a provenance anchor; class preflight; chip-grammar: every non-fact chipped, no fact chipped, one chip per clause; translation-anchor 1:1 — every plain sentence cites a fact id, every surfaced fact has a plain sentence; prose vocabulary-budget count) plus the judgment checks from `references/checklist.md` (vocabulary audit against the D2 closed set, evidence-language audit, glance gate, cold-reader test, so-what test) | audit passes; failures emit structured receipts; at most two correction rounds                                                                                   |
| 4 · Deliver   | write `docs/human-alignment/brief-<date>T<time>Z-<shorthash>.html`; update `manifest.json` (generation record + content SHA-256, appending the prior record to `history[]`); **prune** every brief beyond the newest three (D2 retention ruling); report the delta summary + path in-conversation                                                                                                                                                                                                                                                                                                                                                                                                                     | new file exists; its name's hash equals repo HEAD; `manifest.json` digest matches the file; ≤ 3 brief files remain and every pruned one has a `history[]` record |

- **Why the derive script is v1, not deferred** (owner ruling: faithful
  representation): OBS-5's trust came from _code-derived_ artifacts, and AEP's own
  verification-economics work (v3.1.0) requires evaluator independence — an agent
  auditing numbers it extracted itself is neither. The countable facts are
  trivially scriptable; the word "deterministic" is used only where a script did
  the work.
- **The fact binding is declarative, so the audit is static** (revision 7). The
  simulation's template injected numbers imperatively
  (`$('ontA').textContent = FACTS.code_graph.nodes`), which is correct at runtime
  but leaves nothing for a standalone `audit.mjs` to check: a static scan of the
  delivered HTML finds no prose numbers at all and passes vacuously — an audit
  that cannot fail. The template therefore binds every number as
  `<span data-fact="code_graph.nodes">`, filled by one renderer loop over facts
  JSON. Two mechanical checks become possible without a DOM: **every `data-fact`
  path resolves** in facts JSON, and **no digit appears in authored markup**
  outside a `data-fact` element or a provenance anchor. This is what makes
  acceptance 6 (audit runs standalone, independent of the authoring agent) a real
  gate rather than a formality.
- **Structured repair receipts** (archify's contract): an audit failure names a
  stable `code`, the `subject` (element/selector), the measured `evidence`, and the
  `supportedFixes`; the authoring agent applies a listed fix and re-runs, never
  guesses, and never exceeds **two** correction rounds — a third failure is
  reported, not silently retried.
- **Evidence-language rule** (archify's discipline, applied to prose): narrative
  may not assert causality or impact — _blocks, breaks, guarantees, unblocks,
  proves_ — without citing a fact id from facts JSON. Tense chips say _when_ a
  claim holds; this rule says _whether it may be claimed at all_.
- **Cold-reader authoring rules** (revision 3; the D2 contract, operationalized):
  1. **Answer first.** Every section opens with one plain sentence that _is_ the
     section's conclusion ("all quiet except two repair tasks waiting on you") —
     overview-first applied to text, not just layout.
  2. **The while-you-were-away narrative.** NOW narrates the baseline diff as one
     continuous paragraph — "last brief, you had just dispatched X; it landed;
     meanwhile 3 more landed and 2 repairs failed" — not as bare delta chips. On a
     first run it narrates the newest changelog window and says so.
  3. **Translation is mandatory.** Story titles and changelog entries never
     surface verbatim. Each is re-authored as a consequence sentence — _what
     happened, to what, why the reader cares_ — bound to its source id in the
     anchor. Translations are re-authored fresh each run from current facts, so
     their rot window is one generation.
  4. **No naked numbers.** Every surfaced number sits inside a sentence stating
     its consequence ("38 acceptance criteria, none verified — the 10 finished
     tasks have no evidence yet").
  5. **Fold the queue.** Backlog stories collapse to one plain sentence per layer
     with the full enumeration behind a disclosure element — the every-open-story
     contract is preserved, but a cold reader meets nine sentences, not 44 rows.
  6. **So-what test.** A row that cannot state why the reader should care moves to
     the anchor/disclosure channel; it does not occupy the surface.
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
├── scripts/              # Node (.mjs) throughout — one runtime, no extra toolchain
│   ├── derive.mjs        # Phase 1: deterministic facts extraction (yaml + git) → facts JSON
│   ├── scan-workspace.mjs # Phase 1.5a: package.json/turbo workspace graph → code-graph JSON
│   ├── arch-rules.mjs    # Phase 1.5b: rules R1–R10 (code graph → typed archify IR, 3 tiers)
│   ├── receipt-consumer.mjs # Phase 1.5c: archify validate → apply repair receipts → deliver
│   ├── assemble.mjs      # Phase 4: embed delivered artifacts as srcdoc; prune to newest 3
│   ├── audit.mjs         # Phase 3: independent mechanical audit (provenance, classes, chip grammar)
│   ├── census.mjs        # Phase 1a: populated-path census vs the classification manifest
│   ├── source-census.json # path-template → derived | ignored(reason); the completeness contract
│   └── facts.schema.json # the typed contract between derive, author, and audit
└── assets/
    └── template.html     # seed file; the only source of CSS classes AND the palette
                          # (:root); manifest scaffold; nav + WebGL + degrade JS prebuilt;
                          # every number is a data-fact binding, never typed markup (D4)
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
  P0 checklist item). It is also the **fact-binding** single source: the renderer
  loop and the `data-fact` convention live there, so the authoring agent never
  writes number-rendering JS (D4).
- **The architecture pipeline is code, and it has files** (revision 7). D3 and
  D4's Phase 1.5 make the scanner, the R1–R10 rule table, and the receipt consumer
  load-bearing, but the earlier anatomy listed no home for them, and the
  simulation ran them as an ad-hoc Python script. They are named above and written
  in Node, matching `derive.mjs`/`audit.mjs`: one runtime for the whole skill, and
  the workspace graph comes from `package.json`/turbo, which Node reads natively.
  `arch-rules.mjs` holds R1–R10 as data (an auditable table), not as prose in a
  reference file — the rules are the determinism claim, so they must be executable.

### D6 — Release, acceptance, propagation

- **Implementation PR(s)** against this doc: scaffold the skill (all seven
  `scripts/` files from D5 + the facts schema), author the template with its
  `data-fact` renderer, write the two `_shared` specs (D7), add the marketplace
  entry and skills-index rows, add the routing-eval entry. Additive change →
  minor bump (v3.3.0); no other skill's step semantics change (the
  `/aep-validate` coherence rule from D7 is a separate follow-up PR, and
  `architecture.modules[].paths` is a separate schema PR — a schema field addition
  must propagate to every `product-context-schema.yaml` copy under
  `skills/product-context/*/templates/`, which the build regenerates from
  `_shared/`).
- **Acceptance** (layer-gate style, checkable):
  1. against a fixture `product-context.yaml` containing known attention signals
     (one `failed` story + one pending `amendment_log` entry), the brief passes
     every P0 checklist item **and NOW renders the top-priority signal as the one
     ask with its action verb** — NOW provably non-empty;
  2. re-running with an unchanged YAML produces a new timestamped file whose slow
     blocks (identity, capability list, architecture, lifecycle) collapse to
     stamps (delta-gate proof);
  3. a story-state edit in the YAML surfaces in NOW's delta band on the next run;
  4. with WebGL and the font CDN blocked — the only two network/runtime
     dependencies left on the primary path — the page stays legible (CSS wash,
     system fonts) and the embedded diagrams still render, since they are inlined,
     not fetched; with the **archify CLI absent** at generation time, the diagram
     sections fall to the named degraded rung (bounded mermaid → labeled source
     text) and say so on the surface;
  5. the output filename's commit hash equals repo HEAD at generation, the
     filename's timestamp matches the manifest's `generated` field, and
     `manifest.json`'s content SHA-256 matches the delivered file;
  6. facts JSON validates against `facts.schema.json`, and `audit.mjs` passes when
     run standalone (independent of the authoring agent) — and **provably can
     fail**: a fixture whose markup contains a hand-typed number outside a
     `data-fact` element, and one whose `data-fact` path is absent from facts
     JSON, are both rejected with a structured receipt;
  7. every surfaced item carries a plain-language sentence plus a provenance
     anchor, and the prose channel stays within the seven-word vocabulary budget
     (the cold-reader contract, mechanically counted where possible);
  8. when the archify CLI is present, every embedded artifact (architecture ×3,
     workflow, lifecycle) passes `validate` and `deliver` (layout gates green,
     receipts recorded), the assembled brief is one self-contained file (fonts CDN
     excepted), and re-running the pipeline at the same commit reproduces the
     artifacts byte-identically; when absent, the Engineering band renders the
     named degraded ladder;
  9. after four consecutive runs, `docs/human-alignment/` holds exactly three
     brief files (the newest three) and `manifest.json`'s `history[]` holds four
     records — the pruned generation is still addressable by commit and digest
     (retention proof, D2);
  10. against a fixture whose `architecture.amendment_log[]` entries carry no
      `status` and whose `calibration.plan[]` entries carry no `status`, the run
      **skips those predicates and records the skip** as a derivable-signal
      absence that reaches the surface as an open question — it neither guesses
      nor silently narrows the attention set (field-level schema tolerance, D7);
  11. against a fixture with a `scripted_passed` layer gate, band 2 renders its
      capabilities under an `EXP` chip naming the pending human acceptance, and
      the unchipped capability list contains only `passed` gates (two-phase gate
      honesty, D2).
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
  **calibration checkpoints due** (→ `/aep-calibrate ▸`),
  `product.open_questions[]` (→ `answer ▸`).
- **Calibration is derived from plan-minus-history, not from a status field**
  (revision 7). The schema's `calibration.plan[]` entry is
  `layer · dimensions · trigger` — there is **no `status` field** to test, in
  looplia or upstream; the earlier "pending `.5` checkpoints" phrasing named a
  field that has never existed, and the simulation's `calibration_status` came
  back as fifteen nulls. The derivable predicate is the one `/aep-calibrate` step
  1 already uses: a plan entry is **due** when its layer is at or below the
  current layer and no `calibration.history[]` entry matches its
  `(layer, dimension)` pair. `trigger` is prose and stays advisory — it is
  surfaced next to the ask, never parsed.
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
- **Schema tolerance is field-level, not section-level** (revision 7 correction).
  Real consumer YAMLs diverge from the current schema in two different ways, and
  only the first was handled: whole **sections** may be absent (looplia has no
  `product`, no `opportunity`), and **fields inside present sections** may be
  absent — looplia's `architecture.amendment_log[]` entries carry only
  `date` + `summary`, though the schema defines
  `status: pending | accepted | rejected`. The simulation recorded the two missing
  sections and silently derived nothing from `amendment_log`, which is exactly the
  failure this rule exists to prevent: an attention-set predicate _and_ a drift
  detector both went quiet and the surface said "all clear". So the rule binds per
  **predicate**, not per section: a predicate whose fields are absent **anywhere
  in its path** is skipped and recorded ("signal not derivable in this schema"),
  never guessed, and `derive.mjs` emits one `schema_absent[]` record per skipped
  predicate — `{ predicate, path, reason }` — which the brief surfaces as an open
  question. An empty attention set is only trustworthy when `schema_absent[]` is
  also empty, and the surface must say which of the two it is.
- **Field typing** (simulation finding): the derivation field map types every
  fact — `business_value` is a numeric score in real consumer YAMLs, so a why-line
  derivation must require prose-typed sources and drop numeric ones rather than
  render a meaningless "10".

**The drift facts** — the canonical answer to "where did reality drift", as a spec
in `skills/product-context/_shared/references/drift-facts.md`. Every LEDGER drift
row must cite a derived fact; **hand-authored drift is banned** (OBS-2: underived
drift claims are noise or misses, and both spend the surface's credibility — when
nothing derives, the row is silent, not fabricated). The v1 derivations:

1. **Intent without evidence** — `coverage.criteria_total −
coverage.criteria_covered > 0` on a layer gate: declared acceptance criteria
   no evidence covers. **The counters are the derivation, not `coverage.uncovered`**
   (revision 7 correction): `uncovered[]` is the worklist `/aep-build` fills while
   authoring the missing scenarios, so it is empty precisely when the gate never
   opened — in looplia all nine coverage-bearing gates have `uncovered: []`,
   including L32 at 0-of-38 covered. A spec pointed at that field would have
   derived zero drift from the very case cited as this detector's motivating
   finding. `uncovered[]` remains the **detail channel**: when populated it names
   which criteria and their plan/WAIVER, and the anchor cites it. **Scoped to
   layers with work done or underway** (simulation finding): a future layer whose
   gate is honestly `not_started` with zero stories done is plan, not drift —
   flagging it repeats OBS-2's deferred-stories noise. The detector's boundary is
   stated on the surface when relevant.
2. **Plan behind the architecture** — `architecture.amendment_log[].status ==
pending`: stories were mapped against a structure that has since been amended.
   Subject to the field-level tolerance rule above — looplia's entries carry no
   `status`, so this detector is _skipped and recorded_, not read as "no pending
   amendments".
3. **Reality resisting intent** — `failure_logs` **on open stories**: a story that
   repeatedly fails is evidence the spec and the code disagree. Closed stories'
   failure logs are history — overcome, kept as record, counted separately
   (simulation finding: of looplia's 12 log-bearing stories, 10 were closed —
   8 `completed` and 2 `deferred` — leaving only the 2 `failed` ones as drift).
4. **Control-plane incoherence** — layer↔story-state disagreement: the OBS-4
   class. The predicate is **at least one `completed` story in a layer whose gate
   is `not_started`** — stated explicitly because the simulation prose reported
   this three different ways (6 layers where _every_ story is completed; 8 if
   `deferred` counts as done; 9 under the predicate the script actually ran).
   One reading, written down: 9 in looplia at `d5212571`. Follow-up (separate PR):
   the same check joins `/aep-validate` so incoherence cannot silently pass again
   — SIBYL's own OBS-4 disposition, applied to AEP.
5. **Declared vs. actual architecture** (Phase 1.5): the strongest drift form
   ("the YAML says A does not depend on B; the code says it does"), and the exact
   move that earned OBS-5's trust in SIBYL (its architecture graph was the one
   artifact derived from real imports). The looplia scan made the gap concrete:
   the code is 20 workspace packages while the YAML declares 32 conceptual
   modules — zero name overlap, and no schema field binds a declared module to
   code paths, so the declared architecture is **not code-addressable at all**
   today (itself a surfaced drift fact). **Framework recommendation (separate
   schema PR): add `architecture.modules[].paths`** — a glob binding from
   declared module to source paths — so dependency-cruiser-class tooling can turn
   every declared `depends_on` into a lint rule with file:line evidence.
   Toolchain-dependent, so it degrades honestly: no tool → the view is marked
   `unverified`, never blocked.

**Simulation evidence (2026-07-24).** The full pipeline ran against looplia
(`product-context.yaml`, 25k lines, 395 stories, at `d5212571`): a derive script
implementing both specs produced facts JSON (attention set of 2 — both `failed`
stories, the one ask chosen deterministically; 28 raw drift facts), the brief
rendered all six sections with every number JS-rendered from embedded facts, and
the mechanical audit plus a browser render check passed. The run surfaced the
five spec corrections now folded in above (intent scope; open-vs-historical
failure logs; schema tolerance; field typing; plus the D3 paper-overlay fix for
light sections) and real looplia findings (9 layers holding `completed` stories
under a `not_started` gate; a gate named `completed` outside the two-phase
vocabulary; L32's 38 criteria with zero coverage). The owner's readability
verdict on the result — system vocabulary is illegible even to a returning
owner — produced the revision-3 cold-reader contract (D2).

**Mine deeper before conceding (revision 8).** The rule that governs every
derivation here: **when a signal looks underivable, the next move is to ask what
the project actually records — not to hand the question to agent judgment.**
Checking the field the schema names, finding it empty, and recording
`schema_absent` is a half-measure that reads as diligence.

Revision 7's own implementation broke this three times against the reference
consumer, and each error shipped into the delivered example:

| Reported as underivable                         | Actually available                                                                                                                    |
| ----------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------- |
| "cost plane declared but unwired"               | `cost.total_usd` is 0, but **34 stories carry numeric `cost_usd` totalling \$735.15**, attributable per module (`agent` \$543.80)     |
| "declared architecture is not code-addressable" | **396/396 stories carry `module` + `files_affected`**, 84% of 1,485 paths resolving to a workspace package — the binding is measured  |
| "32 declared modules, 1 name in common"         | Wrong comparison: declared modules against _package_ names. Against `stories[].module` the real finding is **44 used vs 32 declared** |

A roll-up field being zero is a statement about the roll-up, not about the data.
Consequently:

- **Cost derives from `stories[].cost_usd`**, rolled up per module and per layer;
  `cost.*` is used only when it is non-empty and reconciled against the story sum.
  A disagreement between them is itself a drift fact.
- **Drift 5 is restated.** The declared architecture _is_ code-addressable — via
  the work record. `architecture.modules[].paths` remains a worthwhile schema
  addition, but as a way to **declare** what is already **measurable**, not as a
  precondition. Measured bindings do not rot the way declared ones do.
- **New detector — module vocabulary drift.** Compare three sets: modules
  declared in `architecture.modules[]`, modules that stories are filed against,
  and modules that resolve to code. In the reference consumer: **13 used but
  never declared** (`agent-runtime`, `do-agent`, `server`, `infra`, …) and **1
  declared but never worked**. That is the control plane and the work drifting
  apart in the project's own vocabulary — sharper than anything the previous
  name-comparison produced.

**Re-derivation pass (2026-07-25, revision 7).** Every number above was recomputed
from the looplia repo and the surviving artifacts. The counts hold — 395 stories,
25,074 lines at `d5212571`, attention set of 2 (both `failed`; zero `in_review`),
28 raw drift facts (12 + 9 + 7), 32 declared modules with zero `paths` fields
against 20 code packages, code graph 20 nodes / 68 edges, domain overview 9
components / 7 connections, the delivered archify artifact 613,803 bytes, the
one-pager 3,218,172 bytes — and the `business_value` typing bug is visible
verbatim in the facts JSON as `"why": "10"`. Three _derivations_, however, did
not survive re-derivation, and their corrections are folded into the specs above:
the coverage field (drift 1), field-level schema tolerance (the attention set and
drift 2), and the calibration predicate. Two facts are also worth keeping as
evidence rather than erasing: L32's zero coverage had already flipped to 38-of-38
by `8641b716`, _within the same simulation session_, and the reference brief's
Product band rendered that same `scripted_passed` gate as "acceptance passed"
while its own anchor said otherwise. The first is the argument for regeneration;
the second is why D2 now binds band 2 to `passed`.

Revisions 5–6 were proven in the same simulation before being written. The
deterministic pipeline ran end-to-end on looplia: workspace scan (20 packages,
68 edges) → rules R1–R10 → mechanical receipt consumer (gutter routing, one
round) → archify `validate` green with **0 errors** → `deliver`. Five artifacts
shipped in the final single-file brief — the domain overview (9 domains, 7 edges
after aggregation and transitive reduction), the full package graph, the declared
narrative, the AEP loop as a `workflow` diagram, and the story state machine as a
`lifecycle` diagram — every one repaired only through archify's own receipts
(enum, labelAt/labelDy, channel routes, width). Diagram-type usage is therefore:
`architecture` ×3 · `workflow` ×1 · `lifecycle` ×1; `sequence` and `dataflow`
are unused until a structured source exists (recorded in Horizon).

The revision-5 archify integration was then proven in the same simulation: a
12-component typed IR authored from looplia's declared modules (zh labels, real
module names as sublabels, two boundaries, three guided views) went through
archify `validate` — which caught two defects with actionable repair receipts (a
card-dot enum violation; a connection label overlapping the `exec` component,
fixed with the receipt's suggested `labelAt`) — and `deliver`, producing a 614 KB
interactive artifact the brief embeds. During the same session looplia's HEAD
advanced mid-simulation (L32-005 landed; its gate flipped to `scripted_passed`),
which surfaced two more authoring rules now in D4's spirit: changelog
translations bind by entry id + kind, never by list position, and no authored
sentence may carry a literal number — all numbers render from facts.

### D8 — Design Option Sets: suggestions with enough material to disagree with

The Engineering band **may recommend**, and the reason it may is the same reason
the rest of the surface may not fabricate: what makes a claim safe is the
material under it. A bare "consider splitting this package" is an unchipped
assertion with no anchor — the reader can only obey or ignore it. A suggestion
carrying its option space, its measured costs, and its ranking criterion leaves
the judgment where it belongs.

So a suggestion is a **first-class surface element with a required grammar**, in
the same way a drift row is:

| Field                    | Rule                                                                                                                                         |
| ------------------------ | -------------------------------------------------------------------------------------------------------------------------------------------- |
| **Trigger**              | A derived signal, cited. **No trigger, no Options block** — the same rule that bans hand-authored drift.                                     |
| **Measured situation**   | The facts defining the decision space, every number `data-fact` bound.                                                                       |
| **Options**              | **At least three, one of which must be "leave it as is."** Omitting the null option converts information into pressure.                      |
| **Cost / benefit**       | Stated in _this project's measured terms_ — files moved, stories touched, runtime hops added — never in generic architectural principle.     |
| **Design sketch**        | Enough substance to evaluate: which units, which seam, which new edges.                                                                      |
| **What would settle it** | The measurement or event that makes the choice obvious. The `EXP` chip's discipline, applied to a decision.                                  |
| **Tense**                | The whole block is non-fact and is chipped, so the page-level honesty meter still reads true.                                                |
| **Ranking**              | Permitted, but the **criterion must be stated** so the reader can reject it. "On seam cost, C is cheapest" is legal; "I recommend C" is not. |

**The triggers are fixed in this spec, not configurable** (owner ruling). A
configurable threshold is a knob nobody turns; a fixed one gets reviewed. All
three run **only against the Next projection** — open stories. Running them over
history is a category error and was measured to be pure noise (below).

1. **Concept crowding** — a code unit that the queued design plans to fill with
   **≥ 4 concept modules**, **≥ 30 files**, and whose concepts are **≥ 50%
   pairwise file-disjoint**. All three conditions are load-bearing: density alone
   flags shared-type packages, and disjointness alone flags registries where
   every concept simply adds its own file. Together they say _a lot of separable
   code is being planned into one unit that has no internal boundary_.
2. **Homeless concept** — a concept module in the Next projection whose planned
   paths resolve to no code unit at all: the design has not decided where it
   lives. Threshold ≥ 1.

**Calibration against the reference consumer** (45 open stories, 8 net-new
concept modules):

| Detector                  | On the Next projection                                 | On completed work (control)               |
| ------------------------- | ------------------------------------------------------ | ----------------------------------------- |
| Concept crowding          | **1** — `do-agent`: 7 concepts, 85 files, 81% disjoint | 5 — noise                                 |
| Homeless concept          | **0**                                                  | 3 — all false (docs and non-package work) |
| _Entanglement (rejected)_ | 0                                                      | 68 — no principled cut point              |

The control column is why the scope rule exists. It also killed a third detector
I intended to ship: "two modules declared independent but sharing ≥ 3 files"
returned 68 hits on real history with no defensible threshold, so it is
**rejected from v1** rather than tuned until it looked reasonable.

**The worked example the reference consumer produces.** Trigger: `do-agent` is
planned to carry seven concept modules across 85 files, of which 17 of 21
pairs share no file at all. The situation is therefore not "these things are
entangled" but "**the plan has already partitioned this code seven ways and given
the partition nothing to enforce it**". Options: keep one unit and enforce the
seam with import rules (zero migration, unenforceable); split along the measured
seams into separate runtime units (real isolation, one extra RPC hop on the
publish path because the authority check sits there); or extract only
`authority-kernel`, which shares zero files with all six others (highest-value
isolation for one hop). Ranking criterion stated: seam cost — and explicitly
_not_ security, which would reorder it.

### D9 — Completeness by construction

An independent evaluator scored the generated brief and failed it. The findings
matter less than their shape: **eight of nine were the same defect wearing
different clothes.**

| Surface symptom                             | What was actually missing                                                                              |
| ------------------------------------------- | ------------------------------------------------------------------------------------------------------ |
| "spend is zero, this brief cannot say"      | `stories[].cost_usd` was never read                                                                    |
| "the declared architecture is unverifiable" | `stories[].files_affected` was never read                                                              |
| "one module name in common"                 | the wrong two sets were compared                                                                       |
| "automatic repair has already given up"     | `attempt_count` / `max_retries` never read — filled from a diagram's generic label                     |
| "five tasks in two days" (it was eight)     | no fact for _completions in a window_ — counted by hand                                                |
| "the last thing before sign-off"            | `closure_status` / `decision_realignment` never read; the sign-off had been **withdrawn**, not delayed |
| "the newest stretch of work"                | the changelog was sliced from the tail without sorting — **a plain bug**                               |
| "the system is 21 code units"               | the scanner covers one ecosystem and never said so                                                     |
| "the cleanest cut available"                | two concepts tied at zero shared files; the fact returned one                                          |

The pattern: **the prose needed a fact the derivation had not produced, and the
agent supplied it anyway.** The audit could not object because its unit of check
was the digit — and "already given up", "the newest stretch", and "five" are not
digits.

Underneath that sits the real cause. The facts plane was designed **top-down** —
_what should the brief show?_ — against a source carrying **483 populated key
paths**, of which the derivation reads about **thirty**. Every hole was a path
nobody had looked at. Patching them one at a time treats the symptom.

Four mechanisms replace the patching. Each is deterministic.

**1 · Source census — the system must know what it has not looked at.** Walk the
consumer's plan file, enumerate every populated key path, and classify each
against a committed manifest:

- `derived` — reaches facts JSON
- `ignored` — listed with a **reason**, never a bare checkmark (owner ruling)
- `unhandled` — neither, and therefore a reported gap

Classification is at **path-template level** (`stories[].readiness_score`, not
396 leaf paths) — roughly a hundred entries, authored once (owner ruling). The
run reports unhandled paths carrying data, and the brief states its own reading
coverage on the page. Completeness stops being a hope and becomes a number.
This mechanism alone closes rows 1, 2, 4 and 6 above.

**2 · Claims bind, not numbers.** The audit's unit changes from the digit to the
claim. Every assertive block declares the facts it rests on:

```html
<p data-claims="cost.derived_total_usd cost.rollup_disagrees">…</p>
```

The audit then enforces three things: every cited path resolves; **no assertive
block may exist without citations**; and number-words (`five`, `eight`,
`three quarters`) are treated exactly as digits are. One rule closes both the
unbound numeral and the uncited causal claim — the two defects that produced the
worst sentences on the page.

**3 · Facts carry predicates, not just fields.** Prose wants to say _"retries are
exhausted"_, _"the sign-off was withdrawn"_, _"eight landed in two days"_. Those
must be **derived predicates**, not agent inferences:

- `retries_exhausted` = `attempt_count >= max_retries`
- `sign_off_withdrawn` = `closure_status` present
- `root_cause_stated` = whether a failure log asserts a cause or disclaims one
- `completions_in_window(days)`

When the predicate does not exist, mechanism 2 forbids the sentence. The two
planes are forced to co-evolve instead of drifting apart.

A corollary, from the tied-seam defect: **a fact may not collapse ambiguity.**
Where a derivation has ties or several valid answers, the fact carries all of
them — otherwise the prose will assert a uniqueness the data does not support.

**4 · Every tool declares its own coverage.** A scanner reports what it covered
_and what it did not_: `21 JS/TS workspaces; 2 Cargo crates unscanned`. The
coverage statement is itself a fact the prose must use. **No tool is permitted to
imply totality** — the brief priced work on a Rust daemon in one band while
excluding it from "the system" in another, and nothing in the pipeline noticed.

**What this does not fix.** Row 7 — the unsorted changelog slice — is an ordinary
bug. No mechanism above would have caught it; only the evaluator did. That is
recorded rather than papered over: **eight of nine become structurally
impossible, one was simply wrong code.**

**What stays agent-authored.** The essence paragraph (no product-vision field
exists), the translation of module responsibilities into plain language, and the
option-set framing. Each must still cite; whatever can cite nothing is marked
visibly as authored, as the brief already does for its opening paragraph.

**Acceptance for this revision.** Re-running against the reference consumer, the
census must _report_ the unhandled paths that produced the original defects, and
the audit must _reject_ the sentences that shipped. The bar is not "the defects
are gone" — it is "the defects cannot be authored".

### D10 — The unit of delivery is the clock, not the page

Revision 9 diagnosed a velocity problem and got the number wrong. The 1.1-hour
figure is the **interval between edits**, not the **survival time of a fact**.
Re-measured across sixty plan-file commits, the facts this surface rests on do
not share a clock at all:

| Fact class                          | Median survival | Band |
| ----------------------------------- | --------------- | ---- |
| changelog length                    | 0.3 h           | 3    |
| story status distribution           | 1.3 h           | 3    |
| gates passed                        | 14.7 h          | 2    |
| gate status map · coverage          | 33.5 h          | 2    |
| **the attention set · the one ask** | **110 h**       | 1    |

Rolled up: **band 1 ≈ 100 h · band 2 ≈ 29 h · band 3 ≈ 10 h · band 4 ≈ 10 h.**
The band split from revision 6 turns out to be almost exactly the seam in the
survival curve — that decomposition was right.

**What is wrong is fusing the bands into one file.** D3's revision-6 ruling
("one file, nothing else to open") welds four clocks together. To keep band 3
true you regenerate at band 3's rate; every regeneration re-authors band 1's
prose, whose facts move a hundred times more slowly. **You pay roughly a hundred
authoring passes per band-1 fact change**, and each pass is an independent draw
from a defect distribution that has so far produced several content defects per
draw. Regeneration is not the cure for staleness here — it is the delivery
mechanism for the actual failure mode.

The evidence that this, and not staleness, is the failure: of the eighteen
findings across two evaluation rounds, **two or three are staleness**. The rest
are authoring failures against a fact plane that was correct, complete, and
twenty-two minutes old — including a claim that a stage had not started while
three separate correct representations of that stage's completion sat in the
file the sentence cited.

**Ruling: emit three things on three clocks, not one thing on the fastest.**

1. **The fact plane, prose-free, per plan-file commit.** Deterministic, cheap,
   no authoring surface, therefore no defect surface. It already exists.
2. **A read-time answer to "what happened since I last looked"** — computed when
   asked, never written down, dealing in **events rather than states**. "The
   canary recovered at 17:0x" stays true forever; "the canary is broken" had a
   26-hour half-life. A report of states rots; a log of events can at worst be
   incomplete.
3. **The orientation document, per layer rather than per invocation.** Bands 1,
   2 and the Concepts half of 4. That clock comfortably supports careful prose,
   translation, archify and a multi-megabyte body, because it is generated on
   the order of weekly.

The design's own answer to velocity — the delta baseline, the
while-you-were-away narrative — has **never produced a line of output** in any
generation (`delta.first_run` was true both times). Nine revisions hardened the
components that had already run; the one that never ran is the one this ruling
promotes to a first-class emission.

### D11 — Render; do not detect

The skill has been accumulating detectors. Control-plane incoherence, cost
roll-up disagreement, module-vocabulary drift, concept crowding — each was added
here because this was the surface that noticed. That is the wrong home.

**A detector that finds a defect in the plan file belongs where it can block**
— `/aep-validate` — not in a document where it narrates. The distinction is
whether the finding wants an action or a reader: incoherent gates want fixing,
and a brief that reports them every week without stopping anything is a
subscription to a problem rather than a fix for it.

What stays here: **rendering**, and the reader-facing judgment about what
deserves the surface. What moves: the detectors themselves, as
`/aep-validate` rules. What is shared: the derived-view specs
(`attention-set.md`, `drift-facts.md`) remain **framework vocabulary** that both
skills consume — that was always their stated status and it is now load-bearing.

This shrinks the skill to its actual job and removes the pressure that produced
the detector suite: the surface no longer has to be the place a problem is
caught in order for the problem to be caught.

**Implemented (2026-07-27).** The detectors that want an action —
completed work under an unopened gate, an undefined gate status, a roll-up that
disagrees with its record, a module used but never declared, and fields a
consumer invented — moved to `skills/product-context/_shared/scripts/coherence.mjs`
and run as a **blocking Step 0 in `/aep-validate`**. The brief renders what that
detector returns and no longer computes it.

The script is _shared_, not copied: `build-skills.sh` now materializes
`_shared/scripts/` into consumers the same way it always has for references, on
the same per-file rule (a skill receives what its SKILL.md names). Two copies of
a drift detector drift, and a drift detector that drifts is worse than none.

### D12 — Never a private store

One reviewer proposed a dedicated ledger file for the owner's rulings,
obligations and invariants — the durable objects a fast-moving project loses
track of. The insight is right and the mechanism is not, because **the framework
already has homes for most of it**:

| Durable object  | Framework home                                              | Status                                                  |
| --------------- | ----------------------------------------------------------- | ------------------------------------------------------- |
| A ruling        | `product.decisions[]` — decision · reasoning · alternatives | exists                                                  |
| A deferred call | `product.open_questions[]` — with `revisit_trigger`         | exists; this is premise-rot handling, already specified |
| An obligation   | —                                                           | **genuine gap**                                         |

A private ledger would be a **third home** for something the schema defines, and
a stored copy of a derivable truth is exactly what D7 rejects: a second source
validation will not catch when it drifts.

So the rule: **this skill derives and surfaces; it does not store.** Three
consequences.

- Where the framework has the field, derive from it. If a consumer leaves
  `product.decisions[]` empty and records rulings in prose elsewhere, that is
  the consumer's practice, and the surface reports the absence — it does not
  invent a place to put them.
- Where a consumer has **invented fields the schema does not define**, that is
  itself drift worth surfacing. The reference consumer added `closure_status`,
  `decision_realignment`, `notes` and `release` to its layer gates; those carry
  real meaning and no framework consumer can read them.
- Where the framework genuinely lacks the field, **recommend it** — the
  `architecture.modules[].paths` precedent. The gap here is **obligations**:
  `evidence.manual_pending` is a boolean with no `owed_by` and no `since`, so
  nothing can age it or escalate it. An obligation that has been true for
  weeks is indistinguishable from one raised this morning.

**Recommendation to the framework (separate schema PR):** give an obligation a
shape — what is owed, by whom, since when, what it blocks, and what discharges
it. Age is the missing dimension; "what needs a human" without duration is a
list, not a signal.

### D7 corrections (the design's own, not the consumer's)

Two defects in the attention-set spec, both found by review of the spec rather
than of the data:

- **The one ask must respect dependencies.** The tie-break is "layer order, then
  id". In the reference consumer both candidates sat in the same layer and the
  alphabetically-first one **depends on the other** — so the rule selected the
  blocked item over its blocker and told the reader to restart it. Ordering
  within a rank must be topological before it is alphabetical.
- **Every signal carries `since`, and age can be the alarm.** OBS-2 says a
  detector that lists intentional deferrals is noise. A signal continuously true
  for months is that noise — unless its **age** is the finding, which for an
  obligation it usually is. Both require a `since`, which the spec does not
  currently demand.

### D9 corrections (the design's own)

- **The census postcondition must be adversarial.** `unhandled_count == 0` was
  satisfied at 11.5% read coverage, with the great majority of ignores sharing a
  handful of templated reasons — and one of those ignores (`stories[].dependencies[]`,
  populated on 77% of stories) is the direct cause of the one-ask defect above.
  The owner's ruling that an ignore must carry a reason was satisfied in letter
  by writing a few reasons and fanning them out. A stronger postcondition: a
  path populated across most of a collection may not be `ignored` without a
  named consumer that reads it elsewhere.
- **Claim-binding raises apparent verification without raising verification.**
  Mechanism 2 makes citation mandatory; nothing makes the sentence follow from
  the citation, and a citation that resolves without bearing its claim reads
  _verified_. On a surface whose only product is trust this is net-negative as
  it stands. Either the binding gains a checkable relation between claim and
  fact, or the mechanism is downgraded from a guarantee to a lint.

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
- **Import-level verification**: dependency-cruiser (TS/JS) / import-linter
  (Python) as the second rung of the architecture pipeline — declared boundaries
  become lint rules with file:line evidence, enabled once `modules[].paths`
  lands; LSP stays reserved for on-demand point queries (who references X), not
  batch graph extraction.
- **Architecture Delta**: archify's `compare` renders Before/Delta/After between
  two briefs' architecture IRs — "what changed structurally since the last
  brief" as a validated artifact.
- **`sequence` and `dataflow` diagrams** when structured sources exist: the
  newsroom content pipeline (signal → pitch → research → draft → publication) as
  a dataflow; a single story's dispatch→build→review trace as a sequence.
- **TUI summary render** of NOW (the terminal-native owner's glance surface);
  one-file-per-language localization; a stable `latest` pointer if a fixed URL is
  ever needed. (A separate stakeholder mode dissolved in revision 4: the pyramid
  already serves stakeholders — they stop after band 2.)

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
- **System-vocabulary prose** (revision 1–2's implicit register) — rejected in
  revision 3 by the owner after reading the simulated brief: story titles and
  gate/layer jargon are illegible even to a returning owner. SIBYL's
  "use the system's own name" survives in the anchor channel only; prose
  translates (D2 cold-reader contract).
- **URL-parameter reads** (`?read=owner|newcomer|stakeholder`, revisions 1–3) —
  rejected in revision 4 by the owner: one document for everyone, ordered as an
  audience-depth pyramid; role adaptation is scroll depth and disclosure. A mode
  parameter splits the audience into surfaces nobody remembers to use; a pyramid
  lets each reader stop when their questions run out.
- **`skills/human-alignment/human-alignment/` category nesting** — rejected by the
  owner (duplicate directory); category grouping revisits when a second theory
  skill exists under the theme (see D1).
- **A single `brief.html` overwritten in place** — rejected by the owner: the
  filename must carry generation time and commit hash so a reader knows which
  commit a brief describes without opening it; overwriting also erases the record
  of past briefs. A stable `latest` pointer remains a Horizon option.
- **Audience-role band names** (revision 4's "everyone / product user / manager /
  engineer") — replaced in revision 6 by scope names (Overview · Product ·
  Project · Engineering): role names invite readers to self-exclude; scope names
  only classify depth.
- **Agent-authored architecture IR as the primary view** (revision 5) —
  superseded in revision 6: topology now comes from the deterministic code
  pipeline; the authored IR survives as the narrative tier only.
- **Companion artifact files next to the brief** (revision 5) — superseded in
  revision 6 by single-file `srcdoc` embedding (owner: one file, nothing else to
  open); the delivered artifacts become build intermediates.
- **`coverage.uncovered` as the intent-without-evidence source** (revisions 2–6)
  — corrected in revision 7: the field is a `/aep-build` authoring worklist, so it
  is empty exactly when the gate never opened. It stays as the detail channel; the
  counters are the derivation (D7.1).
- **Unbounded accumulation of briefs under `docs/`** (revisions 2–6) — replaced in
  revision 7 by keep-the-newest-three with tool-side pruning: at a measured 3.2 MB
  each, "pruning is the owner's choice" is a policy that will not be executed, and
  the record readers actually need (which brief described which commit) lives in
  `manifest.json`'s `history[]`, not in the 3 MB body.
- **Moving briefs behind `.gitignore`** (D3's alternate caveat) — rejected: it
  breaks the ruling that the file is what you hand to someone asking "what is this
  project?" Bounded retention gets the same size control without that cost.
- **Imperative number injection in the template** (the simulation's shape) —
  rejected in revision 7: it makes a standalone `audit.mjs` unable to fail on
  number provenance. Declarative `data-fact` bindings keep the audit static (D4).
- **A `calibration.plan[].status` field** — rejected: the field does not exist
  upstream, and adding one would store a truth derivable from `plan` ∖ `history`,
  which is the same second-source failure the attention set was designed to avoid
  (D7). The predicate is derived instead.
- **Name-stem package grouping as R7** (revision 7's implementation) — rejected
  in revision 8: deterministic but meaningless. It produced 14 domains from 19
  packages with 11 singletons, and discarded 32 prose module descriptions to
  split strings on a hyphen. Determinism is a means to trust for _claims about
  reality_; a grouping is not a claim about reality, so determinism bought
  nothing there.
- **A per-repo domain override file, plus a tunable folding threshold** — rejected
  as the compromise it was: it concedes the rule does not work and asks the user
  to patch it by hand, or shrinks the picture until it looks acceptable. Neither
  makes the diagram say what the system is.
- **Graph-clustering the package topology** (shared-dependent sets, community
  detection) — rejected: measurable, but it yields `group-1`, `group-2`. A
  domain's _name_ is a human concept that exists only in prose, and no amount of
  graph mathematics recovers it.
- **Annotating the architecture with the fact plane** (work here, spend here,
  failures here) — rejected by the owner in revision 8 as a **band confusion**:
  those are Project's questions at greater depth. Engineering's question is
  structural and forward-looking — what the next design does to what exists.
- **Entanglement detector** ("declared independent, sharing ≥ 3 files") —
  rejected from v1 (D8): 68 hits on real history with no principled cut point.
  Recorded rather than tuned into looking reasonable.
- **Configurable Option-Set thresholds** — rejected by the owner: a knob nobody
  turns. Fixed values get reviewed.
- **Bare one-liner recommendations** — rejected (D8): a suggestion without its
  option space and measured costs leaves the reader only obedience or dismissal.
  Suggestions are welcome; unsupported ones are not.
- **Fixing the evaluator's nine findings one at a time** — rejected in revision 9. Eight of them were one defect wearing different clothes; a patch list would
  have shipped a tenth. The mechanisms in D9 are chosen so the defects cannot be
  authored, not so that these particular nine are absent.
- **A digit-only provenance audit** (revisions 7–8) — superseded: it certifies
  "every number is bound" while `five`, `eight` and `three quarters` walk past
  it, and while an uncited causal claim is not a number at all. The unit of
  check is the claim.
- **Full-path census** (all 483 leaf paths) — rejected by the owner in favour of
  **path-template** classification: leaf-level entries would be dominated by
  per-consumer schema noise, and the review burden would fall on the wrong
  thing. Roughly a hundred templates, authored once.
- **A checkbox `ignored` list** — rejected by the owner: an ignore entry must
  carry a **reason**. A checkmark records that someone clicked past a field; a
  reason records why the field does not matter, and is reviewable when it starts
  to matter.
- **Letting the scanner report only what it found** — rejected: a tool that
  states its finds without stating its blind spots licenses "the system is 21
  units" while two Cargo crates sit outside the scan and get priced elsewhere on
  the same page.
- **A dedicated owner-ledger file** for rulings, obligations and invariants —
  rejected in revision 10 (D12). The insight behind it is correct, but the
  framework already defines `product.decisions[]` and
  `product.open_questions[].revisit_trigger`; a private file would be a third
  home for a truth the schema owns, which is the second-source failure D7 exists
  to prevent. The part that is genuinely missing — an obligation with an age —
  is recommended to the schema instead.
- **Keeping the detectors in this skill** — rejected (D11): a finding that wants
  an action belongs where it can block. Reporting incoherent gates in a document
  every week is a subscription to a problem, not a fix.
- **Regenerating the whole page to keep its fastest band true** — rejected
  (D10): it pays a hundred authoring passes per slow-band fact change, and each
  pass is an independent draw from the defect distribution. The measured failure
  is authoring, not staleness.
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
