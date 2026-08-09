#!/usr/bin/env bash
# Shared, read-only predicates for scaffold audit/converge category A.

aep_parent_dirs_safe() {
  # Runtime roots may be absent (single-agent installs are legal), but any
  # that exist must be real directories — whole-directory aliases and file
  # impostors fail closed before any per-skill judgment.
  local path
  for path in .agents .agents/skills .claude .claude/skills; do
    [ ! -L "$path" ] || return 1
    [ ! -e "$path" ] || [ -d "$path" ] || return 1
  done
}

aep_claude_install_present() {
  # A Claude skill install means .claude/skills holds at least one entry — a
  # real directory or a symlink. An empty directory is scaffolding, not an
  # install, and owes no CLAUDE.md import.
  local entry
  for entry in .claude/skills/*; do
    if [ -e "$entry" ] || [ -L "$entry" ]; then
      return 0
    fi
  done
  return 1
}

aep_mode() {
  local mode
  if mode=$(stat -c '%a' "$1" 2>/dev/null); then
    printf '%s\n' "$mode"
  else
    stat -f '%Lp' "$1" 2>/dev/null
  fi
}

aep_trees_identical() {
  local left="$1" right="$2" rel left_mode right_mode
  diff -qr -- "$left" "$right" >/dev/null 2>&1 || return 1
  [ "$(aep_mode "$left")" = "$(aep_mode "$right")" ] || return 1

  while IFS= read -r rel; do
    [ -n "$rel" ] || continue
    left_mode=$(aep_mode "$left/$rel") || return 1
    right_mode=$(aep_mode "$right/$rel") || return 1
    [ "$left_mode" = "$right_mode" ] || return 1
  done < <(cd "$left" && find . -mindepth 1 \( -type f -o -type d \) -print | sort)
}

aep_layout_is_healthy() {
  # v4.0.0 install contract: plain per-agent installs are the norm — real
  # aep-* dirs under .claude/skills (claude-code) and/or .agents/skills
  # (codex), either side alone being a complete install. The retired v3
  # canonical symlink layout (.claude/skills/aep-* → ../../.agents/skills/aep-*)
  # stays legal where it already exists; nothing normalizes toward or away
  # from it. Unhealthy = an aliased parent dir, a foreign or broken link, a
  # non-directory entry, or version skew — both sides real with differing
  # bytes/modes, which the category E re-pin fixes, never a mechanical
  # collapse.
  local claude_skill codex_skill skill_name
  aep_parent_dirs_safe || return 1
  for codex_skill in .agents/skills/aep-*; do
    [ -e "$codex_skill" ] || [ -L "$codex_skill" ] || continue
    [ -d "$codex_skill" ] && [ ! -L "$codex_skill" ] || return 1
  done
  for claude_skill in .claude/skills/aep-*; do
    [ -e "$claude_skill" ] || [ -L "$claude_skill" ] || continue
    skill_name=${claude_skill##*/}
    codex_skill=".agents/skills/$skill_name"
    if [ -L "$claude_skill" ]; then
      [ "$(readlink "$claude_skill")" = "../../.agents/skills/$skill_name" ] || return 1
      [ -d "$codex_skill" ] && [ ! -L "$codex_skill" ] || return 1
    elif [ -d "$claude_skill" ]; then
      if [ -d "$codex_skill" ]; then
        aep_trees_identical "$claude_skill" "$codex_skill" || return 1
      fi
    else
      return 1
    fi
  done
}
