---
name: roadmap
description: Maintain product direction, journeys, collaboration containers, and decision records, or explain current progress from recorded evidence.
---

# Roadmap and ledger

Distinguish desired product direction from actual implementation and delivery history. Keep backbone and journeys in `project-roadmap/`, ADRs in `project-roadmap/decisions/`, and work in `project-ledger/`, honoring configured store paths.

For a new opportunity or work breakdown, read [Product context](references/product-context.md). Use `aep roadmap`, `aep decision`, `aep story`, `aep layer`, `aep wave`, and `aep release` for records; inspect each command's help for its input. Layers organize a theme or owner; waves/releases organize coordination and delivery. Explicit dependencies and gates establish readiness.

Capture tradeoffs and rejected alternatives in decisions. Accept or supersede them with attribution under task authority. Detailed acceptance belongs in a linked change; read `aep --skill design` when that contract needs resolution. Maintenance or research can exist independently of a product journey.

For a progress summary, read [Status from evidence](references/status.md). Check references with `aep check` and chronology with `aep timeline`. Keep unknown historical dates unknown; recording time establishes when AEP recorded an event.
