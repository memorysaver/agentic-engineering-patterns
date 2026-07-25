# Reference example: a real brief

These files are the **unedited output** of `/aep-human-alignment` run against a
real consumer repo (looplia, 396 stories, 41 layer gates, 21 workspace packages)
at commit `65e359c6`. Nothing here was hand-tuned for presentation — the prose
was authored by the agent under the skill's rules, every number was bound by
`derive.mjs`, and `audit.mjs` passed before assembly.

| File                                   | What it is                                                    |
| -------------------------------------- | ------------------------------------------------------------- |
| `brief-2026-07-25T0308Z-65e359c6.html` | the delivered brief — one self-contained file, 3.2 MB         |
| `facts.json`                           | Phase 1 output: the only legal source of numbers on that page |
| `code-graph.json`                      | Phase 1.5a output: the real package topology the scan found   |
| `manifest.json`                        | the generation ledger — timestamps, commit, content SHA-256   |

Open the HTML directly from disk; it needs no server. The five embedded archify
diagrams are inlined, so they render offline. The only network dependency is the
font CDN, which degrades to system fonts.

## What it demonstrates

- **The honesty model.** Open `facts.json` and grep the HTML for any number: all
  22 are `data-fact` bindings resolved at load. The agent wrote no digit.
- **Two-phase gate honesty.** The current layer's gate is `scripted_passed`, so
  the Product band renders it under an `EXP` chip naming the acceptance run that
  would settle it — not as a shipped capability. An earlier prototype of this
  design asserted "acceptance passed" for exactly this gate; the rule exists
  because of that failure.
- **Silence over fabrication.** `facts.json`'s `schema_absent` records three
  predicates that could not be derived from this repo's schema at all (no
  `product` section, no `status` on amendment-log entries, object maps live
  outside the plan file). The brief surfaces the absence instead of reporting
  "nothing needs you".
- **Derived drift only.** Eleven drift facts, every one with a YAML path. The
  most interesting: the code is 21 packages while the plan file declares 32
  conceptual modules with **one** name in common and no path binding — so the
  declared architecture is not code-addressable, and the brief says so rather
  than pretending the two accounts agree.
- **Deterministic architecture.** The Engineering band's diagrams came from
  `scan-workspace.mjs` → rules R1–R10 → archify `validate`/`deliver`, repaired
  only through archify's own receipts (one round for the overview, two for the
  package graph).

## Known limitation, stated rather than hidden

The domain overview lays out as a wide, shallow chain: looplia's dependency
graph is deep, and longest-path layering gives each depth its own column. It
passes archify's layout gates and it is honest, but a chain of that width is a
scan, not a glance — the glance gate in `references/checklist.md` is a P1 the
example only partly meets. Improving R7's grouping is the obvious next move.

## Reproducing it

The facts change every time the consumer's HEAD moves — that is the point of the
design, and it happened during development: one drift fact cited in the decision
doc (a gate with 38 criteria and zero coverage) had already resolved itself
before the implementation shipped. Re-running against looplia today will not
reproduce this file byte for byte; re-running at `65e359c6` reproduces the
diagrams byte for byte and the facts exactly, while the prose is authored fresh.
