#!/usr/bin/env bash
# /aep-scaffold existing-project converge — mechanical, idempotent fixes.
#
# Applies only the selected mechanical categories:
#   A  skills-layout health check + CLAUDE.md import (verify only — moves nothing)
#   C  workflow entries in .gitignore
#   E  version-pin recommendation (output only; never executes a re-pin)
#
# With no flags, A,C,E are selected for backward compatibility. To honor the
# Phase 2 confirmation, pass one or more `--category A|C|E` flags. Model-driven
# B/C work remains in the calling skill. Category A never rewrites an install:
# plain per-agent installs and the legacy symlink layout are both healthy
# shapes, and version skew (divergent copies of one skill) fails closed toward
# the category E re-pin. Hand-authored content is never overwritten.

set -euo pipefail

SCRIPT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
# shellcheck source=layout-lib.sh
. "$SCRIPT_DIR/layout-lib.sh"

usage() {
  cat <<'EOF'
Usage: converge.sh [--category A|C|E]...
       converge.sh --categories A,C,E

No flags selects A,C,E. Repeat --category to apply only user-confirmed
mechanical categories.
EOF
}

apply_a=0
apply_c=0
apply_e=0
selected=0

select_category() {
  case "$1" in
    A) apply_a=1 ;;
    C) apply_c=1 ;;
    E) apply_e=1 ;;
    *) echo "ERROR: unsupported mechanical category '$1' (expected A, C, or E)" >&2; exit 2 ;;
  esac
  selected=1
}

while [ "$#" -gt 0 ]; do
  case "$1" in
    --category)
      [ "$#" -ge 2 ] || { echo "ERROR: --category requires A, C, or E" >&2; exit 2; }
      select_category "$2"
      shift 2
      ;;
    --categories)
      [ "$#" -ge 2 ] || { echo "ERROR: --categories requires a comma-separated value" >&2; exit 2; }
      category_list=$2
      case "$category_list" in
        ''|,*|*,|*,,*) echo "ERROR: invalid --categories value '$category_list'" >&2; exit 2 ;;
      esac
      shift 2
      while [ -n "$category_list" ]; do
        category=${category_list%%,*}
        if [ "$category" = "$category_list" ]; then
          category_list=''
        else
          category_list=${category_list#*,}
        fi
        select_category "$category"
      done
      ;;
    -h|--help) usage; exit 0 ;;
    *) echo "ERROR: unknown argument '$1'" >&2; usage >&2; exit 2 ;;
  esac
done

if [ "$selected" -eq 0 ]; then
  apply_a=1
  apply_c=1
  apply_e=1
fi

# Validate the layout before any write so a partial run cannot change shape.
# Category A never rewrites an install: aliased parents, foreign or broken
# links, and version skew all fail closed with a human-facing pointer instead
# of a mechanical fix.
if [ "$apply_a" -eq 1 ]; then
  if ! aep_parent_dirs_safe; then
    echo "ERROR: .agents, .claude, and their skills directories must be real directories when present; inspect symlinks or invalid entries manually." >&2
    exit 1
  fi
  for codex_skill in .agents/skills/aep-*; do
    [ -e "$codex_skill" ] || [ -L "$codex_skill" ] || continue
    if [ -L "$codex_skill" ] || [ ! -d "$codex_skill" ]; then
      echo "ERROR: $codex_skill is not a real skill directory; inspect it manually." >&2
      exit 1
    fi
  done
  for claude_skill in .claude/skills/aep-*; do
    [ -e "$claude_skill" ] || [ -L "$claude_skill" ] || continue
    skill_name=${claude_skill##*/}
    codex_skill=".agents/skills/$skill_name"
    if [ -L "$claude_skill" ]; then
      if [ "$(readlink "$claude_skill")" != "../../.agents/skills/$skill_name" ] \
        || [ ! -d "$codex_skill" ] || [ -L "$codex_skill" ]; then
        echo "ERROR: $claude_skill is a link but not a healthy legacy one; inspect it manually." >&2
        exit 1
      fi
    elif [ -d "$claude_skill" ]; then
      if [ -d "$codex_skill" ] && ! aep_trees_identical "$claude_skill" "$codex_skill"; then
        echo "ERROR: $claude_skill and $codex_skill diverge (version skew); re-pin both agents (category E) instead of editing either." >&2
        exit 1
      fi
    else
      echo "ERROR: $claude_skill is not a skill directory; inspect it manually." >&2
      exit 1
    fi
  done
  if { [ -e CLAUDE.md ] || [ -L CLAUDE.md ]; } && [ ! -f CLAUDE.md ]; then
    echo "ERROR: CLAUDE.md exists but is not a readable file; inspect it manually." >&2
    exit 1
  fi
fi

if [ "$apply_c" -eq 1 ] && { [ -L .gitignore ] || { [ -e .gitignore ] && [ ! -f .gitignore ]; }; }; then
  echo "ERROR: .gitignore exists but is not a regular file; refusing to write." >&2
  exit 1
fi

if [ "$apply_a" -eq 1 ]; then
  echo "=== A. Skills layout (verify only) + CLAUDE.md import ==="
  echo "  plain per-agent installs and the legacy symlink layout both pass; nothing is moved, deleted, or linked"

  # Create the import only when absent AND a Claude skill install exists (at
  # least one entry in .claude/skills — an empty directory is scaffolding). A
  # Codex-only repo is owed no CLAUDE.md. Never clobber a file, symlink, or
  # directory that a user already owns.
  if aep_claude_install_present && [ -f AGENTS.md ] && [ ! -e CLAUDE.md ] && [ ! -L CLAUDE.md ]; then
    printf '@AGENTS.md\n' > CLAUDE.md
  fi
  if [ -e CLAUDE.md ] && [ ! -f CLAUDE.md ]; then
    echo "ERROR: CLAUDE.md exists but is not a regular readable file; inspect it manually." >&2
    exit 1
  elif [ -f CLAUDE.md ] && [ "$(head -1 CLAUDE.md | tr -d '[:space:]')" != "@AGENTS.md" ]; then
    echo "NOTE: CLAUDE.md has hand-authored content — merge it into AGENTS.md by hand, then set CLAUDE.md to '@AGENTS.md'."
  fi
fi

if [ "$apply_c" -eq 1 ]; then
  echo "=== C. Gitignore (workflow dirs) ==="
  touch .gitignore
  if ! grep -qxF '.dev-workflow/' .gitignore; then
    printf '\n# Agentic development workflow\n.dev-workflow/\n' >> .gitignore
  fi
  if ! grep -qxF '.feature-workspaces/' .gitignore; then
    printf '.feature-workspaces/\n' >> .gitignore
  fi
fi

if [ "$apply_e" -eq 1 ]; then
  echo "=== E. Version pin (recommend-only — never auto-run) ==="
  echo "  npx skills add memorysaver/agentic-engineering-patterns@<newtag> -a claude-code --skill '*' -y"
  echo "  npx skills add memorysaver/agentic-engineering-patterns@<newtag> -a codex        --skill '*' -y"
  echo "  (run the line for each agent this repo installs), then re-run scripts/audit.sh, bump the AGENTS.md pin note, commit --no-verify"
fi
