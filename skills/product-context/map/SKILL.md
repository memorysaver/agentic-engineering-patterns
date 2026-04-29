---
name: map
description: Decompose a product into system map, layered story graph, and agent topology. Use after /envision, or when the user says "decompose", "story map", "system architecture", "break this down", "plan the stories", "agent topology". Produces the complete execution plan that the feature workflow operates on.
---

# Map

Decompose the Context Document into a system map (modules + interfaces), a layered story graph (work items + dependencies + execution slices), and an agent topology (roles + handoff contracts). This is the hardest phase — a wrong module boundary means dozens of agents produce incompatible code.

**Where this fits:**

```
/envision → /map → /scaffold → [ /design → /launch → /build → /wrap ] → /reflect
             ▲ you are here
```

**Session:** Main, interactive with user (System Map requires human review)
**Input:** Product definition from `product/index.yaml` (split mode) or `product-context.yaml` (v1 mode)
**Output:** `product-context.yaml` updated with `architecture`, `stories`, `waves`, `topology`, `layer_gates`, `cost`, and `changelog` sections

**YAML Schema:** See `templates/product-context-schema.yaml` for the full structure and field definitions.

---

## Before Starting

**File Resolution:**

```bash
ls product/index.yaml 2>/dev/null && echo "SPLIT MODE" || echo "V1 MODE"
cat product-context.yaml
```

- **Split mode** (`product/index.yaml` exists): Read product definition (opportunity, personas, product.\*) from `product/index.yaml`. Read operational state from `product-context.yaml`.
- **V1 mode**: Read everything from `product-context.yaml`.

If product definition is missing (no `product` section in either file), run `/envision` first.

---

## Step 1: System Map (Single Agent + Human Review)

Produce a **System Map** (see `templates/system-map.md`) from the Context Document:

- **Modules:** Major components with clear responsibility boundaries. Each module's "does not" definition is as important as its "does" definition.
- **Interface contracts:** For every module-to-module connection, define the exact API surface — endpoints, data shapes, error contracts. These are not documentation; they are executable specifications enforced by contract tests.
- **Data flow:** How information moves through the system for each user journey in the MVP contract.
- **Third-party boundaries:** External service integration points with failure modes.

Write the system map to the `architecture` section of `product-context.yaml`.

### When to Produce a Technical Specification

If the System Map reveals any of these conditions, suggest producing a Technical Specification (see `templates/technical-spec.md`) before proceeding to story decomposition:

- 3+ interface contracts require multi-step protocol sequences
- The system has 2+ distinct state machines
- There are explicit failure classes with different recovery behaviors
- Trust boundaries cross module lines

The System Map defines WHAT the modules are and HOW they connect. The Technical Spec defines HOW those connections behave under all conditions (success, failure, timeout, recovery). Write the Technical Spec as a standalone document and reference it from the architecture section:

```yaml
architecture:
  technical_spec: "docs/technical-spec.md"
```

See `templates/references/symphony-spec-reference.md` for the exemplar standard.

### Human Review Gate

**The user must review and approve the System Map.** Architecture decisions have the highest error cost in the entire pipeline. Present the map and explicitly ask for approval before proceeding to story decomposition.

If the user wants changes, revise and re-present. Do not proceed until approved.

---

## Step 2: Story Decomposition (Parallel Agents)

Once the System Map is confirmed, decompose into stories:

- **One Decomposition Agent per module:** Receives Context Document + System Map + its module definition. Produces stories tagged with layer (0 = walking skeleton, 1+ = enrichment layers).
- **One Integration Story Agent:** Looks at module connections in the System Map. Produces stories that glue modules together — the end-to-end flows crossing module boundaries. These are especially critical at Layer 0.

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

**All stories start with `status: pending`.** Stories follow a state machine: `pending → ready → in_progress → in_review → completed` (or `blocked` / `failed` as error states). The `/dispatch` skill manages state transitions during execution.

### Activity Mapping

After decomposition agents produce their stories, map each story to a user activity from `product.activities`:

- Stories that directly enable a user-facing capability get the activity they serve (e.g., "Create presigned upload URL" → `create-profile` because it enables the user to upload a selfie).
- **Infrastructure/foundation stories that don't map to any specific user activity leave `activity` as null.** These are implementation enablers — they appear in the architecture view but NOT in the user journey story map. This is correct and expected.
- Integration stories use the primary user activity they validate end-to-end.

Not every story needs an activity. The story map shows the user's perspective — technical plumbing is visible in the architecture view.

### Walking Skeleton (Layer 0)

**Layer 0 is the most important layer.** It is a horizontal slice across the activity backbone — the thinnest story from each user activity, strung together so a user can complete the crudest possible end-to-end journey from the Context Document's Layer 0 MVP Contract.

> "Build a skeleton that can walk before building a perfect leg."

Every activity in `product.activities` with `layer_introduced: 0` should have at least one Layer 0 story. Do not go deep into any module before proving the end-to-end path works. This is the most expensive mistake in this workflow.

---

## Step 3: Dependency Resolution & Waves (Single Agent)

A dedicated agent receives all stories and produces:

- **Story Graph:** A directed acyclic graph organized by layer, showing dependencies and parallelism opportunities.
- **Waves (Execution Slices):** Within each layer, group stories into waves that can be dispatched as a batch. A wave is a set of stories with no mutual dependencies that can run fully in parallel. (The YAML field is `stories[].slice`; the user-facing term is "wave.")
- **Critical path per layer:** The longest dependency chain, determining minimum time to complete that layer.
- **Layer gates:** The integration test definition that must pass before advancing to the next layer.

Write all stories to the `stories` section of `product-context.yaml`. Also populate the `waves` section grouping stories by layer + wave.

### Outcome Contracts

For each layer that has an `outcome_contract` defined (see `product.layers[].outcome_contract`), ensure the layer gate test definition aligns with the success metric. If no outcome contract exists for a layer, consider adding one — Jeff Patton emphasizes that layers should be anchored in outcomes, not just feature completeness.

The outcome contract is evaluated by `/reflect` after layer completion. It answers: "did this layer achieve what we hypothesized?"

### Capability Maps (multi-journey products)

If `product/index.yaml` exists (created by `/envision` for multi-journey products), also write per-capability `map.yaml` files:

- `product/maps/<capability>/map.yaml` — backbone activities, layers, story stubs for this capability
- Story stubs in `map.yaml` are sketches; the full stories in `product-context.yaml` are the operational versions

> **Split mode note:** In split mode, the capability map's `map.yaml` story stubs are narrative sketches. The full stories are written to `product-context.yaml`, and `product/index.yaml` is NOT modified by `/map` (it only reads from it).

- This is additive — if no capability maps exist, skip this step

### Alignment Layers (`.5` Layers)

After defining each implementation layer, review `calibration.plan` from `product-context.yaml` (operational file, both modes) (if populated by `/envision`) or consider which quality dimensions may need human calibration:

- **UI-facing stories** → consider visual-design and/or copy-tone calibration
- **New API endpoints** → consider api-surface calibration
- **New domain entities** → consider data-model calibration
- **First user-testable layer** → consider scope-direction calibration

**For heavy dimensions** (visual-design, ux-flow, copy-tone): plan a `.5` alignment layer with stories tagged `calibration_type: <dimension>`. Run `/calibrate <dimension>` before dispatching to generate a brief and capture decisions into `calibration/<type>.yaml`.

**For light dimensions** (api-surface, data-model, scope-direction, performance-quality): plan a `/calibrate <dimension>` checkpoint BEFORE dispatching the relevant stories in the next integer layer. No `.5` layer needed — decisions update `product-context.yaml` directly.

- **Layer 0.5** (first `.5` layer): Typically establishes the visual design system. Run `/calibrate visual-design` to create `calibration/visual-design.yaml`.
- **Layer 1.5, 2.5** (subsequent `.5` layers): Extend calibration to new patterns. `/calibrate` detects existing calibration artifacts and generates focused briefs covering only the delta.
- **Opt-in, not automatic.** The `/reflect` step after each layer classifies calibration needs by dimension. The human decides which dimensions need attention. But the workflow makes the question unavoidable.

### Feedback Loop

Decomposition agents may discover module boundaries are wrong. They submit amendment proposals to the System Map. When amendments accumulate to 3+ items or any single amendment affects an interface contract, trigger an **Architecture Review** with the user before continuing.

---

## Step 4: Agent Topology Design

**Why this lives here:** Per Anthropic's research, "each subagent needs an objective, an output format, guidance on tools and sources, and clear task boundaries — defined before execution." Topology is a decomposition decision — it determines how `/launch` configures workspaces and what context `/build` agents receive.

Define the agent roles, handoff contracts, and routing rules using the **Agent Topology** template (see `templates/agent-topology.md`):

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

---

## Output

### Before Committing: Validate YAML

See `references/yaml-guardrails.md` for the full checklist. Run:

```bash
npx js-yaml product-context.yaml > /dev/null && echo "YAML OK"
```

If this fails, fix the YAML before committing. Common fixes: quote list items containing colons, flatten nested sub-lists, escape embedded double quotes.

### Commit

```bash
git pull --ff-only origin main
git add product-context.yaml product/
git commit -m "feat: add system map, story graph, and agent topology"
git push origin main
```

**Sections written:**

- `architecture` — system map (modules, interfaces, data flow)
- `stories` — layered story graph with waves (all stories start `status: pending`)
- `waves` — stories grouped by layer + wave for batch dispatch
- `topology` — agent roles, handoff contracts, routing rules
- `layer_gates` — integration test definitions per layer (aligned with outcome contracts if defined)
- `cost` — initial cost budgets and tracking structure
- `changelog` — append an entry recording what was added

Always append to the `changelog` section.

---

## For Iteration

When updating the map (triggered by `/reflect` or new requirements):

1. Read the existing product definition (`product/index.yaml` in split mode, `product-context.yaml` in v1 mode) and operational state from `product-context.yaml`
2. Identify what's changed — new modules, revised interfaces, new stories
3. Update affected sections (`architecture`, `stories`, `topology`)
4. If interface contracts changed → re-verify dependent stories
5. Append to the `changelog` section
6. Commit updated version

---

## Anti-Patterns

- **Do not use more agents to mask unclear decomposition.** If stories are vague or overlapping, adding agents amplifies confusion. Fix the decomposition first.
- **Do not skip the walking skeleton.** Going deep into one module before proving the end-to-end path works is the most expensive mistake in this workflow.
- **Do not allow free-text handoffs.** Every agent-to-agent communication must be schema-defined. Ambiguity compounds exponentially across parallel agents.

---

## Next Step

Decomposition is complete. If no project exists yet:

```
/scaffold
```

If the project already exists, start executing stories:

```
/dispatch
```

`/dispatch` reads the story graph from `product-context.yaml` and begins moving stories through the state machine (`pending → ready → in_progress → ...`), routing each through `/design → /launch → /build → /wrap`.
