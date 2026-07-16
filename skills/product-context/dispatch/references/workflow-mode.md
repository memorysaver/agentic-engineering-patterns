# Dynamic Workflow Mode (`--batch wave` + "…with workflow")

The opt-in hands-free batch path for `/aep-dispatch` Step 5. Load this only when the
user explicitly asks to dispatch a wave **"with workflow"** AND the host is Claude
Code with the dynamic-workflow (Workflow) tool.

When both hold, route the batch through **workflow mode** instead of creating N
workers. The dispatch front-end is identical — sync, cascade, score, lock, assemble
context — only the execution plane changes: instead of N `/aep-launch` workers,
author one dynamic workflow that fans out `pipeline(stories, build, verify)` with one
agent per story (recipe: `/aep-executor` `references/backends.md` → "Mode: workflow").

```
Locks + creates OpenSpec changes for the ready stories in Wave N — up to the WIP limit (as usual)
Creates the .feature-workspaces/<name> worktrees (launch guardrails apply)
Then: one dynamic workflow, one agent per locked story (build → verify), each bound to its worktree
After the run: collect `gated` results → ask the human → resume gated stories with the answers
```

## Stale-base hazard — AEP-created worktrees are required (machine-assembled STEP-0 brief)

The Workflow/Agent tool's own `isolation: "worktree"` bases agents on a **stale
`origin/<base>`**, not your local integration-branch HEAD — which by this point
carries the dispatch-lock commit itself, so a host-isolated worker would start from a
base that predates the very lock that dispatched it, and the drift surfaces only at
merge time. If host-managed isolation is ever unavoidable, the brief must be
**machine-assembled**: after the lock commit, read the base by command —
`SHA=$(git rev-parse "$BASE")` — and print an explicit STEP-0 line into every agent
brief: `git checkout -B story/<id> <sha>`. Never let an agent recall or re-type the
base. See `/aep-autopilot` `references/deterministic-orchestration.md` →
Machine-assembled dispatch briefs.

## The STEP-0 brief carries the verification tier

Workflow mode bypasses `/aep-launch` entirely, so the tier cannot ride the criteria file — the
STEP-0 brief carries each story's **provisional verification tier** (from
`references/context-assembly.md`; grouped stories take the max), and the workflow's **verify
stage — that mode's evaluator — honors the tier's round cap and dimension preset**
(`/aep-gen-eval` → `references/verification-economics.md`): `light` → self-review only,
`standard` → one decisive fix-and-reverify cycle against the derived preset, `deep` → the full
cap at top effort. The build stage still runs the binding re-derivation at its Phase 5 entry;
the verify stage reads the resulting `verification-recipe.json` from the story's worktree.

## WIP limit still applies

Workflow mode does not exempt the wave from the WIP cap: each workflow agent still
opens a PR, so the integration/merge bottleneck is the same as Wave Batch. Lock at
most `available_slots` stories into the workflow (`available_slots =
concurrency_limit − current in_progress`); the workflow's own per-agent concurrency
cap is a separate, lower-level limit and does not replace this one.

## Announce the mode (this path bypasses `/aep-launch`)

Because dispatch authors the workflow directly instead of handing to `/aep-launch`,
dispatch owns the announcement that `/aep-launch` normally makes. State: "workflow
mode (dynamic workflow) — autonomous, billed, background; **no mid-stage steering**;
human gates **park and return here** for confirmation, then gated stories resume"
before authoring the workflow.

This is the hands-free batch path: autonomous, billed, background. Steering is at
stage boundaries only — but human decisions are NOT lost: a worker that hits one
returns a `gated` result (gate-and-park), this session asks you, and the story
resumes in its worktree with your answer. Use it when you want a wave built
autonomously without watching individual workers. Requires Claude Code + Workflow
tool (see `/aep-executor` `references/backends.md` → "Mode: workflow"). If the host
can't support it, fall back to Wave Batch and say so.
