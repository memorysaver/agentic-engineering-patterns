#!/usr/bin/env bash
# /aep-scaffold existing-project audit (read-only, drift-aware).
# Prints detected stack + AEP pin, then [ok]/[DRIFT] per category (A canonical
# layout, B e2e shape, C infra, D observability). Changes NOTHING.
# Exit code: 0 when no [DRIFT] remains, 1 while any [DRIFT] is present — so the
# converge loop ("run converge, re-run audit until it exits 0") has a checkable
# termination. Categories D (detection) and E (pin, recommend-only) never count
# toward drift. See references/converge-flow.md for how to interpret each block.

SCRIPT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
# shellcheck source=layout-lib.sh
. "$SCRIPT_DIR/layout-lib.sh"

DRIFT=0

# ── Phase 0E: Status Check (stack + pin) ──
echo "=== Detecting stack ==="
[ -f "package.json" ] && echo "Language: TypeScript/JavaScript"
[ -f "pyproject.toml" ] && echo "Language: Python"
[ -f "Cargo.toml" ] && echo "Language: Rust"
[ -f "go.mod" ] && echo "Language: Go"
for lk in "bun.lockb:bun" "pnpm-lock.yaml:pnpm" "package-lock.json:npm" "yarn.lock:yarn" "uv.lock:uv"; do
  [ -f "${lk%%:*}" ] && echo "Package manager: ${lk##*:}"
done
[ -f "turbo.json" ] && echo "Monorepo: Turborepo"; [ -f "nx.json" ] && echo "Monorepo: Nx"
[ -f "package.json" ] && {
  for f in '"hono":Backend:Hono' '"express":Backend:Express' '"next":Frontend:Next.js' \
           '"@tanstack/react-router":Frontend:TanStack Router' '"nuxt":Frontend:Nuxt' '"svelte":Frontend:Svelte' \
           '"native-uniwind":Frontend:React Native' '"@tauri-apps/api":Frontend:Tauri' '"electrobun":Frontend:Electrobun'; do
    grep -q "${f%%:*}" package.json 2>/dev/null && echo "${f#*:}" | tr ':' ' '
  done
}

# AEP pin (skills CLI) + latest release
echo "=== AEP pin ==="
[ -f "skills-lock.json" ] && echo "skills-lock.json: present" || echo "skills-lock.json: MISSING (skills CLI not used here)"
grep -oE 'pinned at \*\*v[0-9.]+\*\*' AGENTS.md 2>/dev/null || echo "AGENTS.md pin note: none"
echo "latest release: https://github.com/memorysaver/agentic-engineering-patterns/releases/latest"

# ── Phase 1E: Audit (drift-aware), grouped by category ──
echo "=== A. Canonical skills layout (cross-tool) ==="
chk() { printf "  %-52s" "$1:"; shift; if "$@"; then echo "[ok]"; else echo "[DRIFT]"; DRIFT=1; fi; }
chk "skills-lock.json present"            test -f skills-lock.json
chk ".agents/skills exists (codex install)" test -d .agents/skills
chk "AGENTS.md present"                    test -f AGENTS.md
chk "AGENTS.md has AEP Workflow section"   bash -c 'grep -q "AEP Workflow" AGENTS.md 2>/dev/null'
chk "CLAUDE.md = @AGENTS.md import"        bash -c '[ "$(head -1 CLAUDE.md 2>/dev/null | tr -d "[:space:]")" = "@AGENTS.md" ]'
chk "AEP skills use canonical cross-tool layout" aep_layout_is_canonical
# project-owned skills must be real in skills/ and symlinked into both runtimes
same_directory() {
  [ -d "$1" ] && [ -d "$2" ] || return 1
  [ "$(cd "$1" 2>/dev/null && pwd -P)" = "$(cd "$2" 2>/dev/null && pwd -P)" ]
}
real_directory() {
  [ -d "$1" ] && [ ! -L "$1" ]
}
if [ -d skills ]; then
  while IFS= read -r -d '' project_skill; do
    s=${project_skill##*/}
    chk "skills/$s is a real directory" real_directory "$project_skill"
    chk "skills/$s exposed to .claude" same_directory "$project_skill" ".claude/skills/$s"
    chk "skills/$s exposed to .agents" same_directory "$project_skill" ".agents/skills/$s"
  done < <(find skills -mindepth 1 -maxdepth 1 \( -type d -o -type l \) -print0)
fi

echo "=== B. E2E-test skill shape ==="
# Canonical = policy.md (the single source of truth, always emitted) OR a BDD journeys/ library.
# A none-target project (no runnable surface) legitimately has policy.md and NO journeys/ — keying on
# journeys/README.md alone would mislabel it as DRIFT forever (breaking idempotency).
# (A cli project DOES ship journeys/ — its Tier-2 is a bash journey.)
if   [ -f skills/e2e-test/policy.md ] || [ -f skills/e2e-test/journeys/README.md ]; then echo "  canonical     [ok]"
elif [ -d skills/e2e-test ];                     then echo "  real-non-bdd  [DRIFT → upgrade to BDD]"; DRIFT=1
elif [ -d .claude/skills/e2e-test ] && [ ! -L .claude/skills/e2e-test ]; then echo "  thin-legacy   [DRIFT → migrate to skills/ + BDD]"; DRIFT=1
else echo "  absent        [DRIFT → generate]"; DRIFT=1; fi

echo "=== C. Infrastructure ==="
chk "openspec/ initialized"               test -d openspec
chk ".claude/commands/opsx/ aliases"      test -d .claude/commands/opsx
chk ".claude/hooks/workspace-setup.sh"    test -f .claude/hooks/workspace-setup.sh
chk ".dev-workflow/ gitignored"           grep -qxF '.dev-workflow/' .gitignore
chk ".feature-workspaces/ gitignored"     grep -qxF '.feature-workspaces/' .gitignore

echo "=== D. Observability (telemetry candidates for /aep-map) ==="
deps="$(cat package.json 2>/dev/null) $(cat pyproject.toml 2>/dev/null)"
for probe in "sentry:error_stream" "datadog:monitoring" "posthog:analytics" "amplitude:analytics" "@opentelemetry:monitoring" "newrelic:monitoring"; do
  printf "  %-45s" "${probe%%:*} (${probe##*:}):"; echo "$deps" | grep -qi "${probe%%:*}" && echo "[detected]" || echo "[ ]"
done
printf "  %-45s" "health endpoint (/healthz|/readyz|/health):"
health_found=0
for source_root in apps packages src app lib server; do
  [ -d "$source_root" ] || continue
  while IFS= read -r -d '' source_file; do
    if grep -qiE '/(healthz|readyz|health)\b' "$source_file"; then
      health_found=1
      break 2
    fi
  done < <(find "$source_root" -type f \( -name '*.ts' -o -name '*.js' -o -name '*.py' \) ! -path '*/node_modules/*' -print0)
done
[ "$health_found" -eq 1 ] && echo "[detected]" || echo "[ ]"

exit "$DRIFT"
