# Observer procedure

The parameters arrive in the prompt: project root, eval run id, run directory and the target pane. Confirm `HERDR_ENV=1`, then `herdr agent get <target>` to learn the working agent's kind and state. Work in the project root with `--root` when a command runs elsewhere.

## Rhythm

1. Baseline: `aep eval snapshot --run <id>` once at the start; read `aep status`, `aep check` and `git status` yourself so later comparisons have a reference.
2. Wait for milestones: `herdr agent wait <target> --until idle --timeout <ms>` (blocked and done also end a turn). On each milestone read the recent transcript with `herdr agent read <target> --source recent-unwrapped --lines <n>`, take another snapshot, and inspect what changed: new records (`aep timeline`, `aep query --kind event`), Git commits, worktrees, checks and reviews.
3. Compare: for each claim the agent made about its work (accepted, verified, delivered, committed, pushed) find the record or Git fact that supports it, or note that none does. Separate live observation from reconstruction from transcript text, and mark what remains unverified.
4. Record: write one JSON object per observation and store it with `aep eval record --run <id> --file <path> --note "<one line>"`. A useful shape is `{"claim": "...", "source": "transcript|record|git", "evidence": {...}, "matches": true|false, "practice": "verification|records|delivery|git|legacy|devops", "note": "..."}`.
5. Close: `aep eval report --run <id>` for the rule findings, then a final observation summarizing what held, what did not, and what stays unverified. Report the run directory to the person who started the watch.

## Reading the working agent without steering it

Read panes with `herdr agent read` and `herdr pane read`; do not use `herdr agent prompt`, `send-keys` or answer its dialogs. If the agent asks a question that only a person can answer, note that it is waiting; the person decides. If the transcript scrolled past what Herdr can recover, say the section is unrecoverable rather than reconstructing it.

The project's files are read-only for the observer. Copies for evidence go to the run directory (`copies/`), not next to the originals. `aep` read commands (`status`, `check`, `context`, `query`, `timeline`, `migrate verify`, `doctor`) are safe; `aep eval snapshot` and `record` write only under `~/.aep`.

## What to look for

- Records: did the agent read `aep --skill` and the applicable reference before acting; are decisions accepted with attribution that names the person's statement; do containers carry outcomes; are events written with notes; are imported records reconciled or superseded rather than left inert.
- Verification: did `aep verify run` execute the configured checks in the attempt's worktree; is evidence bound to the revision that was reviewed; if policy requires independent review, who performed it.
- Delivery: is integration recorded with a receipt whose head matches Git; are specifications published before a change is closed; are worktrees and attempts closed when the work ends.
- Git: are commits made and pushed on the project's own cadence; is the tree clean at handoff; are secrets scanned before commits.
- Handoff: does the final report name story, change, container, worktree and remaining work so another agent can resume from `aep context`.

Write what was seen and what supports it. Leave interpretation of product intent to the project.
