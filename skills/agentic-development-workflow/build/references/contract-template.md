# Sprint Contract Template

A sprint contract is generated during Phase 0 for each task in the jj change stack. It bridges the gap between OpenSpec's high-level specs and concrete implementation by defining **what will be built** and **how success will be verified** before any code is written.

Anthropic's harness design research found that having the generator propose implementation details and success criteria — then having the evaluator review them before coding begins — prevents the common failure where agents build the wrong thing. This "sprint contract" negotiation continues until both sides agree.

**Source:** [Harness Design for Long-Running Application Development](https://www.anthropic.com/engineering/harness-design-long-running-apps)

---

## How Contracts Are Generated

During Phase 0, after creating the jj change stack:

1. Read `tasks.md` for the task list
2. Read `specs/*.md` for detailed requirements
3. Read `design.md` for technical approach
4. For each task, extract the matching spec requirements and generate a contract

Contracts are written to `.dev-workflow/contracts.md` as a single file with one section per task.

---

## Contract Format

```markdown
# Sprint Contracts

Generated from OpenSpec change: <change-name>
Date: <YYYY-MM-DD>

---

## Task: <task description from tasks.md>

**Change ID:** <jj change short ID>
**Source spec:** <which spec file this maps to>

### What will be built
- [Specific files to create or modify]
- [Components, routes, or endpoints to add]
- [Database changes if any]

### Success criteria
- [Extracted from matching spec — what "done" looks like]
- [Observable behaviors, not implementation details]
- [Include error and edge case handling from spec]

### Verification steps
1. [Concrete, executable step — e.g., "Navigate to /settings"]
2. [What to check — e.g., "Verify form shows current values"]
3. [Edge case — e.g., "Submit with empty required field, verify error shown"]
4. [Error path — e.g., "Disconnect network, verify offline message"]

### Dependencies
- [Other tasks this depends on, if any]
- [External services or APIs needed]

### Risks
- [Known unknowns or areas of uncertainty]

---
```

---

## Example Contract

```markdown
## Task: Add password reset flow

**Change ID:** kqx8m2vn
**Source spec:** specs/authentication.md

### What will be built
- `apps/web/src/pages/forgot-password.tsx` — Request reset form
- `apps/web/src/pages/reset-password.tsx` — New password form
- `apps/server/src/routes/auth/reset.ts` — Reset token API endpoints
- `packages/db/src/schema/reset-tokens.ts` — Token table schema
- Email template for reset link

### Success criteria
- User can request a password reset by entering their email
- System sends a reset email with a time-limited token
- User can set a new password using the token link
- Expired tokens show a clear error message
- Invalid tokens show a clear error message
- After reset, user is redirected to login

### Verification steps
1. Navigate to /forgot-password
2. Enter a registered email, submit — verify success message shown
3. Check server logs for reset email sent
4. Navigate to /reset-password?token=<valid-token>
5. Enter new password, submit — verify redirect to /login
6. Log in with new password — verify success
7. Try /reset-password?token=<expired-token> — verify "expired" error
8. Try /reset-password?token=<invalid-token> — verify "invalid" error
9. Submit /forgot-password with unregistered email — verify same success message (no email enumeration)

### Dependencies
- Authentication system from Task 1 must be complete
- Email service must be configured

### Risks
- Email delivery may be slow in dev environment
- Token expiration timing needs to be configurable
```

---

## Rules

1. **Contracts are written before implementation** — they capture intent, not after-the-fact documentation
2. **Success criteria come from specs** — don't invent new requirements
3. **Verification steps must be concrete** — "verify it works" is not a valid step
4. **The generator writes contracts, the evaluator reviews them** — if a separate evaluator is running, it checks contracts before Phase 4 begins
5. **Contracts are not immutable** — if implementation reveals the contract was wrong, update it and note why
6. **Contracts feed into `feature-verification.json`** — the verification steps are extracted into the JSON verification list for the evaluator
