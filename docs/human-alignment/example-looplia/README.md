# Reference example: a real brief — **currently failing evaluation**

> Two independent generator/evaluator rounds have failed this artifact. Read
> [`eval-findings.md`](eval-findings.md) before treating anything here as
> exemplary. It is committed as evidence of where the pipeline stands, not as a
> model to copy.

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
  31 are `data-fact` bindings resolved at load. The agent wrote no digit.
- **Two-phase gate honesty.** The current layer's gate is `scripted_passed`, so
  the Product band renders it under an `EXP` chip naming the acceptance run that
  would settle it — not as a shipped capability.
- **Mining before conceding.** Three things an earlier build reported as
  underivable are derived here instead: spend comes from `stories[].cost_usd`
  (\$735.15 across 34 tasks) rather than the zeroed roll-up; the concept-to-code
  binding is measured from `stories[].module` × `files_affected` rather than
  waiting on a schema field; and the plan-versus-work vocabulary gap (44 modules
  in use, 32 declared) replaces a meaningless comparison against package names.
- **Silence over fabrication.** `schema_absent` still records the predicates
  that genuinely cannot be derived, so an empty result is distinguishable from
  an unasked question.
- **Engineering is prospective.** The band answers what exists now, what the
  work record says each unit carries, and where the queued design lands — eight
  net-new subsystems, none of which gets a new home.
- **A Design Option Set with its material.** One trigger fired: seven subsystems
  planned into one unit across 85 files, 17 of 21 pairings sharing no file. The
  brief gives three options including leaving it alone, each with what it buys,
  what it costs in this project's terms, a design sketch, and what would settle
  it — plus the ranking criterion, stated so it can be rejected.

## Known limitation, stated rather than hidden

The "what exists now" view is 18 units laid out by longest-path layering, which
for a deep dependency graph reads wide and shallow. It passes archify's layout
gates and every edge is real, but a graph that wide is a scan, not a glance —
the P1 glance gate in `references/checklist.md` is only partly met. The earlier
attempt to fix it by synthesizing a "domain" layer from package-name prefixes
was rejected (decision doc, revision 8): it compressed 19 packages into 14
groups, 11 of them singletons, and discarded the project's own prose vocabulary
to do it. Better compression has to come from something that carries meaning.

## Reproducing it

The facts change every time the consumer's HEAD moves — that is the point of the
design, and it happened during development: one drift fact cited in the decision
doc (a gate with 38 criteria and zero coverage) had already resolved itself
before the implementation shipped. Re-running against looplia today will not
reproduce this file byte for byte; re-running at `65e359c6` reproduces the
diagrams byte for byte and the facts exactly, while the prose is authored fresh.
