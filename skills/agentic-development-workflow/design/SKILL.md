---
name: aep-design
description: >-
  Designs a feature interactively on the integration branch with OpenSpec
  exploration, proposal, review, and committed artifacts. Use for "design a
  feature" or "explore and propose"; next run /aep-launch.
---

# Design

Interactive feature design on the **integration branch** (`$BASE` — resolve per /aep-git-ref "Resolving $BASE"). Explore the problem, propose a solution, review the design, and commit artifacts — all in conversation with the user.

**Where this fits:**

```
/aep-onboard → /aep-scaffold → [ /aep-design → /aep-launch → /aep-build → /aep-wrap ]
                          ▲ you are here
```

**Session:** Main session, interactive with user
**Input:** Feature idea or user request (optionally informed by product context)
**Output:** OpenSpec change committed to the integration branch (proposal, design, specs, tasks)

---

## Operating Mode

Auto-detect at startup:

```bash
ls product-context.yaml 2>/dev/null
```

- **Standalone** (no `product-context.yaml`) — the feature lifecycle runs independently; proceed to Prerequisites.
- **Product-cycle** (has `product-context.yaml`) — the feature is a story in `/aep-envision` → `/aep-map` → `/aep-dispatch` → `/aep-design`. Read references/modes.md for how a dispatched story's context (acceptance criteria, interface obligations, existing change) loads before the phases.

---

## Prerequisites

Two one-line checks — `/aep-onboard` installs missing tools, `/aep-scaffold` installs missing OpenSpec skills:

```bash
which git openspec >/dev/null 2>&1 && { which gh >/dev/null 2>&1 || which glab >/dev/null 2>&1; } || echo "MISSING cli tool → run /aep-onboard"
ls .claude/skills/openspec-{explore,propose,apply-change,archive-change}/SKILL.md >/dev/null 2>&1 || echo "MISSING openspec skill → run /aep-scaffold"
```

---

## Workflow Mode Selection

Pick a workflow mode — **full** or **light** — before designing. It governs Phase 3 below and carries through to `/aep-launch` and `/aep-build`. Read references/workflow-modes.md for the two modes, the selection criteria, and how the choice is recorded so launch and build honor it.

---

## Phase 1: OpenSpec Explore

```
/opsx:explore
```

Clarify requirements and scope with the user, investigate the codebase for relevant patterns, identify risks and unknowns, and create a `docs/` architecture note if the feature warrants it.

**Postcondition:** requirements, scope, and risks are captured in the change directory (`openspec/changes/<change-name>/`).

---

## Phase 2: OpenSpec Propose

```
/opsx:propose
```

Generates the full OpenSpec change: `proposal.md` (what/why), `design.md` (how, key decisions, risks), `specs/**/*.md` (requirements and scenarios), `tasks.md` (implementation checklist). When a story was dispatched, the change already exists — `/opsx:propose` refines it.

**Postcondition:** `openspec/changes/<change-name>/proposal.md` exists (alongside `design.md`, `specs/`, `tasks.md`).

---

## Phase 3: Design Review

Review scope — architecture, interfaces, task decomposition — from non-functional angles:

1. **Security** — auth gaps, injection surfaces, data exposure?
2. **Performance** — N+1 queries, large payloads, blocking operations?
3. **Existing patterns** — does it follow codebase conventions?
4. **Edge cases** — concurrency, race conditions, failure modes?

Update the OpenSpec change files directly if adjustments are needed.

> **Light mode:** skip Phase 3 entirely.

**Postcondition:** the user has approved the design, recorded in `proposal.md` (an approval note or checked box).

---

## Commit to the Integration Branch

Commit the change directory and any architecture docs to `$BASE` as a control-plane commit — per /aep-git-ref "Control-Plane Commits" (resolve `$BASE` per /aep-git-ref "Resolving $BASE"). Stage explicitly with `git add openspec/changes/<change-name>/ docs/` and message `feat: add <change-name> architecture doc and OpenSpec change`. This ensures the workspace has all artifacts when it is created from `$BASE`.

**Postcondition:** the change directory is committed and pushed to `$BASE`.

---

## Next Step

Design is complete. Proceed to `/aep-launch`, which spawns an autonomous workspace session to implement the feature.
