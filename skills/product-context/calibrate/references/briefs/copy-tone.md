# Copy/Tone Brief: [project name]

## Product Identity

- **What:** [from opportunity.bet + product.problem]
- **For whom:** [from product.persona.description]
- **Core job:** [from product.persona.jtbd]

## Product Voice (Inferred)

[from product.persona — infer the voice the product should have for this audience]

- **Audience technical level:** [from persona description]
- **Context of use:** [when/where do they interact with this product?]
- **Expected tone:** [formal/casual/technical/friendly — inferred from persona + domain]

## Sample UI Text (Current)

[scan components for actual text content]

| Location                           | Current Text    | Observation                                  |
| ---------------------------------- | --------------- | -------------------------------------------- |
| [button/heading/error/empty state] | "[actual text]" | [too formal / too casual / too vague / etc.] |

## Tone Mismatch

[from /reflect observation or ambient detection]

- **Current feel:** [how the product reads now]
- **Expected feel:** [how it should read]
- **Specific examples:** [texts that feel wrong and why]

## Questions for Calibration

1. Pick 3 products whose voice you'd steal for this product.
2. Pick 1 product whose voice you'd actively avoid.
3. This error message: "[current]" — how would you rewrite it?
4. Button says "[current label]" — is that what a user would look for?
5. Empty state: what should it say? Encouraging? Instructional? Minimal?
6. How formal should headings be? (e.g., "Dashboard" vs "Your Dashboard" vs "Here's what's happening")
7. Technical jargon: embrace it (audience expects it) or translate it?

---

## Extraction Map

| Brief section | Source                                                     |
| ------------- | ---------------------------------------------------------- |
| Audience      | `product.persona.description`                              |
| JTBD context  | `product.persona.jtbd`                                     |
| Current text  | File scan: UI components with string literals              |
| Brand context | `calibration/visual-design.yaml` brand section (if exists) |
