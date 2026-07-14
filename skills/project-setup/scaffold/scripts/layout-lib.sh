#!/usr/bin/env bash
# Shared, read-only predicates for scaffold audit/converge category A.

aep_parent_dirs_safe() {
  local path
  for path in .agents .agents/skills .claude .claude/skills; do
    [ ! -L "$path" ] || return 1
    [ ! -e "$path" ] || [ -d "$path" ] || return 1
  done
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

aep_layout_is_canonical() {
  local canonical_skill claude_skill skill_name expected_target
  aep_parent_dirs_safe || return 1
  [ -d .agents/skills ] && [ -d .claude/skills ] || return 1

  for canonical_skill in .agents/skills/aep-*; do
    [ -e "$canonical_skill" ] || [ -L "$canonical_skill" ] || continue
    [ -d "$canonical_skill" ] && [ ! -L "$canonical_skill" ] || return 1
    skill_name=${canonical_skill##*/}
    claude_skill=".claude/skills/$skill_name"
    expected_target="../../.agents/skills/$skill_name"
    [ -L "$claude_skill" ] || return 1
    [ "$(readlink "$claude_skill")" = "$expected_target" ] || return 1
  done

  for claude_skill in .claude/skills/aep-*; do
    [ -e "$claude_skill" ] || [ -L "$claude_skill" ] || continue
    [ -L "$claude_skill" ] || return 1
    skill_name=${claude_skill##*/}
    canonical_skill=".agents/skills/$skill_name"
    expected_target="../../.agents/skills/$skill_name"
    [ "$(readlink "$claude_skill")" = "$expected_target" ] || return 1
    [ -d "$canonical_skill" ] && [ ! -L "$canonical_skill" ] || return 1
  done
}
