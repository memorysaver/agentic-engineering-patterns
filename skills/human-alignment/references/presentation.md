# Presentation

Author-side layout guidance. The template owns the classes and the palette; this
file owns the judgment.

## The glance gate

**Every diagram must be readable in one look.** A 220-node graph rendered whole
is a machine artifact, not a human surface — this is a measured failure, not a
preference.

When a diagram fails the gate: bound it, decompose it, or move it behind a
disclosure. Never ship it and hope. If a bounded version still fails, **record
the failure on the page** — a diagram labeled "too dense to read at a glance,
deep-dive below" is honest; an unreadable one presented as an overview is not.

## The architecture view

Three tiers, one frame, a tab strip:

1. **Domain overview** — the embedded default. Packages grouped by rule R7.
2. **Package deep-dive** — every package the rules kept.
3. **Declared narrative** — the plan file's own account. Authored, **not**
   verified; the tier's card says so.

Code is the source of truth for edges; the plan file for meaning. Their
disagreement is a drift fact, never silently reconciled.

The rules R1–R10 live in `scripts/arch-rules.mjs` as data, not prose here — the
determinism claim requires them to be executable and auditable. The delivered
diagram carries a scan-receipt card stating what was folded and why.

**Known bound:** a deep dependency chain lays out as a wide, shallow diagram
(longest-path layering gives each depth its own column). It validates and it is
honest, but a chain of many columns is a scan, not a glance. When that happens,
say so in the section's prose rather than pretending the overview is glanceable.

## The canvas layer

A subtle background wash, most visible at the top, subdued behind content, bound
tonally to the live/record split. Data-free by definition. Degrades to a CSS
gradient when WebGL is unavailable, and the corner tag says which is running.

This is the one layer exempt from the plainness law. It is also the one layer
allowed to be beautiful.

## Navigation

Keys **jump**, they do not page: ↑/↓, PageUp/PageDown, j/k, Home/End move
between bands. No CSS scroll-snap — snap re-creates pages, and the page must
stay scannable as a whole. `#band` anchors make any position shareable.

A slim sticky rail keeps the glance overview present at any depth: current
layer, open count, needs-you count. It labels bands by **scope name**.

## Typography

Three roles, stated:

- **serif** — section heads and counts
- **sans** — body
- **mono** — kickers, chips, ids, anchors, stamps

Identifiers are mono, never italic. This is an editorial/instrument split: the
brief is half narrative, half instrument.

## Palette

The template's `:root` block is the single palette source: paper + ink, two
accents with fixed meanings (sienna = needs-you, olive = drift), and the wash
derived from the same variables. **No per-run or per-repo variation.** Saturated
color is meaning, never decoration.

## The degrade ladder

| Rung               | Behavior                                                    |
| ------------------ | ----------------------------------------------------------- |
| WebGL unavailable  | CSS paper gradient; corner tag reads `wash: css`            |
| Font CDN blocked   | system serif / sans / mono                                  |
| archify CLI absent | bounded mermaid under author guidance → labeled source text |

Embedded diagrams survive an offline reader: they are inlined at assembly, not
fetched. The only network dependency on the primary path is the font CDN.

Every rung keeps the page legible. The diagram rung is legible but degraded —
and **says so on the surface**.

## Cost of the single file

Each embedded artifact carries the archify viewer runtime (~600 KB), so a
five-diagram brief lands near 3 MB. That is fine for open-from-docs use and it
is why retention is bounded to the newest three briefs, pruned by
`assemble.mjs`, with the record kept in `manifest.json`'s `history[]`.
