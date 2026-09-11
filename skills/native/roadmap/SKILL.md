---
name: roadmap
description: Maintain product direction, journeys, collaboration containers, and decision records, or explain current progress from recorded evidence.
---

# Roadmap and ledger

Distinguish desired product direction from actual implementation and delivery history. Keep backbone and journeys in `project-roadmap/`, ADRs in `project-roadmap/decisions/`, and work in `project-ledger/`, honoring configured store paths.

For a new opportunity or work breakdown, read [Product context](references/product-context.md) (`aep --skill roadmap --ref product-context`). Use `aep roadmap`, `aep decision`, `aep story`, `aep layer`, `aep wave`, and `aep release` for records; inspect each command's help for its input. A layer encapsulates one concept: its observable outcome, design source and member stories; waves and releases organize coordination and delivery. When a design or decision breaks into several stories, create the layer before its changes and stories. Explicit dependencies and gates establish readiness.

Before recommending a product priority, technical direction or benchmark, identify evidence gaps that could change the choice. Read [Purpose-driven research](references/purpose-research.md) (`aep --skill roadmap --ref purpose-research`) when those gaps need investigation. Research depth follows the decision's uncertainty; reuse sufficient current evidence.

Present the recommendation and tradeoffs, then invite the user's view on the proposed direction. Once the direction is clear from current feedback or prior instructions, complete the authorized product-context updates in the same task using the product-context reference. Carry unresolved choices as drafts and continue settled work. A discussion-only request retains its stated boundary.

Capture tradeoffs and rejected alternatives in decisions. Accept or supersede them with attribution under task authority. Continue with `aep --skill design` to resolve the selected direction's design and behavior contract when needed. Maintenance or research can exist independently of a product journey.

For a progress summary, read [Status from evidence](references/status.md) (`aep --skill roadmap --ref status`). Check references with `aep check` and chronology with `aep timeline`. Keep unknown historical dates unknown; recording time establishes when AEP recorded an event.
