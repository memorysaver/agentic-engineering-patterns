---
name: project
description: Install or inspect AEP project entrypoints, scaffold missing project capabilities, and classify project-specific maintenance rules.
---

# Project setup

Inspect the repository, runtime instruction discovery, actual scripts, and current Git changes. Use `aep doctor` for CLI/configuration compatibility and capabilities. In a Git project, `aep init --dry-run` previews the minimal configuration and entrypoints; apply with `aep init`. Add `--claude` only when that host needs a CLAUDE.md pointer.

For an existing project, classify its current AGENTS content yourself. Keep the generic agreement and CLI discovery short; move applicable code, testing, package, DevOps, release, and workflow rules into the indexed `project-rules/` surface. Preserve host settings and unrelated content. Initial setup preserves existing instructions for this review.

Configure real checks in `.aep/config.toml` using command argument arrays, working directories, timeouts, and required environment variable names. Scaffold only the missing capabilities authorized by the task, retaining the project's stack and working commands.

Use the migration skill for an existing product-context/OpenSpec store. Verify AGENTS discovery, applicable rules, `aep skills`, and `aep check`; a second init should produce no diff.

Inspect executable policy with `aep config show --json`. Submit reviewed TOML/YAML/JSON settings with `aep config update --file <file> --expect <revision>`. Keep populated store moves within an explicit migration.
