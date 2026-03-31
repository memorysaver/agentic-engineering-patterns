# Context Document Template

This is the root context document for the project. Every downstream agent inherits this document. Precision here prevents confusion downstream — every vague statement multiplies into ambiguity across parallel agents.

Quality standard: **every statement must be convertible into a verification condition.** If it cannot be tested, it is not precise enough.

---

## Project Identity

- **Project name**: [Working name]
- **Opportunity Brief**: [Link to the Phase 0 Opportunity Brief]
- **Created**: [Date]
- **Last updated**: [Date]
- **Version**: [Increment on each meaningful change]

---

## Problem Statement

[Who has this problem? What is the problem, specifically? How do they deal with it today? Why is the current approach inadequate? This should be sharp enough that a stranger could read it and understand the pain without further explanation.]

---

## Persona / JTBD

### Primary Persona

[Describe the primary user concretely. Not "developers" but "solo developers building SaaS products who deploy to edge platforms and want to integrate AI agents without managing infrastructure." Include relevant context: technical skill level, tools they already use, constraints they operate under.]

### Job To Be Done

[What job is the user hiring this product to do? Frame it as: "When [situation], I want to [motivation], so I can [expected outcome]."]

---

## MVP Boundary

### In Scope

[Concrete list of what this system does. Each item should be specific enough to verify.]

### Explicitly Out of Scope

[What this system does NOT do, even if users might expect it. This prevents agents from expanding scope.]

### Deferred (Possible Future Scope)

[Things that might be added later but are not part of the current effort. Agents should not build toward these unless they come for free.]

---

## Technical Constraints

### Required Stack

[Non-negotiable technology choices and why.]

### Preferred Stack

[Preferences that can be overridden with good reason. State the preference and the reasoning.]

### Infrastructure

[Where this runs, deployment targets, environment constraints.]

### External Dependencies

[Third-party services this project depends on. For each: what it provides, failure behavior, known limitations.]

---

## User Activities (Story Map Backbone)

The backbone of the user story map. Each activity is a discrete step in the user's journey, ordered left-to-right as a narrative. These are discovered BEFORE defining layers — the backbone comes first, release boundaries (layers) are drawn across it second.

Read the activities as a sentence to verify narrative coherence: "The user [activity 1], then [activity 2], then [activity 3]..."

| Order | Activity ID | Name | Description | Layer Introduced |
| ----- | ----------- | ---- | ----------- | ---------------- |
| 1     |             |      |             | 0                |
| 2     |             |      |             | 0                |

Activities from Layer 0 form the core backbone. Later layers may introduce new activities that extend the backbone to the right.

---

## Layered MVP Contract

Each layer is a complete, testable, deployable increment. Layer 0 is the walking skeleton — a horizontal slice across ALL activities in the backbone. Each subsequent layer adds capabilities. Most MVPs need 2–4 layers. More than that suggests scope is too large.

### Layer 0: Walking Skeleton

**User can**: [End-to-end journey in concrete steps. "User does X → sees Y → gets Z." Every step observable and verifiable.]

**Verification**: [The test scenario that proves Layer 0 works.]

### Layer 1: [Name]

**User can**: [Everything from Layer 0, plus new capabilities.]

**Verification**: [Test scenario for new capabilities.]

### Layer 2: [Name]

**User can**: [Everything from Layer 1, plus new capabilities.]

**Verification**: [Test scenario.]

---

## Success Criteria

### Functional

[What must work for this to be a successful MVP? Specific, testable conditions.]

### Non-Functional

[Performance, reliability, security. Each with a measurable threshold.]

---

## Open Questions

Decisions explicitly deferred. Downstream agents will see these and know not to assume answers.

| Question | Why Deferred | Default Assumption | Revisit Trigger |
| -------- | ------------ | ------------------ | --------------- |
|          |              |                    |                 |

---

## Key Decisions Log

Significant decisions made during Product Framing. Helps downstream agents understand not just what was decided, but why — enabling consistent decisions in ambiguous situations.

| Decision | Reasoning | Alternatives Considered |
| -------- | --------- | ----------------------- |
|          |           |                         |

---

## Stress Test Record

Challenges raised during Phase 1 Stage 3, and how they were resolved.

| Challenge | Source Angle                                              | Resolution                                                            |
| --------- | --------------------------------------------------------- | --------------------------------------------------------------------- |
|           | Product viability / Technical feasibility / Scope control | Refined document / Marked as open question / Dismissed with reasoning |
