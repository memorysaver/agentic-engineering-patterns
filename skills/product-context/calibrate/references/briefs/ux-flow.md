# UX Flow Brief: [project name]

## Product Identity

- **What:** [from opportunity.bet + product.problem]
- **For whom:** [from product.persona.description]
- **Core job:** [from product.persona.jtbd]

## Current Journey

[from product.activities, ordered left-to-right as a narrative]

1. **[Activity name]** — [description]
2. **[Activity name]** — [description]
3. [...]

## What Was Built

[from stories with status: completed in current/recent layer, summarized by page/route]

## Observed Friction

[from /reflect observation that triggered this calibration]

- **Where in the journey:** [activity or transition]
- **What feels wrong:** [description — confusing navigation, dead ends, wrong information density, etc.]
- **User expectation vs. reality:** [the gap]

## Flow Options

### Option A: "[Name]"

[Restructured flow — reordered steps, combined/split pages, changed transitions]
**Tradeoff:** [what improves, what gets harder]

### Option B: "[Name]"

[Alternative restructured flow]
**Tradeoff:** [what improves, what gets harder]

## Questions for Exploration

1. Walk through Option A mentally. Where do you hesitate?
2. Walk through Option B. Where do you hesitate?
3. Is there a step in the current flow that users skip or fight against?
4. What's the one transition that must feel instant/seamless?
5. Should any pages be combined? Should any be split?

---

## Extraction Map

| Brief section     | Source                                               |
| ----------------- | ---------------------------------------------------- |
| Current journey   | `product.activities` (ordered by `order` field)      |
| What was built    | `stories` where `status: completed` in recent layers |
| Friction          | `/reflect` observation text                          |
| Technical context | File scan: `routes/` or `pages/` directory           |
