---
name: openspec-setup
description: Initialize OpenSpec for spec-driven development. Use after scaffolding a new project, or when the user says "setup openspec", "init specs", "spec-driven", or wants to add OpenSpec to an existing project.
---

# OpenSpec Setup

Initialize [OpenSpec](https://openspec.dev) in your project for artifact-driven, spec-first development. This skill runs `openspec init`, configures the project context, and sets up command aliases.

---

## Prerequisites

Verify `openspec` is installed:

```bash
which openspec >/dev/null 2>&1 && echo "OK" || echo "MISSING — install with: bun add -g openspec"
```

If missing, install it:

```bash
bun add -g openspec
```

---

## Step 1: Initialize OpenSpec

Run the init command with multi-tool support:

```bash
openspec init --tools claude,opencode,pi,codex
```

This creates the OpenSpec directory structure and generates tool-specific skills/instructions for each configured tool:

| Path | Purpose |
|------|---------|
| `openspec/` | Root OpenSpec directory |
| `openspec/config.yaml` | Project configuration + context |
| `openspec/specs/` | Specification documents (source of truth) |
| `openspec/changes/` | Change proposals and artifacts |
| `.claude/skills/openspec-*/SKILL.md` | Claude Code skills (explore, propose, apply, archive) |
| `.opencode/` | OpenCode tool instructions |
| `.pi/` | Pi tool instructions |
| `.codex/` | Codex tool instructions |

> The `--tools` flag accepts a comma-separated list. Supported tools include: `claude`, `opencode`, `pi`, `codex`, `cursor`, `windsurf`, `cline`, `kiro`, `gemini`, `github-copilot`, `roocode`, `trae`, and [many more](https://github.com/Fission-AI/OpenSpec/blob/main/docs/cli.md). Use `--tools all` to configure every supported tool.

---

## Step 2: Configure Project Context

Update `openspec/config.yaml` with the project's tech stack. Read the project's `package.json` and `bts.jsonc` to determine the stack, then update the config:

```yaml
schema: spec-driven

context: |
  Tech stack: TypeScript, <frontend>, <backend>, <database>/<orm>
  Monorepo: Turborepo + <package-manager>
  Auth: <auth-provider>
  API: <api-layer>
  Conventions: conventional commits, trunk-based development
```

---

## Step 3: Set Up Command Aliases

Create OpenSpec command aliases in `.claude/commands/opsx/` for quick access:

### `.claude/commands/opsx/explore.md`

```markdown
---
name: "OPSX: Explore"
description: "Enter explore mode — think through ideas, investigate, clarify requirements"
category: Workflow
tags: [workflow, explore, thinking]
---

Enter explore mode for thinking and investigation.

**IMPORTANT:** Explore mode is for thinking, not implementing. Read files and search code freely, but never write code. You MAY create OpenSpec artifacts if asked — that's capturing thinking, not implementing.

Invoke the openspec-explore skill to begin.
```

### `.claude/commands/opsx/propose.md`

```markdown
---
name: "OPSX: Propose"
description: "Create a new change proposal with all artifacts"
category: Workflow
tags: [workflow, propose, change]
---

Create a new OpenSpec change proposal. This generates:
- proposal.md — what and why
- design.md — how, key decisions, risks
- specs/**/*.md — detailed requirements
- tasks.md — implementation checklist

Invoke the openspec-propose skill to begin.
```

### `.claude/commands/opsx/apply.md`

```markdown
---
name: "OPSX: Apply"
description: "Implement tasks from an existing change proposal"
category: Workflow
tags: [workflow, apply, implement]
---

Implement tasks from an existing OpenSpec change. Reads the change artifacts and works through each task.

Invoke the openspec-apply-change skill to begin.
```

### `.claude/commands/opsx/archive.md`

```markdown
---
name: "OPSX: Archive"
description: "Archive a completed change after merge"
category: Workflow
tags: [workflow, archive, cleanup]
---

Archive a completed change after its PR/MR has been merged. Run this on the main branch only.

Invoke the openspec-archive-change skill to begin.
```

---

## Step 4: Verify Setup

```bash
# Check OpenSpec is initialized
openspec list

# Check skills were created
for skill in openspec-explore openspec-propose openspec-apply-change openspec-archive-change; do
  printf "%-35s" "$skill:"
  [ -f ".claude/skills/$skill/SKILL.md" ] && echo "OK" || echo "MISSING"
done

# Check commands were created
for cmd in explore propose apply archive; do
  printf "%-15s" "/opsx:$cmd:"
  [ -f ".claude/commands/opsx/$cmd.md" ] && echo "OK" || echo "MISSING"
done
```

---

## Usage After Setup

| Command | Purpose |
|---------|---------|
| `/opsx:explore` | Think through a feature or problem |
| `/opsx:propose` | Create a formal change proposal |
| `/opsx:apply` | Implement tasks from a proposal |
| `/opsx:archive` | Archive after merge |
| `openspec list` | List active changes |
| `openspec status` | Show artifact completion status |
| `openspec view` | Interactive dashboard |

---

## Guardrails

- **Always run in the project root** — `openspec init` needs to find or create `openspec/`
- **Never overwrite existing OpenSpec config** — check if `openspec/config.yaml` exists first
- **Commit OpenSpec artifacts to git** — they are part of the project record
- **Archive only on main branch** — never archive from a feature branch
