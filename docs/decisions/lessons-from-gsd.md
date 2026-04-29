# Lessons from GSD (Get Shit Done) — Comparative Study

**Status:** Observation / proposal — no changes to AEP yet
**Date:** 2026-04-17
**Subject:** `gsd-build/get-shit-done` v1.36.0
**Related:** [aep-v2-improvement-guideline.md](../aep-v2-improvement-guideline.md)

## Context

GSD ("Get Shit Done") is a spec-driven, context-engineering framework for
Claude Code (and 13 other runtimes) built by a solo developer under the handle
**TÂCHES**. It is positioned explicitly against "enterprise-theater" frameworks
(BMAD, SpecKit, Taskmaster) and markets itself as _"the context engineering
layer that makes Claude Code reliable."_

It is worth studying because it has shipped, has distribution (npm package
`get-shit-done-cc`, multi-runtime installer), and solves the same surface
problem AEP solves — turning an LLM into a reliable software-producing system —
but with different design choices. Understanding those choices sharpens AEP's
own positioning and surfaces concrete ergonomics that AEP currently lacks.

This document captures:

1. A summary of GSD's design and command surface
2. A neutral comparison against AEP on ten axes
3. A prioritized list of ideas worth adopting (Tier 1 / 2 / 3)
4. An explicit list of what **not** to adopt, with reasons

Sources: GSD README v1.36.0 (retrieved 2026-04-17 via `defuddle`), this repo's
`README.md` and `docs/aep-v2-improvement-guideline.md`, and a repo-wide Explore
pass over the 16 AEP skills.

---

## 1. GSD in One Paragraph

GSD is a **thin-orchestrator, fresh-context** framework. Every major action
(research, plan, execute, verify) is a slash command whose orchestrator spawns
one or more specialized subagents — each in a brand-new ~200K context —
collects their results, and routes them forward. The main session is kept at
30–40% utilization because the "real work" happens in those sub-contexts. The
workflow lifecycle is `new-project → discuss-phase → plan-phase → execute-phase
→ verify-work → ship → complete-milestone → new-milestone`, with `/gsd-next`
auto-detecting and running the next step. Plans are written as **XML
documents** with explicit `<task>`, `<files>`, `<action>`, `<verify>`, `<done>`
tags, one plan per atomic task. Every task produces its own git commit with a
phase-prefixed message for bisectability. The command surface is **broad**
(~40+ commands): core workflow, workstreams, multi-repo workspaces, UI design,
brownfield import (`/gsd-map-codebase`), session pause/resume (with
`HANDOFF.json`), threads, seeds/backlog/todos/notes, security enforcement,
forensics, health checks, cross-AI peer review, model profiles
(quality/balanced/budget). Distribution is a single `npx get-shit-done-cc@latest`
that offers global-vs-local install and targets 14 runtimes.

## 2. AEP in One Paragraph

AEP is a **two-plane, spec-precision-first** framework. Its core thesis: when
agents can execute in parallel, spec precision — not coding speed — is the
bottleneck, so heavy upfront investment in unambiguous specs pays back
exponentially. The **Control Plane** (`/envision`, `/map`, `/dispatch`,
`/calibrate`, `/reflect`, `/design`) is where humans and AI decide _what_ to
build; the **Execution Plane** (`/launch`, `/build`, `/wrap`) is where agents
build _in isolation_ — each story in its own `git worktree` on a `feat/<name>` branch, each agent session
hosted in `tmux/cmux`, with communication strictly through structured signal
files rather than chat. Structural primitives come from Jeff Patton's user
story mapping: **activities** (user journey, left→right), **layers**
(enrichment, top→bottom), **waves** (parallel batches within a layer),
**walking skeleton** (Layer 0 end-to-end), **layer gates** (integration tests
at layer boundaries), and **`.5` alignment layers** where humans calibrate
across one of seven quality dimensions (visual-design, ux-flow, api-surface,
data-model, scope-direction, copy-tone, performance-quality). A single
`product-context.yaml` (or, in v2 split mode, `product/index.yaml` +
per-capability maps) is the single source of truth. The surface is **narrow**
(16 skills across four plugins) and an explicit v2 principle forbids adding
new commands. Distribution is a local `scripts/sync.sh` / `sync-downstream.sh`
that pushes skills into downstream projects' `.claude/skills/`.

---

## 3. Side-by-Side — Ten Axes

| Axis                     | GSD                                                                                                                                                                                                                                                                        | AEP                                                                                                                                                                             |
| ------------------------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Philosophical frame**  | Indie-builder pragmatism; anti-enterprise-theater                                                                                                                                                                                                                          | Anthropic-engineering derived; spec-precision thesis                                                                                                                            |
| **Primary unit of work** | Phase (numbered, within a milestone)                                                                                                                                                                                                                                       | Story (point on `activity × layer × wave` grid)                                                                                                                                 |
| **Context store**        | Many markdown files: `PROJECT.md`, `REQUIREMENTS.md`, `ROADMAP.md`, `STATE.md`, per-phase `CONTEXT.md`, per-task `PLAN.md`, `SUMMARY.md`, `threads/`, `seeds/`, `todos/`                                                                                                   | Single `product-context.yaml` (optionally split as `product/index.yaml` + capability maps); `technical-spec.md` for complex systems                                             |
| **Orchestration model**  | Task subagents of main CC session; each gets fresh ~200K                                                                                                                                                                                                                   | Separate `tmux` + `git worktree` per story (on a `feat/<name>` branch); main session never inspects workspace code directly; signal files are the only channel                  |
| **Design refinement**    | `/gsd-discuss-phase` surfaces gray areas by feature type (visual / API / content / organization); writes `CONTEXT.md` consumed by researcher + planner; has `discuss` vs `assumptions` modes                                                                               | `/design` refines one spec interactively; `/calibrate` handles seven quality dimensions with heavy / light classes; `.5` alignment layers are structural                        |
| **Verification**         | `/gsd-verify-work` extracts testable deliverables, walks user through each, auto-diagnoses failures, spawns fix-plans; `/gsd-audit-milestone` checks Definition of Done; `/gsd-secure-phase` anchors verification to threat model; `/gsd-review` runs cross-AI peer review | Layer gates (integration tests at layer boundaries); generator/evaluator separation (`/gen-eval`); `/validate` artifact quality gates; `/reflect` classifies post-ship feedback |
| **Distribution**         | `npx get-shit-done-cc@latest`, 14 runtimes, `--global` / `--local`, `--uninstall` per runtime                                                                                                                                                                              | Local repo + `scripts/sync.sh` to copy skills into any project's `.claude/skills/` with `aep-` prefix                                                                           |
| **Session management**   | `/gsd-pause-work` writes `HANDOFF.json`; `/gsd-resume-work`; `/gsd-session-report`; `/gsd-thread`; `/gsd-plant-seed` (trigger-based future ideas); `/gsd-note`; `/gsd-add-todo`; `/gsd-forensics`; `/gsd-health --repair`                                                  | `lessons.md` per workspace, escalated to `lessons-learned/` on `/wrap`; `/workflow-feedback` classifies process learnings — no ad-hoc session artifacts                         |
| **Quality gates**        | Tactical: schema-drift detection, scope-reduction detection, `gsd-prompt-guard` hook (injection vector scanning), path-traversal validation, centralized `security.cjs` module                                                                                             | Structural: layer gates (integration tests), generator/evaluator separation, seven-dimension calibration taxonomy                                                               |
| **Command surface**      | ~40+ commands (rich but high cognitive load)                                                                                                                                                                                                                               | 16 skills with an explicit "no new commands" v2 principle                                                                                                                       |

**The high-order difference.** GSD optimizes for **ergonomic breadth**: a
command for every moment of the workflow, rich session plumbing, broad runtime
reach. AEP optimizes for **structural rigor**: fewer primitives, stronger
agent isolation, a defensible story-map topology, a calibration taxonomy that
routes feedback precisely. Both are valid; they are on different Pareto fronts.

---

## 4. What AEP Should Adopt — Prioritized

### Tier 1 — High ROI, low friction (recommended)

#### 1.1 Forward-looking "seeds" with triggers

GSD's `/gsd-plant-seed <idea>` captures forward-looking ideas bound to a
trigger condition (milestone reached, metric hit). The idea surfaces
automatically at the right moment instead of rotting in a parking-lot file.
AEP currently has `open_questions` in `product-context.yaml`, but those are
passive; there is no trigger model and nothing surfaces them to `/dispatch` or
`/autopilot`.

**Proposed addition:** new `seeds:` section in `product-context.yaml`:

```yaml
seeds:
  - id: oauth-social-login
    idea: "Add Google / GitHub OAuth"
    trigger:
      type: layer_reached
      value: 1
    status: dormant # dormant | surfaced | promoted | dropped
    notes: "Wait until core auth proves out"
```

`/dispatch` (and `/autopilot`) would check dormant seeds against current state
on each tick and flip them to `surfaced` when their trigger fires, prompting
the human to either promote to a story or drop.

#### 1.2 Mid-story handoff artifact

GSD's `/gsd-pause-work` writes `HANDOFF.json` when a user stops mid-phase;
`/gsd-resume-work` restores position on the next session. AEP tracks story
status in `product-context.yaml` but has no first-class artifact for "I paused
in the middle of `/build`, here is the partial state" — so resuming a paused
workspace is brittle across session boundaries.

**Proposed addition:** a `handoff.md` signal file under
`.dev-workflow/signals/` that `/build` writes on graceful pause, capturing:
current phase, last completed task, outstanding review comments, uncommitted
changes (as a unified diff or reference), next intended step. `/launch --resume`
consumes it.

#### 1.3 `/next` auto-advance (as a flag, not a new command)

GSD's `/gsd-next` inspects state and runs the next logical step. AEP has
`/autopilot` (fully autonomous loop) but nothing for "I am here manually, do
the obvious next thing." This is a genuine ergonomic gap, but a new command
would violate the "no new commands" principle.

**Proposed addition:** `/dispatch --auto` mode that reads
`product-context.yaml`, determines the last-completed status per story, and
routes to the correct next skill (`/design`, `/launch`, `/calibrate`,
`/wrap`, or `/reflect`) without requiring the user to remember which command
comes next.

#### 1.4 `/validate --diagnose` forensics mode

GSD has both `/gsd-forensics` (post-mortem for failed workflow runs) and
`/gsd-health --repair` (integrity check and auto-fix). AEP's `/autopilot` and
`/validate` cover parts of this but there is no single diagnostic pass that
checks workspace state, `git worktree` integrity, signal-file consistency, and OpenSpec
artifact completeness.

**Proposed addition:** `/validate --diagnose` that runs an integrity sweep and
reports anomalies — stuck ticks, missing signal files, orphan workspaces,
unpushed feature branches, stories with status mismatched against PR state.
Optional `--repair` flag for safe auto-fixes (e.g., re-sync story status from
PR).

#### 1.5 Brownfield entry path

GSD's `/gsd-map-codebase [area]` runs parallel agents to analyze stack,
architecture, conventions, and concerns before `/gsd-new-project`. AEP's
`/onboard` and `/envision` implicitly assume greenfield — there is no way to
feed existing-codebase knowledge into the context document.

**Proposed addition:** `/envision --import` mode (or equivalent flag on
`/map`) that runs a parallel scan of the repo, extracts stack/architecture
inventory, and pre-populates `architecture` + `constraints` in
`product-context.yaml` before the human answers opportunity questions. This
makes AEP viable for mid-project adoption, not only day-zero use.

### Tier 2 — Worth considering

#### 2.1 Model profiles per skill

GSD ships four profiles — `quality` (Opus-heavy), `balanced` (Opus plan /
Sonnet exec — default), `budget` (Sonnet + Haiku), `inherit` (follow runtime
selection) — mapped per agent type (planning / execution / verification). AEP
has no explicit model routing. Adopting a `.aep/config.yaml` field
`agent_models: { build: sonnet, design: opus, calibrate: opus, ... }` would
give teams a cost dial without changing any skill logic.

#### 2.2 Security as an eighth calibration dimension

GSD's `/gsd-secure-phase` anchors verification to a threat model — the agent
maps requirements to threats, verifies each is covered. AEP's seven
calibration dimensions include `performance-quality` but not security. Adding
a `security` dimension (heavy class, produces `calibration/security.yaml`)
would let `/calibrate` drive threat-model-first verification at each layer.

#### 2.3 Retroactive visual-design audit

GSD's `/gsd-ui-review` is a **retroactive** six-pillar audit of _implemented_
frontend code against its UI spec. AEP's `/calibrate visual-design` is
currently forward-only — it captures the design system, then agents build
against it. Adding `/calibrate visual-design --audit` that diffs merged code
against the captured calibration (component visual parity, spacing tokens,
color tokens, interaction states) would close the loop.

#### 2.4 Granularity dial

GSD's top-level config has `granularity: coarse | standard | fine` which
controls how finely scope is sliced (phases × plans). AEP has per-story S/M/L
complexity, but no global dial for "I want fewer, bigger stories this time"
or "slice this finely." Adding `planning.granularity` to `.aep/config.yaml`,
honored by `/map` when writing stories, gives the user a single knob.

#### 2.5 Interactive operations dashboard

GSD's `/gsd-manager` is a TUI command center for managing multiple phases.
AEP has `/autopilot` running continuously but no dashboard view. An
`/autopilot --status` (read-only) mode showing: active workspaces, current
phase per workspace, last signal timestamps, cost-to-date per story, and
blocked-on flags would close a visibility gap during long autonomous runs.

### Tier 3 — Strategic, higher cost

#### 3.1 npm distribution

GSD's `npx get-shit-done-cc@latest` installer is a significant adoption lever.
AEP currently requires cloning this repo and running `sync.sh` to push skills
into `.claude/skills/` — fine for the author, high-friction for third-party
adoption. If AEP is intended to spread, an `npx aep-cc` installer (with
`--global` / `--local`, runtime selection, and `--uninstall`) is the right
move. Tradeoff: locks the repo into a packaging + release workflow.

#### 3.2 Persistent `threads/` directory

GSD's `/gsd-thread [name]` provides lightweight persistent context for
investigations or debugging that span multiple sessions. AEP has
`lessons-learned/` for _post-hoc_ capture but nothing for _in-flight_
cross-session context. A `threads/` directory with a thin create / append /
archive slot in `/workflow-feedback` would fill the gap.

#### 3.3 Formal `backlog` section in `product-context.yaml`

GSD has both `/gsd-add-backlog` (999.x numbering, outside the active
sequence) and `/gsd-review-backlog` (promote to active milestone or drop
stale). AEP scatters equivalent information across `open_questions`, the
proposed `seeds`, and the semantic changelog. A single `backlog:` section
with a promote-to-story action would consolidate this — and, unlike GSD,
could share the trigger model with `seeds` instead of being a separate
concept.

---

## 5. What AEP Should NOT Adopt — Guardrails

### 5.1 The ~40-command surface

GSD's surface area is a weakness disguised as a feature. Commands like
`/gsd-add-todo`, `/gsd-note`, `/gsd-add-backlog`, `/gsd-plant-seed`,
`/gsd-thread`, `/gsd-check-todos`, `/gsd-review-backlog` represent one concept
("capture an idea, surface it later") spread across six+ commands. Each new
command is a new mental model and a new decision point. AEP's v2 principle
("no new commands, only flags and schema extensions on the existing 16") is a
genuine strength — lessons adopted from GSD should honor it.

### 5.2 Moving agents back into the main session

GSD's orchestrator-spawns-Task-subagents model is simpler than AEP's
`tmux + git worktree` isolation, but it loses two properties:

1. **Long-running parallel isolation.** Subagents share the main process; a
   crashed agent affects the session. AEP's `tmux` hosts survive main-session
   restarts.
2. **Filesystem-level separation.** Two subagents editing adjacent files can
   create race conditions. AEP's `git worktree add -b feat/<name>` gives each
   agent its own working tree on its own branch.

For long-running autonomous work, AEP's model is structurally safer and
should not be abandoned in the name of simplicity.

### 5.3 Scattered markdown files in place of a YAML schema

GSD's `PROJECT.md` + `REQUIREMENTS.md` + `ROADMAP.md` + `STATE.md` + per-phase
`CONTEXT.md` + per-task `PLAN.md` are human-readable but **not machine-
introspectable**. Skills cannot easily query "how many stories are in layer
1?" or "what is the dispatch score of story X?" without grepping. AEP's
`product-context.yaml` is programmatically readable — that is the enabler for
`/dispatch` scoring formulas, `/validate` coverage checks, and the
outcome-contract additions in the v2 roadmap. Keep the schema.

### 5.4 Collapsing the calibration taxonomy

GSD's `/gsd-discuss-phase` surfaces gray areas _by feature type_ (visual,
API, content, organization). This is coarser than AEP's seven-dimension ×
heavy/light calibration classification. The taxonomy lets `/reflect` route
feedback precisely ("this is a copy-tone issue, not a scope issue") and
makes `.5` alignment layers composable. Don't trade it for GSD's simpler
rubric.

### 5.5 Removing layer gates

GSD has phases and milestones but no equivalent of "this layer works
end-to-end as a whole before the next layer starts." Layer gates are a
silent strength of the story-map topology — they catch integration
regressions that phase-level verification misses. Any proposed change that
would collapse layers into a single phase-like unit should be rejected on
this basis.

---

## 6. Open Questions

### 6.1 Merge or stay separate?

Tier 1 items 1–5 are concrete enough to be folded into
`aep-v2-improvement-guideline.md` as numbered proposals. The open question is
whether to do that in one pass (risk: large diff, merge conflict with
existing roadmap items) or to introduce them as individual proposals over
time (risk: momentum loss).

### 6.2 Pilot first?

Each Tier 1 proposal is additive and non-breaking, but "additive" is where
complexity creep starts. A conservative path is to pilot one proposal
(recommend: 1.2 mid-story handoff, since it solves a concrete observed pain
point) end-to-end on a real build before adopting the others. If pilots
don't show value, the proposal is dropped — including from this document.

### 6.3 GSD's license and any reuse

GSD is MIT-licensed. Nothing in this document proposes code reuse — only
conceptual borrowing — but any future decision to lift templates or XML
structures should retain attribution.

---

## 7. References

- **GSD repo:** https://github.com/gsd-build/get-shit-done (README v1.36.0 as
  of 2026-04-17)
- **GSD user guide:** https://github.com/gsd-build/get-shit-done/blob/main/docs/USER-GUIDE.md
- **GSD distribution:** `npx get-shit-done-cc@latest`
- **AEP philosophy:** [`README.md`](../../README.md)
- **AEP v2 roadmap:** [`aep-v2-improvement-guideline.md`](../aep-v2-improvement-guideline.md)
- **AEP docs routing:** [`docs/README.md`](../README.md)
- **Source material — Anthropic Engineering:**
  - Harness Design for Long-Running Application Development
  - Effective Harnesses for Long-Running Agents
  - Effective Context Engineering for AI Agents
