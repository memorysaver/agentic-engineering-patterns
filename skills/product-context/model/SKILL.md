---
name: aep-model
description: OOUX/ORCA object modeling for UI-facing products. Run after /aep-map, before dispatching UI-facing stories, or on "object model", "object map", "OOUX", "ORCA", "noun-first IA". Mines a draft Object Map, gates it through a short human review, then writes the noun-first blueprint so build stops inventing task-wizard screens per story.
---

# Model

Turn the verb-first story map into a noun-first **Object Map** before UI gets
built. AEP's spine (`/aep-envision` → `/aep-map`) plans what the user _does_;
this skill plans the _objects_ the user acts on — which objects appear, what their
fields are, how they nest, and what actions hang off each. Structure only: look,
voice, and journey stay in `/aep-calibrate` (visual-design, copy-tone, ux-flow).

> **The one rule:** a story-map slice cuts _scope_, not _interface type_. Keep the
> backbone's one-step-one-screen as object-first structure; translating it into a
> wizard is the deepest trap this skill exists to prevent. MVP slice ≠ wizard.

**Where this fits:**

```
/aep-envision → /aep-map → /aep-model → [ /aep-calibrate ] → /aep-dispatch → /aep-launch → /aep-build → /aep-wrap → /aep-reflect
                        ▲ you are here (UI-facing products)
```

**Session:** Main, interactive with user (object boundaries + IA need human review)
**Input:** Product definition (`product/index.yaml` split mode, `product-context.yaml` v1) + `stories`, `architecture.domain_model` from `product-context.yaml`
**Output:** `product/object-model.yaml` (cross-capability ontology) + one `product/maps/<capability>/object-map.yaml` per UI-facing capability (`status: approved`); thin `calibration.history` entry + `changelog` in `product-context.yaml`

**Schemas:** `templates/object-model-schema.yaml`, `templates/object-map-schema.yaml`.
**Process detail:** the four ORCA rounds + the object-first/task-oriented decision
framework are canonical in `references/orca-process.md` — read it before mining in
Step 1.

---

## When this skill applies

Run `/aep-model` for **UI-facing** products/capabilities only. A capability is
UI-facing when it declares the `object-model` quality dimension (set by
`/aep-envision`), or declares `visual-design`/`ux-flow`, or has user-facing stories
(non-null `activity` whose module is `kind: ui`). Pure-backend/CLI products skip it
— there are no user-perceived objects to model.

If nothing is UI-facing, say so and route the user straight to `/aep-dispatch`.

---

## Before Starting

**File resolution:** the split-vs-V1 probe and mode semantics are canonical in
`references/file-resolution.md` — read it when the active mode is unclear. In
**split mode** model reads the product definition (`personas`, `capabilities`,
`activities`, `quality_dimensions`) from `product/index.yaml` and `stories`,
`architecture.domain_model` from `product-context.yaml`; in **V1 mode** it reads
everything from `product-context.yaml`. In both modes it writes the standalone
artifacts under `product/` (create the directory in V1) — the object model is a
stable design file, never inlined into operational YAML.

If `stories` is empty, run `/aep-map` first. If no product definition exists, run
`/aep-envision` first.

**Mode detection:**

- **Establishment** — no `product/object-model.yaml` yet → full ORCA over all
  UI-facing capabilities.
- **Extension** — it exists → focused pass over only NEW objects/capabilities not
  yet covered (e.g., a later layer introduced new activities). Do not redo
  approved maps; extend them and re-gate only the delta.

---

## Step 1: Generate (or refine) the Draft (ORCA, automated)

**If `/aep-map` already wrote draft artifacts** (`product/object-model.yaml` and
`product/maps/<cap>/object-map.yaml` with `status: draft`), **read and refine them**
— do not regenerate from scratch. Preserve their `provenance`/`source_evidence`,
fill gaps, and fix obvious errors. Generate fresh only when no draft exists. (Set
`provenance.generated_by`/`status` to reflect reality — `aep-map` for an untouched
draft, refined in place here.)

Run the four ORCA rounds per `references/orca-process.md`, then the representation
pass — this step is agent-driven, so mine, don't ask yet:

1. **Round O — Objects (Noun Foraging):** forage nouns, promote user-perceived
   things, demote implementation nouns, record `source_evidence` + `confidence`.
2. **Round R — Relationships (Nested Object Matrix):** set cardinality + nesting
   per object pair; cross-capability → object-model, capability-local → object-map.
3. **Round C — CTAs (object × role matrix):** hang every story/activity verb onto
   its object, tagged by persona, with `placement`, `priority`, and `from_story`.
4. **Round A — Attributes:** rank each object's fields core / secondary / metadata.
5. **Representation hints:** project primary objects into `collection` + `detail`
   views and pick each capability's `anchor_object`. IA only, no visual design.

Write:

- `product/object-model.yaml` — the cross-capability ontology
  (`provenance.reviewed: false`).
- `product/maps/<capability>/object-map.yaml` per UI-facing capability with
  **`status: draft`**, including the `coverage` index (story → objects/screens; screen
  ids use the canonical `<object-id>:<view>` grammar). Use `capabilities[]` ids for
  `<capability>`; in v1/single-journey (no `capabilities[]`) use one default
  capability = the project slug.

Default every flow to `object_first`. Only add an `interaction_modes` entry to
mark a flow `task_oriented`, with a reason grounded in the decision framework.

**Postcondition:** `product/object-model.yaml` and one draft `object-map.yaml` per
UI-facing capability exist; every noun foraged from an activity maps to an object
(or is justified implementation-only) and every UI story's verbs map to a CTA
(`coverage` complete). Nothing has been asked of the user yet.

---

## Step 2: Review Gate (the human decides)

Object boundaries and IA are the highest-leverage design decisions here — same
error-cost logic as the `/aep-map` System Map gate. Present the draft compactly
(objects, the NOM, primary screens, any task-flow exceptions) and ask a SHORT set
of high-leverage questions, one at a time:

1. **Object boundaries** — are these the objects the user actually thinks in? Any
   wrong names, bad merges, or missing splits? (Surface every low-`confidence`
   object explicitly.)
2. **Primary anchor** — per capability, which object should the user see / choose
   first?
3. **Task-flow exceptions** — which flows should be explicit wizards (onboarding /
   checkout / one-shot) instead of object-first? Capture the reason.

Apply the answers to the draft. Keep the questions few — this is a gate, not a
redesign workshop; taste decisions stay in `/aep-calibrate` (see the intro).

---

## Step 3: Approve & Write

On approval:

1. Set each reviewed `product/maps/<capability>/object-map.yaml` →
   `status: approved`, `approved_by: human`, `approved_at: <ISO 8601>`.
2. Set `product/object-model.yaml` `provenance.reviewed: true` (+ `reviewed_at`).
3. Back-annotate stories: add `object_model_refs` to UI stories the map covers,
   e.g. `object_model_refs: ["product/maps/dashboard/object-map.yaml#order"]`.
   Keep it a thin reference — the map body never goes into `product-context.yaml`.
4. Append a thin record to `calibration.history` in `product-context.yaml`:

   ```yaml
   - dimension: object-model
     calibrated_at: "<ISO date>"
     calibrated_from_layer: <layer> # the active layer whose UI stories this approval unblocks
     mode: establishment # or extension
     artifact_path: "product/maps/<capability>/object-map.yaml"
     summary: "Approved object-first IA for <capability> — N primary objects, M task-flow exceptions"
   ```

5. Append a `changelog` entry (`type: map_update`, author: human,
   `sections_changed: [calibration, stories]`).

---

## Step 4: Validate YAML & Commit

```bash
# Validate every YAML touched
python3 -c "import yaml,glob; [yaml.safe_load(open(f)) for f in ['product/object-model.yaml','product-context.yaml']+glob.glob('product/maps/*/object-map.yaml')]; print('YAML OK')"
```

If it fails, fix before committing — see `references/yaml-guardrails.md` for the
common `product-context.yaml` pitfalls (colons in list items, embedded quotes,
`@`/`{`/`}`, nested sub-lists).

Then commit the design artifacts directly to the integration branch per
`/aep-git-ref` "Control-Plane Commits" (resolve `$BASE` per `/aep-git-ref`
"Resolving `$BASE`"): `git add product/ product-context.yaml`, commit
`feat: object model — approved object-first IA for <capabilities>`, push to `$BASE`.

**Postcondition:** `YAML OK` printed and the commit is on `$BASE`.

---

## Next Step

Object model approved. Heavy taste dimensions next (if planned), then dispatch:

```
/aep-calibrate visual-design   # look & feel (optional, if a .5 layer plans it)
/aep-dispatch                  # inject object-map slices and start building
```
