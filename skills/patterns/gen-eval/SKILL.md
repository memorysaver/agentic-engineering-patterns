---
name: gen-eval
description: |-
  Reusable generator/evaluator pattern for honest artifact validation. This is a utility skill — it provides the scoring framework, agent contracts, evaluation protocol, and findings format used by /build, /validate, and any skill that needs to evaluate agent-produced work. Use directly when you need to run a gen/eval loop on any artifact, or reference its files from other skills. Triggers on "gen/eval", "generator evaluator", "evaluate honestly", "separate evaluator", "scoring framework".
---

# Generator/Evaluator Pattern

A reusable design pattern for honest evaluation of agent-produced artifacts. Separates the agent that creates work (generator) from the agent that evaluates it (evaluator), because agents consistently praise their own work.

> "When asked to evaluate work they've produced, agents tend to respond by confidently praising the work — even when, to a human observer, the quality is obviously mediocre."
> — Anthropic, ["Harness Design for Long-Running Application Development"](https://www.anthropic.com/engineering/harness-design-long-running-apps)

**This skill is both a utility library and a standalone skill:**
- **As a library:** Other skills reference its `references/` files for scoring, prompts, protocol, and findings format.
- **As a standalone skill:** Invoke directly to run a gen/eval loop on any artifact.

---

## How Other Skills Use This

| Skill | What it uses | Reference files |
|-------|-------------|-----------------|
| `/build` Phase 5 | Scoring framework + eval protocol | `scoring-framework.md`, `eval-protocol.md` |
| `/launch` | Dimension presets for brainstorming | `scoring-framework.md` (presets section) |
| `/validate` | Agent prompts + findings format | `agent-contracts.md`, `findings-format.md`, `scoring-framework.md` |

### Cross-skill reference paths

After sync with `aep-` prefix, reference files are at:
```
.claude/skills/aep-gen-eval/references/scoring-framework.md
.claude/skills/aep-gen-eval/references/agent-contracts.md
.claude/skills/aep-gen-eval/references/eval-protocol.md
.claude/skills/aep-gen-eval/references/findings-format.md
```

---

## The Core Principle

**Generator and evaluator must be separate agents.** This is not optional — it is the single most impactful quality improvement in agentic workflows.

Why:
1. Agents cannot honestly evaluate their own work (demonstrated by Anthropic research)
2. Self-evaluation produces inflated scores and rationalized problems
3. Separate evaluation catches issues the generator is blind to
4. The cost of a second agent is trivial compared to shipping broken work

---

## Reference Files

Read these files for detailed specifications. Each file is self-contained.

| File | Contents | When to read |
|------|----------|-------------|
| [`references/scoring-framework.md`](references/scoring-framework.md) | Dimension definitions (1-5 scale), hard failure thresholds, dimension presets (UI, API, security, data, mixed), few-shot examples, anti-patterns | Setting up evaluation criteria, scoring work, calibrating evaluators |
| [`references/agent-contracts.md`](references/agent-contracts.md) | Generator/evaluator role separation, prompt templates (generator, evaluator, protocol checker), context assembly rules | Spawning evaluation agents, assembling prompts |
| [`references/eval-protocol.md`](references/eval-protocol.md) | Eval request/response format, verification JSON schema, the eval loop (request → response → fix → re-evaluate), execution contexts (tmux, Agent tool, subagent) | Running the evaluation loop, tracking verification state |
| [`references/findings-format.md`](references/findings-format.md) | Severity categorization (blocking/important/minor), deduplication protocol, presentation format, changelog entry format | Consolidating findings from multiple agents, presenting results |

---

## Standalone Usage

When invoked directly, this skill runs a gen/eval loop on any artifact.

### Step 1: Identify the artifact

What is being evaluated? Options:
- A document (product context, architecture, design doc)
- Code changes (implementation, PR diff)
- An OpenSpec change (proposal, design, specs, tasks)
- A structured file (YAML, JSON config, migration plan)

### Step 2: Choose execution mode

| Mode | Agents | When to use |
|------|--------|-------------|
| **Parallel** | Generator + Evaluator spawned simultaneously | Documents, designs, product context — agents work independently |
| **Sequential** | Generator first, then Evaluator reads generator's work | Code review — evaluator needs to see the implementation |
| **Loop** | Generator → Evaluator → fix → repeat (max 5 rounds) | Active development — generator can fix issues between rounds |

### Step 3: Configure dimensions

Read `references/scoring-framework.md` and select the appropriate preset:
- **Code:** Completeness, Correctness, UX Quality, Security, Code Quality
- **Product/design:** Completeness, Consistency, Implementability, Security, Downstream Compatibility
- **Documents:** Accuracy, Executability, Completeness

Or define custom dimensions for the specific artifact.

### Step 4: Spawn agents

Read `references/agent-contracts.md` for prompt templates. Customize the templates with:
- The artifact content
- The technical constraints
- The verification checklist (what the evaluator should check against the codebase)

### Step 5: Process results

Read `references/findings-format.md` for how to consolidate, categorize, and present findings. Apply fixes to the artifact.

---

## Design Decisions

**Why a utility skill, not just reference files:**
- A utility skill can be invoked directly (`/aep-gen-eval`) for ad-hoc validation
- It appears in the skill list, making the pattern discoverable
- It has its own description for triggering, so agents use it when appropriate
- The `references/` directory is still accessible to other skills via path

**Why not merge with `/validate`:**
- `/validate` is a product-context skill with 4 specific modes (product, design, code, document)
- The gen/eval pattern is more general — it applies to any evaluation scenario
- `/validate` consumes the gen/eval pattern; it is not the pattern itself

**Why not keep in `/launch`:**
- Launch only sets up criteria; it doesn't run the pattern
- The scoring framework is consumed by build, validate, AND launch
- Keeping it in launch creates a confusing ownership model

---

## Next Step

After running gen/eval, proceed based on what was evaluated:
- Product context → `/dispatch`
- Design artifacts → `/launch`
- Code → create PR or continue `/build`
- Documents → publish or share
