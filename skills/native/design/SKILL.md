---
name: design
description: Resolve change intent, product criteria, and technical uncertainty through contracts, BDD scenarios, decisions, and useful prototypes.
---

# Design

Read the relevant story, accepted contracts, rules, and ADRs through `aep context <id>` and task-selected sources. Establish outcome, boundaries, interfaces, failure cases, and observable success. Use `aep --skill project --ref context-sources` when important context is missing.

When product purpose, recommendation evidence or benchmark fit could change the design, use `aep --skill roadmap --ref purpose-research`. Carry the user's settled direction into canonical context through `aep --skill roadmap --ref product-context`; use existing feedback and authority to complete the document handoff without another request to write it.

Read [Design criteria](references/design-criteria.md) (`aep --skill design --ref design-criteria`) when domain structure, usability, or the user's quality expectations need resolution. For a concrete uncertainty that an experiment can answer, read [Prototype decisions](references/prototype.md) (`aep --skill design --ref prototype`). Choose exploration depth from the actual unknowns.

When behavior and work scope are concrete, author the change contract and link its story. Keep unresolved exploration as a draft with its next decision or experiment. Keep drafts in `docs/design/`, accepted tradeoffs in `project-roadmap/decisions/`, and scenario deltas under the change, honoring configured stores. Use `aep change new --file <file>` and `aep decision new --file <file>`; inspect command help for operation syntax.

Read [BDD contracts](references/bdd.md) (`aep --skill design --ref bdd`) when writing scenarios and [Record inputs](references/records.md) (`aep --skill design --ref records`) when creating initial records. Run `aep spec check --change <id>` and `aep check`; structural success still needs semantic validation against the task.

Accept through `aep change accept <id> --by <actor>` when current authority supports the decision. Preserve unresolved intent as an explicit unknown. Handoff includes acceptance sources, observed design evidence, relevant constraints and verification path.

Complete the requested context/design deliverable with durable linked artifacts. When the request also authorizes implementation, continue with `aep --skill implement`, then validation and delivery toward the carried endpoint. A design-only task finishes with the selected design and explicit remaining work.
