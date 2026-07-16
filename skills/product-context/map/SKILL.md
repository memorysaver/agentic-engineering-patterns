---
name: aep-map
description: >-
  Decomposes a product into a system map, layered story graph, and agent
  topology. Use after /aep-envision or for "decompose", "story map", "system
  architecture", "break this down", and "plan the stories".
---

# Map

Decompose the Context Document into a system map (modules + interfaces), a layered story graph (work items + dependencies + execution slices), and an agent topology (roles + handoff contracts). This is the hardest phase — a wrong module boundary means dozens of agents produce incompatible code.

**Where this fits:**

```
/aep-envision → /aep-map → /aep-scaffold → [ /aep-design → /aep-launch → /aep-build → /aep-wrap ] → /aep-reflect
             ▲ you are here
```

**Session:** Main, interactive with user (System Map requires human review)
**Input:** Product definition from `product/index.yaml` (split mode) or `product-context.yaml` (v1 mode)
**Output:** `product-context.yaml` updated with `architecture`, `stories`, `waves`, `topology`, `layer_gates`, `cost`, and `changelog` sections

**YAML Schema:** See `templates/product-context-schema.yaml` for the full structure and field definitions.

---

## Before Starting

Probe which mode the product context is in:

```bash
ls product/index.yaml 2>/dev/null && echo "SPLIT MODE" || echo "V1 MODE"
cat product-context.yaml
```

Mode semantics are canonical in `references/file-resolution.md`. Map reads the product definition (opportunity, personas, `product.*`) from `product/index.yaml` (split) or `product-context.yaml` (v1); it reads operational state from, and writes all output to, `product-context.yaml`.

If product definition is missing (no `product` section in either file), run `/aep-envision` first.

---

## Step 1: System Map (Single Agent + Human Review)

Produce a **System Map** (see `templates/system-map.md`) from the Context Document:

- **Modules:** Major components with clear responsibility boundaries. Each module's "does not" definition is as important as its "does" definition. Set each module's `kind` (`ui` | `backend` | `shared`) — `ui` modules render user-facing surfaces, which drives the UI-facing story trigger used by `/aep-model`, dispatch, and launch.
- **Interface contracts:** For every module-to-module connection, define the exact API surface — endpoints, data shapes, error contracts. These are not documentation; they are executable specifications enforced by contract tests.
- **Data flow:** How information moves through the system for each user journey in the MVP contract.
- **Third-party boundaries:** External service integration points with failure modes.

Write the system map to the `architecture` section of `product-context.yaml`.

When the System Map exposes multi-step protocols, multiple state machines, distinct failure classes, or trust boundaries crossing module lines, read `references/technical-spec-triggers.md` and suggest producing a Technical Specification before decomposition.

### Human Review Gate

**The user must review and approve the System Map.** Architecture decisions have the highest error cost in the entire pipeline. Present the map and explicitly ask for approval. If the user wants changes, revise and re-present; proceed to story decomposition only after the user explicitly approves.

**Postcondition:** the `architecture` section is written to `product-context.yaml` and the user has approved the System Map.

---

## Step 2: Story Decomposition (Parallel Agents)

Once the System Map is confirmed, decompose into stories:

- **One Decomposition Agent per module:** Receives Context Document + System Map + its module definition. Produces stories tagged with layer (0 = walking skeleton, 1+ = enrichment layers).
- **One Integration Story Agent:** Looks at module connections in the System Map. Produces stories that glue modules together — the end-to-end flows crossing module boundaries. These are especially critical at Layer 0.

Sharpen a vague or overlapping decomposition before adding agents — more agents amplify unclear decomposition rather than resolve it.

Each story follows the **Story Spec** format (see `templates/story-spec.md`) and must include:

- What changes when complete (observable behavior)
- Acceptance criteria automatable as tests
- Layer assignment (0 = walking skeleton, 1+ = enrichment)
- Module assignment
- Dependency declarations
- Interface obligations (if touching module boundaries)
- Files likely affected (for conflict detection)
- `business_value` (1-10, or null to derive from priority)
- `compile_mode` (default `single_change`; use `grouped_change` for tightly coupled stories, `shared_enabler` for infrastructure)

**All stories start with `status: pending`.** Story states follow the product-context schema; `/aep-dispatch` manages transitions during execution.

### Activity Mapping

After decomposition agents produce their stories, map each story to a user activity from `product.activities` (and, in split mode, set `story.capability` to the owning `capabilities[]` id — this is how dispatch/launch later locate the story's Object Map; leave null in v1/single-journey, where the default capability is the project slug):

- Stories that directly enable a user-facing capability get the activity they serve (e.g., "Create presigned upload URL" → `create-profile` because it enables the user to upload a selfie).
- **Infrastructure/foundation stories that don't map to any specific user activity leave `activity` as null.** These are implementation enablers — they appear in the architecture view but NOT in the user journey story map. This is correct and expected.
- Integration stories use the primary user activity they validate end-to-end.

Not every story needs an activity. The story map shows the user's perspective — technical plumbing is visible in the architecture view.

### Walking Skeleton (Layer 0)

**Layer 0 is the most important layer.** It is a horizontal slice across the activity backbone — the thinnest story from each user activity, strung together so a user can complete the crudest possible end-to-end journey from the Context Document's Layer 0 MVP Contract.

> "Build a skeleton that can walk before building a perfect leg."

Every activity in `product.activities` with `layer_introduced: 0` should have at least one Layer 0 story. Prove the end-to-end path works before going deep into any module — depth-first before the skeleton walks is the most expensive mistake in this workflow.

**Postcondition:** every story is written with a layer, a module, and `status: pending`; each `layer_introduced: 0` activity has ≥1 Layer 0 story.

---

## Step 3: Dependency Resolution & Waves (Single Agent)

A dedicated agent receives all stories and produces:

- **Story Graph:** A directed acyclic graph organized by layer, showing dependencies and parallelism opportunities.
- **Waves (Execution Slices):** Within each layer, group stories into waves that can be dispatched as a batch. A wave is a set of stories with no mutual dependencies that can run fully in parallel. (The YAML field is `stories[].slice`; the user-facing term is "wave.")
- **Critical path per layer:** The longest dependency chain, determining minimum time to complete that layer.
- **Layer gates:** The integration test definition that must pass before advancing to the next layer. Each gate also lists its **planned `journeys:`** (`skills/e2e-test/journeys/<NN-slug>.md`, one per capability area; empty when `dogfood_target == none`). Those journeys are a **pre-merge build deliverable** — `/aep-build` Phase 6 authors them from this layer's `acceptance_criteria` (one scenario per criterion, each `Then` → a concrete `Verify`, intent-level & tool-agnostic), committed with the feature **before any dogfood**. `journey_timing` (set in `skills/e2e-test/policy.md`) governs only **when** those journeys are _executed_ (pre-merge vs at the post-deploy gate), never when they are authored; `live_policy` (same file: `every_gate | milestone_gates_only | none`) governs which gates spend the **cost-bearing live half** of that execution — when proposing milestone layers, name them so they can be listed in `policy.md`. (`journeys:` is the plan; `evidence.journeys` records what actually ran.)

Write all stories to the `stories` section of `product-context.yaml`. Also populate the `waves` section grouping stories by layer + wave.

### Outcome Contracts

For each layer that has an `outcome_contract` defined (see `product.layers[].outcome_contract`), ensure the layer gate test definition aligns with the success metric. If no outcome contract exists for a layer, consider adding one — Jeff Patton emphasizes that layers should be anchored in outcomes, not just feature completeness.

The outcome contract is evaluated by `/aep-reflect` after layer completion. It answers: "did this layer achieve what we hypothesized?"

### Telemetry Binding (observability)

This is where the project **decides its telemetry sources** — metric-driven, then inventory. Bind every quantitative `success_metric` and any monitored `health_signals` to a source per `references/telemetry-ingestion.md` (§1.5 coverage rule; §1 Source config for the `metric_map`/`endpoint`/`token_env` binding). Write the result to `topology.routing.telemetry_sources` (+ `health_signals`).

`/aep-reflect`, `/aep-watch`, and `/aep-autopilot` run `coverage_check()` against this before trusting any auto path; an incomplete binding **blocks auto**, it does not silently no-op.

### Capability Maps (multi-journey products)

If `product/index.yaml` exists (created by `/aep-envision` for multi-journey products), also write per-capability `map.yaml` files:

- `product/maps/<capability>/map.yaml` — backbone activities, layers, story stubs for this capability
- Story stubs in `map.yaml` are sketches; the full stories in `product-context.yaml` are the operational versions

> **Split mode note:** In split mode, the capability map's `map.yaml` story stubs are narrative sketches. The full stories are written to `product-context.yaml`, and `product/index.yaml` is NOT modified by `/aep-map` (it only reads from it).

This is additive — if no capability maps exist, skip this step.

### Alignment Layers (`.5` Layers)

After defining each implementation layer, plan the `.5` alignment layers that pause execution to calibrate human intent. Review `calibration.plan` (or the quality dimensions each layer touches) and read `references/alignment-layers.md` — the canonical home for the `.5`-layer concept — for the heavy-vs-light dimension rules, the layer progression, and how an approved Object Map feeds the visual-design/ux-flow briefs. Heavy dimensions get a `.5` layer with `calibration_type`-tagged stories; light dimensions get a `/aep-calibrate` checkpoint before the next integer layer.

### Object Map Drafts (UI-facing capabilities)

After stories are decomposed, produce a **draft** noun-first Object Map for each UI-facing capability (declared `object-model`, `visual-design`/`ux-flow`, or user-facing stories). This bridges the verb-first story map to the UI. Read `references/object-map-drafts.md` for the ORCA derivation and the draft/approved/stale rules, then write `product/object-model.yaml` and `product/maps/<capability>/object-map.yaml` with **`status: draft`** — `/aep-model` owns approval. Skip entirely for pure-backend/CLI projects with no UI-facing capability.

### Feedback Loop

Decomposition agents may discover module boundaries are wrong. They submit amendment proposals to the System Map. When amendments accumulate to 3+ items or any single amendment affects an interface contract, trigger an **Architecture Review** with the user before continuing.

**Postcondition:** the `stories` and `waves` sections are written; each layer has a `layer_gates` entry (with planned `journeys:`); UI-facing capabilities each have a `status: draft` object-map.

---

## Step 4: Agent Topology Design

Topology is a decomposition decision — it determines how `/aep-launch` configures workspaces and what context `/aep-build` agents receive, so it is defined before execution. Define the agent roles, handoff contracts, and routing rules using the **Agent Topology** template (see `templates/agent-topology.md`). Every agent-to-agent handoff is schema-defined, not free text — ambiguity compounds exponentially across parallel agents.

### Agent Role Definition

For each role in the execution pipeline, define:

- **Role name:** What this agent type is called (e.g., `implementer`, `contract-verifier`, `integration-tester`)
- **Responsibility boundary:** What this agent does and does not do. Single-responsibility is the rule.
- **Input contract:** The exact structure of the work object this agent receives. Schema-defined, not free text.
- **Output contract:** The exact structure of the artifact this agent produces. Schema-defined.
- **Context window composition:** What goes into this agent's context — which sections of the Context Document, which parts of the System Map, what dependency artifacts. Irrelevant context degrades performance.
- **Cost budget:** Expected token usage and time per invocation.

### Handoff Contracts

For every agent-to-agent transition:

- **Trigger:** What event causes the handoff
- **Payload:** What artifact is passed, in what schema
- **Validation:** What checks run on the payload before the receiving agent starts

### Routing Rules

- **Dispatch policy:** How stories are assigned from the ready queue
- **Concurrency limit:** Maximum parallel agents (start conservative: 5-10)
- **Conflict detection:** Stories modifying the same files must not run in parallel
- **Retry routing:** Same agent retry (2x) → fresh agent with failure log (1x) → human escalation

Write the topology to the `topology` section of `product-context.yaml`. Also initialize the `layer_gates` and `cost` sections.

**Postcondition:** the `topology` section is written, and `layer_gates` and `cost` are initialized.

---

## Output

### Before Committing: Validate YAML

Validate before committing (checklist in `references/yaml-guardrails.md`):

```bash
npx js-yaml product-context.yaml > /dev/null && echo "YAML OK"
```

If this fails, fix the YAML per `references/yaml-guardrails.md` before committing.

### Commit

Commit and push per `/aep-git-ref` "Control-Plane Commits": `git add product-context.yaml product/`, commit (`feat: add system map, story graph, and agent topology`), and push to `$BASE` (resolve `$BASE` per `/aep-git-ref` "Resolving `$BASE`").

**Sections written:**

- `architecture` — system map (modules, interfaces, data flow)
- `stories` — layered story graph with waves (all stories start `status: pending`)
- `waves` — stories grouped by layer + wave for batch dispatch
- `topology` — agent roles, handoff contracts, routing rules
- `layer_gates` — integration test definitions per layer (aligned with outcome contracts if defined); list each gate's planned `journeys:` (the pre-merge journey deliverables `/aep-build` authors from the layer's acceptance criteria; an empty list when `dogfood_target == none`). Which gates spend live dogfood is priced by `policy.md` `live_policy`, not stored here.
- `cost` — initial cost budgets and tracking structure
- `changelog` — append an entry recording what was added

Always append to the `changelog` section.

**Postcondition:** `npx js-yaml` exits 0 and the commit lands on `$BASE`.

---

## For Iteration

When updating the map (triggered by `/aep-reflect` or new requirements):

1. Read the existing product definition (`product/index.yaml` in split mode, `product-context.yaml` in v1 mode) and operational state from `product-context.yaml`
2. Identify what's changed — new modules, revised interfaces, new stories
3. Update affected sections (`architecture`, `stories`, `topology`)
4. If interface contracts changed → re-verify dependent stories
5. Append to the `changelog` section
6. Commit updated version

---

## Next Step

Decomposition is complete. If no project exists yet:

```
/aep-scaffold
```

If the project has UI-facing capabilities, approve the Object Map drafts before dispatching UI stories:

```
/aep-model
```

`/aep-model` presents the draft Object Map for a short human review gate and flips it to `approved`. Then start executing stories:

```
/aep-dispatch
```

`/aep-dispatch` reads the story graph from `product-context.yaml` and begins moving stories through the state machine (`pending → ready → in_progress → ...`), routing each through `/aep-design → /aep-launch → /aep-build → /aep-wrap`. For UI-facing stories it injects the approved Object Map slice and refuses to dispatch if no approved Object Map exists.
