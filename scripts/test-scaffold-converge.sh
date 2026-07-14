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

# Claude-only installs must be promoted, linked, and remain stable on rerun.
case_dir="$TMP_ROOT/claude-only"
mkdir -p "$case_dir/.claude/skills/aep-demo"
printf 'claude-only\n' > "$case_dir/.claude/skills/aep-demo/SKILL.md"
printf '# Agent guide\n' > "$case_dir/AGENTS.md"
(
  cd "$case_dir"
  bash "$CONVERGE" --categories A,C >/dev/null
  bash "$CONVERGE" --categories A,C >/dev/null
)
assert_file_text "claude-only" "$case_dir/.agents/skills/aep-demo/SKILL.md"
[ -L "$case_dir/.claude/skills/aep-demo" ] || fail "Claude-only skill was not linked"
[ "$(readlink "$case_dir/.claude/skills/aep-demo")" = '../../.agents/skills/aep-demo' ] || fail "wrong Claude link target"
assert_file_text "@AGENTS.md" "$case_dir/CLAUDE.md"
[ "$(grep -xcF '.dev-workflow/' "$case_dir/.gitignore")" -eq 1 ] || fail ".dev-workflow entry is not idempotent"
[ "$(grep -xcF '.feature-workspaces/' "$case_dir/.gitignore")" -eq 1 ] || fail ".feature-workspaces entry is not idempotent"
pass "Claude-only promotion and rerun"

# Codex-only installs must gain the corresponding Claude link.
case_dir="$TMP_ROOT/codex-only"
mkdir -p "$case_dir/.agents/skills/aep-demo"
printf 'codex-only\n' > "$case_dir/.agents/skills/aep-demo/SKILL.md"
(cd "$case_dir" && bash "$CONVERGE" --category A >/dev/null)
assert_file_text "codex-only" "$case_dir/.agents/skills/aep-demo/SKILL.md"
[ -L "$case_dir/.claude/skills/aep-demo" ] || fail "Codex-only skill was not exposed to Claude"
pass "Codex-only exposure"

# Byte-identical duplicate real directories may be collapsed safely.
case_dir="$TMP_ROOT/identical"
mkdir -p "$case_dir/.agents/skills/aep-demo" "$case_dir/.claude/skills/aep-demo"
printf 'same\n' > "$case_dir/.agents/skills/aep-demo/SKILL.md"
printf 'same\n' > "$case_dir/.claude/skills/aep-demo/SKILL.md"
(cd "$case_dir" && bash "$CONVERGE" --category A >/dev/null)
[ -L "$case_dir/.claude/skills/aep-demo" ] || fail "identical duplicate was not collapsed"
assert_file_text "same" "$case_dir/.agents/skills/aep-demo/SKILL.md"
pass "identical duplicate collapse"

# Matching bytes with different executable modes are not identical: collapsing
# them would silently discard runtime semantics.
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
[ ! -x "$case_dir/.agents/skills/aep-demo/scripts/run.sh" ] || fail "canonical mode was changed"
pass "mode-divergent duplicate preservation"

# Divergent copies are ambiguous: fail closed and preserve both.
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
[ ! -L "$case_dir/.claude/skills/aep-a-same" ] || fail "preflight changed an earlier identical copy"
assert_file_text "same" "$case_dir/.claude/skills/aep-a-same/SKILL.md"
pass "divergent duplicate preflight preservation"

# A whole-directory symlink aliases canonical files and must never be traversed
# by the per-skill replacement loop.
case_dir="$TMP_ROOT/aliased-skills-dir"
mkdir -p "$case_dir/.agents/skills/aep-demo" "$case_dir/.claude"
printf 'aliased\n' > "$case_dir/.agents/skills/aep-demo/SKILL.md"
ln -s '../.agents/skills' "$case_dir/.claude/skills"
if (cd "$case_dir" && bash "$CONVERGE" --category A >/dev/null 2>&1); then
  fail "whole-directory skills symlink returned success"
fi
assert_file_text "aliased" "$case_dir/.agents/skills/aep-demo/SKILL.md"
[ -L "$case_dir/.claude/skills" ] || fail "whole-directory skills symlink was modified"
pass "whole-directory symlink preservation"

# A symlinked parent can make apparently relative writes escape the project.
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

# Write failures must propagate instead of reporting a successful converge.
case_dir="$TMP_ROOT/write-failure"
mkdir -p "$case_dir/.gitignore"
if (cd "$case_dir" && bash "$CONVERGE" --category C >/dev/null 2>&1); then
  fail "unwritable .gitignore shape returned success"
fi
[ -d "$case_dir/.gitignore" ] || fail "invalid .gitignore fixture was modified"
pass "write failure propagation"

# No flags retains the documented all-mechanical-categories behavior.
case_dir="$TMP_ROOT/default-categories"
mkdir -p "$case_dir"
(cd "$case_dir" && bash "$CONVERGE" >/dev/null)
[ -d "$case_dir/.agents/skills" ] || fail "default run did not apply category A"
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
