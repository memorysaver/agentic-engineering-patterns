---
name: aep-workflow-feedback
description: |-
  Capture workflow learnings in a downstream project and route them upstream to AEP. Use after /aep-wrap or /aep-reflect to standardize process/tech-stack observations, or in the AEP repo to pull learnings from downstreams. Triggers on "workflow feedback", "capture learnings", "pull learnings upstream".
---

# Workflow Feedback

A reusable pattern for capturing workflow observations in downstream projects and routing them upstream to improve AEP skills and documentation. Ensures lessons learned during builds don't stay buried in individual project repos.

**Two modes:**

- **Capture** — run in a downstream project after builds to standardize observations.
- **Review** — run in the AEP repo to pull and route upstream candidates from downstreams.

```
DOWNSTREAM PROJECT                           AEP REPO
━━━━━━━━━━━━━━━━━━                           ━━━━━━━━
/aep-build → lessons.md                          /aep-workflow-feedback review
/aep-wrap  → lessons-learned/                      ↓
/aep-workflow-feedback capture                   Read .aep/config.yaml
  ↓                                            ↓
.dev-workflow/feedback.md  ──────────────→   Route to docs/
  (standardized + classified)                  ↓
                                             Human approves
                                               ↓
                                             tag release → re-pin downstreams ──→ updated skills flow back
```

**Session:** Main, interactive with user.

**Hard guardrail — this skill never edits AEP skill files.** In either mode, propose amendments in the feedback/lesson file; a human applies them upstream via a deliberate re-pin.

---

## Mode 1: Capture

Run this in a **downstream project** after completing a layer, a batch of stories, or an autopilot run. The goal is to standardize raw observations into a format AEP can review.

### Step 1 — Gather sources

Collect observations from all available sources, including AEP skill/process behavior, not only product bugs (product-specific bugs go to `/aep-reflect` → story creation, not here):

1. **Archived lessons:** `lessons-learned/*.md` (written by `/aep-wrap`)
2. **Process lessons:** `lessons-learned/process/*.md` (from `/aep-reflect`)
3. **Unarchived workspace lessons:** `.feature-workspaces/*/dev-workflow/lessons.md` (if workspaces not yet wrapped)
4. **User observations:** Ask the user what they noticed during the run that isn't captured above.

Postcondition: each user observation is recorded in the feedback file (Step 3), or the user explicitly confirms there were none.

### Step 2 — Classify each observation

Assign each observation a classification:

| Classification  | Description                                                         | Upstream? |
| --------------- | ------------------------------------------------------------------- | --------- |
| `process`       | AEP workflow improvement — a skill, phase, or gate should change    | Yes       |
| `tech-stack`    | Technology-specific gotcha — applies to any project using this tech | Yes       |
| `discovery`     | New understanding about the product domain or architecture          | Maybe     |
| `project-local` | Specific to this project's codebase, not generalizable              | No        |

Mark `upstream_candidate: yes` only for items that would benefit other projects using AEP. Postcondition: every observation carries a classification.

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
- **Skill affected:** /aep-calibrate, /aep-build, /aep-autopilot, etc. (if applicable)
- **Technology:** Rust, Cloudflare, etc. (if tech-stack)
- **Observation:** <what happened>
- **Recommendation:** <proposed change>
- **Upstream candidate:** yes | no
```

Postcondition: `.dev-workflow/feedback.md` exists with one Observations entry per gathered observation.

### Step 4 — Commit

Commit `.dev-workflow/feedback.md` to the downstream project, making it available for AEP review mode. Postcondition: `git status` shows `.dev-workflow/feedback.md` committed.

---

## Mode 2: Review

Run this in the **AEP repo** to pull feedback from downstream projects and route it into AEP documentation.

### Step 1 — Scan downstreams

Read `.aep/config.yaml` to find registered downstream project paths. For each project:

1. Check for `.dev-workflow/feedback.md` (standardized feedback from Capture mode).
2. If no `feedback.md` exists, check `lessons-learned/**/*.md` (raw lessons from builds) — feedback may be incomplete, so read these directly when present.

If a downstream has no feedback file, note it and move on — don't block on incomplete data. Postcondition: every registered downstream is either scanned or noted as having no feedback.

### Step 2 — Filter upstream candidates

Pull observations that are:

- Marked `upstream_candidate: yes`, or
- Classified `process` or `tech-stack` (almost always upstream-relevant), or
- Classified `discovery` only when they reveal a pattern applicable beyond one project.

Pull `project-local` items only when the human explicitly requests it.

### Step 3 — Route items

For each upstream candidate, determine the destination, noting which AEP skills a `process` observation affects:

| Classification | Destination                                      | Format                                          |
| -------------- | ------------------------------------------------ | ----------------------------------------------- |
| `process`      | `docs/lessons/YYYY-MM-DD-<project>-<context>.md` | Date-prefixed lesson with skill amendment notes |
| `tech-stack`   | `docs/tech-stack/<technology>-<topic>.md`        | Standalone tech gotcha doc                      |
| `discovery`    | Present to human for decision                    | May go to `docs/decisions/` or `docs/workflow/` |

### Step 4 — Present summary

Show the human a table of every upstream candidate — including minor ones — with proposed routing:

```
| # | Source | Classification | Title | Proposed destination |
|---|--------|---------------|-------|---------------------|
| 1 | looplia | process | /aep-calibrate should modify real components | docs/lessons/... |
| 2 | looplia | tech-stack | Rust keyring needs platform features | docs/tech-stack/... |
```

The human approves, modifies, or rejects each item.

### Step 5 — Write approved items

For each approved item, create the target file following the conventions in `docs/README.md`. Record proposed skill amendments inside the lesson/decision file (per the hard guardrail above) for a human to apply.

After writing, remind the human that approved skill improvements reach downstream projects only via a deliberate re-pin: cut a new AEP release tag, then re-pin each downstream registered in `.aep/config.yaml` per the README "Upgrading to a new release" flow. Postcondition: each approved item exists as a file under `docs/`.

---

## When to Use This Skill

| Situation                                              | Mode    |
| ------------------------------------------------------ | ------- |
| Just finished a layer in a downstream project          | Capture |
| Autopilot run completed, want to capture learnings     | Capture |
| `/aep-reflect` identified process observations         | Capture |
| Time to review what downstream projects have learned   | Review  |
| Preparing an AEP release with accumulated improvements | Review  |

Relationship to adjacent skills: `/aep-reflect` classifies _product_ feedback (bugs, refinements, discoveries, polish); this skill handles the _process_ and _tech-stack_ observations it surfaces but doesn't route upstream. `/aep-wrap` archives workspace lessons to `lessons-learned/` and `/aep-build` writes raw observations to `.dev-workflow/lessons.md` — Capture reads both. `/aep-autopilot`'s `orchestration-learning.md` captures meta-patterns across workspaces that Review can pull upstream.
