#!/usr/bin/env bash
# Functional fixtures for templates/derive-verification-recipe.sh.tmpl.
#
# `bash -n` alone once passed a parser bug that silently disabled the deep hard
# floor (an awk range that terminated on its own start line), so the rendered
# script is exercised against a real git fixture: each case commits a diff and
# asserts the emitted verification-recipe.json carries the documented tier,
# preset, and drift flags (canon: gen-eval references/verification-economics.md).
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
TMPL="$ROOT/skills/project-setup/e2e-skill-scaffolding/templates/derive-verification-recipe.sh.tmpl"
WORK="$(mktemp -d)"
trap 'rm -rf "$WORK"' EXIT

fail=0
check() { # label, expected-substring
  if grep -qF "$2" .dev-workflow/verification-recipe.json; then
    echo "ok   $1"
  else
    echo "FAIL $1 — expected: $2"
    sed 's/^/     /' .dev-workflow/verification-recipe.json
    fail=1
  fi
}

fresh_branch() { # name — always cut from a clean main; a failed checkout must abort, not cascade
  git checkout -q main
  git checkout -qb "$1"
}

# ── fixture repo: rendered script + a policy.md with a real sensitive_paths block ──
cd "$WORK"
git init -q fixture && cd fixture
git config user.email fixture@test && git config user.name fixture
mkdir -p skills/e2e-test/scripts src/auth apps/web/components docs packages/db
sed 's/{{PROJECT_NAME}}/fixture/g' "$TMPL" > skills/e2e-test/scripts/derive-verification-recipe.sh
cat > skills/e2e-test/policy.md <<'EOF'
# Project E2E Policy — fixture

## Sensitive paths (verification hard floor — human-owned)

```yaml
sensitive_paths:
  - "**/auth/**"
  - "**/migrations/**"
  - ".github/workflows/**"
```
EOF
printf '.dev-workflow/\n' > .gitignore
echo base > src/app.ts
echo docs > docs/readme.md
git add -A && git commit -qm base && git branch -M main

derive() { # story-id [provisional] [declared-file]
  git add -A && git commit -qm "$1"
  bash skills/e2e-test/scripts/derive-verification-recipe.sh main "$@" >/dev/null 2>"$WORK/derive-stderr.log" \
    || { echo "FAIL $1 — script exited non-zero:"; sed 's/^/     /' "$WORK/derive-stderr.log"; fail=1; }
}

# 1. A diff touching a sensitive glob derives deep from a provisional light — the hard floor.
fresh_branch case-sensitive
echo x > src/auth/login.ts
derive story-sensitive light
check "sensitive diff ⇒ deep"          '"tier": "deep"'
check "matched glob recorded"          '"**/auth/**"'
check "security preset + floors"       '"dimension_preset": "security-sensitive"'

# 2. A docs-only diff derives light.
fresh_branch case-docs
echo more >> docs/readme.md
derive story-docs light
check "docs-only diff ⇒ light"         '"tier": "light"'

# 3. A plain source diff derives standard with the mixed preset (no dominant signal).
fresh_branch case-standard
echo x >> src/app.ts
derive story-standard light
check "plain diff ⇒ standard"          '"tier": "standard"'
check "no dominant signal ⇒ mixed"     '"dimension_preset": "mixed"'

# 4. A missing provisional tier fails OPEN to deep — even on a docs-only diff.
fresh_branch case-failopen
echo more >> docs/readme.md
derive story-failopen
check "no brief tier ⇒ deep"           '"tier": "deep"'

# 5. A UI-dominant diff derives the ui-heavy preset with its floors.
fresh_branch case-ui
echo x > apps/web/components/button.tsx
derive story-ui light
check "ui diff ⇒ ui-heavy preset"      '"dimension_preset": "ui-heavy"'
check "ui-heavy carries its floors"    '"Visual Design": 3'

# 6. A diff outside the declared files_affected records scope_drift.
fresh_branch case-drift
echo x >> src/app.ts
printf 'docs/**\n' > "$WORK/declared.txt"
derive story-drift light "$WORK/declared.txt"
check "out-of-scope diff ⇒ drift"      '"scope_drift": true'

# 7. A diff inside the declared files_affected records no drift.
fresh_branch case-nodrift
echo x >> src/app.ts
printf 'src/**\n' > "$WORK/declared.txt"
derive story-nodrift light "$WORK/declared.txt"
check "in-scope diff ⇒ no drift"       '"scope_drift": false'

if [ "$fail" -eq 0 ]; then echo "derive-recipe fixtures: all cases green"; fi
exit $fail
