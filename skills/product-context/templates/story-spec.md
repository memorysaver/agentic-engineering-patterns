# Story Specification Template

The atomic unit of work for the execution plane. A well-written story spec gives an agent everything it needs to implement, verify, and submit a PR without asking questions.

Quality bar: **a single-responsibility agent reading only this spec, the Context Document, and the relevant System Map slice should produce correct, mergeable code.**

---

## Metadata

- **Story ID**: [Unique identifier, e.g., `SANDBOX-001`]
- **Title**: [Short, descriptive]
- **Layer**: [0, 1, 2… — which development layer]
- **Module**: [Primary module, as defined in System Map]
- **Activity**: [Which user activity this enables, from `product.activities`. Null for infrastructure stories that don't directly serve a user journey step.]
- **Execution Slice**: [Which slice within the layer this belongs to]
- **Dependencies**: [Story IDs that must complete before this starts]
- **Estimated complexity**: [S / M / L]

---

## Description

### What changes when this story is complete

[Observable difference in the system. Focus on behavior, not implementation. The agent decides how; this spec defines what.]

### Why this story exists

[Connect to the Context Document. Which layer of the MVP contract does this serve? Why this layer and not a later one?]

---

## Acceptance Criteria

Each must be automatable as a test. If it cannot be automated, it is too vague or belongs in manual review.

1. [Criterion — specific, observable, testable]
2. [Criterion]
3. [Criterion]

---

## Interface Obligations

If this story touches a module boundary:

- **Implements**: [Endpoints/APIs created or modified, referencing System Map contracts]
- **Consumes**: [Other module APIs called, referencing System Map contracts]
- **Contract tests required**: [Yes/No]

---

## Technical Notes

[Optional. Only include guidance that prevents known pitfalls. Do not over-specify — let the agent choose its approach.]

---

## Files Likely Affected

[Optional. Helps orchestrator detect conflicts between parallel stories.]

---

## Verification Strategy

- **Unit tests**: [What to unit test]
- **Integration tests**: [Cross-module interaction tests, if applicable]
- **Contract tests**: [Interface compliance tests, if applicable]

---

## Definition of Done

All of the following must be true:

1. All acceptance criteria pass as automated tests
2. All relevant contract tests pass
3. Code follows project conventions
4. PR submitted with description linking to this Story ID
5. No regressions in existing tests
6. Structured status report produced
