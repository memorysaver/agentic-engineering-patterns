# Design Operating Modes

`/aep-design` auto-detects its mode at startup from the presence of `product-context.yaml`
(`ls product-context.yaml 2>/dev/null`).

## Standalone mode — no `product-context.yaml`

The feature lifecycle runs independently. There is no product-wide context to load; go straight
to Prerequisites and the design phases with the user's feature idea as the only input.

## Product-cycle mode — has `product-context.yaml`

The feature is one story in a larger product lifecycle:
`/aep-envision` → `/aep-map` → `/aep-dispatch` → `/aep-design`. Before starting the phases:

- **Read `product-context.yaml`** for project-wide context (goals, architecture, conventions).
- **If a story was dispatched** (the YAML has `openspec_change` set), load that story's
  acceptance criteria, interface obligations, and the relevant architecture module so the
  proposal satisfies the contract `/aep-dispatch` already committed to.
- **The OpenSpec change already exists** when dispatched — Phase 2's `/opsx:propose` _refines_
  the existing change rather than starting from scratch.

The product-cycle branch changes what feeds the phases; it does not change the phase sequence
itself (Explore → Propose → Review → Commit).
