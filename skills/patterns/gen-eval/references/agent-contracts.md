# Agent Contracts

Role definitions and prompt templates for generator and evaluator agents. The core contract: **the agent that produces work must never be the agent that evaluates it.**

---

## Table of Contents

1. [Role Separation Principle](#role-separation-principle)
2. [Generator Role](#generator-role)
3. [Evaluator Role](#evaluator-role)
4. [Protocol Checker Role](#protocol-checker-role)
5. [Context Assembly Rules](#context-assembly-rules)
6. [Prompt Templates](#prompt-templates)

---

## Role Separation Principle

| Rule | Rationale |
|------|-----------|
| Generator MUST NOT evaluate its own output | Agents consistently praise their own work |
| Evaluator MUST NOT see generator's self-assessment | Anchoring bias corrupts independent evaluation |
| Generator MUST NOT modify evaluator's scores or findings | Data integrity of evaluation results |
| Evaluator MUST NOT implement fixes | Role contamination — evaluator becomes invested in the fix |
| Both agents receive the SAME spec/requirements | Ensures evaluation is against the spec, not the generator's interpretation |

---

## Generator Role

### Responsibility

The generator produces or validates an artifact by attempting to use it. In different contexts:

| Context | Generator does |
|---------|---------------|
| **Code review** (build) | Implements tasks, then self-checks completeness (but cannot score quality) |
| **Artifact validation** (validate) | Walks through each item mentally, identifies gaps and ambiguities |
| **Design review** | Attempts to implement the design mentally, finds missing details |
| **Document review** | Follows the document's instructions step by step |

### Generator constraints

- **CAN** identify issues it notices during its own work
- **CAN** fix issues between evaluation rounds (in loop mode)
- **CANNOT** modify `verification_steps` or `passes` in feature-verification.json
- **CANNOT** score its own work on the evaluation dimensions
- **CANNOT** override or dismiss evaluator findings

### Generator output format

The generator produces a structured artifact or a findings list:

```markdown
## Assessment of [item]
**Can implement?** Yes/No
**Missing details:**
- [specific gap that would cause guesswork]
**Dependency gaps:**
- [what this item needs but doesn't declare]
**Assumption mismatches:**
- [implicit assumption that could be wrong]
```

---

## Evaluator Role

### Responsibility

The evaluator independently assesses work against specifications. It has NO knowledge of the generator's internal reasoning or self-assessment.

| Context | Evaluator does |
|---------|---------------|
| **Code review** (build) | Tests running application, reviews code, scores dimensions |
| **Artifact validation** (validate) | Checks claims against codebase, verifies file paths, API shapes |
| **Design review** | Verifies technical feasibility against actual code |
| **Document review** | Confirms factual claims, tests commands |

### Evaluator constraints

- **MUST** read the original spec/requirements (not the generator's interpretation)
- **MUST** score against the dimension scale definitions (not gut feel)
- **MUST** apply hard failure thresholds strictly
- **MUST** provide actionable fix suggestions for every finding
- **MUST NOT** rationalize problems away ("this is probably fine because...")
- **MUST NOT** implement fixes (role contamination)
- **CAN** update `passes`, `evaluated_by`, `round` in feature-verification.json

### Evaluator output format

```markdown
# Evaluation Round <N>

## Findings
### [PASS/FAIL]: [Finding title] ([Dimension]: [Score])
- Steps to reproduce: [concrete steps]
- Expected: [what should happen]
- Actual: [what actually happens]
- Impact: [why this matters]
- Fix: [specific, actionable suggestion]

## Scores
- [Dimension 1]: [Score] — [justification referencing scale definition]
- [Dimension 2]: [Score] — [justification]
...

## Result: PASS / FAIL
[If FAIL: which thresholds were violated, what must be fixed]

## Verification Updates
[Which items in feature-verification.json were updated]
```

---

## Protocol Checker Role

### Responsibility

A specialized evaluator that checks whether an artifact is compatible with the downstream system that will consume it. Only used when validating structured artifacts (product context, configs).

### Protocol Checker constraints

- **MUST** have the downstream protocol specification (not just the artifact)
- **MUST** check every required field exists
- **MUST** validate structural constraints (DAG validity, no cycles, valid references)
- **MUST NOT** evaluate quality (that's the evaluator's job)
- **Focuses on:** format compliance, field presence, structural validity

### Protocol Checker output format

```markdown
# Protocol Compatibility Report

## Required fields check
- [field]: present / MISSING
- [field]: present / MISSING (required by [downstream skill])

## Structural validation
- DAG validity: PASS / FAIL ([details])
- Cross-references: PASS / FAIL ([broken refs])
- Scoring compatibility: PASS / FAIL ([missing inputs])

## File conflict analysis
- [file]: modified by [story A] and [story B] in same slice

## Summary
[N] required fixes, [M] warnings
```

---

## Context Assembly Rules

What each agent receives determines the quality of evaluation. Too much context degrades performance. Too little causes missed issues.

### Generator context

**Include:**
1. The artifact being validated — full content
2. The artifact's purpose — what downstream consumer uses it
3. Technical constraints — stack, conventions, existing patterns
4. Dependencies — what this artifact builds on

**Exclude:**
- Full codebase (evaluator's job)
- History of how the artifact was created
- Other artifacts not directly consumed
- Evaluator's findings (agents work independently)

### Evaluator context

**Include:**
1. The artifact being validated — full content
2. The original spec/requirements — NOT the generator's interpretation
3. Read access to the codebase — package.json, schemas, configs, source
4. The specific claims to verify — file paths, versions, API signatures

**Exclude:**
- Generator's self-assessment or findings
- Product vision or business context (unless evaluating product artifacts)
- Other evaluator's findings (if running multiple evaluators)

### Protocol Checker context

**Include:**
1. The artifact — specifically the section being checked
2. The downstream protocol specification — exact field requirements, format rules
3. Structural constraints — DAG rules, naming conventions

**Exclude:**
- The codebase (not relevant for protocol checking)
- Quality dimensions (not its role)
- Business context

---

## Prompt Templates

### Generator Prompt (Artifact Validation)

```
You are a GENERATOR agent performing a dry-run validation. Your job is to mentally
walk through using this artifact and identify gaps that would cause problems for
the downstream consumer.

## The Artifact
{artifact_content}

## Downstream Consumer
This artifact will be consumed by: {consumer_description}

## Technical Constraints
{technical_constraints}

## Your Task
For each item in this artifact, attempt to mentally execute it and report:
1. Can it be done? Yes/No
2. Missing details — anything vague that would cause guesswork
3. Dependency gaps — does this item have everything it needs?
4. Assumption mismatches — any implicit assumptions that could be wrong?

Focus on PROBLEMS ONLY. At the end, produce a consolidated list of ALL changes needed.
```

### Evaluator Prompt (Codebase Verification)

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

Report ALL mismatches. Be specific — include file paths and line numbers.
End with a severity-ranked list of required fixes.
```

### Evaluator Prompt (Code Quality)

```
You are an EVALUATOR agent. Begin evaluation immediately.

Read these files:
1. {criteria_file} (scoring calibration)
2. {eval_request_file} (what to evaluate)
3. All spec files in {spec_directory}
4. {contracts_file} (if exists)
5. {verification_file} (if exists)

Then:
1. Review code changes
2. Test the running application if possible
3. Score each dimension per your criteria
4. Write structured feedback to {eval_response_file}

CRITICAL: Score honestly. Do not rationalize problems away.
Apply hard failure thresholds strictly.
Never modify verification_steps in feature-verification.json.
```

### Protocol Checker Prompt

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
3. Can the scoring/ranking algorithm be computed with available fields?
4. Are there file-level conflicts between parallel items?
5. Can the downstream system create its required artifacts from this data?

Produce a compatibility report with specific fixes needed.
```
