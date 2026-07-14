#!/usr/bin/env bash
# Regression fixtures for scaffold's read-only audit.

set -euo pipefail

REPO_ROOT=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
AUDIT="$REPO_ROOT/skills/project-setup/scaffold/scripts/audit.sh"
TMP_ROOT=$(mktemp -d "${TMPDIR:-/tmp}/aep-audit.XXXXXX")
trap 'rm -rf "$TMP_ROOT"' EXIT

pass_count=0
pass() { pass_count=$((pass_count + 1)); echo "PASS: $1"; }
fail() { echo "FAIL: $1" >&2; exit 1; }

baseline() {
  local root="$1"
  mkdir -p "$root/.agents/skills/aep-scaffold" "$root/.claude/skills"
  mkdir -p "$root/skills/e2e-test" "$root/openspec"
  mkdir -p "$root/.claude/commands/opsx" "$root/.claude/hooks"
  printf '%s\n' 'name: aep-scaffold' > "$root/.agents/skills/aep-scaffold/SKILL.md"
  ln -s '../../.agents/skills/aep-scaffold' "$root/.claude/skills/aep-scaffold"
  printf 'policy\n' > "$root/skills/e2e-test/policy.md"
  ln -s '../../skills/e2e-test' "$root/.agents/skills/e2e-test"
  ln -s '../../skills/e2e-test' "$root/.claude/skills/e2e-test"
  printf '# AEP Workflow\n' > "$root/AGENTS.md"
  printf '@AGENTS.md\n' > "$root/CLAUDE.md"
  printf '{}\n' > "$root/skills-lock.json"
  printf '#!/usr/bin/env bash\n' > "$root/.claude/hooks/workspace-setup.sh"
  printf '.dev-workflow/\n.feature-workspaces/\n' > "$root/.gitignore"
}

case_dir="$TMP_ROOT/baseline"
baseline "$case_dir"
(cd "$case_dir" && bash "$AUDIT" >/dev/null) || fail "canonical baseline reported drift"
pass "canonical audit baseline"

case_dir="$TMP_ROOT/injection"
baseline "$case_dir"
malicious='x);touch${IFS}PWNED;#'
mkdir -p "$case_dir/skills/$malicious"
ln -s "../../skills/$malicious" "$case_dir/.agents/skills/$malicious"
ln -s "../../skills/$malicious" "$case_dir/.claude/skills/$malicious"
(cd "$case_dir" && bash "$AUDIT" >/dev/null) || fail "safe unusual skill name reported drift"
[ ! -e "$case_dir/PWNED" ] || fail "skill name executed shell code"
pass "skill-name command injection prevention"

case_dir="$TMP_ROOT/divergent-aep"
baseline "$case_dir"
rm "$case_dir/.claude/skills/aep-scaffold"
mkdir -p "$case_dir/.claude/skills/aep-scaffold"
printf 'divergent\n' > "$case_dir/.claude/skills/aep-scaffold/SKILL.md"
if (cd "$case_dir" && bash "$AUDIT" >/dev/null); then
  fail "divergent AEP copies passed audit"
fi
pass "divergent AEP layout detection"

case_dir="$TMP_ROOT/gitignore-substring"
baseline "$case_dir"
printf 'xdev-workflow/\nxfeature-workspaces/\n' > "$case_dir/.gitignore"
if (cd "$case_dir" && bash "$AUDIT" >/dev/null); then
  fail "gitignore substrings passed exact audit"
fi
pass "exact gitignore matching"

echo "scaffold audit fixtures: $pass_count passed"
