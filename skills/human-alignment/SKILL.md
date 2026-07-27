---
name: aep-human-alignment
description: >-
  Answers where a project stands, what is owed to a human and for how long, and
  what changed since you last looked — from product-context.yaml. Use for
  project pulse, status check, what needs me, or a project brief; not planning
  (/aep-envision).
---

# Human Alignment

Turn `product-context.yaml` into **one self-contained HTML page** that answers the
three questions a returning human actually has: _where are we, what needs me,
where did reality drift from intent?_

Numbers are derived by code. Prose is written by you. Both are labeled, and an
independent audit refuses to ship a page that blurs the line.

Rationale and the decisions behind every rule:
[`docs/decisions/human-alignment.md`](https://github.com/memorysaver/agentic-engineering-patterns/blob/main/docs/decisions/human-alignment.md).

## The contract in one paragraph

Every read is a **first read**. Each surfaced item is a plain-language sentence
first — what happened, to what, why the reader cares — with the system
identifier demoted to a small provenance anchor beneath it. Facts are unchipped;
only non-facts carry a tense chip. **You never author a number.**

## Three emissions, three clocks

The facts this surface rests on do not share a clock (decision doc D10). The
attention set survives ~110 hours; story-status counts survive ~1. Fusing them
into one artifact means re-authoring the slow prose at the fast plane's rate,
and every re-authoring is a fresh chance to write something wrong. So there are
three outputs, not one:

| Emission                                           | Clock                         | Form                                              |
| -------------------------------------------------- | ----------------------------- | ------------------------------------------------- |
| **1 · the fact plane** (`derive.mjs` → facts JSON) | every plan-file commit        | data, no prose                                    |
| **2 · the pulse** (`pulse.mjs`)                    | whenever you ask              | events, computed at read time, never written down |
| **3 · the orientation document** (the brief)       | per layer, not per invocation | careful prose                                     |

Emission 2 is the one to reach for when the question is _what happened since I
last looked_. It asserts nothing, so it cannot assert wrongly:

```bash
node scripts/pulse.mjs --repo . [--since <commit>] [--json]
```

It prints what you are owed and for how long, the transitions since your cursor,
what needs you (dependency-ordered), and open work that has stopped moving. The
cursor defaults to the last orientation document's commit.

Emission 3 — the rest of this file — is worth its cost only at its own clock.
Generating it per invocation is the mistake revisions 1–9 made.

## Steps

Each phase ends in a checkable postcondition. Do not start a phase whose
predecessor's postcondition is unmet.

### 0 · Preflight

`product-context.yaml` must exist (else stop and point to `/aep-envision`). Read
`docs/human-alignment/manifest.json` for the delta baseline.

→ _baseline commit known, or first run declared._

### 1 · Derive

```bash
node scripts/census.mjs --context product-context.yaml   # what am I blind to?
node scripts/derive.mjs --context product-context.yaml --repo . \
  --out docs/human-alignment/facts.json [--code-graph docs/human-alignment/code-graph.json]
```

The facts JSON is the **only legal source of numbers**. It validates against
`scripts/facts.schema.json` or the run stops.

The census classifies every populated path in the plan file against
`scripts/source-census.json` as `derived` / `ignored` (**with a reason**) /
`unhandled`. An unhandled path is a field this brief cannot see — every content
defect the first implementation shipped was one. Add a rule before proceeding.

→ _census reports zero unhandled paths; facts JSON exists and validates._

### 1.5 · Scan (architecture)

```bash
node scripts/scan-workspace.mjs --repo . --out docs/human-alignment/code-graph.json
node scripts/arch-rules.mjs --code-graph docs/human-alignment/code-graph.json \
  --context product-context.yaml --out-dir <build-dir>
for tier in overview packages declared; do
  node scripts/receipt-consumer.mjs --type architecture \
    --in <build-dir>/architecture-$tier.architecture.json --out <build-dir>/architecture-$tier.html
done
node scripts/receipt-consumer.mjs --type workflow  --in assets/aep-loop.workflow.json      --out <build-dir>/aep-loop.html
node scripts/receipt-consumer.mjs --type lifecycle --in assets/story-states.lifecycle.json --out <build-dir>/story-states.html
```

Re-run `derive.mjs` with `--code-graph` afterwards so the declared-vs-actual
drift fact is included. No workspace manifest, or no archify CLI (exit 3)? The
architecture view **degrades and says so on the page** — it never blocks.

→ _artifacts delivered `code-verified`, or the view is marked degraded._

### 2 · Author

Copy `assets/template.html` and fill its `{{SLOTS}}`. **Read the template's
contract comment before writing a single section** — it is the only source of
CSS class names and of the `data-fact` binding convention.

The authoring rules are in [`references/guideline.md`](references/guideline.md).
The five that fail an audit fastest:

1. **Answer first** — every section opens with one plain sentence that _is_ the
   section's conclusion.
2. **Translate** — story titles and changelog entries never surface verbatim;
   re-author each as a consequence sentence bound to its id in the anchor.
3. **No naked numbers, in digits or in words.** Every number is `data-fact`.
   `five`, `eight`, `three quarters` are numbers too — that hole once shipped
   "five tasks in two days" when the truth was eight.
4. **Every assertive block cites what it rests on** — `data-claims="<paths>"`,
   or `data-authored` if it is narrative that can cite nothing. A claim the
   facts do not carry is not a sentence to write more carefully; it is a fact to
   derive first.
5. **Fold the queue** — backlog collapses to one sentence per layer, full list
   behind a disclosure.
6. **Never chip a fact** — an unchipped page is a page of facts; diluting that
   destroys the only trust gauge the reader has.

→ _every section rendered or stamped; no `{{SLOT}}` left._

### 3 · Audit

```bash
node scripts/audit.mjs --html <authored.html> --facts docs/human-alignment/facts.json \
  --template assets/template.html
```

After Phase 4, audit the **delivered** file too — Phase 3 runs before assembly
injects the diagrams, and nothing else re-checks afterwards:

```bash
node scripts/audit.mjs --html <delivered.html> --facts <facts.json> \
  --template assets/template.html --delivered
```

`--delivered` excludes embedded artifact payloads from the claim rules (they are
archify's prose, validated separately) while still checking that the diagrams
and the facts describe the same commit.

Failures are structured receipts (`code` · `subject` · `evidence` ·
`supportedFixes`). Apply a **listed** fix and re-run. Never guess, and never
exceed **two** correction rounds — a third failure is reported, not retried.
Then walk the judgment checks in [`references/checklist.md`](references/checklist.md).

→ _audit passes; judgment checks walked._

### 4 · Deliver

```bash
node scripts/assemble.mjs --authored <authored.html> --facts docs/human-alignment/facts.json \
  --artifacts <build-dir> --out-dir docs/human-alignment --keep 3
```

Run it **in the output directory**; never move briefs by hand. Removing one
outside this script discards the delta baseline, and the next run then reports
itself as the first. `assemble.mjs` refuses to run when the ledger names a brief
that is no longer present, and when the facts and the code graph disagree about
which commit they describe.

Writes `brief-<date>T<time>Z-<shorthash>.html`, updates `manifest.json`
(appending the prior record to `history[]`), and prunes every brief beyond the
newest three. Report the path and the delta summary in conversation — the owner
opens the file.

→ _new file exists; its name's hash equals HEAD; ≤ 3 briefs remain._

## What the framework specs own

Two derivations are **framework vocabulary**, not skill-private logic. Do not
re-invent them here:

- [`references/attention-set.md`](references/attention-set.md) — what needs a
  human, its priority order, and the per-predicate schema-tolerance rule.
- [`references/drift-facts.md`](references/drift-facts.md) — the five drift
  derivations. Hand-authored drift is banned: when nothing derives, the row is
  silent, not fabricated.

Control-plane defects — a gate that no longer describes its work, a roll-up that
disagrees with the record, a module used but never declared — are **not detected
here** (D11). They want an action, so they live in `scripts/coherence.mjs`,
shared with `/aep-validate`, which blocks on them. This skill renders what that
detector returns.

`derive.mjs` implements both. If a predicate's fields are absent, it is skipped
**and recorded** — an empty attention set is only trustworthy when the skip list
is empty too, and the page must say which it is.

## Boundaries

- **Gates**: only `passed` yields an unchipped capability. A `scripted_passed`
  gate may appear **only** under an `EXP` chip naming the acceptance run that
  would settle it. Anything lower does not reach the product band.
- **Language**: one file per run in the owner's language (an invocation
  parameter, defaulting to the repo's working language). System identifiers stay
  untranslated in anchors.
- **Not this skill**: planning (`/aep-envision`), decomposition (`/aep-map`),
  design theory (`/aep-design-lens`).

## Layout notes

Presentation bounds, the glance gate, and the degrade ladder live in
[`references/presentation.md`](references/presentation.md). The field map from
YAML path to fact is in [`references/derivation.md`](references/derivation.md).
