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
**Input:** Context Document from `/envision`
**Output:** System Map + Story Graph + Agent Topology (committed to `product-context/`)

---

## Before Starting

Read the product context:

```bash
cat product-context/context-document.md
cat product-context/opportunity-brief.md
```

If these don't exist, run `/envision` first.

---

## Step 1: System Map (Single Agent + Human Review)

Produce a **System Map** (see `templates/system-map.md`) from the Context Document:

- **Modules:** Major components with clear responsibility boundaries. Each module's "does not" definition is as important as its "does" definition.
- **Interface contracts:** For every module-to-module connection, define the exact API surface — endpoints, data shapes, error contracts. These are not documentation; they are executable specifications enforced by contract tests.
- **Data flow:** How information moves through the system for each user journey in the MVP contract.
- **Third-party boundaries:** External service integration points with failure modes.

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

### Walking Skeleton (Layer 0)

**Layer 0 is the most important layer.** Each module contributes one minimal story, strung together so a user can complete the crudest possible end-to-end journey from the Context Document's Layer 0 MVP Contract.

> "Build a skeleton that can walk before building a perfect leg."

Do not go deep into any module before proving the end-to-end path works. This is the most expensive mistake in this workflow.

---

## Step 3: Dependency Resolution & Execution Slices (Single Agent)

A dedicated agent receives all stories and produces:

- **Story Graph:** A directed acyclic graph organized by layer, showing dependencies and parallelism opportunities.
- **Execution Slices:** Within each layer, group stories into slices that can be dispatched as a batch. A slice is a set of stories with no mutual dependencies that can run fully in parallel.
- **Critical path per layer:** The longest dependency chain, determining minimum time to complete that layer.
- **Layer gates:** The integration test definition that must pass before advancing to the next layer.

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

---

## Output

Write all artifacts to `product-context/`:

```bash
# Write system-map.md, story-graph.md, agent-topology.md
git add product-context/
git commit -m "feat: add system map, story graph, and agent topology"
```

**Artifacts produced:**
- `product-context/system-map.md`
- `product-context/story-graph.md` (layered story graph with execution slices)
- `product-context/agent-topology.md`

---

## For Iteration

When updating the map (triggered by `/reflect` or new requirements):

1. Read the existing System Map and Story Graph
2. Identify what's changed — new modules, revised interfaces, new stories
3. Update affected sections
4. If interface contracts changed → re-verify dependent stories
5. Commit updated versions

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
/design
```

Each story (or execution slice) from the story graph becomes an OpenSpec change that flows through `/design → /launch → /build → /wrap`.
