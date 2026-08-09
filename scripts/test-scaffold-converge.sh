#!/usr/bin/env bash
# Regression fixtures for /aep-scaffold's mechanical converge script.

set -euo pipefail

REPO_ROOT=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
CONVERGE="$REPO_ROOT/skills/project-setup/scaffold/scripts/converge.sh"
TMP_ROOT=$(mktemp -d "${TMPDIR:-/tmp}/aep-converge.XXXXXX")
trap 'rm -rf "$TMP_ROOT"' EXIT

pass_count=0

pass() {
  pass_count=$((pass_count + 1))
  echo "PASS: $1"
}

fail() {
  echo "FAIL: $1" >&2
  exit 1
}

assert_file_text() {
  expected=$1
  file=$2
  [ -f "$file" ] || fail "missing file $file"
  actual=$(cat "$file")
  [ "$actual" = "$expected" ] || fail "$file content changed"
}

# A Claude-only plain install is complete: nothing is moved or linked, the
# CLAUDE.md import appears, and a rerun is a no-op.
case_dir="$TMP_ROOT/claude-only"
mkdir -p "$case_dir/.claude/skills/aep-demo"
printf 'claude-only\n' > "$case_dir/.claude/skills/aep-demo/SKILL.md"
printf '# Agent guide\n' > "$case_dir/AGENTS.md"
(
  cd "$case_dir"
  bash "$CONVERGE" --categories A,C >/dev/null
  bash "$CONVERGE" --categories A,C >/dev/null
)
[ -d "$case_dir/.claude/skills/aep-demo" ] && [ ! -L "$case_dir/.claude/skills/aep-demo" ] || fail "Claude-only plain install was rewritten"
assert_file_text "claude-only" "$case_dir/.claude/skills/aep-demo/SKILL.md"
[ ! -e "$case_dir/.agents/skills/aep-demo" ] || fail "Claude-only skill was moved or copied to .agents"
assert_file_text "@AGENTS.md" "$case_dir/CLAUDE.md"
[ "$(grep -xcF '.dev-workflow/' "$case_dir/.gitignore")" -eq 1 ] || fail ".dev-workflow entry is not idempotent"
[ "$(grep -xcF '.feature-workspaces/' "$case_dir/.gitignore")" -eq 1 ] || fail ".feature-workspaces entry is not idempotent"
pass "Claude-only plain install left intact"

# A Codex-only plain install is complete: no Claude link is created and no
# CLAUDE.md is owed.
case_dir="$TMP_ROOT/codex-only"
mkdir -p "$case_dir/.agents/skills/aep-demo"
printf 'codex-only\n' > "$case_dir/.agents/skills/aep-demo/SKILL.md"
printf '# Agent guide\n' > "$case_dir/AGENTS.md"
(cd "$case_dir" && bash "$CONVERGE" --category A >/dev/null)
assert_file_text "codex-only" "$case_dir/.agents/skills/aep-demo/SKILL.md"
[ ! -e "$case_dir/.claude/skills/aep-demo" ] && [ ! -L "$case_dir/.claude/skills/aep-demo" ] || fail "Codex-only skill grew a Claude link"
[ ! -e "$case_dir/CLAUDE.md" ] || fail "CLAUDE.md was created without a Claude skill install"
pass "Codex-only plain install left intact"

# Byte-and-mode-identical dual plain copies are a healthy dual install:
# verified, never collapsed.
case_dir="$TMP_ROOT/identical"
mkdir -p "$case_dir/.agents/skills/aep-demo" "$case_dir/.claude/skills/aep-demo"
printf 'same\n' > "$case_dir/.agents/skills/aep-demo/SKILL.md"
printf 'same\n' > "$case_dir/.claude/skills/aep-demo/SKILL.md"
(cd "$case_dir" && bash "$CONVERGE" --category A >/dev/null)
[ -d "$case_dir/.claude/skills/aep-demo" ] && [ ! -L "$case_dir/.claude/skills/aep-demo" ] || fail "identical dual install was collapsed"
assert_file_text "same" "$case_dir/.agents/skills/aep-demo/SKILL.md"
assert_file_text "same" "$case_dir/.claude/skills/aep-demo/SKILL.md"
pass "identical dual plain install left intact"

# GNU stat treats BSD's `-f FORMAT` as filesystem mode and may emit output for
# the valid path before returning non-zero for FORMAT. The mode probe must
# capture one dialect at a time so a failed-probe stdout cannot turn a healthy
# identical pair into a false skew.
case_dir="$TMP_ROOT/gnu-stat-probe"
mkdir -p "$case_dir/.agents/skills/aep-demo" "$case_dir/.claude/skills/aep-demo" "$case_dir/fake-bin"
printf 'same\n' > "$case_dir/.agents/skills/aep-demo/SKILL.md"
printf 'same\n' > "$case_dir/.claude/skills/aep-demo/SKILL.md"
printf '%s\n' \
  '#!/usr/bin/env bash' \
  'if [ "$1" = "-c" ]; then' \
  '  [ -d "$3" ] && printf "755\n" || printf "644\n"' \
  '  exit 0' \
  'fi' \
  'if [ "$1" = "-f" ]; then' \
  '  printf "GNU filesystem status for %s\n" "$3"' \
  '  exit 1' \
  'fi' \
  'exit 2' > "$case_dir/fake-bin/stat"
chmod +x "$case_dir/fake-bin/stat"
(cd "$case_dir" && PATH="$case_dir/fake-bin:$PATH" bash "$CONVERGE" --category A >/dev/null)
[ -d "$case_dir/.claude/skills/aep-demo" ] && [ ! -L "$case_dir/.claude/skills/aep-demo" ] || fail "GNU-stat probe rewrote a healthy identical pair"
pass "GNU/BSD stat probe isolation"

# Matching bytes with different executable modes are version skew: converge
# must fail closed and change neither side.
case_dir="$TMP_ROOT/mode-divergent"
mkdir -p "$case_dir/.agents/skills/aep-demo/scripts" "$case_dir/.claude/skills/aep-demo/scripts"
printf '#!/usr/bin/env bash\n' > "$case_dir/.agents/skills/aep-demo/scripts/run.sh"
printf '#!/usr/bin/env bash\n' > "$case_dir/.claude/skills/aep-demo/scripts/run.sh"
chmod 0644 "$case_dir/.agents/skills/aep-demo/scripts/run.sh"
chmod 0755 "$case_dir/.claude/skills/aep-demo/scripts/run.sh"
if (cd "$case_dir" && bash "$CONVERGE" --category A >/dev/null 2>&1); then
  fail "mode-divergent duplicate returned success"
fi
[ ! -L "$case_dir/.claude/skills/aep-demo" ] || fail "mode-divergent copy was collapsed"
[ -x "$case_dir/.claude/skills/aep-demo/scripts/run.sh" ] || fail "Claude executable mode was lost"
[ ! -x "$case_dir/.agents/skills/aep-demo/scripts/run.sh" ] || fail "Codex mode was changed"
pass "mode-divergent duplicate preservation"

# Divergent copies are version skew: fail closed and preserve both.
case_dir="$TMP_ROOT/divergent"
mkdir -p "$case_dir/.agents/skills/aep-a-same" "$case_dir/.claude/skills/aep-a-same"
mkdir -p "$case_dir/.agents/skills/aep-z-demo" "$case_dir/.claude/skills/aep-z-demo"
printf 'same\n' > "$case_dir/.agents/skills/aep-a-same/SKILL.md"
printf 'same\n' > "$case_dir/.claude/skills/aep-a-same/SKILL.md"
printf 'codex-copy\n' > "$case_dir/.agents/skills/aep-z-demo/SKILL.md"
printf 'claude-copy\n' > "$case_dir/.claude/skills/aep-z-demo/SKILL.md"
if (cd "$case_dir" && bash "$CONVERGE" --category A >/dev/null 2>&1); then
  fail "divergent duplicate returned success"
fi
assert_file_text "codex-copy" "$case_dir/.agents/skills/aep-z-demo/SKILL.md"
assert_file_text "claude-copy" "$case_dir/.claude/skills/aep-z-demo/SKILL.md"
[ ! -L "$case_dir/.claude/skills/aep-z-demo" ] || fail "divergent Claude copy was replaced"
[ ! -L "$case_dir/.claude/skills/aep-a-same" ] || fail "an identical sibling was rewritten"
assert_file_text "same" "$case_dir/.claude/skills/aep-a-same/SKILL.md"
pass "divergent duplicate preservation"

# A whole-directory symlink aliases another runtime's files and must fail
# closed, never be traversed or modified.
case_dir="$TMP_ROOT/aliased-skills-dir"
mkdir -p "$case_dir/.agents/skills/aep-demo" "$case_dir/.claude"
printf 'aliased\n' > "$case_dir/.agents/skills/aep-demo/SKILL.md"
ln -s '../.agents/skills' "$case_dir/.claude/skills"
if (cd "$case_dir" && bash "$CONVERGE" --category A >/dev/null 2>&1); then
  fail "whole-directory skills symlink returned success"
fi
assert_file_text "aliased" "$case_dir/.agents/skills/aep-demo/SKILL.md"
[ -L "$case_dir/.claude/skills" ] || fail "whole-directory skills symlink was modified"
pass "whole-directory alias fail-closed"

# A symlinked parent could make apparently relative paths point outside the
# project: fail closed, touch nothing.
case_dir="$TMP_ROOT/symlinked-parent"
external_dir="$TMP_ROOT/external-agents"
mkdir -p "$case_dir/.claude/skills/aep-demo" "$external_dir/skills"
printf 'outside-must-stay-empty\n' > "$external_dir/sentinel"
ln -s "$external_dir" "$case_dir/.agents"
printf 'claude-only\n' > "$case_dir/.claude/skills/aep-demo/SKILL.md"
if (cd "$case_dir" && bash "$CONVERGE" --category A >/dev/null 2>&1); then
  fail "symlinked .agents parent returned success"
fi
[ ! -e "$external_dir/skills/aep-demo" ] || fail "skill escaped through symlinked .agents parent"
assert_file_text "outside-must-stay-empty" "$external_dir/sentinel"
pass "symlinked parent escape prevention"

# A foreign per-skill link (anything but the exact legacy target) fails closed
# and is left exactly as found.
case_dir="$TMP_ROOT/foreign-link"
mkdir -p "$case_dir/.claude/skills" "$case_dir/elsewhere"
ln -s '../../elsewhere' "$case_dir/.claude/skills/aep-demo"
if (cd "$case_dir" && bash "$CONVERGE" --category A >/dev/null 2>&1); then
  fail "foreign skill link returned success"
fi
[ -L "$case_dir/.claude/skills/aep-demo" ] || fail "foreign link was removed"
[ "$(readlink "$case_dir/.claude/skills/aep-demo")" = '../../elsewhere' ] || fail "foreign link target was rewritten"
pass "foreign link fail-closed"

# Write failures must propagate instead of reporting a successful converge.
case_dir="$TMP_ROOT/write-failure"
mkdir -p "$case_dir/.gitignore"
if (cd "$case_dir" && bash "$CONVERGE" --category C >/dev/null 2>&1); then
  fail "unwritable .gitignore shape returned success"
fi
[ -d "$case_dir/.gitignore" ] || fail "invalid .gitignore fixture was modified"
pass "write failure propagation"

# No flags retains the documented all-mechanical-categories behavior — and
# category A creates nothing on an empty project.
case_dir="$TMP_ROOT/default-categories"
mkdir -p "$case_dir"
(cd "$case_dir" && bash "$CONVERGE" >/dev/null)
[ ! -e "$case_dir/.agents" ] || fail "default run created .agents"
[ ! -e "$case_dir/CLAUDE.md" ] || fail "default run created CLAUDE.md without a Claude skill install"
grep -qxF '.dev-workflow/' "$case_dir/.gitignore" || fail "default run did not apply category C"
pass "default category compatibility"

# Category filtering must not apply unconfirmed A/C changes.
case_dir="$TMP_ROOT/category-filter"
mkdir -p "$case_dir/.claude/skills/aep-demo"
printf 'untouched\n' > "$case_dir/.claude/skills/aep-demo/SKILL.md"
(cd "$case_dir" && bash "$CONVERGE" --category E >/dev/null)
[ ! -e "$case_dir/.agents" ] || fail "category E unexpectedly applied category A"
[ ! -e "$case_dir/.gitignore" ] || fail "category E unexpectedly applied category C"
assert_file_text "untouched" "$case_dir/.claude/skills/aep-demo/SKILL.md"
pass "category filtering"

echo "scaffold converge fixtures: $pass_count passed"
