# Observer procedure

Start when the person asks to observe an agent from this pane. Confirm `HERDR_ENV=1`, run `aep eval watch` to see the agents working in this project, and confirm with the person which pane to observe; if they named it, restate it and continue. Then `aep eval watch --target <pane-id>` creates the run and returns the parameters: project root, run id, run directory and the target pane; they are also saved as `procedure.md` in the run. Work in the project root with `--root` when a command runs elsewhere.

## Rhythm

The CLI paces the observer. After the run is created, loop on `aep eval tick --run <id>`: the command waits until at least the configured interval (ten minutes by default, `--interval` to change it) has passed since the previous tick, then waits for the target to reach idle, done or blocked, takes a snapshot and reports only the structural facts that changed since the last snapshot. Run it with a long tool timeout or as a background command; it blocks on purpose.

- `changed: no` and the target is working or idle: there is nothing to record. Run the next tick.
- `changed: yes`: read the recent transcript with `herdr agent read <target> --source recent-unwrapped --lines <n>`, find the claims the agent made about the work that changed (accepted, verified, delivered, committed, pushed), and check each against the record or Git fact that should support it. Record one observation for the milestone with `aep eval record --run <id> --file <path> --note "<one line>"`; a useful shape is `{"claim": "...", "source": "transcript|record|git", "evidence": {...}, "matches": true|false, "practice": "verification|records|delivery|git|legacy|devops", "note": "..."}`.
- `target needs attention` (blocked or done): note what it is waiting for; the person decides.

Separate live observation from reconstruction from transcript text, and mark what remains unverified. An observation is worth recording when a claim and its evidence disagree, when a milestone was independently verified, or when the target is waiting on a person; routine progress with matching evidence needs no record. Close with `aep eval report --run <id>` and a final observation summarizing what held, what did not, and what stays unverified; report the run directory to the person who started the watch.

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
