---
name: aep-model
description: Object-first UI structure modeling (OOUX/ORCA) for UI-facing products. Use after /aep-map and before dispatching UI-facing stories, or when the user says "object model", "object map", "objectify", "OOUX", "ORCA", "noun-first IA", "what objects/screens". Mines a draft Object Map (objects → relationships → CTAs → attributes → screens) from the story map, presents it for a short human review gate, and on approval writes the noun-first blueprint that build agents follow. Bridges the verb-first story map to the UI so agents stop inventing task-wizard screens per story.
---

# Model

Turn the verb-first story map into a noun-first **Object Map** before UI gets
built. AEP's spine (`/aep-envision` → `/aep-map`) plans what the user _does_;
this skill plans the _objects_ the user acts on — which objects appear, what their
fields are, how they nest, and what actions hang off each. Without it, build
agents invent screen structure per story, which drifts into disjointed
task-wizard UIs. With it, they inherit one consistent object-oriented blueprint.

> **The one rule:** a story-map slice cuts _scope_, not _interface type_. Never
> translate the backbone one-step-one-screen into a wizard. MVP slice ≠ wizard.

**Where this fits:**

```
/aep-envision → /aep-map → /aep-model → [ /aep-calibrate ] → /aep-dispatch → /aep-launch → /aep-build → /aep-wrap → /aep-reflect
                        ▲ you are here (UI-facing products)
```

**Session:** Main, interactive with user (object boundaries + IA need human review)
**Input:** Product definition (`product/index.yaml` split mode, `product-context.yaml` v1) + `stories`, `architecture.domain_model` from `product-context.yaml`
**Output:** `product/object-model.yaml` (cross-capability ontology) + one `product/maps/<capability>/object-map.yaml` per UI-facing capability (`status: approved`); thin `calibration.history` entry + `changelog` in `product-context.yaml`

**Schemas:** `templates/object-model-schema.yaml`, `templates/object-map-schema.yaml`.
**Process detail:** `references/orca-process.md` (ORCA round-by-round derivation + the object-first/task-oriented decision framework).

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

**File Resolution:**

```bash
ls product/index.yaml 2>/dev/null && echo "SPLIT MODE" || echo "V1 MODE"
cat product-context.yaml
```

- **Split mode** (`product/index.yaml` exists): read product definition,
  `personas`, `capabilities`, `activities`, `quality_dimensions` from
  `product/index.yaml`; read `stories`, `architecture.domain_model` from
  `product-context.yaml`. Write `product/object-model.yaml` and
  `product/maps/<capability>/object-map.yaml`.
- **V1 mode**: read everything from `product-context.yaml`. Still write the
  standalone artifacts under `product/` (create the directory) — the object model
  is a stable design file, never inlined into operational YAML.

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

Run the four ORCA rounds per `references/orca-process.md`. This step is
agent-driven — mine, don't ask yet.

1. **Round O — Objects (Noun Foraging):** forage nouns from `product.activities`,
   `stories[].description`, `product.problem`, `personas`, and
   `architecture.domain_model`. Promote user-perceived things to objects; demote
   implementation nouns. Cross-link `backs_onto` to domain entities. Record
   `source_evidence` + `confidence`.
2. **Round R — Relationships (Nested Object Matrix):** for each object pair, set
   cardinality + whether nested/navigable. Cross-capability links → object-model;
   capability-local → object-map.
3. **Round C — CTAs (object × role matrix):** hang every story/activity verb onto
   the object it acts on, tagged by persona, with placement + priority + the
   `from_story` it came from.
4. **Round A — Attributes:** rank each object's fields core / secondary /
   metadata.
5. **Round 4 — Representation hints:** project primary objects into structural
   `collection` + `detail` views; pick each capability's `anchor_object`. IA only,
   no visual design.

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
redesign workshop. Heavy taste decisions (look, voice) are NOT this skill's job;
they stay in `/aep-calibrate` (visual-design, copy-tone) and journey/page/
transition stays in ux-flow.

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

If it fails, fix before committing (see `templates/product-context-schema.yaml`
guidance: quote list items with colons, flatten nested sub-lists, escape quotes).

```bash
# Resolve $BASE (integration branch) — see git-ref "Integration Branch" (override → develop → main)
BASE=$(git config --get aep.integration-branch 2>/dev/null || true)
[ -z "$BASE" ] && { git show-ref --verify --quiet refs/heads/develop \
  || git show-ref --verify --quiet refs/remotes/origin/develop; } && BASE=develop
BASE=${BASE:-main}

git pull --ff-only origin "$BASE"
git add product/ product-context.yaml
git commit -m "feat: object model — approved object-first IA for <capabilities>"
git push origin "$BASE"
```

---

## How downstream consumes the Object Map

- **`/aep-dispatch`** injects only the **slice** for the objects a story touches
  (from `coverage`) into the story's context package — not the whole model — and
  **refuses** to dispatch a UI-facing story whose capability lacks an `approved`
  object-map (run `/aep-model` first), mirroring the calibration gate.
- **`/aep-launch`** aborts a UI-facing story with no approved object-map.
- **`/aep-build`** treats the injected slice as binding: objects, their core
  attributes, CTA placement, and screen structure come from the map;
  visual-design/copy-tone/ux-flow calibration still own look, voice, and journey.
- **`/aep-validate`** Mode A runs the completeness checks (coverage, anchors,
  task-flow reasons).

---

## Anti-Patterns

- **Backbone → wizard 1:1.** The deepest trap. A slice is scope, not screen shape.
- **Designing visuals here.** No palette/typography/spacing — that's
  `/aep-calibrate visual-design`. This skill is structure only.
- **Modeling backend entities as objects.** If the user never perceives it, it's
  `architecture`, not the object model.
- **Skipping the gate.** Auto-approving object boundaries reproduces the guesswork
  the skill exists to prevent. Always run Step 2.
- **Fat operational YAML.** Never inline object-map bodies into
  `product-context.yaml`; keep them under `product/` with thin references.

---

## Next Step

Object model approved. Heavy taste dimensions next (if planned), then dispatch:

```
/aep-calibrate visual-design   # look & feel (optional, if a .5 layer plans it)
/aep-dispatch                  # inject object-map slices and start building
```
