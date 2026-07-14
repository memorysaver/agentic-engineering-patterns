# Object Map Drafts (UI-facing capabilities)

`/aep-map` produces these drafts during story decomposition; `/aep-model` consumes them, presents each for a short human review gate, and flips `status: approved`. Map owns drafting; model owns approval.

After stories are decomposed, produce a **draft** noun-first Object Map for each UI-facing capability (a capability that declared the `object-model` quality dimension, or `visual-design`/`ux-flow`, or has user-facing stories). This is the bridge from the verb-first story map to the UI — it stops build agents from inventing one-step-one-screen task-wizard UIs.

Mine the draft with the ORCA rounds (Objects → Relationships → CTAs → Attributes → screens) from `product.activities`, `stories[].description`, and `architecture.domain_model`. See the `/aep-model` skill and its `references/orca-process.md` for the derivation, and `templates/object-model-schema.yaml` + `templates/object-map-schema.yaml` for the structure. Write:

- `product/object-model.yaml` — cross-capability object ontology (`provenance.reviewed: false`)
- `product/maps/<capability>/object-map.yaml` — per UI-facing capability, **`status: draft`**

Use the `capabilities[]` ids for the `<capability>` path segment. In v1 / single-journey products (no `capabilities[]`), use a single default capability = the project slug (`product:` / `project:` in the YAML) and set every UI story's `coverage` entry under that one map.

## Drafts only — never mark them approved

Object boundaries and IA are high-impact design decisions. `/aep-model` presents the draft for a short human review gate and flips `status: approved`. Dispatch/launch refuse UI-facing stories without an approved Object Map.

## Re-runs invalidate approval

If a later `/aep-map` run re-decomposes stories or activities under a capability whose object-map is already `approved`, flip that map's `status: stale` (and `provenance.reviewed: false` on the shared `object-model.yaml` if its objects changed). The dispatch/launch gates treat `stale` like `draft` — they abort until `/aep-model` re-approves the delta.

## Pure-backend / CLI projects

If a project has no UI-facing capability, skip this step — produce no Object Map drafts.
