# ORCA Process — Deriving the Object Map from AEP Artifacts

How `/aep-model` runs OOUX's ORCA process over what AEP already produced. This is
the noun-first bridge between the verb-first story map and the UI. Full background
and citations: `docs/research/ooux-object-modeling.md`.

> **The one rule that matters most:** never translate the story-map backbone
> one-step-one-screen into a wizard. A slice cuts _scope_ ("what to learn / ship
> first"), not _interface type_. MVP slice ≠ wizard. The Object Map exists to stop
> exactly that failure.

ORCA = **O**bjects → **R**elationships → **C**alls-to-action → **A**ttributes.
Sophia Prater's full process is four iterative rounds (Discovery → Requirements →
Prioritization → Representation); `/aep-model` runs a compressed,
artifact-grounded version because AEP already did the upstream research.

---

## Inputs (what to read before mining)

| ORCA needs                     | AEP source                                                                                                                      |
| ------------------------------ | ------------------------------------------------------------------------------------------------------------------------------- |
| Candidate nouns                | `product.activities[]`, `stories[].description.what_changes` (string or `{what_changes, why}`), `product.problem`, `personas[]` |
| Entity backing + relationships | `architecture.domain_model[]`, `architecture.modules[].key_concepts`                                                            |
| Verbs (→ CTAs)                 | activity names, `stories[].description`, acceptance criteria                                                                    |
| Roles                          | `product/index.yaml` `personas[]`                                                                                               |
| Capability scoping             | `product/index.yaml` `capabilities[]`, `product/maps/<cap>/frame.yaml`                                                          |

Read story `description` defensively — it is a string in some projects and a
`{what_changes, why}` object in others. Handle both.

**Capability scoping fallbacks:** `frame.yaml` only exists for multi-journey
products (`/aep-envision` skips it for single-journey). If it is absent, scope the
capability from `product.activities` + the stories whose `capability` (or, in v1,
all UI stories) belong to it. In v1 / single-journey products there is no
`capabilities[]` — use one default capability whose id is the project slug.

---

## Round O — Objects (Noun Foraging)

1. **Forage nouns** from the inputs above. Collect every concrete thing the user
   refers to or acts on.
2. **Promote to objects** the nouns that are (a) in the user's mental model and
   (b) something the user views or acts on. Demote pure implementation nouns
   (token, cache, queue) — they belong to `architecture`, not the object model.
3. **Disambiguate object vs. backend entity.** A `domain_model` entity that the
   user never perceives is NOT an object. An object the user perceives that has no
   single backing table (e.g., a composed "Dashboard") IS an object. Cross-link
   with `backs_onto` where they coincide.
4. **Merge synonyms, split overloaded nouns.** Record the call in
   `naming_decisions`. Keep names consistent with `docs/glossary.md`.
5. Record `source_evidence` and `confidence` for each object. Low-confidence
   objects become review-gate questions. **Confidence rubric:** `high` = named
   directly in an activity or as a `domain_model` entity; `medium` = inferred from
   story text / appears once; `low` = guessed, a synonym merge, or a composed object
   with no single backing entity. Surface every `low` at the gate.

Output → `product/object-model.yaml` `objects[]`.

## Round R — Relationships (Nested Object Matrix)

For each ordered pair of objects, ask: does one contain, reference, or list the
other? Record `cardinality` (one_to_one / one_to_many / many_to_many) and whether
the link is `nested` (shown inline in the parent's detail view) and/or a `nav`
path. Seed from `domain_model` foreign-key-style relationships, then add
user-perceived links the data model doesn't capture.

- Cross-capability links → `product/object-model.yaml` `relationships[]`.
- Capability-local links → `product/maps/<cap>/object-map.yaml` `relationships[]`.

Relationships are what pave navigation paths — get them right and the IA falls out.

## Round C — Calls-to-Action (CTA Matrix)

Build an **object × role** matrix. Every verb in the capability's stories and
activities is an action the user does _to some object_ — hang it there.

- Map each story/activity verb to its object and the acting `role` (persona).
- Set `placement` (collection / detail / inline / global) and `priority`.
  **Priority rule:** a CTA is `primary` when its story is `priority: critical|high`
  or `business_value >= 7`, or it is the object's defining action (create/the verb in
  the activity name); otherwise `secondary`.
- Keep `from_story` provenance so dispatch can trace a CTA back to the story that
  needs it, and so coverage can be checked.

A verb with no object is a smell: either an object is missing (go back to Round O)
or the action is really a task-flow (see the escape hatch below).

Output → `product/maps/<cap>/object-map.yaml` `ctas[]`.

## Round A — Attributes

For each object, list its content elements + metadata and rank them:

- **core** — identity-defining; shown on the card and the detail view.
- **secondary** — detail view only.
- **metadata** — sort/filter keys (timestamps, counts, status).

Mine from `domain_model` fields, acceptance criteria, and existing UI. **Tier
inference when there's no existing UI** (new products): name/title/label and the
object's primary identifier → `core`; other `required: true` domain fields →
`core`/`secondary`; timestamps, counts, status/enum flags → `metadata`; everything
else → `secondary`. Shared attributes → object-model; capability-specific →
object-map.

## Round 4 — Representation hints (structural only)

Project the above into structural views — for each primary object a `collection`
(list/grid) and `detail`, composed via the Nested Object Matrix; `create`/`edit`
where CTAs require them. Pick the capability's `anchor_object` (what the user sees
first). **This is IA, not visual design** — no colors, type, or spacing. Those
stay in `calibration/visual-design.yaml`; journey/page/transition stay in
`calibration/ux-flow.yaml`.

---

## Object-first default vs. task-oriented escape hatch

Default every flow to **object_first** (noun→verb: the user picks an object, then
acts). Deviate to **task_oriented** (a deliberate wizard) ONLY when the decision
framework says so, and record it in `interaction_modes` with a reason.

| Lean task-oriented (wizard)             | Lean object-first                  |
| --------------------------------------- | ---------------------------------- |
| Goal undefined, user needs guiding      | Goal clear, user wants to explore  |
| Single object                           | Many objects / relationships       |
| One linear flow                         | One object, many actions           |
| Novice / onboarding                     | Repeat / power user                |
| Low off-path tolerance                  | High off-path tolerance            |
| e.g. onboarding, checkout, setup wizard | e.g. dashboard, library, admin/CRM |

Staged design is legitimate: a capability can be task-oriented on first use and
object-first thereafter. Record the deviation per flow; everything unlisted is
object_first.

---

## The review gate (why this is not fully automatic)

Object boundaries and IA are high-impact design decisions — wrong boundaries cost
more than code bugs (same reason `/aep-map` gates the System Map). `/aep-model`
generates the _draft_, then asks the human a SHORT, high-leverage set:

1. **Object boundaries** — are these the objects the user actually thinks in?
   Any wrong names, bad merges, or missing splits?
2. **Primary anchor** — for this capability, which object should the user see /
   choose first?
3. **Task-flow exceptions** — which flows should be explicit wizards
   (onboarding / checkout / one-shot) rather than object-first?

Approve → flip `status: approved` and set `provenance.reviewed: true`. Only
`approved` object-maps unblock UI-facing dispatch/launch.

---

## Completeness checks (also enforced by /aep-validate Mode A)

- Every UI-facing capability has an `object-map.yaml`.
- Every noun foraged from an activity maps to an object (or is justified as
  implementation-only).
- Every UI story's verbs map to a CTA on some object (`coverage` is complete).
- Every primary object has a home screen (`collection` or `detail`).
- Every `task_oriented` flow has a written `reason`.
- Object names are consistent with `docs/glossary.md` and `domain_model`.
