# Visual Design — Capture Questions

Ask these questions one at a time during `/calibrate capture` for visual-design.

## Questions

1. **Direction:** Which direction did you choose? (or describe the hybrid)
2. **Palette:** Primary, secondary, accent, destructive, muted colors (any format — hex, rgb, hsl — the skill converts to oklch for `globals.css`)
3. **Typography:** Font family changes? Size scale adjustments? Weight usage patterns?
4. **Components:** Recurring patterns observed? Card styles, button variants, spacing rhythm?
5. **Layout:** Max-width, grid vs flex, sidebar vs top-nav, content density?
6. **Brand signals:** What visual cues communicate "this is [product name]"?
7. **Reference files:** Did you save HTML/CSS/screenshots to `docs/design-references/`? List them.

## Output Artifact

Write to `calibration/visual-design.yaml` using schema at `references/schemas/visual-design-schema.yaml`.
Also update `globals.css` with captured palette (convert to oklch).
