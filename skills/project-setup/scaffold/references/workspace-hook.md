# Workspace Setup Hook

The project-specific init script that `/aep-build` Phase 0 (and `init.sh` session recovery) call. The
workflow plugin doesn't know your stack — this script does. `/aep-scaffold` Phase 7 generates it.

## Contract

The hook **MUST**:

- Install dependencies (bun/npm/pnpm/cargo/uv/etc.).
- Start the dev server (or verify it's running).
- Write `.dev-workflow/ports.env` with at minimum:
  ```
  WEB_PORT=<port>
  SERVER_PORT=<port>
  BASE_URL=http://localhost:<web-port>
  SERVER_URL=http://localhost:<server-port>
  ```
- Handle port scanning for parallel workspace isolation.

The hook **MAY**: validate `.env` files against `.env.example`; run database migrations; seed test
accounts (call `skills/e2e-test/scripts/seed.sh` if present); clean container/cache state; copy config
from the main workspace (for worktree isolation).

## Template

```bash
#!/usr/bin/env bash
# Workspace Setup Hook
# Called by /aep-build Phase 0 and init.sh (session recovery)
# Contract: MUST write .dev-workflow/ports.env
set -euo pipefail

REPO_ROOT="$(git rev-parse --show-toplevel 2>/dev/null || pwd)"   # || pwd: don't abort under set -e before git init
cd "$REPO_ROOT"

# ── Detect workspace vs main ──
# AEP runs feature work in git worktrees at .feature-workspaces/<name>/.
# `git worktree list --porcelain` lists the main checkout first, so its first
# entry is the canonical main repo. Compare to detect whether we're in a worktree.
MAIN_REPO="$(git worktree list --porcelain 2>/dev/null | head -1 | sed 's/^worktree //')" || MAIN_REPO="$REPO_ROOT"
IS_WORKSPACE=false
[ "$REPO_ROOT" != "$MAIN_REPO" ] && IS_WORKSPACE=true

# ── PROJECT-SPECIFIC: Validate .env files ──
# for example in apps/server/.env.example apps/web/.env.example; do
#   env="${example%.example}"; [ ! -f "$env" ] && cp "$example" "$env"
# done

# ── PROJECT-SPECIFIC: Install dependencies ──
# bun install   |   npm install   |   cargo build   |   uv sync

# ── Port scanning (parallel workspace isolation) ──
SERVER_PORT=3000
WEB_PORT=3001
while lsof -i :"$SERVER_PORT" -sTCP:LISTEN >/dev/null 2>&1 || \
      lsof -i :"$WEB_PORT" -sTCP:LISTEN >/dev/null 2>&1; do
  SERVER_PORT=$((SERVER_PORT + 10))
  WEB_PORT=$((SERVER_PORT + 1))
done

# ── PROJECT-SPECIFIC: Update config with assigned ports ──
# sed -i '' "s|^SERVER_PORT=.*|SERVER_PORT=$SERVER_PORT|" apps/server/.env

# ── Write ports.env (CONTRACT — required) ──
mkdir -p .dev-workflow
cat > .dev-workflow/ports.env <<EOF
SERVER_PORT=$SERVER_PORT
WEB_PORT=$WEB_PORT
SERVER_URL=http://localhost:$SERVER_PORT
BASE_URL=http://localhost:$WEB_PORT
EOF

# ── PROJECT-SPECIFIC: Start dev server ──
# if ! lsof -ti :$SERVER_PORT >/dev/null 2>&1; then bun run dev & fi

# ── PROJECT-SPECIFIC: Seed database ──
# SCRIPT_DIR="$REPO_ROOT/skills/e2e-test/scripts"
# [ -f "$SCRIPT_DIR/seed.sh" ] && bash "$SCRIPT_DIR/seed.sh"

echo "Setup complete. Server: http://localhost:$SERVER_PORT  Web: http://localhost:$WEB_PORT"
```

Make executable: `chmod +x .claude/hooks/workspace-setup.sh`

> The seed path is the **canonical** `skills/e2e-test/scripts/seed.sh` (real dir). The
> `.claude/skills/e2e-test` / `.agents/skills/e2e-test` symlinks resolve to it, so either path works.

## Idempotency

The hook runs once during `/aep-build` Phase 0 and again on every `init.sh` (session recovery). Design it
to be safe to run multiple times — check whether the dev server is already running before starting a new
one, and don't fail if dependencies are already installed.
