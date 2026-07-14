# File Resolution — Split Mode vs V1 Mode

How every product-context skill decides where the product context lives. Run the probe, then apply the matching case. A skill's own SKILL.md states only what it reads and writes in each mode; the mode semantics below are canonical.

## Probe

```bash
ls product/index.yaml 2>/dev/null && echo "SPLIT MODE" || echo "V1 MODE"
ls product-context.yaml 2>/dev/null
```

## Cases

- **Split mode** — `product/index.yaml` exists. Stable intent (opportunity, personas, capabilities, activities, quality dimensions, constraints) lives in `product/index.yaml`; mutable operational state (architecture, stories, topology, cost, changelog) lives in `product-context.yaml`. Multi-journey products may add per-capability maps under `product/maps/<capability-id>/`.
- **V1 mode** — only `product-context.yaml` exists. Everything lives in that single file. When a skill runs interactively at a natural boundary (envision, map), offer the migration once: "Migrate to split mode (product/index.yaml + product-context.yaml) or keep the single file?" — never migrate silently.
- **Neither exists** — new project. `/aep-envision` creates it (default: split mode, creating `product/`). Every other skill stops and routes to `/aep-envision` (then `/aep-map`) instead of inventing files.

## Rules

- Detection is automatic — the user never picks a mode manually, and skills never ask which mode is active.
- Preserve all sections a skill is not updating, in either mode.
- Stable design artifacts (object model, object maps, calibration files) live under `product/` in both modes — create the directory in V1 mode when needed.
