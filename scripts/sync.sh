#!/usr/bin/env bash
# Sync AEP Skills
#
# Copy skills from agentic-engineering-patterns into your project's .claude/skills/ directory.
#
# Usage:
#   bash scripts/sync.sh              # sync all groups
#   bash scripts/sync.sh --dry-run    # preview changes
#   bash scripts/sync.sh --prune      # also delete orphan ${prefix}* skills no longer in source
#   bash scripts/sync.sh workflow     # sync one group
#   bash scripts/sync.sh --help       # show help
#
# Environment:
#   AEP_REPO    Path to the agentic-engineering-patterns repo (auto-detected if run from within)
#   TARGET_DIR  Override target directory (default: $PWD/.claude/skills)
#
# Groups: workflow, product, setup, patterns

set -euo pipefail

show_help() {
  cat <<'HELP'
Usage: bash scripts/sync.sh [OPTIONS] [GROUP]

Sync AEP skills into your project's .claude/skills/ directory.

Options:
  --dry-run    Preview changes without modifying files
  --prune      Delete `aep-*` skills in TARGET_DIR that no longer exist in
               any source group (e.g. the renamed `aep-jj-ref`). Honors
               --dry-run. Only touches dirs matching the `aep-` prefix.
  --help       Show this help message

Groups:
  workflow     Feature development lifecycle (design, launch, build, wrap, jj-ref)
  product      Product planning skills (envision, map, validate, dispatch, reflect)
  setup        Project scaffolding (onboard, scaffold, testing-guide)
  patterns     Reusable patterns (gen-eval, autopilot)

Environment variables:
  AEP_REPO     Path to the agentic-engineering-patterns repo clone.
               Auto-detected when running from within the repo.
  TARGET_DIR   Override the target skills directory.
               Default: $PWD/.claude/skills

Examples:
  # From your project directory, with AEP_REPO set:
  AEP_REPO=~/agentic-engineering-patterns bash scripts/sync.sh

  # Preview what would change:
  bash scripts/sync.sh --dry-run

  # Sync only the workflow skills:
  bash scripts/sync.sh workflow
HELP
}

# --- Resolve AEP_REPO ---

resolve_aep_repo() {
  # If AEP_REPO is set, use it
  if [ -n "${AEP_REPO:-}" ]; then
    AEP_REPO="$(cd "$AEP_REPO" && pwd)"
    return
  fi

  # Auto-detect: if this script lives inside the aep repo
  local script_dir
  script_dir="$(cd "$(dirname "$0")" && pwd)"
  local candidate="$(dirname "$script_dir")"

  if [ -d "$candidate/skills" ] && [ -f "$candidate/.claude-plugin/marketplace.json" ]; then
    AEP_REPO="$candidate"
    return
  fi

  echo "ERROR: Cannot find agentic-engineering-patterns repo."
  echo ""
  echo "  Set AEP_REPO to the path of your local clone:"
  echo "    AEP_REPO=/path/to/agentic-engineering-patterns bash scripts/sync.sh"
  echo ""
  echo "  Or run this script from within the repo itself."
  exit 1
}

# --- Parse args ---

DRY_RUN=false
PRUNE=false
FILTER=""

for arg in "$@"; do
  case "$arg" in
    --dry-run) DRY_RUN=true ;;
    --prune) PRUNE=true ;;
    --help|-h) show_help; exit 0 ;;
    workflow|product|setup|patterns) FILTER="$arg" ;;
    *)
      echo "Unknown argument: $arg"
      echo "Run with --help for usage."
      exit 1
      ;;
  esac
done

# --- Resolve paths ---

resolve_aep_repo

TARGET="${TARGET_DIR:-$PWD/.claude/skills}"

if [ ! -d "$AEP_REPO/skills" ]; then
  echo "ERROR: No skills/ directory found in $AEP_REPO"
  exit 1
fi

# --- Read version ---

VERSION=""
MARKETPLACE="$AEP_REPO/.claude-plugin/marketplace.json"
if [ -f "$MARKETPLACE" ]; then
  # Extract version without requiring jq
  VERSION=$(grep -o '"version"[[:space:]]*:[[:space:]]*"[^"]*"' "$MARKETPLACE" | head -1 | grep -o '"[^"]*"$' | tr -d '"')
fi

echo "Syncing AEP skills${VERSION:+ (v$VERSION)}"
echo "  from: $AEP_REPO/skills/"
echo "    to: $TARGET/"
echo ""

# --- Sync logic ---

sync_group() {
  local key="$1"
  local dir="$2"
  local prefix="$3"
  local src="$AEP_REPO/skills/$dir"

  if [ -n "$FILTER" ] && [ "$FILTER" != "$key" ]; then
    return
  fi

  if [ ! -d "$src" ]; then
    echo "WARNING: $src not found, skipping"
    return
  fi

  echo "=== $key ($dir) → prefix: '${prefix:-<none>}' ==="

  # Flatten: copy each sub-skill directory to top-level .claude/skills/{prefix}{skill-name}/
  # Claude Code only discovers skills at depth 1 (.claude/skills/*/SKILL.md)
  # Skip subdirs that aren't skills (no SKILL.md inside)
  local sub_count=0
  for sub in "$src"/*/; do
    [ -d "$sub" ] || continue
    [ -f "$sub/SKILL.md" ] || continue
    local subname
    subname="$(basename "$sub")"
    local prefixed="${prefix}${subname}"
    local dst="$TARGET/$prefixed"

    if [ "$DRY_RUN" = true ]; then
      if [ -d "$dst" ]; then
        diff -rq "$sub" "$dst" 2>/dev/null || true
      else
        echo "  Would create $prefixed/"
      fi
    else
      mkdir -p "$TARGET"
      rm -rf "$dst"
      cp -r "$sub" "$dst"
      # Update the name field in SKILL.md frontmatter to match the prefixed name
      if [ -n "$prefix" ]; then
        sed -i '' "s/^name: ${subname}$/name: ${prefixed}/" "$dst/SKILL.md" 2>/dev/null \
          || sed -i "s/^name: ${subname}$/name: ${prefixed}/" "$dst/SKILL.md"
      fi
      echo "  → $prefixed/"
    fi
    sub_count=$((sub_count + 1))
  done

  # Sync shared directories (references/, templates/) into each skill that references them
  for shared in references templates; do
    local shared_src="$src/$shared"
    [ -d "$shared_src" ] || continue

    # Copy shared dir into every synced skill in this group so relative paths work
    for sub in "$src"/*/; do
      [ -d "$sub" ] || continue
      [ -f "$sub/SKILL.md" ] || continue
      local subname
      subname="$(basename "$sub")"
      local prefixed="${prefix}${subname}"
      local dst="$TARGET/$prefixed/$shared"

      # Skip if the skill already has its own version of this shared dir
      [ -d "$sub/$shared" ] && continue

      if [ "$DRY_RUN" = true ]; then
        echo "  Would copy shared $shared/ → $prefixed/$shared/"
      else
        cp -r "$shared_src" "$dst"
        echo "  → $prefixed/$shared/ (shared)"
      fi
    done
  done

  # Remove the old nested group directory if it still exists
  if [ "$DRY_RUN" = false ] && [ -d "$TARGET/$dir" ]; then
    rm -rf "$TARGET/$dir"
    echo "  Removed old nested dir: $dir/"
  fi

  echo "  $sub_count skills synced"
  echo ""
  SYNCED=$((SYNCED + 1))
}

SYNCED=0

sync_group "workflow" "agentic-development-workflow" "aep-"
sync_group "product"  "product-context"              "aep-"
sync_group "setup"    "project-setup"                "aep-"
sync_group "patterns" "patterns"                     "aep-"

if [ "$SYNCED" -eq 0 ]; then
  echo "Nothing synced. Available groups: workflow, product, setup, patterns"
  exit 1
fi

# --- Prune orphans ---
# Remove `aep-*` skills in TARGET that don't correspond to any current source skill.
# Only runs when --prune is set, and never when a single group filter is in effect
# (because then we don't have visibility into the other groups' skills and would
# falsely flag them as orphans).
if [ "$PRUNE" = true ]; then
  if [ -n "$FILTER" ]; then
    echo "Skipping prune: --prune ignored when a single group is specified"
    echo ""
  elif [ ! -d "$TARGET" ]; then
    : # nothing to prune; sync would have created TARGET if it had work
  else
    # Build the set of valid `aep-*` skill names from the source.
    valid_names=""
    for group_dir in agentic-development-workflow product-context project-setup patterns; do
      src="$AEP_REPO/skills/$group_dir"
      [ -d "$src" ] || continue
      for sub in "$src"/*/; do
        [ -d "$sub" ] || continue
        [ -f "$sub/SKILL.md" ] || continue
        valid_names="$valid_names aep-$(basename "$sub")"
      done
    done

    pruned=0
    for entry in "$TARGET"/aep-*/; do
      [ -d "$entry" ] || continue
      name="$(basename "$entry")"
      # Match exact name against the valid set (space-padded so partial matches don't pass).
      if [[ " $valid_names " != *" $name "* ]]; then
        if [ "$DRY_RUN" = true ]; then
          echo "  Would prune orphan: $name/"
        else
          rm -rf "$entry"
          echo "  Pruned orphan: $name/"
        fi
        pruned=$((pruned + 1))
      fi
    done

    if [ "$pruned" -gt 0 ]; then
      echo ""
    fi
  fi
fi

if [ "$DRY_RUN" = true ]; then
  echo "=== Dry run complete (no files changed) ==="
else
  echo "=== Sync complete ($SYNCED groups) ==="
fi
