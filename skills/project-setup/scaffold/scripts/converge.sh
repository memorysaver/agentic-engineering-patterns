#!/usr/bin/env bash
# /aep-scaffold existing-project converge — mechanical, idempotent fixes.
#
# Applies only the selected mechanical categories:
#   A  canonical aep-* skills layout + CLAUDE.md import
#   C  workflow entries in .gitignore
#   E  version-pin recommendation (output only; never executes a re-pin)
#
# With no flags, A,C,E are selected for backward compatibility. To honor the
# Phase 2 confirmation, pass one or more `--category A|C|E` flags. Model-driven
# B/C work remains in the calling skill. Hand-authored content is never
# overwritten; ambiguous duplicate skill directories fail closed.

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

# Validate every ambiguous shape before the first write so a later conflict
# cannot leave an earlier skill normalized in an otherwise failed run.
if [ "$apply_a" -eq 1 ]; then
  if ! aep_parent_dirs_safe; then
    echo "ERROR: .agents, .claude, and their skills directories must be real directories; inspect symlinks or invalid entries manually." >&2
    exit 1
  fi
  for claude_skill in .claude/skills/aep-*; do
    [ -e "$claude_skill" ] || [ -L "$claude_skill" ] || continue
    skill_name=${claude_skill##*/}
    canonical_skill=".agents/skills/$skill_name"
    expected_target="../../.agents/skills/$skill_name"
    if [ -L "$claude_skill" ]; then
      if [ "$(readlink "$claude_skill")" != "$expected_target" ] || [ ! -d "$canonical_skill" ] || [ -L "$canonical_skill" ]; then
        echo "ERROR: $claude_skill is not a healthy canonical link; inspect it manually." >&2
        exit 1
      fi
    elif [ ! -d "$claude_skill" ]; then
      echo "ERROR: $claude_skill is not a skill directory; inspect it manually." >&2
      exit 1
    elif [ -L "$canonical_skill" ]; then
      echo "ERROR: canonical $canonical_skill is itself a symlink; inspect it manually." >&2
      exit 1
    elif [ -e "$canonical_skill" ] && { [ ! -d "$canonical_skill" ] || ! aep_trees_identical "$claude_skill" "$canonical_skill"; }; then
      echo "ERROR: divergent or invalid copies at $claude_skill and $canonical_skill; refusing to change either." >&2
      exit 1
    fi
  done
  for canonical_skill in .agents/skills/aep-*; do
    [ -e "$canonical_skill" ] || [ -L "$canonical_skill" ] || continue
    if [ -L "$canonical_skill" ] || [ ! -d "$canonical_skill" ]; then
      echo "ERROR: canonical $canonical_skill is not a real skill directory; inspect it manually." >&2
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
  echo "=== A. Canonical skills layout ==="
  mkdir -p .agents/skills .claude/skills

  # Promote every Claude-only real aep-* directory to the canonical Codex
  # location, then link Claude to it. If both sides contain the same skill,
  # collapse only content-and-mode-identical copies; divergent copies require
  # a human so executable bits and other meaningful modes are never lost.
  for claude_skill in .claude/skills/aep-*; do
    [ -e "$claude_skill" ] || [ -L "$claude_skill" ] || continue
    skill_name=${claude_skill##*/}
    canonical_skill=".agents/skills/$skill_name"
    expected_target="../../.agents/skills/$skill_name"

    if [ -L "$claude_skill" ]; then
      actual_target=$(readlink "$claude_skill")
      if [ "$actual_target" != "$expected_target" ] || [ ! -e "$canonical_skill" ]; then
        echo "ERROR: $claude_skill is not a healthy canonical link; inspect it manually." >&2
        exit 1
      fi
      continue
    fi
    if [ ! -d "$claude_skill" ]; then
      echo "ERROR: $claude_skill is not a skill directory; inspect it manually." >&2
      exit 1
    fi

    if [ -L "$canonical_skill" ]; then
      echo "ERROR: canonical $canonical_skill is itself a symlink; inspect it manually." >&2
      exit 1
    elif [ -e "$canonical_skill" ]; then
      if ! aep_trees_identical "$claude_skill" "$canonical_skill"; then
        echo "ERROR: divergent copies at $claude_skill and $canonical_skill; refusing to delete either." >&2
        exit 1
      fi
      rm -rf -- "$claude_skill"
    else
      mv -- "$claude_skill" "$canonical_skill"
    fi
    ln -s "$expected_target" "$claude_skill"
  done

  # Expose Codex-only canonical skills to Claude.
  for canonical_skill in .agents/skills/aep-*; do
    [ -e "$canonical_skill" ] || [ -L "$canonical_skill" ] || continue
    skill_name=${canonical_skill##*/}
    claude_skill=".claude/skills/$skill_name"
    expected_target="../../.agents/skills/$skill_name"

    if [ -L "$canonical_skill" ]; then
      echo "ERROR: canonical $canonical_skill is a symlink; inspect it manually." >&2
      exit 1
    elif [ ! -d "$canonical_skill" ]; then
      echo "ERROR: canonical $canonical_skill is not a skill directory; inspect it manually." >&2
      exit 1
    fi
    if [ ! -e "$claude_skill" ] && [ ! -L "$claude_skill" ]; then
      ln -s "$expected_target" "$claude_skill"
    elif [ ! -L "$claude_skill" ]; then
      echo "ERROR: unexpected real directory at $claude_skill after normalization." >&2
      exit 1
    fi
  done

  # Create the import only when absent. Never clobber a file, symlink, or
  # directory that a user already owns.
  if [ -f AGENTS.md ] && [ ! -e CLAUDE.md ] && [ ! -L CLAUDE.md ]; then
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
  echo "  then normalize .claude/skills/aep-* symlinks (category A), bump the AGENTS.md pin note, commit --no-verify"
fi
