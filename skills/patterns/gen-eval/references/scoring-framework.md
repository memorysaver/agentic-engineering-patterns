# Scoring Framework

Calibration document for evaluator agents. The evaluator is a **separate agent** from the generator — this separation is critical because agents consistently praise their own work.

> "When asked to evaluate work they've produced, agents tend to respond by confidently praising the work — even when, to a human observer, the quality is obviously mediocre."
> — Anthropic, "Harness Design for Long-Running Application Development"

---

## Table of Contents

1. [Default Dimensions (Code)](#default-dimensions-code)
2. [Hard Failure Thresholds](#hard-failure-thresholds)
3. [Dimension Presets](#dimension-presets)
4. [Product & Design Dimensions](#product--design-dimensions)
5. [Document Dimensions](#document-dimensions)
6. [Few-Shot Examples](#few-shot-examples)
7. [Anti-Patterns](#anti-patterns)
8. [Customization Guide](#customization-guide)

---

## Default Dimensions (Code)

Evaluate each dimension on a 1–5 scale. Score honestly — the value of evaluation comes from catching problems the generator missed.

> **Customize per project:** These dimensions are defaults. Anthropic's research found that scoring dimensions should be task-specific and weighted toward areas where the model falls short. Adjust based on where you observe the generator producing mediocre output.

### 1. Completeness (1–5)

Does the implementation cover all tasks and specs?

| Score | Definition |
|-------|-----------|
| 1 | Multiple tasks unimplemented or stubbed out |
| 2 | Most tasks attempted but significant gaps remain |
| 3 | All tasks addressed but some have missing edge cases or incomplete flows |
| 4 | All tasks fully implemented with minor omissions |
| 5 | Every task, edge case, and spec requirement implemented and verified |

### 2. Correctness (1–5)

Does the implementation work as specified? Are edge cases handled?

| Score | Definition |
|-------|-----------|
| 1 | Core functionality broken — primary flows fail |
| 2 | Main flows work but secondary flows or error paths fail |
| 3 | Flows work under normal conditions but break on edge cases |
| 4 | All flows work correctly with minor edge case gaps |
| 5 | All flows work correctly including error states, empty states, and boundary conditions |

### 3. UX Quality (1–5)

Is the interface intuitive, responsive, and accessible?

| Score | Definition |
|-------|-----------|
| 1 | Interface is confusing — users cannot complete basic tasks without guessing |
| 2 | Interface works but has unintuitive interactions or missing feedback |
| 3 | Functional UX with standard patterns but nothing polished |
| 4 | Clean, intuitive UX with proper loading states, error messages, and responsive layout |
| 5 | Polished UX with thoughtful transitions, accessibility, and delight details |

### 4. Security (1–5)

Input validation, auth checks, data exposure?

| Score | Definition |
|-------|-----------|
| 1 | Critical vulnerabilities — SQL injection, XSS, or auth bypass possible |
| 2 | Major gaps — missing input validation on user-facing endpoints |
| 3 | Basic validation present but inconsistent; some endpoints lack auth checks |
| 4 | Solid validation and auth coverage with minor gaps in edge cases |
| 5 | Comprehensive validation, parameterized queries, proper auth on all routes, no data leaks |

### 5. Code Quality (1–5)

Conventions, maintainability, performance?

| Score | Definition |
|-------|-----------|
| 1 | Inconsistent patterns, duplicated logic, no error handling |
| 2 | Works but fragile — magic numbers, unclear naming, mixed conventions |
| 3 | Acceptable quality following basic conventions; some areas need cleanup |
| 4 | Clean, consistent code with proper error handling and clear structure |
| 5 | Exemplary — clear abstractions, well-named, efficient, follows all project conventions |

---

## Hard Failure Thresholds

Any of these conditions means the evaluation **FAILS** and the generator must fix before re-evaluation:

- **Completeness below 4** — Missing features are not acceptable
- **Correctness below 3** — Broken flows must be fixed
- **Security below 3** — Security gaps must be addressed
- Any single dimension below 2 — Critical deficiency

**Overall pass:** All dimensions >= 3 AND Completeness >= 4 AND no dimension at 1.

---

## Dimension Presets

Select the preset matching the artifact type, then adjust with the user during evaluator setup.

### UI-heavy (forms, dashboards, layouts)

```
Dimensions:  Completeness, Correctness, UX Quality, Originality, Accessibility
Weight:      UX Quality (high), Originality (high)
De-weight:   Code Quality (still check but don't hard-fail)
Add:         Originality — penalize generic "AI slop" (purple gradients, card layouts)
             Accessibility — WCAG AA compliance, keyboard navigation, screen readers
Hard fail:   UX Quality < 3, Completeness < 4
```

### API-only (endpoints, services, integrations)

```
Dimensions:  Completeness, Correctness, API Design, Security, Performance
Weight:      Correctness (high), Security (high)
Drop:        UX Quality (no frontend)
Add:         API Design — consistent naming, proper status codes, pagination, error format
             Performance — response times, query efficiency, no N+1
Hard fail:   Correctness < 3, Security < 3
```

### Security-sensitive (auth, payments, data handling)

```
Dimensions:  Completeness, Correctness, Security, Data Privacy, Code Quality
Weight:      Security (high), Data Privacy (high)
Drop:        UX Quality (unless auth UI is involved)
Add:         Data Privacy — PII handling, encryption at rest, audit logging
Hard fail:   Security < 4, Data Privacy < 4
```

### Data pipeline (ETL, migrations, batch processing)

```
Dimensions:  Completeness, Correctness, Performance, Data Integrity, Error Recovery
Weight:      Data Integrity (high), Performance (high)
Drop:        UX Quality, Security (unless processing sensitive data)
Add:         Data Integrity — no data loss, idempotent operations, schema validation
             Error Recovery — partial failure handling, retry logic, dead letter queues
Hard fail:   Data Integrity < 4, Completeness < 4
```

### Mixed / Full-stack

```
Dimensions:  Completeness, Correctness, UX Quality, Security, Code Quality
Weight:      All equal (default)
Add:         None — use the 5 defaults
Adjust:      Weight toward the area the user identifies as highest risk
Hard fail:   Default thresholds (any < 3, Completeness < 4)
```

---

## Product & Design Dimensions

When evaluating product context, architecture, or design artifacts (not code):

### Completeness (1–5)

| Score | Definition |
|-------|-----------|
| 1 | Major sections missing, enums undefined, no defaults specified |
| 2 | Sections present but sparse — many fields lack values or constraints |
| 3 | All sections present with some gaps in specificity |
| 4 | Comprehensive with minor omissions (e.g., a missing enum value) |
| 5 | Every field specified, all enums listed, all defaults documented |

### Consistency (1–5)

| Score | Definition |
|-------|-----------|
| 1 | Field names conflict across sections, broken references |
| 2 | Some naming mismatches, a few invalid cross-references |
| 3 | Generally consistent with isolated inconsistencies |
| 4 | Consistent naming and valid references with minor style variations |
| 5 | Perfectly consistent naming, all cross-references valid, uniform conventions |

### Implementability (1–5)

| Score | Definition |
|-------|-----------|
| 1 | Stories cannot be implemented — critical technical details missing |
| 2 | Most stories implementable but several have ambiguous acceptance criteria |
| 3 | All stories have a path to implementation with some guesswork needed |
| 4 | Clear implementation path with minor ambiguities |
| 5 | Every story is unambiguous — an implementer agent could build it without questions |

### Security (1–5)

| Score | Definition |
|-------|-----------|
| 1 | No security considerations in the design |
| 2 | Security mentioned but critical gaps (e.g., no auth model, PII unaddressed) |
| 3 | Basic security covered but edge cases missing |
| 4 | Comprehensive security design with minor gaps |
| 5 | Security-first design with threat model, data lineage, and compliance considerations |

### Downstream Compatibility (1–5)

| Score | Definition |
|-------|-----------|
| 1 | Artifact cannot be consumed by downstream skills (missing required fields) |
| 2 | Most fields present but format mismatches prevent consumption |
| 3 | Consumable with minor fixups needed |
| 4 | Fully compatible with minor cosmetic issues |
| 5 | Perfect compatibility — downstream skills can consume without any transformation |

### Walking Skeleton Validity (1–5)

Does Layer 0 represent the thinnest possible end-to-end user journey?

| Score | Definition |
|-------|-----------|
| 1 | Layer 0 has gold-plated features, infrastructure-only stories, or no clear user journey |
| 2 | A user journey exists but includes unnecessary scope — some stories could move to Layer 1+ |
| 3 | Mostly minimal but 1-2 stories feel over-scoped for a walking skeleton |
| 4 | Genuinely thin path with one minor luxury that could be deferred |
| 5 | The absolute minimum — a user can complete the crudest possible journey, nothing more |

> "Build a skeleton that can walk before building a perfect leg." — Jeff Patton

### Layer Ordering (1–5)

Does each layer add meaningful new user capability in the right order?

| Score | Definition |
|-------|-----------|
| 1 | Layers are arbitrary groupings with no clear progression of user value |
| 2 | Some layers add user value, but ordering doesn't match priority |
| 3 | Layers generally progress from core to enrichment, with 1-2 misplacements |
| 4 | Clear value progression — each layer unlocks a meaningful new user capability |
| 5 | Optimal ordering — users get the highest-value capabilities earliest, each layer builds naturally on the previous |

### Vision Alignment (1–5)

Do all stories trace back to the opportunity brief and product vision?

| Score | Definition |
|-------|-----------|
| 1 | Multiple stories serve no user need — pure technical infrastructure or scope creep |
| 2 | Most stories serve the vision but some are "nice to have" that crept in |
| 3 | All stories connect to user needs but some are indirect |
| 4 | Clear traceability from each story to the opportunity brief |
| 5 | Every story directly serves a stated user need, with explicit mapping to JTBD |

### INVEST Compliance (1–5)

Do stories follow the INVEST criteria (Independent, Negotiable, Valuable, Estimable, Small, Testable)?

| Score | Definition |
|-------|-----------|
| 1 | Stories are coupled, vague, and untestable — they are task lists, not stories |
| 2 | Some stories meet INVEST but many are too large or have hidden dependencies |
| 3 | Most stories are independent and testable but some are oversized or bundled |
| 4 | Stories are well-formed with minor violations (e.g., one L story that should be split) |
| 5 | Every story is independent, delivers observable value, has clear acceptance criteria, and is right-sized |

### Story Mapping Hard Failure Thresholds

- **Walking Skeleton Validity < 3** — Layer 0 is not minimal enough
- **Vision Alignment < 3** — Stories have drifted from the product vision
- **INVEST Compliance < 3** — Stories are not actionable by an autonomous agent

---

## Document Dimensions

When evaluating structured documents (RFCs, migration plans, runbooks):

### Accuracy (1–5)

| Score | Definition |
|-------|-----------|
| 1 | Multiple factual errors — wrong file paths, incorrect API signatures, outdated versions |
| 2 | Some claims incorrect or unverifiable |
| 3 | Mostly accurate with a few unverified claims |
| 4 | All verifiable claims checked and correct |
| 5 | Every claim verified against current codebase/documentation |

### Executability (1–5)

| Score | Definition |
|-------|-----------|
| 1 | Cannot be followed — missing steps, wrong commands, undefined prerequisites |
| 2 | Followable with significant guesswork required |
| 3 | Can be followed but some steps need interpretation |
| 4 | Clear step-by-step with minor assumptions |
| 5 | Fully executable — every command correct, every prerequisite listed |

### Completeness (1–5)

Same as code completeness scale — are all required sections present?

---

## Few-Shot Examples

### Example 1: Lenient Evaluator (BAD)

This is how an uncalibrated evaluator behaves — finding real problems then rationalizing them away:

```
EVALUATOR OUTPUT (UNCALIBRATED):

I tested the login form. Clicking "Submit" with empty fields shows no error message
and the form just silently fails. However, this is a minor UX issue since users
would typically fill in the fields before submitting. The core login flow works
when valid credentials are provided.

I also noticed the delete endpoint doesn't check if the requesting user owns the
resource. But since this is an internal tool, the risk is probably low.

Score: Completeness 4, Correctness 4, UX 4, Security 3, Code Quality 4
Result: PASS
```

**Why this is wrong:** The evaluator identified two legitimate problems (no form validation feedback, missing authorization check) but rationalized both away. The security issue alone should fail the evaluation.

### Example 2: Calibrated Evaluator (GOOD)

This is how a properly calibrated evaluator handles the same findings:

```
EVALUATOR OUTPUT (CALIBRATED):

## Findings

### FAIL: Missing form validation feedback (Correctness: 2, UX: 2)
- Steps: Navigate to /login → Click Submit with empty fields
- Expected: Error messages showing which fields are required
- Actual: Form silently fails, no visual feedback
- Impact: Users have no way to know what went wrong
- Fix: Add client-side validation with inline error messages

### FAIL: Missing authorization on delete endpoint (Security: 2)
- Steps: POST /api/items/42/delete with a different user's auth token
- Expected: 403 Forbidden
- Actual: 200 OK — item deleted regardless of ownership
- Impact: Any authenticated user can delete any other user's data
- Fix: Add ownership check before delete operation

## Scores
- Completeness: 4 — All features implemented
- Correctness: 2 — Login flow broken for empty input case
- UX: 2 — No validation feedback on primary form
- Security: 2 — Authorization bypass on delete endpoint
- Code Quality: 4 — Clean code, good conventions

## Result: FAIL
Correctness (2) and Security (2) are below threshold.
Generator must fix both issues before re-evaluation.
```

**Why this is correct:** Same findings, scored honestly against scale definitions, hard failure thresholds applied. Actionable fix descriptions included.

---

## Anti-Patterns

These are common evaluator failure modes — watch for them:

| Anti-Pattern | What Happens | Why It's Wrong |
|-------------|-------------|---------------|
| **Surface testing** | Only test the happy path | Bugs hide in error paths and edge cases |
| **Rationalization** | "This is probably fine because..." | If you found a problem, score it honestly |
| **Score inflation** | Everything gets 4-5 | Compare against scale definitions, not gut feel |
| **Scope creep** | "It would be nice if..." | Only evaluate against the spec, not wishlist items |
| **Premature approval** | Passing after finding only minor issues | Minor issues compound — evaluate the whole surface first |
| **Self-persuasion** | Identifying a problem then arguing it away | The problem exists. Score accordingly |

---

## Customization Guide

### How to use presets

1. During evaluator setup, identify the artifact type
2. Select the matching preset from the dimension presets section
3. Present to the user for customization
4. Write the final criteria to `.dev-workflow/evaluator-criteria.md` (for workspace evaluation) or use directly in agent prompts (for ad-hoc validation)
5. The evaluator reads the per-workspace file instead of this default reference

### Adding project-specific dimensions

Create a `validation-criteria.md` in your project's `.dev-workflow/` directory:

```markdown
# Project Validation Criteria

## Additional dimensions
- API Design: Check for consistent naming, proper status codes, error format
- Data Privacy: Verify PII handling, encryption, deletion cascade

## Project-specific hard failures
- Any endpoint missing Zod validation → Security FAIL
- Any database change missing migration → Completeness FAIL
```
