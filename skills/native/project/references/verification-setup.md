# Establish project-owned verification

Find existing test commands, CLI/API/browser controls, fixtures, and journeys. Start with an operation a user can actually perform, including an observable result or side effect. Unit tests remain useful, but product claims also need evidence at the relevant public surface.

Create or repair a local skill such as `project-rules/skills/verify-<project>/SKILL.md`, adapting to the configured rules store or an existing registered `skill_paths` location. Give it a distinct frontmatter name and an applicability description; local names cannot shadow bundled skills. The initialized project's `aep --skill` and `aep --skill <name>` should discover it. Keep references as directly contained Markdown files with simple names for `--ref`; link existing deeper journey files directly when needed.

The procedure should teach the parts another agent cannot infer reliably:

- Preflight and startup: exact cwd, real commands, required env names, readiness observation, test identity/data.
- Operation: public CLI invocation, API calls, browser/desktop driver, or consumer harness appropriate to this project.
- Observation: output, state, persistence, permissions or UI feedback that establishes the expected result.
- Isolation and cleanup: owned worktree/data/process handles, reset method, and retained evidence locations.
- Feature Map: where a user feature is reached, how it is operated, expected observable result, and its canonical acceptance/journey reference.

The map indexes current operation; acceptance stays in its existing canonical source. Reuse hand-written journeys instead of generating parallel requirements. A map entry can be a short row, for example “save profile → settings URL → edit and save using browser driver → reload preserves value → profile acceptance.” Include the real fixture identity and observation method in the linked procedure.

Configure repeatable scripted probes in `.aep/config.toml` using the current `aep config` contract. Interactive observations can remain durable notes/artifacts with revision and environment; describe that evidence boundary accurately. AEP executes configured checks, rather than providing a universal app driver.

Execute at least one representative path from the written procedure. Verify the expected effect, capture the commands/results, then exercise cleanup and confirm retained evidence remains readable. If access or a driver is unavailable, name the missing capability and untested path. A generated procedure is a draft until a real run supports it.

After changed behavior, rerun affected map entries. Correct stale navigation/tool instructions; a regression against acceptance is a defect to repair, not a reason to rewrite expected results.
