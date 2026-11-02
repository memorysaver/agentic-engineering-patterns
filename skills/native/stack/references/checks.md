# Starter checks

These are the configured checks for a project on the default stack, in execution order: dependencies first, then AEP's own record check, then format, lint and type checks, tests and the build. `aep verify run` executes them in the order they appear in the configuration. The AEP check calls `aep` from PATH, never an absolute path to a particular build.

To apply them (step 7 of `aep --skill stack`):

1. `aep config show --json` and note `revision`.
2. Copy `.aep/config.toml` to a scratch file. In the copy, delete the line `checks = []`, set `required_checks` under `[policy]` to the five IDs below, and append the block below at the end.
3. `aep config update --file <copy> --expect <revision>`, then `aep config show` to confirm five checks in this order.

```toml
[[checks]]
id = "install"
command = ["bun", "install", "--frozen-lockfile"]
environment = "local"
rules = []
cwd = "."
timeout_seconds = 600
paths = []
env = []
required = true

[[checks]]
id = "aep-records"
command = ["aep", "check"]
environment = "local"
rules = []
cwd = "."
timeout_seconds = 120
paths = []
env = []
required = true

[[checks]]
id = "check"
command = ["bun", "run", "check"]
environment = "local"
rules = []
cwd = "."
timeout_seconds = 900
paths = []
env = []
required = true

[[checks]]
id = "test"
command = ["bun", "run", "test"]
environment = "local"
rules = []
cwd = "."
timeout_seconds = 600
paths = []
env = []
required = true

[[checks]]
id = "build"
command = ["bun", "run", "build"]
environment = "local"
rules = []
cwd = "."
timeout_seconds = 900
paths = []
env = []
required = true
```

```toml
required_checks = ["install", "aep-records", "check", "test", "build"]
```

`.github/workflows/ci.yml` from the files reference runs `install`, `check`, `test` and `build` with the same commands and states why `aep-records` is not run there. When a check is added, changed or removed, change the workflow in the same story; nothing compares the two automatically.

A story that changes one app can add a scoped check with `vp run --filter ./apps/<app> <task>`. Leave the starter checks required: a story's `required_checks` may name more checks, not fewer.
