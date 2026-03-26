---
name: testing-guide
description: Reference guide for setting up project-level quality infrastructure. Use when creating a new project's test setup, adding test layers, or when the user asks "how do I add tests?", "set up e2e", "testing strategy", "quality gates". Covers workspace setup hook, e2e-test skill creation, and the testing pyramid mapped to /build phases.
---

# Testing Guide

How to set up the project-level quality infrastructure that the workflow plugin (`/design` → `/launch` → `/build` → `/wrap`) relies on.

**Where this fits:**

```
/onboard → /scaffold → /testing-guide → [ /design → /launch → /build → /wrap ]
                        ▲ you are here
```

`/scaffold` creates the project. This guide creates the quality infrastructure. `/build` uses it during feature development.

---

## Two Things to Set Up

Every project needs two things before `/build` can run autonomously:

1. **Workspace Setup Hook** — `.claude/hooks/workspace-setup.sh`
2. **E2E Test Skill** — `.claude/skills/e2e-test/`

---

## Part 1: Workspace Setup Hook

### What it is

A convention-based script that `/build` calls during Phase 0 (workspace init) and session recovery (`init.sh`). The workflow plugin doesn't know your stack — this script does.

### Contract

The hook **MUST**:
- Install dependencies (bun/npm/pnpm/cargo/poetry/etc.)
- Start the dev server (or verify it's running)
- Write `.dev-workflow/ports.env` with at minimum:
  ```
  WEB_PORT=<port>
  SERVER_PORT=<port>
  BASE_URL=http://localhost:<web-port>
  SERVER_URL=http://localhost:<server-port>
  ```
- Handle port scanning for parallel workspace isolation

The hook **MAY**:
- Validate `.env` files against `.env.example` templates
- Run database migrations
- Seed test accounts
- Clean container/cache state
- Copy config from main workspace (for worktree/workspace isolation)

### Template

```bash
#!/usr/bin/env bash
# Workspace Setup Hook
# Called by /build Phase 0 and init.sh (session recovery)
#
# Contract: MUST write .dev-workflow/ports.env
set -euo pipefail

REPO_ROOT="$(git rev-parse --show-toplevel 2>/dev/null)"
cd "$REPO_ROOT"

# ── PROJECT-SPECIFIC: Detect workspace vs main ──
# Uncomment if your project uses git worktrees alongside jj workspaces:
# MAIN_REPO="$(git worktree list --porcelain 2>/dev/null | head -1 | sed 's/^worktree //')" || MAIN_REPO="$REPO_ROOT"
# IS_WORKSPACE=false
# [ "$REPO_ROOT" != "$MAIN_REPO" ] && IS_WORKSPACE=true

# ── PROJECT-SPECIFIC: Validate .env files ──
# Example for monorepo with multiple .env files:
# for example in apps/server/.env.example apps/web/.env.example; do
#   env="${example%.example}"
#   [ ! -f "$env" ] && cp "$example" "$env"
# done

# ── PROJECT-SPECIFIC: Install dependencies ──
# bun install
# npm install
# cargo build
# pip install -r requirements.txt

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
# sed -i '' "s|^WEB_PORT=.*|WEB_PORT=$WEB_PORT|" apps/web/.env

# ── Write ports.env (CONTRACT — required) ──
mkdir -p .dev-workflow
cat > .dev-workflow/ports.env <<EOF
SERVER_PORT=$SERVER_PORT
WEB_PORT=$WEB_PORT
SERVER_URL=http://localhost:$SERVER_PORT
BASE_URL=http://localhost:$WEB_PORT
EOF

# ── PROJECT-SPECIFIC: Start dev server ──
# if ! lsof -ti :$SERVER_PORT >/dev/null 2>&1; then
#   bun run dev &
# fi

# ── PROJECT-SPECIFIC: Seed database ──
# SCRIPT_DIR="$(cd "$(dirname "$0")/../skills/e2e-test/scripts" && pwd)"
# [ -f "$SCRIPT_DIR/seed.sh" ] && bash "$SCRIPT_DIR/seed.sh"

echo "Setup complete. Server: http://localhost:$SERVER_PORT  Web: http://localhost:$WEB_PORT"
```

Make executable: `chmod +x .claude/hooks/workspace-setup.sh`

### Idempotency

The hook will be called:
- Once during `/build` Phase 0 (initial setup)
- Again on every `init.sh` run (session recovery after context reset)

Design it to be safe to run multiple times — check if the dev server is already running before starting a new one, don't fail if dependencies are already installed.

---

## Part 2: E2E Test Skill

### What it is

A project-level skill that lives in `.claude/skills/e2e-test/`. It documents what tests exist, how to run them, and how to add new ones. The build agent reads this skill during Phases 6-8 to understand testing capabilities.

### Directory structure

```
.claude/skills/e2e-test/
├── SKILL.md              # Documents test infrastructure + how to add tests
└── scripts/
    ├── seed.sh           # DB migrations + test account creation (idempotent)
    └── <feature>-e2e.sh  # One script per feature (added during /build Phase 7)
```

### SKILL.md template

```markdown
---
name: e2e-test
description: E2E testing infrastructure for [PROJECT_NAME]. Use when running
  tests, adding test coverage, or understanding what tests exist. Contains
  setup scripts, test scripts, and patterns for adding new tests.
---

# E2E Test Infrastructure

## Prerequisites

- Dev server running (started by workspace-setup.sh hook)
- `.dev-workflow/ports.env` exists (written by workspace-setup.sh)
- [PROJECT-SPECIFIC: any other prerequisites]

## Setup

Source ports before running any test:

\`\`\`bash
source .dev-workflow/ports.env
\`\`\`

## Test Scripts

| Script | What it tests | Tools |
|--------|--------------|-------|
| seed.sh | DB migrations + test account | curl, sqlite3 |
| [feature]-e2e.sh | [description] | agent-browser, curl |

## Adding a New Test

1. Create `.claude/skills/e2e-test/scripts/<feature>-e2e.sh`
2. Follow the E2E script pattern (see below)
3. Add the script to the table above
4. Run it: `bash .claude/skills/e2e-test/scripts/<feature>-e2e.sh`

## E2E Script Pattern

All scripts follow this structure — copy it when creating new tests.

## Test Account

| Field | Value |
|-------|-------|
| Email | [PROJECT-SPECIFIC] |
| Password | [PROJECT-SPECIFIC] |
```

### E2E Script Pattern

Every E2E test script should follow this pattern:

```bash
#!/usr/bin/env bash
# <Feature Name> E2E Test
#
# Prerequisites: dev server running, admin account seeded
# What it tests:
#   - [test 1]
#   - [test 2]
#   - [test 3]

set -euo pipefail

# ── Port resolution ──
REPO_ROOT="$(git rev-parse --show-toplevel)"
if [ -f "$REPO_ROOT/.dev-workflow/ports.env" ]; then
  source "$REPO_ROOT/.dev-workflow/ports.env"
fi
BASE_URL="${BASE_URL:-http://localhost:3001}"
SERVER_URL="${SERVER_URL:-http://localhost:3000}"

# ── Test helpers ──
PASS=0 FAIL=0 SKIP=0
pass() { echo "  PASS: $1"; PASS=$((PASS + 1)); }
fail() { echo "  FAIL: $1"; FAIL=$((FAIL + 1)); }
skip() { echo "  SKIP: $1"; SKIP=$((SKIP + 1)); }

# ── 1. [Test Group Name] ──
echo "=== 1. [Test Group] ==="

# API-level test (curl)
RESPONSE=$(curl -s -o /dev/null -w "%{http_code}" "$SERVER_URL/api/health")
if [ "$RESPONSE" = "200" ]; then
  pass "Health endpoint returns 200"
else
  fail "Health endpoint returned $RESPONSE"
fi

# Browser-level test (agent-browser)
if command -v agent-browser &>/dev/null; then
  agent-browser navigate "$BASE_URL/login"
  # ... browser assertions ...
  pass "Login page loads"
else
  skip "Login page (agent-browser not installed)"
fi

# ── 2. [Next Test Group] ──
echo "=== 2. [Next Group] ==="
# ...

# ── Results ──
echo ""
echo "=== Results ==="
echo "  Passed:  $PASS"
echo "  Failed:  $FAIL"
echo "  Skipped: $SKIP"
[ "$FAIL" -gt 0 ] && exit 1
exit 0
```

### Key patterns

- **Auto-source ports.env** — never hardcode ports
- **Graceful degradation** — skip tests when tools are unavailable (SKIP, not FAIL)
- **Idempotent** — safe to run multiple times
- **Self-contained** — each script tests one feature, can run independently
- **CI-ready** — exit 1 on failure, exit 0 on success

---

## Part 3: Testing Pyramid

Three layers of tests, each mapped to a `/build` phase. Add them incrementally as features are built.

### Layer 1: Unit Tests (Phase 4 — implementation)

| Aspect | Detail |
|--------|--------|
| **What** | Individual functions, modules, utilities |
| **Runner** | Project's test framework (vitest, jest, pytest, cargo test, go test) |
| **Scope** | Pure logic, data transforms, validators, helpers |
| **Convention** | Co-located with source (`*.test.ts`, `*_test.go`, `test_*.py`) |
| **When** | As you implement each task in the jj change stack |
| **Who runs** | Build agent, per-change (`jj edit <change>` → implement → run tests) |
| **Catches** | Logic errors, off-by-one bugs, edge cases, data transform errors |

**How to add:**
```bash
# After implementing a function, add a test next to it:
# src/utils/validate.ts → src/utils/validate.test.ts

# Run with the project's test runner:
# bun test src/utils/validate.test.ts
# vitest run src/utils/validate.test.ts
# pytest tests/test_validate.py
```

### Layer 2: API Integration Tests (Phase 5 — code review)

| Aspect | Detail |
|--------|--------|
| **What** | Endpoint contracts: request/response shapes, auth, error codes |
| **Runner** | curl scripts or test framework with HTTP client |
| **Scope** | API boundaries, auth flows, error handling, data persistence |
| **Convention** | `scripts/api/` or `__tests__/api/` or inline in E2E scripts |
| **When** | After Phase 4 implementation, as part of Phase 5 verification |
| **Who runs** | Build agent during code review |
| **Catches** | Contract breaks, auth gaps, missing error handling, wrong status codes |

**How to add:**

```bash
# Test an API endpoint with curl:
source .dev-workflow/ports.env

# Test: POST /api/widget returns 201
RESPONSE=$(curl -s -w "\n%{http_code}" -X POST "$SERVER_URL/api/widget" \
  -H "Content-Type: application/json" \
  -d '{"name": "test"}')
STATUS=$(echo "$RESPONSE" | tail -1)
BODY=$(echo "$RESPONSE" | head -1)

if [ "$STATUS" = "201" ]; then
  echo "PASS: Create widget returns 201"
else
  echo "FAIL: Expected 201, got $STATUS"
fi
```

API tests can live inside E2E scripts (the curl-based sections) or as standalone scripts. For projects with many API endpoints, consider a dedicated `scripts/api/` directory.

### Layer 3: E2E Browser Tests (Phases 6-7 — testing)

| Aspect | Detail |
|--------|--------|
| **What** | Full user flows through the UI |
| **Runner** | agent-browser (headless Chrome) |
| **Scope** | Login, navigation, form submission, visual state, multi-step flows |
| **Convention** | `.claude/skills/e2e-test/scripts/<feature>-e2e.sh` |
| **When** | Phase 7, after dogfood exploration (Phase 6) identifies what to cover |
| **Who runs** | Build agent (Phase 7-8) + CI/CD |
| **Catches** | Integration failures, UI regressions, flow breaks, visual state errors |

**How to add:** Follow the E2E Script Pattern in Part 2.

### The Incremental Pattern

Features start with zero tests and build up through `/build` phases:

```
Phase 4 (implement)  → unit tests alongside code
                        └─ test the function you just wrote
Phase 5 (review)     → API tests to verify contracts
                        └─ curl the endpoint you just created
Phase 6 (dogfood)    → manual exploration finds what needs coverage
                        └─ agent-browser explores, reports gaps
Phase 7 (e2e)        → browser test scripts for CI/CD
                        └─ codify the dogfood findings into scripts
Phase 8 (review)     → all layers run, gaps identified
                        └─ fix gaps, re-run, move to PR
```

Each layer catches different failure modes. Together they form a safety net that grows with every feature.

### When to Skip Layers

Not every project needs all three layers. Use this guide:

| Project type | Unit | API | E2E Browser |
|---|---|---|---|
| Full-stack web app | Yes | Yes | Yes |
| API-only service | Yes | Yes | Skip |
| CLI tool | Yes | Skip | Skip |
| Static site / landing page | Skip | Skip | E2E only |
| Library / package | Yes | Skip | Skip |
| Mobile app (API backend) | Yes | Yes | Skip (use native testing) |

---

## Part 4: Growing Tests With the Project

### Layer 0 — Walking Skeleton

For the first feature, focus on proving the test infrastructure works:

1. Create the workspace setup hook
2. Create the e2e-test skill with a single seed script
3. Write one E2E test that logs in and verifies the home page loads
4. Run it — if it passes, the infrastructure works

Don't worry about coverage. The goal is **one green test end-to-end**.

### Layer 1+ — Feature Development

Each subsequent feature adds tests at the appropriate layers:

- New utility function → unit test
- New API endpoint → curl-based contract test
- New user flow → E2E browser test script

The build agent does this automatically during `/build` Phases 4-7. The e2e-test SKILL.md tells it where to put scripts and what patterns to follow.

### Evaluator Integration

In full mode (`/launch` with evaluator), the evaluator agent reads `.dev-workflow/feature-verification.json`. Verification steps should map to test assertions where possible:

```json
{
  "task": "Add user profile page",
  "verification_steps": [
    "GET /api/user/profile returns 200 with user data",
    "Profile page renders user name and email",
    "Edit profile form saves changes"
  ]
}
```

Each verification step becomes a test assertion — either in an API test (step 1) or E2E script (steps 2-3).

### CI/CD Integration

E2E scripts in `.claude/skills/e2e-test/scripts/` are designed to run in CI:

- Auto-source `ports.env` with fallback defaults
- Handle missing tools gracefully (SKIP, not FAIL)
- Exit 1 on any FAIL, exit 0 on all PASS/SKIP
- Self-contained — no shared state between scripts

Add them to your CI pipeline:

```yaml
# Example: GitHub Actions
- name: Run E2E tests
  run: |
    for script in .claude/skills/e2e-test/scripts/*-e2e.sh; do
      echo "Running $script..."
      bash "$script" || exit 1
    done
```

---

## Quick Reference

| What | Where | When |
|------|-------|------|
| Workspace setup hook | `.claude/hooks/workspace-setup.sh` | Before `/build` (created during project setup) |
| E2E test skill | `.claude/skills/e2e-test/SKILL.md` | Before `/build` (created during project setup) |
| Seed script | `.claude/skills/e2e-test/scripts/seed.sh` | Phase 0 (called by setup hook) |
| Unit tests | Co-located with source | Phase 4 (per-change) |
| API tests | In E2E scripts or `scripts/api/` | Phase 5 (code review) |
| E2E browser tests | `.claude/skills/e2e-test/scripts/<feature>-e2e.sh` | Phase 7 (after dogfood) |
| Feature verification | `.dev-workflow/feature-verification.json` | Phase 5 (evaluator) |
