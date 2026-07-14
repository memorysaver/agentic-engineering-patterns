---
name: aep-gen-eval
description: >-
  Canonical generator/evaluator pattern for artifact scoring, agent contracts,
  eval protocol, and findings. Used by /aep-build, /aep-validate, and
  /aep-launch; invoke for "gen/eval", "separate evaluator", or artifact review.
---

# Generator/Evaluator Pattern

A reusable design pattern for honest evaluation of agent-produced artifacts. Separates the agent that creates work (generator) from the agent that evaluates it (evaluator), because agents consistently praise their own work.

> "When asked to evaluate work they've produced, agents tend to respond by confidently praising the work — even when, to a human observer, the quality is obviously mediocre."
> — Anthropic, ["Harness Design for Long-Running Application Development"](https://www.anthropic.com/engineering/harness-design-long-running-apps)

Dual-use: consumer skills read this skill's `references/` files — the canonical homes for scoring, contracts, protocol, recovery, and findings — while invoking `/aep-gen-eval` directly runs a full gen/eval loop on any artifact.

---

## How Other Skills Use This

| Skill                | What it uses                        | Reference files                                                    |
| -------------------- | ----------------------------------- | ------------------------------------------------------------------ |
| `/aep-build` Phase 5 | Scoring framework + eval protocol   | `scoring-framework.md`, `eval-protocol.md`, `recovery-ladder.md`   |
| `/aep-launch`        | Dimension presets for brainstorming | `scoring-framework.md` (presets section)                           |
| `/aep-validate`      | Agent prompts + findings format     | `agent-contracts.md`, `findings-format.md`, `scoring-framework.md` |

---

## The Core Principle

**Generator and evaluator must be separate agents.** This is not optional — it is the single most impactful quality improvement in agentic workflows.

Why:

1. Agents cannot honestly evaluate their own work (demonstrated by Anthropic research)
2. Self-evaluation produces inflated scores and rationalized problems
3. Separate evaluation catches issues the generator is blind to
4. The cost of a second agent is trivial compared to shipping broken work

> **Scaling up:** generator/evaluator is the canonical instance of _adversarial
> verification_. When one task produces many findings/claims that each need an
> independent check, `/aep-workflow` generalizes this to a fan-out of N
> verifiers/refuters — reusing this skill's scoring framework and findings format
> per finding.

---

## Reference Files

These files are the canonical homes for the gen/eval contracts every consumer (`/aep-build`, `/aep-validate`, `/aep-launch`) points at. Read the one the branch needs; each file is self-contained.

| File                                                                 | Contents                                                                                                                                                                                                    | When to read                                                         |
| -------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------- |
| [`references/scoring-framework.md`](references/scoring-framework.md) | Dimension definitions (1-5 scale), hard failure thresholds, dimension presets (UI, API, security, data, mixed), few-shot examples, anti-patterns                                                            | Setting up evaluation criteria, scoring work, calibrating evaluators |
| [`references/agent-contracts.md`](references/agent-contracts.md)     | Generator/evaluator role separation, prompt templates (generator, evaluator, protocol checker), context assembly rules                                                                                      | Spawning evaluation agents, assembling prompts                       |
| [`references/eval-protocol.md`](references/eval-protocol.md)         | Eval request/response format, verification JSON schema, the eval loop (request → response → fix → re-evaluate), execution contexts (Task subagent, codex exec, tmux, workflow), the needs-human gate record | Running the evaluation loop, tracking verification state             |
| [`references/recovery-ladder.md`](references/recovery-ladder.md)     | Escalating change-strategy ladder (same-fix → re-ground → fresh generator → decompose → human gate) for a stalled eval loop                                                                                 | A FAIL loop is not converging after 2+ rounds                        |
| [`references/findings-format.md`](references/findings-format.md)     | Severity categorization (blocking/important/minor), deduplication protocol, presentation format, changelog entry format                                                                                     | Consolidating findings from multiple agents, presenting results      |

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

| Mode           | Agents                                                 | When to use                                                     |
| -------------- | ------------------------------------------------------ | --------------------------------------------------------------- |
| **Parallel**   | Generator + Evaluator spawned simultaneously           | Documents, designs, product context — agents work independently |
| **Sequential** | Generator first, then Evaluator reads generator's work | Code review — evaluator needs to see the implementation         |
| **Loop**       | Generator → Evaluator → fix → repeat (max 5 rounds)    | Active development — generator can fix issues between rounds    |

### Step 3: Configure dimensions

Read `references/scoring-framework.md` and select the preset that matches the artifact (UI-heavy, API-only, security-sensitive, data pipeline, mixed/full-stack, product/design, or document), or define custom dimensions. The preset tables, hard-failure thresholds, and few-shot calibration all live in that file.

### Step 4: Spawn agents

Read `references/agent-contracts.md` for prompt templates. Customize the templates with:

- The artifact content
- The technical constraints
- The verification checklist (what the evaluator should check against the codebase)

### Step 5: Process results

Read `references/findings-format.md` to consolidate, categorize, and present findings, then converge to one of two checkable end states:

- **Fixes applied and re-scored:** the generator applies the fixes and the artifact passes a fresh evaluation round with no blocking findings remaining. If rounds stall, climb `references/recovery-ladder.md` before escalating to a human.
- **Findings handed off:** a consolidated findings file is written to a named path (e.g. `<artifact-dir>/eval-findings.md`) for a downstream owner to act on.

---

## Design Decisions

Gen/eval is packaged as its own invocable skill that doubles as a reference library, rather than folded into `/aep-validate` or `/aep-launch`. Rationale: [`docs/decisions/gen-eval-rationale.md`](https://github.com/memorysaver/agentic-engineering-patterns/blob/main/docs/decisions/gen-eval-rationale.md).

---

## Next Step

After running gen/eval, proceed based on what was evaluated:

- Product context → `/aep-dispatch`
- Design artifacts → `/aep-launch`
- Code → create PR or continue `/aep-build`
- Documents → publish or share
