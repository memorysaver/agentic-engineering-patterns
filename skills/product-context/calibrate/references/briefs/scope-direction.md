# Scope/Direction Brief: [project name]

## Product Identity

- **What:** [from opportunity.bet + product.problem]
- **For whom:** [from product.persona.description]
- **Core job:** [from product.persona.jtbd]

## What Was Built (Current Layer)

[from stories with status: completed in current layer, summarized]

- **Layer [N] delivered:** [list of capabilities, one per line]
- **Layer [N] intended:** [from product.layers[N].user_can]

## The Gap

[from /reflect observation or user's own assessment]

- **Built:** [what exists right now]
- **Expected:** [what the PM/developer imagined]
- **Delta:** [specific capabilities missing, divergent, or unwanted]

## Gap Type Assessment

Is this a **scope gap** (missing features within the right direction) or a **direction gap** (features built in the wrong direction)?

- **Scope gap indicators:** "It does the right things, just not enough of them"
- **Direction gap indicators:** "It does things I didn't ask for" or "It works but it's not what I meant"

## Questions for Calibration

1. Looking at what exists, is this 70% right or 30% right?
2. What one thing, if added, would make you say "yes, this is what I meant"?
3. Is the gap in scope (missing features) or direction (wrong features)?
4. Should we adjust the next layer to close this gap, or revise the current layer?
5. Are there completed stories that should be undone or significantly reworked?
6. Has the persona or JTBD shifted since `/envision`?

---

## Extraction Map

| Brief section     | Source                                               |
| ----------------- | ---------------------------------------------------- |
| What was built    | `stories` where `status: completed` in current layer |
| What was intended | `product.layers[current].user_can`                   |
| Goals             | `product.goals`                                      |
| MVP boundary      | `product.mvp_boundary`                               |
| Activities        | `product.activities`                                 |
