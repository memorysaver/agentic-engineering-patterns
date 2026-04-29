---
name: design
description: Interactive feature design on main branch. Use when starting a new feature, or when the user says "design a feature", "let's design", "explore and propose". Runs OpenSpec explore, propose, and design review phases interactively with the user, then commits artifacts to main. The first step in the feature lifecycle — followed by /launch.
---

# Design

Interactive feature design on the `main` branch. Explore the problem, propose a solution, review the design, and commit artifacts — all in conversation with the user.

**Where this fits:**

```
/onboard → /scaffold → [ /design → /launch → /build → /wrap ]
                          ▲ you are here
```

**Session:** Main session, interactive with user
**Input:** Feature idea or user request (optionally informed by product context)
**Output:** OpenSpec change committed to main (proposal, design, specs, tasks)

---

## Operating Mode

This skill works in two modes, auto-detected at startup:

```bash
ls product-context.yaml 2>/dev/null
```

**Standalone mode** _(no `product-context.yaml`)_ — Feature lifecycle runs independently. Proceed directly to prerequisites and design phases.

**Product-cycle mode** _(has `product-context.yaml`)_ — Feature is part of a larger product lifecycle (`/envision` → `/map` → `/dispatch` → `/design`):

- Read from `product-context.yaml` for project-wide context
- If a story was dispatched (has `openspec_change` set in the YAML), load that story's acceptance criteria, interface obligations, and relevant architecture module
- When dispatched from `/dispatch`, the OpenSpec change already exists — `/opsx:propose` refines it rather than starting from scratch

---

## Prerequisites

Before starting, verify dependencies are available.

### CLI Tools

Run this check:

```bash
for cmd in git openspec; do
  printf "%-15s" "$cmd:"
  which $cmd >/dev/null 2>&1 && echo "OK ($(which $cmd))" || echo "MISSING"
done
# PR/MR tool (need at least one):
printf "%-15s" "gh or glab:"
(which gh >/dev/null 2>&1 || which glab >/dev/null 2>&1) && echo "OK" || echo "MISSING"
```

If any required tool is missing, run `/onboard` first.

### Required Skills

Check that OpenSpec skills exist:

```bash
for skill in openspec-explore openspec-propose openspec-apply-change openspec-archive-change; do
  printf "%-35s" "$skill:"
  [ -f ".claude/skills/$skill/SKILL.md" ] && echo "OK" || echo "MISSING"
done
```

If OpenSpec skills are missing, run `/scaffold` first.

---

## Workflow Mode Selection

Before starting design, decide on the workflow mode. This choice carries through to `/launch` and `/build`.

### Full mode (default)

All phases + separate evaluator agent. Use for:

- Complex features with 3+ tasks
- UI-heavy work
- Security-sensitive features
- Anything at the edge of model capability

### Light mode

Simplified flow, no evaluator. Use for:

- Simple CRUD
- Config changes
- Small bug fixes

### Tuning principle

> "Every component in a harness encodes an assumption about what the model can't do on its own. Those assumptions deserve stress-testing."
> — Anthropic, ["Harness Design for Long-Running Application Development"](https://www.anthropic.com/engineering/harness-design-long-running-apps)

With each model upgrade, re-evaluate which phases provide value.

---

## Phase 1: OpenSpec Explore

Invoke the explore skill to think through the feature:

```
/opsx:explore
```

Use this phase to:

- Clarify requirements and scope with the user
- Investigate the codebase for relevant patterns
- Identify risks or unknowns
- Build shared understanding
- Create architecture documentation in `docs/` if the feature warrants it

---

## Phase 2: OpenSpec Propose

Invoke the propose skill to generate a full proposal:

```
/opsx:propose
```

This creates the OpenSpec change with all artifacts:

- `proposal.md` — what and why
- `design.md` — how, key decisions, risks
- `specs/**/*.md` — detailed requirements and scenarios
- `tasks.md` — implementation checklist

---

## Phase 3: Design Review

Before implementation, review the proposal from non-functional angles:

1. **Security** — Auth gaps, injection surfaces, data exposure?
2. **Performance** — N+1 queries, large payloads, blocking operations?
3. **Existing patterns** — Does it follow codebase conventions?
4. **Edge cases** — Concurrency issues, race conditions, failure modes?

**What NOT to review:** Business logic (decided in Phase 1), cosmetic preferences.

If adjustments are needed, update the OpenSpec change files directly.

> **Light mode:** Skip Phase 3 entirely.

---

## Commit to Main

After design is complete, commit all artifacts to `main`:

```bash
git pull --ff-only origin main
git add openspec/changes/<change-name>/ docs/
git commit -m "feat: add <change-name> architecture doc and OpenSpec change"
git push origin main
```

This ensures the workspace will have all artifacts when it's created from `main`. The `--ff-only` pull avoids overwriting concurrent pushes.

---

## Next Step

Design is complete. Proceed to:

```
/launch
```

This spawns an autonomous workspace session to implement the feature.
