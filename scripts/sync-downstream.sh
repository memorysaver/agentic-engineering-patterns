#!/usr/bin/env bash
# Sync Downstream — Push AEP skills to registered downstream projects
#
# Usage:
#   bash scripts/sync-downstream.sh              # push to all projects
#   bash scripts/sync-downstream.sh --dry-run    # preview changes
#   bash scripts/sync-downstream.sh 91app        # push to one project (name match)
#   bash scripts/sync-downstream.sh --init       # create .aep/config.yaml template
#   bash scripts/sync-downstream.sh --help       # show help
#
# Config: .aep/config.yaml (gitignored — paths are machine-local)

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
AEP_REPO="$(dirname "$SCRIPT_DIR")"
CONFIG="$AEP_REPO/.aep/config.yaml"

show_help() {
  cat <<'HELP'
Usage: bash scripts/sync-downstream.sh [OPTIONS] [PROJECT_NAME]

Push AEP skills to registered downstream projects.

Options:
  --dry-run    Preview changes without modifying files
  --init       Create .aep/config.yaml template
  --help       Show this help message

Arguments:
  PROJECT_NAME  Partial name match to sync only one project (e.g., "91app")

Config file: .aep/config.yaml
  Downstream projects are registered in this file. It is gitignored because
  paths are machine-local (different devs have repos in different locations).

  Run --init to create a template, then fill in your project paths.

Examples:
  bash scripts/sync-downstream.sh              # push to all
  bash scripts/sync-downstream.sh --dry-run    # preview all
  bash scripts/sync-downstream.sh 91app        # push to matching project
  bash scripts/sync-downstream.sh monet        # push to matching project
HELP
}

init_config() {
  mkdir -p "$AEP_REPO/.aep"
  if [ -f "$CONFIG" ]; then
    echo "Config already exists: $CONFIG"
    echo "Edit it directly to add or modify downstream projects."
    exit 0
  fi

  cat > "$CONFIG" <<'YAML'
# Downstream projects that receive AEP skills via sync-downstream.
# Paths are machine-local — this file is gitignored.
#
# Fields:
#   name:    Display name (also used for filtering: bash sync-downstream.sh 91app)
#   path:    Absolute path or ~/relative to the downstream project root
#   groups:  Which skill groups to sync. Options: workflow, product, setup, patterns
#            Default: all groups
#   prefix:  Skill name prefix. Default: aep-

downstream:
  - name: 91app-agent-platform
    path: ~/Documents/github/91app-agent-platform
    # groups: [workflow, product, setup, patterns]
    # prefix: aep-

  - name: monetlab-ai
    path: ~/Documents/github/monetlab-ai
    # groups: [workflow, product, setup, patterns]
    # prefix: aep-
YAML

  echo "Created $CONFIG"
  echo "Edit the file to set your project paths, then run:"
  echo "  bash scripts/sync-downstream.sh"
}

# --- Parse args ---

DRY_RUN=false
NAME_FILTER=""

for arg in "$@"; do
  case "$arg" in
    --dry-run) DRY_RUN=true ;;
    --init) init_config; exit 0 ;;
    --help|-h) show_help; exit 0 ;;
    --*) echo "Unknown option: $arg"; echo "Run with --help for usage."; exit 1 ;;
    *) NAME_FILTER="$arg" ;;
  esac
done

# --- Validate ---

if [ ! -f "$CONFIG" ]; then
  echo "ERROR: No config file found at $CONFIG"
  echo ""
  echo "  Run --init to create a template:"
  echo "    bash scripts/sync-downstream.sh --init"
  exit 1
fi

if [ ! -f "$SCRIPT_DIR/sync.sh" ]; then
  echo "ERROR: sync.sh not found at $SCRIPT_DIR/sync.sh"
  exit 1
fi

# --- Parse YAML (lightweight, no jq/yq required) ---
# Extracts downstream entries from the simple YAML format.
# Each entry has: name, path, groups (optional), prefix (optional).

parse_entries() {
  local in_entry=false
  local name="" path="" groups="" prefix=""

  while IFS= read -r line; do
    # Skip comments and empty lines
    [[ "$line" =~ ^[[:space:]]*# ]] && continue
    [[ -z "${line// /}" ]] && continue

    # New entry starts with "  - name:"
    if [[ "$line" =~ ^[[:space:]]*-[[:space:]]*name:[[:space:]]*(.+) ]]; then
      # Emit previous entry if exists
      if [ -n "$name" ] && [ -n "$path" ]; then
        echo "${name}|${path}|${groups}|${prefix}"
      fi
      name="${BASH_REMATCH[1]}"
      path=""
      groups=""
      prefix=""
      in_entry=true
      continue
    fi

    if [ "$in_entry" = true ]; then
      if [[ "$line" =~ ^[[:space:]]*path:[[:space:]]*(.+) ]]; then
        path="${BASH_REMATCH[1]}"
      elif [[ "$line" =~ ^[[:space:]]*groups:[[:space:]]*\[(.+)\] ]]; then
        groups="${BASH_REMATCH[1]}"
      elif [[ "$line" =~ ^[[:space:]]*prefix:[[:space:]]*(.+) ]]; then
        prefix="${BASH_REMATCH[1]}"
      fi
    fi
  done < "$CONFIG"

  # Emit last entry
  if [ -n "$name" ] && [ -n "$path" ]; then
    echo "${name}|${path}|${groups}|${prefix}"
  fi
}

# --- Read version ---

VERSION=""
MARKETPLACE="$AEP_REPO/.claude-plugin/marketplace.json"
if [ -f "$MARKETPLACE" ]; then
  VERSION=$(grep -o '"version"[[:space:]]*:[[:space:]]*"[^"]*"' "$MARKETPLACE" | head -1 | grep -o '"[^"]*"$' | tr -d '"')
fi

echo "AEP Sync Downstream${VERSION:+ (v$VERSION)}"
echo "  source: $AEP_REPO/skills/"
if [ -n "$NAME_FILTER" ]; then
  echo "  filter: *${NAME_FILTER}*"
fi
if [ "$DRY_RUN" = true ]; then
  echo "  mode: dry-run"
fi
echo ""

# --- Sync each downstream project ---

TOTAL=0
SYNCED=0
FAILED=0

while IFS='|' read -r name path groups prefix; do
  # Apply name filter
  if [ -n "$NAME_FILTER" ]; then
    if [[ ! "$name" == *"$NAME_FILTER"* ]]; then
      continue
    fi
  fi

  TOTAL=$((TOTAL + 1))

  # Expand ~ in path
  path="${path/#\~/$HOME}"
  # Trim whitespace
  path="$(echo "$path" | xargs)"
  name="$(echo "$name" | xargs)"

  echo "━━━ $name ━━━"
  echo "  path: $path"

  # Validate path
  if [ ! -d "$path" ]; then
    echo "  ERROR: directory not found — skipping"
    FAILED=$((FAILED + 1))
    echo ""
    continue
  fi

  # Set defaults
  prefix="${prefix:-aep-}"
  prefix="$(echo "$prefix" | xargs)"
  local_target="$path/.claude/skills"

  # Build sync args
  SYNC_ARGS=()
  if [ "$DRY_RUN" = true ]; then
    SYNC_ARGS+=(--dry-run)
  fi

  # If groups specified, sync each group separately
  if [ -n "$groups" ]; then
    # Parse comma-separated groups: "workflow, product, setup" → loop
    IFS=',' read -ra GROUP_LIST <<< "$groups"
    for group in "${GROUP_LIST[@]}"; do
      group="$(echo "$group" | xargs)"  # trim whitespace
      AEP_REPO="$AEP_REPO" TARGET_DIR="$local_target" \
        bash "$SCRIPT_DIR/sync.sh" "${SYNC_ARGS[@]}" "$group" 2>&1 | sed 's/^/  /'
    done
  else
    # Sync all groups
    AEP_REPO="$AEP_REPO" TARGET_DIR="$local_target" \
      bash "$SCRIPT_DIR/sync.sh" "${SYNC_ARGS[@]}" 2>&1 | sed 's/^/  /'
  fi

  SYNCED=$((SYNCED + 1))
  echo ""

done < <(parse_entries)

# --- Summary ---

if [ "$TOTAL" -eq 0 ]; then
  if [ -n "$NAME_FILTER" ]; then
    echo "No projects matched '$NAME_FILTER'."
    echo "Check your .aep/config.yaml for registered project names."
  else
    echo "No downstream projects configured."
    echo "Edit $CONFIG to add projects."
  fi
  exit 1
fi

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
if [ "$DRY_RUN" = true ]; then
  echo "Dry run complete: $TOTAL projects checked"
else
  echo "Sync complete: $SYNCED/$TOTAL projects synced"
  if [ "$FAILED" -gt 0 ]; then
    echo "  $FAILED projects failed (check paths in $CONFIG)"
  fi
fi
