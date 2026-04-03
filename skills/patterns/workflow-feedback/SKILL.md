---
name: workflow-feedback
description: |-
  Capture and route workflow learnings between downstream projects and AEP. Use after /wrap or /reflect when process observations need to be standardized, when reviewing what a build run taught about AEP workflows, or when pulling learnings from downstream projects back upstream. Triggers on "workflow feedback", "capture learnings", "what did we learn about process", "pull learnings from downstreams", "upstream lessons".
---

# Workflow Feedback

A reusable pattern for capturing workflow observations in downstream projects and routing them upstream to improve AEP skills and documentation. Ensures that lessons learned during builds don't stay buried in individual project repos.

**This skill has two modes:**

- **Capture mode:** Run in a downstream project after builds to standardize observations
- **Review mode:** Run in the AEP repo to pull and route upstream candidates from downstreams

```
DOWNSTREAM PROJECT                           AEP REPO
━━━━━━━━━━━━━━━━━━                           ━━━━━━━━
/build → lessons.md                          /workflow-feedback review
/wrap  → lessons-learned/                      ↓
/workflow-feedback capture                   Read .aep/config.yaml
  ↓                                            ↓
.dev-workflow/feedback.md  ──────────────→   Route to docs/
  (standardized + classified)                  ↓
                                             Human approves
                                               ↓
                                             sync-downstream.sh ──→ updated skills flow back
```

**Session:** Main, interactive with user
**Relates to:** `/reflect` (classifies product feedback), `/wrap` (archives workspace lessons), `/build` (captures lessons during execution)

---

## Mode 1: Capture

Run this in a **downstream project** after completing a layer, a batch of stories, or an autopilot run. The goal is to standardize raw observations into a format that AEP can review.

### Step 1 — Gather sources

Collect observations from all available sources:

1. **Archived lessons:** `lessons-learned/*.md` (written by `/wrap`)
2. **Process lessons:** `lessons-learned/process/*.md` (from `/reflect`)
3. **Unarchived workspace lessons:** `.feature-workspaces/*/dev-workflow/lessons.md` (if workspaces not yet wrapped)
4. **User observations:** Ask the user what they noticed during the run that isn't captured above

### Step 2 — Classify each observation

For each observation, assign a classification:

| Classification  | Description                                                         | Upstream? |
| --------------- | ------------------------------------------------------------------- | --------- |
| `process`       | AEP workflow improvement — a skill, phase, or gate should change    | Yes       |
| `tech-stack`    | Technology-specific gotcha — applies to any project using this tech | Yes       |
| `discovery`     | New understanding about the product domain or architecture          | Maybe     |
| `project-local` | Specific to this project's codebase, not generalizable              | No        |

### Step 3 — Write standardized feedback

Write to `.dev-workflow/feedback.md`:

```markdown
# Workflow Feedback: <project> <layer/context>

Date: YYYY-MM-DD
Project: <name>
Layer: <layer>
Stories: <count>

## Observations

### <title>

- **Classification:** process | tech-stack | discovery | project-local
- **Skill affected:** /calibrate, /build, /autopilot, etc. (if applicable)
- **Technology:** Rust, Cloudflare, etc. (if tech-stack)
- **Observation:** <what happened>
- **Recommendation:** <proposed change>
- **Upstream candidate:** yes | no
```

### Step 4 — Commit

Commit `.dev-workflow/feedback.md` to the downstream project. This makes it available for AEP review mode.

### Guardrails (Capture)

- **DO** include observations about AEP skill behavior, not just product bugs
- **DO** mark `upstream_candidate: yes` only for items that would benefit other projects using AEP
- **DO NOT** include product-specific bugs — those belong in `/reflect` → story creation
- **DO NOT** edit AEP skills from a downstream project — always route upstream

---

## Mode 2: Review

Run this in the **AEP repo** to pull feedback from downstream projects and route it into AEP documentation.

### Step 1 — Scan downstreams

Read `.aep/config.yaml` to find registered downstream project paths. For each project:

1. Check for `.dev-workflow/feedback.md` (standardized feedback from Capture mode)
2. Check for `lessons-learned/**/*.md` (raw lessons from builds, if no feedback.md exists)

If a downstream has no feedback file, note it and move on — don't block on incomplete data.

### Step 2 — Filter upstream candidates

From all collected observations, filter for:

- Items marked `upstream_candidate: yes`
- Items classified as `process` or `tech-stack` (these are almost always upstream-relevant)
- Items classified as `discovery` only if they reveal a pattern applicable beyond one project

### Step 3 — Route items

For each upstream candidate, determine the destination:

| Classification | Destination                                      | Format                                          |
| -------------- | ------------------------------------------------ | ----------------------------------------------- |
| `process`      | `docs/lessons/YYYY-MM-DD-<project>-<context>.md` | Date-prefixed lesson with skill amendment notes |
| `tech-stack`   | `docs/tech-stack/<technology>-<topic>.md`        | Standalone tech gotcha doc                      |
| `discovery`    | Present to human for decision                    | May go to `docs/decisions/` or `docs/workflow/` |

### Step 4 — Present summary

Show the human a table of all upstream candidates with proposed routing:

```
| # | Source | Classification | Title | Proposed destination |
|---|--------|---------------|-------|---------------------|
| 1 | looplia | process | /calibrate should modify real components | docs/lessons/... |
| 2 | looplia | tech-stack | Rust keyring needs platform features | docs/tech-stack/... |
```

The human approves, modifies, or rejects each item. **Never auto-edit skill files** — proposed skill amendments are documented in the lesson/decision file for manual application.

### Step 5 — Write approved items

For each approved item, create the target file following the conventions in `docs/README.md`.

After writing, remind the human to run `bash scripts/sync-downstream.sh` to push any resulting skill improvements back to downstream projects.

### Guardrails (Review)

- **DO** present all upstream candidates, even if they seem minor
- **DO** note which AEP skills are affected by process observations
- **DO NOT** auto-edit skill files — document proposed amendments, let human apply
- **DO NOT** pull items marked `project-local` unless the human explicitly requests it
- **DO NOT** assume feedback.md is complete — some downstreams may have lessons only in `lessons-learned/`

---

## When to Use This Skill

| Situation                                              | Mode    |
| ------------------------------------------------------ | ------- |
| Just finished a layer in a downstream project          | Capture |
| Autopilot run completed, want to capture learnings     | Capture |
| `/reflect` identified process observations             | Capture |
| Time to review what downstream projects have learned   | Review  |
| Preparing an AEP release with accumulated improvements | Review  |

---

## Relationship to Other Skills

- **`/reflect`** classifies product feedback (bugs, refinements, discoveries, polish). `/workflow-feedback` handles the process and tech-stack observations that `/reflect` identifies but doesn't route upstream.
- **`/wrap`** archives workspace lessons to `lessons-learned/`. `/workflow-feedback` capture reads those archives and standardizes them.
- **`/build`** writes raw observations to `.dev-workflow/lessons.md`. `/workflow-feedback` capture reads those if workspaces haven't been wrapped yet.
- **`/autopilot`** `orchestration-learning.md` captures meta-patterns across workspaces. `/workflow-feedback` review can pull those patterns upstream.
