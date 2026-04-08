# Scope/Direction — Capture Questions

Ask these questions one at a time during `/calibrate capture` for scope-direction.

## Questions

1. **Alignment score:** On a scale of 0-100, how close is what was built to what you imagined? What accounts for the gap?
2. **Gap type:** Is this a scope gap (right direction, missing features) or a direction gap (wrong features or wrong approach)?
3. **Keep list:** What was built that you want to keep exactly as-is?
4. **Change list:** What was built that needs to change? For each item: what's wrong and what should it be?
5. **Add list:** What's missing that would make you say "this is what I meant"? Prioritize: which one thing matters most?
6. **Remove list:** Was anything built that should be removed entirely?
7. **Layer impact:** Should these changes happen in the current layer (rework) or the next layer (iterate forward)?
8. **Release line cutting:** Looking at upcoming layers, what is the absolute minimum to ship a useful release? Which features are essential vs. nice-to-have? Should any stories move between layers to hit a viable release point sooner?
9. **Framing check:** Has the persona, JTBD, or opportunity hypothesis shifted? (If yes, route to `/envision` instead)

## Output

Update `product.goals`, `product.mvp_boundary`, and/or `product.layers` in `product-context.yaml` with calibrated scope and direction. Create new stories or modify existing ones as needed.

For release line cutting: update `stories[].layer` assignments to reflect the new release boundary. Move essential stories earlier and nice-to-have stories later. This implements Jeff Patton's "release line is a pencil line" philosophy — release boundaries shift based on what you learn about scope and direction.
