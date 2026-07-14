#!/usr/bin/env bash
# /aep-scaffold existing-project audit (read-only, drift-aware).
# Prints detected stack + AEP pin, then [ok]/[DRIFT] per category (A canonical
# layout, B e2e shape, C infra, D observability). Changes NOTHING.
# Exit code: 0 when no [DRIFT] remains, 1 while any [DRIFT] is present — so the
# converge loop ("run converge, re-run audit until it exits 0") has a checkable
# termination. Categories D (detection) and E (pin, recommend-only) never count
# toward drift. See references/converge-flow.md for how to interpret each block.

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
# project-owned skills must be real in skills/ and symlinked into both runtimes
for s in $( [ -d skills ] && ls skills 2>/dev/null ); do
  chk "skills/$s exposed to .claude" bash -c "r=\$(readlink -f skills/$s 2>/dev/null); l=\$(readlink -f .claude/skills/$s 2>/dev/null); [ -n \"\$r\" ] && [ \"\$l\" = \"\$r\" ]"
  chk "skills/$s exposed to .agents" bash -c "r=\$(readlink -f skills/$s 2>/dev/null); l=\$(readlink -f .agents/skills/$s 2>/dev/null); [ -n \"\$r\" ] && [ \"\$l\" = \"\$r\" ]"
done

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
chk ".dev-workflow/ gitignored"           bash -c 'grep -q ".dev-workflow/" .gitignore 2>/dev/null'
chk ".feature-workspaces/ gitignored"     bash -c 'grep -q ".feature-workspaces/" .gitignore 2>/dev/null'

echo "=== D. Observability (telemetry candidates for /aep-map) ==="
deps="$(cat package.json 2>/dev/null) $(cat pyproject.toml 2>/dev/null)"
for probe in "sentry:error_stream" "datadog:monitoring" "posthog:analytics" "amplitude:analytics" "@opentelemetry:monitoring" "newrelic:monitoring"; do
  printf "  %-45s" "${probe%%:*} (${probe##*:}):"; echo "$deps" | grep -qi "${probe%%:*}" && echo "[detected]" || echo "[ ]"
done
printf "  %-45s" "health endpoint (/healthz|/readyz|/health):"
grep -rqiE '/(healthz|readyz|health)\b' . --include='*.ts' --include='*.js' --include='*.py' 2>/dev/null && echo "[detected]" || echo "[ ]"

exit "$DRIFT"
