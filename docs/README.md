# AEP Documentation Structure

Rules for organizing documentation in this directory.

## Categories

| Category       | Directory     | What goes here                                                   | Naming convention                                               |
| -------------- | ------------- | ---------------------------------------------------------------- | --------------------------------------------------------------- |
| **Decisions**  | `decisions/`  | ADRs, post-mortems, improvement proposals that change AEP itself | Descriptive slug (`aep-v2-lesson-learning.md`)                  |
| **Workflow**   | `workflow/`   | How AEP patterns work — process docs, not decisions about them   | Descriptive slug (`autonomous-loop.md`)                         |
| **Tech-stack** | `tech-stack/` | Technology-specific gotchas, deployment patterns, library quirks | `<technology>-<topic>.md` (`rust-keyring-platform-features.md`) |
| **Lessons**    | `lessons/`    | Date-prefixed observations from downstream project runs          | `YYYY-MM-DD-<project>-<context>.md`                             |
| **Plans**      | `plans/`      | Design plans from `/design` skill                                | `YYYY-MM-DD-<topic>.md`                                         |

## Key Distinctions

- **Lessons** = raw observations from running builds in downstream projects. They capture what happened, not what to change.
- **Decisions** = reasoned changes to AEP itself. They explain why a pattern changed and what was decided.
- **Tech-stack** = reusable technical knowledge that applies across projects. Not tied to a specific build run.
- **Workflow** = how AEP patterns work day-to-day. Reference material for understanding the system.

## Top-level Files

These stay at the docs root because they're cross-cutting references:

- `glossary.md` — Ubiquitous language across all skills
- `skills-quick-reference.md` — Quick lookup for all AEP skills

## Routing Guide

When adding new documentation, ask:

1. **"Did this come from a specific build run?"** → `lessons/` (date-prefix it)
2. **"Is this a technology gotcha that applies to any project?"** → `tech-stack/`
3. **"Does this change how AEP works?"** → `decisions/`
4. **"Does this explain an existing AEP pattern?"** → `workflow/`
5. **"Is this a forward-looking design for a feature?"** → `plans/`
