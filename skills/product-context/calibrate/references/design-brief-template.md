# Design Brief: [project name]

## Product Identity

- **What:** [from opportunity.bet + product.problem]
- **For whom:** [from product.persona.description]
- **Core job:** [from product.persona.jtbd]
- **Why now:** [from opportunity.why_now]

## Pages to Design

### 1. [Page name] ([route])

- **Purpose:** [from story description]
- **Content blocks:** [inferred from story acceptance criteria]
- **Key interactions:** [from activity definition]

### 2. [Page name] ([route])

[repeat for each page in the .5 layer]

## Design Directions

### Direction A: "[Name]"

[Description of the visual direction — mood, color family, layout approach]
**Reference products:** [2-3 existing products as mood board anchors]
**Strengths:** [what this direction does well for the persona]
**Risks:** [where it could miss]

### Direction B: "[Name]"

[...]

### Direction C: "[Name]"

[...]

## Technical Constraints

- **Stack:** [from product.constraints.required_stack + preferred_stack]
- **Current theme:** [extracted from globals.css — color space, font, radius, etc.]
- **Available components:** [list from packages/ui/src/components/]
- **Requirements:** Responsive, light + dark mode

## Deliverable

Explore the directions. Produce designs for all pages listed above.
The goal is to establish: color palette, typography, component styling,
layout patterns, and overall brand feel for the product.

---

## Extraction Map

| Brief section        | `product-context.yaml` source                            |
| -------------------- | -------------------------------------------------------- |
| What                 | `opportunity.bet` + `product.problem`                    |
| For whom             | `product.persona.description`                            |
| Core job             | `product.persona.jtbd`                                   |
| Why now              | `opportunity.why_now`                                    |
| Pages                | `stories` where `layer` = active `.5` layer              |
| Stack constraints    | `product.constraints.required_stack` + `preferred_stack` |
| Current theme        | File scan: `packages/ui/src/styles/globals.css`          |
| Available components | Directory scan: `packages/ui/src/components/`            |
