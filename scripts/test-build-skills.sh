#!/usr/bin/env bash
# Security and shape fixtures for generated product-context resources.

set -euo pipefail

REPO_ROOT=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
BUILD="$REPO_ROOT/scripts/build-skills.sh"
TMP_ROOT=$(mktemp -d "${TMPDIR:-/tmp}/aep-build-skills.XXXXXX")
trap 'rm -rf "$TMP_ROOT"' EXIT

pass_count=0
pass() { pass_count=$((pass_count + 1)); echo "PASS: $1"; }
fail() { echo "FAIL: $1" >&2; exit 1; }

fixture() {
  local root="$1"
  mkdir -p "$root/repo/scripts"
  mkdir -p "$root/repo/skills/product-context/_shared/references"
  mkdir -p "$root/repo/skills/product-context/demo"
  cp "$BUILD" "$root/repo/scripts/build-skills.sh"
  printf 'shared\n' > "$root/repo/skills/product-context/_shared/references/shared.md"
  printf '%s\n' '---' 'name: aep-demo' 'description: fixture' '---' '' 'Read references/shared.md.' > "$root/repo/skills/product-context/demo/SKILL.md"
  (cd "$root/repo" && bash scripts/build-skills.sh >/dev/null)
}

case_dir="$TMP_ROOT/traversal"
fixture "$case_dir"
printf 'outside-safe\n' > "$case_dir/outside.txt"
printf '../../../../../outside.txt\n' >> "$case_dir/repo/skills/product-context/demo/references/.aep-generated"
if (cd "$case_dir/repo" && bash scripts/build-skills.sh >/dev/null 2>&1); then
  fail "traversal marker returned success"
fi
[ "$(cat "$case_dir/outside.txt")" = 'outside-safe' ] || fail "traversal marker changed an outside file"
pass "marker traversal rejection"

case_dir="$TMP_ROOT/symlink"
fixture "$case_dir"
printf 'outside-safe\n' > "$case_dir/outside.txt"
rm "$case_dir/repo/skills/product-context/demo/references/shared.md"
ln -s "$case_dir/outside.txt" "$case_dir/repo/skills/product-context/demo/references/shared.md"
if (cd "$case_dir/repo" && bash scripts/build-skills.sh >/dev/null 2>&1); then
  fail "managed symlink returned success"
fi
[ "$(cat "$case_dir/outside.txt")" = 'outside-safe' ] || fail "managed symlink overwrote its target"
pass "managed symlink rejection"

case_dir="$TMP_ROOT/directory"
fixture "$case_dir"
rm "$case_dir/repo/skills/product-context/demo/references/shared.md"
mkdir "$case_dir/repo/skills/product-context/demo/references/shared.md"
if (cd "$case_dir/repo" && bash scripts/build-skills.sh >/dev/null 2>&1); then
  fail "managed directory returned success"
fi
[ ! -e "$case_dir/repo/skills/product-context/demo/references/shared.md/shared.md" ] || fail "directory destination gained a nested copy"
pass "managed directory rejection"

echo "build-skills fixtures: $pass_count passed"
