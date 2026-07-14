# Worktree Onboarding (orientation)

You are an agent spawned by `/aep-launch` into an isolated git worktree to
implement a feature autonomously. The design phases (1–3) were completed on the
integration branch by the user; your worktree is `.feature-workspaces/<name>` on
a fresh `feat/<name>` branch, matching the active OpenSpec change in
`openspec/changes/`. Your job is to execute Phases 0 and 4–12.

**Your prompt names your worktree path — operate exclusively inside it, and
report everything through `.dev-workflow/signals/`.** The answer to any human
gate always returns via the main agent (hub-and-spoke); parked workers are
resumed into this worktree with the answer.

## Start here

**Run `/aep-build`.** Its **Phase 0 is the onboarding** — it runs the worktree
guard (self-healing into the launch-made worktree), reads the change artifacts,
and generates every `.dev-workflow/` tracking artifact. The full autonomous
workflow, the Human-Gate Protocol, and your launch-mode transport table are all
in `/aep-build`.

**Resuming an interrupted session?** If `.dev-workflow/` already exists, do NOT
re-run the full bootstrap: run `bash .dev-workflow/init.sh` (restarts the dev
server, shows branch state + progress), read `.dev-workflow/progress-*.md` for
your current phase, check `.dev-workflow/signals/feedback.md`, and continue from
the first unchecked phase.
