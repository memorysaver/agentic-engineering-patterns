# Evaluator Calibration Criteria

Calibration document for the evaluator agent. The evaluator is a **separate agent** from the generator — this separation is critical because agents consistently praise their own work even when quality is mediocre.

> "When asked to evaluate work they've produced, agents tend to respond by confidently praising the work — even when, to a human observer, the quality is obviously mediocre."
> — Anthropic, ["Harness Design for Long-Running Application Development"](https://www.anthropic.com/engineering/harness-design-long-running-apps)

---

## Scoring Dimensions

Evaluate each dimension on a 1–5 scale. Score honestly — the value of evaluation comes from catching problems the generator missed.

> **Customize per project:** These dimensions are defaults. Anthropic's research found that scoring dimensions should be task-specific and weighted toward areas where the model falls short. For UI-heavy projects, weight UX Quality and add "Originality." For API-only work, drop UX Quality and add "API Design." Adjust based on where you observe the generator producing mediocre output.

### 1. Completeness (1–5)

Does the implementation cover all tasks in `tasks.md` and specs?

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

## Few-Shot Examples

### Example 1: Lenient Evaluator (BAD)

This is how an uncalibrated evaluator behaves — finding real problems then talking itself out of them:

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

**Why this is correct:** Same findings, but the evaluator scored them honestly against the scale definitions and applied the hard failure thresholds. Actionable fix descriptions are included.

---

## Evaluation Protocol

Follow these steps for each evaluation round:

1. **Read all OpenSpec artifacts:**
   - `proposal.md` — understand the intent
   - `design.md` — understand the approach
   - `specs/*.md` — understand the requirements
   - `tasks.md` — understand what should be implemented

2. **Read `.dev-workflow/feature-verification.json`** (if present):
   - Run through each verification step literally
   - Mark `passes: true` only after confirming each step works
   - Never modify `verification_steps` — only update `passes`, `evaluated_by`, `round`

3. **Read `.dev-workflow/contracts.md`** (if present):
   - Check each task's success criteria
   - Verify the predicted files were actually modified

4. **Test the running application:**
   - Use agent-browser / Playwright MCP to interact with the app
   - Test happy paths first, then error paths, then edge cases
   - Take screenshots of failures

5. **Review the code:**
   - `jj diff` to see all changes
   - Check for security issues, performance problems, convention violations

6. **Score each dimension** against the scale definitions above

7. **Apply hard failure thresholds** — be honest, not lenient

8. **Write structured feedback** to `.dev-workflow/eval-response-<round>.md`:
   ```markdown
   # Evaluation Round <N>

   ## Findings
   [Each finding with steps to reproduce, expected vs actual, impact, fix suggestion]

   ## Scores
   [Each dimension with score and justification]

   ## Result: PASS / FAIL
   [If FAIL: which thresholds were violated]

   ## Verification Updates
   [Which items in feature-verification.json were updated]
   ```

---

## Anti-Patterns to Avoid

These are common evaluator failure modes — watch for them:

| Anti-Pattern | What Happens | Why It's Wrong |
|-------------|-------------|---------------|
| **Surface testing** | Only test the happy path | Bugs hide in error paths and edge cases |
| **Rationalization** | "This is probably fine because..." | If you found a problem, score it honestly |
| **Score inflation** | Everything gets 4-5 | Compare against scale definitions, not gut feel |
| **Scope creep** | "It would be nice if..." | Only evaluate against the spec, not wishlist items |
| **Premature approval** | Passing after finding only minor issues | Minor issues compound — evaluate the whole surface first |
| **Self-persuasion** | Identifying a problem then arguing it away | The problem exists. Score accordingly. |

---

## Dimension Presets

Use these as starting points when brainstorming project-specific criteria (evaluator setup section of `/launch`). Select the preset that matches the feature type, then adjust with the user.

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

### How to use presets

1. During `/launch` evaluator setup (Step 0), identify the feature type
2. Select the matching preset
3. Present to the user for customization
4. Write the final criteria to `.dev-workflow/evaluator-criteria.md`
5. The evaluator reads this per-workspace file instead of this default reference
