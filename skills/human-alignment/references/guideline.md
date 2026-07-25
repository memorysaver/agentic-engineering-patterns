# The Brief Guideline

The media-neutral contract. It holds whether the surface is HTML, a terminal
summary, or print.

## Contents

- [The reader](#the-reader)
- [The four bands](#the-four-bands)
- [Three tenses, encoded in ink solidity](#three-tenses-encoded-in-ink-solidity)
- [The canonical vocabulary](#the-canonical-vocabulary)
- [The plainness law and its scope](#the-plainness-law-and-its-scope)
- [The delta gate](#the-delta-gate)
- [Evidence language](#evidence-language)
- [Language](#language)

## The reader

**Every read is a first read.** The owner runs many repos, returns from days
away, and meets every brief cold. Writing for the reader who has memorized your
vocabulary produces a page only its author can read — this was proven, not
assumed.

### Two channels, inverted hierarchy

Every surfaced item is a **plain-language sentence** first — what happened, to
what, why the reader cares — with the system identifier (story id, YAML path,
SHA, PR number) demoted to a small mono **provenance anchor** beneath it.

System names are citations, not prose.

### The vocabulary budget

The prose channel may leave at most **seven** system words undefined (Miller's
bound). Each gets one clause of definition at first use, never a legend three
screens away. Everything else — epoch, wave, SHA, PR numbers, module ids —
lives only in anchors, where it costs the reader nothing.

`layer / gate / task / needs-you` are the expected residents.

## The four bands

One document, one fixed order, increasing depth. Role adaptation is **scroll
depth**, not a mode switch. Stopping early is the feature: a reader who stops
after Product has read a complete, honest surface.

Bands are named by **scope, never by role** — role names invite readers to
self-exclude.

| Band            | Answers                                  | Regeneration gate     |
| --------------- | ---------------------------------------- | --------------------- |
| **Overview**    | what is this, and what needs me          | identity: rarely      |
| **Product**     | what works today, what changed for users | gate reaches `passed` |
| **Project**     | plan, progress, ledger, drift            | never (the record)    |
| **Engineering** | the system, the vocabulary, provenance   | structure change      |

## Three tenses, encoded in ink solidity

- **IS** — unmarked. Fact is the default.
- **GOAL** — a solid-outline chip that **must carry its binding** (a story or
  layer id).
- **EXP** — a dotted chip that **must carry the event that settles it**.

Two rules the honesty meter depends on:

1. **Never chip a fact.** The page-level read — _how much of this screen is
   hollow_ — is the trust gauge. Diluting it destroys the gauge. An IS chip is
   legal only to ground a fact inside an otherwise aspirational sentence.
2. **One chip governs one clause**, never a paragraph.

Accents keep their sole meanings: sienna = needs-you, olive = drift. Tense never
uses a third color.

## The canonical vocabulary

A closed set whose only definitions are the two lifecycle diagrams in
Engineering. Nothing on the page may name a part or a stage with a word outside
it.

- **Parts (8):** `ENVISION · MAP · DISPATCH · BUILD · WRAP · REFLECT` (the loop
  verbs), `CONTEXT` (the plan file every verb reads or writes), `VIEW` (this
  brief, derived from CONTEXT and nothing else).
- **Stages (5):** `PENDING → READY → IN_PROGRESS → IN_REVIEW → COMPLETED` — the
  five-cell grammar on every task row.

`failed · blocked · deferred` are **exception markers on a cell**, not stages. A
task is _at_ a stage and _in_ an exception; collapsing those loses information.

## The plainness law and its scope

Every element must carry a meaning a human needs — words, diagrams, chips,
cells, numbers, legends. This governs the **content layer**.

The **canvas layer** (the background wash) is an explicitly scoped aesthetic
surface: data-free, subtle, degradable, allowed to be beautiful. The one binding
it keeps is tonal — it follows the live/record split rather than decorating at
random.

## The delta gate

The manifest records per-block `gate` / `changed` / `stamp`. A gated block that
did not change **collapses to a one-line stamp** and stays reachable. A block
shows its content or its stamp, never both.

## Per-section self-legends

Every encoding — stage cells, tense chips, line styles — is defined where it is
used. No section assumes the reader remembers another.

## Evidence language

Narrative may not assert causality or impact — _blocks, breaks, guarantees,
unblocks, proves_ — without citing a fact id. Tense chips say _when_ a claim
holds; this rule says _whether it may be claimed at all_.

## Language

One file per run, in the owner's language. System identifiers stay untranslated
in the anchor channel. Native-language prose is the single largest
cognitive-load lever for a cold reader.
