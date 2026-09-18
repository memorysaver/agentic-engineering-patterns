# Human-readable native CLI

Accepted 2026-09-10 by the project owner for the v5 preview.

## Decision

Use clap for argument parsing, command definitions, usage errors and nested help. Keep the product a textual CLI; a TUI is outside this change. The existing runtime already uses clap, so this is an interface redesign rather than a framework migration.

The front page groups commands by purpose and includes practical examples. Ordinary commands return readable summaries with explicit outcomes, readiness blockers and omission notices. `--json` retains complete structured data and stable exit semantics. Failed verification may save evidence, but its visible result must still say FAIL and expose the check failure.

The only guidance entrypoint is `aep --skill`. It emits a complete Markdown skill with YAML frontmatter and a discovered procedure list. `aep --skill NAME` emits the selected SKILL.md verbatim; `aep --skill NAME --ref RESOURCE` emits its resource. Metadata stays in JSON responses. Remove `aep skills` and do not add `aep --skills` aliases.

This supersedes earlier native design examples using `skills`/`skills show`. Update active source and downstream instructions together with the installed preview. Retain historical audit commands, imported source files and legacy v4 skills as provenance. The preview version remains 5.0.0-preview.1; local builds are distinguished by source commit and binary checksum. Publishing a tagged release is a separate action.

## Context and verification

The root skill explains context completeness, full-data inspection, revision-bound verification and explicit project review policy. It leaves procedure selection and work order to the agent. Its new steering baseline contains zero negation lines and zero hard imperatives; existing ceilings remain unchanged.

Verification covers help and argument safety, exact Markdown output, project-owned procedure discovery, bounded human summaries versus complete JSON, failed checks and reference validator diagnostics, ordinary writes and dry runs. The copied release binary also exercises a disposable migration and reads the three real downstream projects. These checks do not certify downstream product behavior or external provider operations.
