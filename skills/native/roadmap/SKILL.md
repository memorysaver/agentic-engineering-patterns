---
name: roadmap
description: Maintain product direction, journeys, collaboration containers, and decision records without requiring every maintenance ticket to map to a journey.
---

# Roadmap and ledger

Distinguish desired product direction from current implementation and delivery history. Keep backbone and journeys in `project-roadmap/`, ADRs in `project-roadmap/decisions/`, and work in `project-ledger/`. Link code and evidence when asserting implemented behavior.

Use `aep roadmap`, `aep decision`, `aep story`, `aep layer`, `aep wave`, and `aep release` to create or inspect records. Read each command's help for its input contract. Layers organize a theme or owner; waves and releases organize coordination and delivery scope. Use explicit dependencies and gates instead of inferring readiness from numeric labels.

Capture tradeoffs and rejected alternatives in a decision record. Accept or supersede it through the decision commands with attribution. Use a linked change contract for detailed BDD acceptance. Maintenance and research work can have no product-journey reference.

Check references with `aep check` and inspect chronology with `aep timeline`. Unknown historical dates remain unknown; event recording time does not establish merge or release time.
