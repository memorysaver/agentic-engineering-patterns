---
name: validate
description: |-
  Generator/evaluator validation for any AEP artifact — product context, architecture, stories, code, or documents. Use after any generation phase (/envision, /map, /design, /build) or when the user says "validate", "verify", "check the design", "dry-run", "evaluate", "gen/eval", "generator evaluator". Spawns parallel agents: a Generator (dry-run the artifact), an Evaluator (check against reality), and optionally a Protocol Checker (verify downstream compatibility). Modifies only the artifact being validated — never implements code.
---

# Validate

Run a generator/evaluator pattern against any artifact produced by the AEP workflow. The generator attempts to use the artifact (dry-run), the evaluator checks it against reality (codebase, constraints, downstream protocols), and the results are consolidated into fixes applied to the artifact itself.

> "When asked to evaluate work they've produced, agents tend to respond by confidently praising the work — even when, to a human observer, the quality is obviously mediocre."
> — Anthropic, "Harness Design for Long-Running Application Development"

**Why separate agents:** The agent that produced an artifact cannot honestly evaluate it. The generator/evaluator separation is the single most impactful quality improvement in agentic workflows. This skill applies the gen/eval pattern to product artifacts.

**Uses the gen/eval utility pattern.** Read these reference files for the underlying framework:
- **Scoring:** `.claude/skills/aep-gen-eval/references/scoring-framework.md`
- **Agent contracts:** `.claude/skills/aep-gen-eval/references/agent-contracts.md`
- **Eval protocol:** `.claude/skills/aep-gen-eval/references/eval-protocol.md`
- **Findings format:** `.claude/skills/aep-gen-eval/references/findings-format.md`

**Where this fits:**

```
/envision → /map → /validate → /dispatch → /design → /launch → /build → /wrap
                    ▲ you are here

Also usable after any phase:
  /envision → /validate   (validate product context)
  /map      → /validate   (validate architecture + stories)
  /design   → /validate   (validate specs before launch)
  /build    → /validate   (already built into Phase 5 — use that instead)
```

**Session:** Main, can be autonomous or interactive
**Input:** Any AEP artifact (product-context.yaml, OpenSpec change, design doc, code)
**Output:** The same artifact, with issues fixed. A validation report appended to changelog.

---

## Before Starting

Identify what is being validated:

```bash
# Check for product context
ls product-context.yaml 2>/dev/null

# Check for OpenSpec changes
ls openspec/changes/ 2>/dev/null

# Check for design artifacts
ls .dev-workflow/ 2>/dev/null
```

If the user doesn't specify what to validate, auto-detect based on the most recently modified artifact.

---

## Step 1: Determine Validation Mode

The skill operates in one of four modes based on the artifact type. Each mode configures which agents to spawn and what they check.

### Mode A: Product Context Validation

**When:** After `/envision` or `/map` — validating `product-context.yaml`

Mode A runs **two passes** — product design quality first, then technical correctness:

#### Pass 1: Product Design Evaluation ("Are we building the right thing?")

**Agents:** Product Design Evaluator + Vision Alignment Checker

| Agent | Role | What it checks |
|-------|------|---------------|
| Product Design Evaluator | Review against user story mapping principles | Walking skeleton validity, layer ordering, INVEST compliance, dependency graph quality |
| Vision Alignment Checker | Trace stories to opportunity brief | Every story maps to a stated user need, no scope creep, JTBD coverage |

Read the Product Design Evaluator prompt from `.claude/skills/aep-gen-eval/references/agent-contracts.md`.
Score using the story mapping dimensions from `.claude/skills/aep-gen-eval/references/scoring-framework.md` (Walking Skeleton Validity, Layer Ordering, Vision Alignment, INVEST Compliance).

**Pass 1 hard failures:**
- Walking Skeleton Validity < 3 — Layer 0 is not minimal enough
- Vision Alignment < 3 — Stories have drifted from the product vision
- INVEST Compliance < 3 — Stories are not actionable by an autonomous agent

#### Pass 2: Technical Validation ("Can we build it correctly?")

**Agents:** Generator + Evaluator + Protocol Checker

| Agent | Role | What it checks |
|-------|------|---------------|
| Generator | Dry-run each story/layer | Can each story be implemented? Missing details, ambiguous criteria, dependency gaps |
| Evaluator | Compare design vs codebase | Package versions, import paths, existing patterns, file existence, API compatibility |
| Protocol Checker | Verify downstream compatibility | Dispatch-required fields, DAG validity, scoring compatibility, file conflict detection |

**Why two passes:** Pass 1 catches product design problems (wrong stories, bad layering, vision drift). Pass 2 catches technical problems (missing fields, broken references, codebase mismatches). Both are required before dispatching to autonomous agents — the agents will faithfully build whatever you give them, right or wrong.

### Mode B: Design Validation

**When:** After `/design` — validating OpenSpec artifacts (proposal, design, specs, tasks)
**Agents:** Generator + Evaluator

| Agent | Role | What it checks |
|-------|------|---------------|
| Generator | Walk through implementation mentally | Are tasks implementable? Missing technical details, unclear acceptance criteria |
| Evaluator | Check specs against codebase | Do referenced files exist? Are API assumptions correct? Do types match? |

### Mode C: Code Validation

**When:** After implementation — validating code changes
**Agents:** Generator + Evaluator (same as `/build` Phase 5)

| Agent | Role | What it checks |
|-------|------|---------------|
| Generator | Review code against spec | Does the code match what was specified? Missing features, incomplete flows |
| Evaluator | Test the running application | Functional testing, edge cases, security, performance |

> **Note:** For code validation in a workspace, prefer `/build` Phase 5 which has the full evaluator loop with tmux split panes. Use this skill for code review on main branch or for lighter validation.

### Mode D: Document Validation

**When:** Validating any structured document (architecture doc, RFC, migration plan)
**Agents:** Generator + Evaluator

| Agent | Role | What it checks |
|-------|------|---------------|
| Generator | Follow the document's instructions | Can someone execute this document? Missing steps, ambiguous instructions |
| Evaluator | Check claims against reality | Do referenced tools/files/APIs exist? Are version numbers correct? |

---

## Step 2: Assemble Validation Context

For each agent, prepare a focused context package. Irrelevant context degrades evaluation quality.

### Generator Context

The generator needs:
1. **The artifact being validated** — full content
2. **The artifact's purpose** — what downstream consumer will use it (e.g., "dispatch will read stories", "an implementer agent will follow these tasks")
3. **Constraints** — technical stack, project conventions, existing patterns

The generator does NOT need:
- The full codebase (that's the evaluator's job)
- History of how the artifact was created
- Other artifacts not directly consumed

### Evaluator Context

The evaluator needs:
1. **The artifact being validated** — full content
2. **Read access to the codebase** — package.json files, existing schemas, config files, source code
3. **The specific claims to verify** — file paths, import statements, version numbers, API signatures

The evaluator does NOT need:
- Product vision or business context
- The generator's findings (agents work independently)

### Protocol Checker Context (Mode A only)

The protocol checker needs:
1. **The artifact being validated** — specifically the stories section
2. **The downstream protocol specification** — e.g., the dispatch skill's requirements for story fields, scoring formula, DAG validation rules
3. **The topology and layer gate definitions**

---

## Step 3: Spawn Agents

Launch all agents in parallel. Each agent works independently — they do not see each other's output.

### Generator Agent Prompt Template

```
You are a GENERATOR agent performing a dry-run validation. Your job is to mentally
walk through using this artifact and identify gaps that would cause problems.

## The Artifact
{artifact_content}

## Your Task
For each item in this artifact, attempt to mentally execute it and report:
1. Can it be done? Yes/No
2. Missing details — anything vague or ambiguous that would cause guesswork
3. Dependency gaps — does this item have everything it needs?
4. Assumption mismatches — any implicit assumptions that could be wrong?

## Constraints
{technical_constraints}

## Output Format
For each item, output a brief assessment. Focus on PROBLEMS ONLY — don't describe
what's fine. At the end, produce a consolidated list of ALL changes needed.
```

### Evaluator Agent Prompt Template

```
You are an EVALUATOR agent. Your job is to compare this artifact against the
ACTUAL state of the codebase and find mismatches. Read real files and verify claims.

## The Artifact
{artifact_content}

## What to Verify
{verification_checklist}

## Your Task
Read the actual files referenced in this artifact. For each claim, check:
1. Does the referenced file/function/type exist?
2. Does it have the signature/shape the artifact assumes?
3. Are version numbers and dependency versions correct?
4. Do import paths resolve correctly?

## Output Format
Report ALL mismatches between the artifact and reality. Be specific — include
file paths and line numbers. End with a severity-ranked list of required fixes.
```

### Protocol Checker Agent Prompt Template (Mode A)

```
You are a PROTOCOL CHECKER. Your job is to verify this artifact is compatible
with the downstream protocol that will consume it.

## The Artifact
{artifact_content}

## The Downstream Protocol
{protocol_specification}

## Your Task
1. Are all required fields present on every item?
2. Is the dependency graph a valid DAG (no cycles, no missing references)?
3. Can the scoring/ranking algorithm be computed with the available fields?
4. Are there file-level conflicts between parallel items?
5. Can the downstream system create its required artifacts from this data?

## Output Format
Produce a compatibility report with specific fixes needed in the artifact.
```

---

## Step 4: Consolidate Findings

After all agents return, consolidate their findings into a single action list.

### Categorize by severity

| Category | Description | Action |
|----------|-------------|--------|
| **Blocking** | Would stop downstream consumers from working | Fix immediately |
| **Important** | Would cause friction, confusion, or rework | Fix before proceeding |
| **Minor** | Cosmetic, missing optional fields, documentation gaps | Fix if time permits |

### Deduplicate

Multiple agents may find the same issue from different angles. Merge these into a single finding with the combined evidence.

### Present to user

Show the consolidated findings with counts:

```
Validation complete: {N} blocking, {M} important, {K} minor issues found.

Blocking:
  1. [issue] — found by Generator + Evaluator
  2. [issue] — found by Protocol Checker

Important:
  3. [issue] — found by Generator
  ...
```

---

## Step 5: Apply Fixes

Apply all blocking and important fixes to the artifact. Minor fixes are optional.

**Rules for fixes:**
- Only modify the artifact being validated — never create new files or modify other artifacts
- Preserve the artifact's existing structure and conventions
- Add a changelog entry recording what was validated and what changed
- If a fix requires a decision the agent can't make (architectural choice, business priority), mark it as an `open_question` with a default assumption

### Changelog entry format

```yaml
- date: <today>
  author: aep-validate
  summary: >
    Generator/evaluator validation. Found {N} blocking, {M} important, {K} minor issues.
    Fixed: [brief list of key fixes].
```

---

## Step 6: Commit

```bash
jj describe -m "fix: validate {artifact-name} — {N} issues found and fixed"
jj new
```

---

## Validation Dimensions

When the agents evaluate, they should consider these dimensions (adapted from the evaluator criteria in `/build`). Not all dimensions apply to every mode.

### For Product Context (Mode A)

| Dimension | What to check |
|-----------|--------------|
| **Completeness** | Are all required sections present? Are enums listed explicitly? Are defaults specified? |
| **Consistency** | Do field names match across sections? Do stories reference valid module IDs? |
| **Implementability** | Can each story be implemented with the information given? Missing technical details? |
| **Security** | Are there security implications in the design that aren't addressed? (auth, data access, PII) |
| **Downstream compatibility** | Does the artifact work with its consumers? (dispatch, design, build) |

### For Design Artifacts (Mode B)

| Dimension | What to check |
|-----------|--------------|
| **Completeness** | Do specs cover all capabilities in the proposal? Are acceptance criteria testable? |
| **Feasibility** | Can the tasks be implemented with the stated approach? Are file paths correct? |
| **Scope control** | Are tasks properly bounded? Any scope creep beyond the proposal? |

### For Code (Mode C)

See `references/evaluator-criteria.md` for the full 5-dimension scoring framework (Completeness, Correctness, UX Quality, Security, Code Quality).

### For Documents (Mode D)

| Dimension | What to check |
|-----------|--------------|
| **Accuracy** | Are all factual claims correct? Do referenced resources exist? |
| **Executability** | Can someone follow this document step by step? Are commands correct? |
| **Completeness** | Are there missing steps or assumptions? |

---

## When NOT to Use This Skill

- **During `/build` Phase 5** — use the built-in evaluator loop instead (it has tmux split, verification JSON, scoring framework)
- **For subjective quality** — this skill validates factual correctness and completeness, not aesthetic judgment
- **For tiny changes** — single-file edits or typo fixes don't need a 3-agent validation

---

## Customization

### Adding domain-specific checks

Create a `validation-criteria.md` file in your project's `.dev-workflow/` directory to add project-specific validation checks. The agents will read this file if it exists.

```markdown
# Project Validation Criteria

## Additional checks for Mode A (Product Context)
- All stories must have `business_value` field (required by our dispatch)
- Complexity must use S/M/L format (not small/medium/large)
- All file paths must be verified against the actual filesystem

## Additional checks for Mode B (Design)
- All API endpoints must include Zod validation schemas
- Database schema changes must include migration plan
```

### Adjusting agent count

By default, Mode A spawns 3 agents, Modes B-D spawn 2. You can adjust:

- **Lighter validation** (1 agent): Use when the artifact is small or low-risk. The single agent combines generator + evaluator roles.
- **Heavier validation** (4+ agents): Add domain-specific agents for complex artifacts. Examples: a "security reviewer" for auth-related designs, a "performance reviewer" for data pipeline architectures.

---

## Anti-Patterns

- **Generator evaluating its own work** — The agent that created an artifact must never be the one validating it. Use separate agent invocations.
- **Evaluator without codebase access** — An evaluator that can't read the actual code is just guessing. Always give the evaluator read access to relevant files.
- **Applying fixes without presenting findings** — Always show the user what was found before applying fixes. They may disagree with some findings or want to prioritize differently.
- **Validating against stale state** — Always read the current file/codebase state, not cached versions. Files may have changed since the artifact was created.
- **Over-validating** — Not every artifact needs 3-agent validation. Use judgment about the artifact's risk and complexity.

---

## Next Step

After validation, proceed to the appropriate downstream skill:

```
Product context validated → /dispatch
Design validated          → /launch
Code validated            → create PR (or /build Phase 9)
Document validated        → publish/share
```
