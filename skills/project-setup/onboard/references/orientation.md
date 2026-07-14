# AEP Orientation — Installed Guide

This guide is bundled with `/aep-onboard`, so it remains available after an
individual skill is copied into a downstream repository.

## 1. Why AEP exists

Agent execution is abundant; precise specifications, product judgment, and
integration discipline are scarce. AEP spends human attention on deciding and
specifying the right work, then gives implementation agents isolated,
checkable contracts.

## 2. Three mental models

### Control plane and execution plane

- **Control plane:** you and the main-session model decide what to build.
  Product intent, story order, architecture, and acceptance criteria live here.
- **Execution plane:** a workspace-bound agent implements one approved change.
  It does not silently redefine product intent.
- **Boundary:** product context, OpenSpec artifacts, git state, and
  `.dev-workflow/signals/` carry decisions between the planes.

Use `/aep-envision`, `/aep-map`, `/aep-dispatch`, `/aep-design`, and
`/aep-reflect` for control-plane work. `/aep-launch` creates an execution-plane
workspace and `/aep-build` runs there.

### Story map

AEP organizes work as a Jeff Patton story map:

- **Activities** are the left-to-right user journey.
- **Layers** are top-to-bottom increments. Layer 0 is the thinnest working
  end-to-end skeleton.
- **Waves** group stories within a layer that can run without file or dependency
  conflicts.
- **Layer gates** prove the integrated layer before the next layer starts.
- **Alignment layers** insert human calibration where quality is subjective.

### Two-session workflow

- The **main session** stays on the integration branch and owns decisions,
  dispatch, monitoring, wrap-up, and context updates.
- A **workspace session** stays in `.feature-workspaces/<name>` on
  `feat/<name>` and owns implementation, verification, and its pull request.
- Signal files are the durable coordination channel. Do not rely on two agents
  sharing conversational context.

## 3. Skills by responsibility

- **Understand and plan:** `/aep-envision`, `/aep-map`, `/aep-model`,
  `/aep-calibrate`, `/aep-validate`.
- **Select and learn:** `/aep-dispatch`, `/aep-watch`, `/aep-reflect`.
- **Ship one change:** `/aep-design`, `/aep-launch`, `/aep-build`, `/aep-wrap`.
- **Run continuously:** `/aep-autopilot`.
- **Set up a repository:** `/aep-onboard`, `/aep-scaffold`,
  `/aep-e2e-skill-scaffolding`.
- **Reusable mechanics:** `/aep-git-ref`, `/aep-executor`, `/aep-gen-eval`,
  `/aep-workflow`, `/aep-workflow-feedback`, `/aep-design-lens`.

## 4. Pick one of four paths

1. **New product:** `/aep-envision` → `/aep-map` → `/aep-validate` →
   `/aep-scaffold`, then drive stories manually or run `/aep-autopilot`.
2. **Existing project adopting AEP:** `/aep-scaffold`, optionally create product
   context with `/aep-envision`, then start with `/aep-dispatch`.
3. **One feature without product context:** `/aep-design` → `/aep-launch` →
   `/aep-build` → `/aep-wrap`.
4. **Hands-free execution:** use `/aep-autopilot` only after product context and
   the ready queue have been validated.

Decision rule: if you are still deciding whether or what to build, stay on the
control plane. If an approved OpenSpec change exists, launch an isolated
workspace. If several validated stories should run without repeated human
dispatch, use autopilot.

## 5. First-hour checklist

- Finish `/aep-onboard` and commit the installed skills plus `skills-lock.json`.
- Confirm whether the integration branch is `main`, `develop`, or a configured
  override using `/aep-git-ref`.
- Choose exactly one path above.
- Run its first skill and inspect the produced artifact before continuing.
- Keep main-session decisions and workspace implementation separate.
- Require observable postconditions: files, exit codes, git state, structured
  fields, or signal state—not statements of confidence.
