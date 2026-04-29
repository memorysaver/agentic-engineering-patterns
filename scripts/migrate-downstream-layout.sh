#!/usr/bin/env bash
# Migrate Downstream Layout — Convert a downstream project to the canonical
# `skills/` + symlinks layout (newsroom Option A).
#
# Before:
#   <project>/.claude/skills/   (real dir; contains aep-* + project-owned + dotfiles symlinks)
#   <project>/.agents/skills/   (real dir; contains aep-* duplicates from dual-write)
#
# After:
#   <project>/skills/           (canonical real dir; everything moved here)
#   <project>/.claude/skills -> ../skills    (symlink)
#   <project>/.agents/skills -> ../skills    (symlink)
#
# Usage:
#   bash scripts/migrate-downstream-layout.sh             # migrate all
#   bash scripts/migrate-downstream-layout.sh --dry-run   # preview
#   bash scripts/migrate-downstream-layout.sh looplia     # one project (name match)
#   bash scripts/migrate-downstream-layout.sh --help
#
# Pre-flight (per project, no override):
#   - Must be a git repo
#   - On `main`
#   - Working tree clean (so the moves show up as one reviewable diff)
#
# The script is idempotent — re-running on an already-migrated project is a no-op.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
AEP_REPO="$(dirname "$SCRIPT_DIR")"
CONFIG="$AEP_REPO/.aep/config.yaml"

show_help() {
  cat <<'HELP'
Usage: bash scripts/migrate-downstream-layout.sh [OPTIONS] [PROJECT_NAME]

Convert downstream projects to the canonical `skills/` + symlinks layout
(newsroom Option A): one canonical skills/ tree at the project root with
.claude/skills and .agents/skills as symlinks pointing into it.

Pre-flight (per project, no override):
  - Project must be a git repo
  - Must be on `main` branch
  - Working tree must be clean
Projects failing any check are SKIPPED with a clear reason; the run continues.

Options:
  --dry-run    Preview changes without modifying files
  --help       Show this help message

Arguments:
  PROJECT_NAME  Partial name match to migrate only one project (e.g., "looplia")

The script is idempotent — re-running on an already-migrated project is a no-op.
HELP
}

# --- Parse args ---

DRY_RUN=false
NAME_FILTER=""

for arg in "$@"; do
  case "$arg" in
    --dry-run) DRY_RUN=true ;;
    --help|-h) show_help; exit 0 ;;
    --*) echo "Unknown option: $arg"; echo "Run with --help for usage."; exit 1 ;;
    *) NAME_FILTER="$arg" ;;
  esac
done

# --- Validate ---

if [ ! -f "$CONFIG" ]; then
  echo "ERROR: No config file found at $CONFIG"
  exit 1
fi

# --- Parse YAML (lightweight, no jq/yq required) ---
# Reuses the same minimal parser as sync-downstream.sh. Only `name` and `path`
# are needed for migration.

parse_entries() {
  local in_entry=false
  local name="" path=""

  while IFS= read -r line; do
    [[ "$line" =~ ^[[:space:]]*# ]] && continue
    [[ -z "${line// /}" ]] && continue

    if [[ "$line" =~ ^[[:space:]]*-[[:space:]]*name:[[:space:]]*(.+) ]]; then
      if [ -n "$name" ] && [ -n "$path" ]; then
        echo "${name}|${path}"
      fi
      name="${BASH_REMATCH[1]}"
      path=""
      in_entry=true
      continue
    fi

    if [ "$in_entry" = true ]; then
      if [[ "$line" =~ ^[[:space:]]*path:[[:space:]]*(.+) ]]; then
        path="${BASH_REMATCH[1]}"
      fi
    fi
  done < "$CONFIG"

  if [ -n "$name" ] && [ -n "$path" ]; then
    echo "${name}|${path}"
  fi
}

# --- Helpers ---

# Resolve a path's canonical real path. Returns empty if path doesn't exist.
real_path() {
  local p="$1"
  if [ -L "$p" ] || [ -d "$p" ]; then
    (cd "$p" 2>/dev/null && pwd -P) || echo ""
  else
    echo ""
  fi
}

# Echo "yes" if `<rel>/<entry>` is tracked by git in the given project.
git_tracked_at() {
  local proj="$1"
  local rel="$2"
  local entry="$3"
  if git -C "$proj" ls-files --error-unmatch "$rel/$entry" >/dev/null 2>&1; then
    echo "yes"
  else
    echo "no"
  fi
}

# Back-compat: tracked check against .claude/skills/.
git_tracked() {
  git_tracked_at "$1" ".claude/skills" "$2"
}

# --- Migration logic for one project ---

migrate_project() {
  local project_path="$1"

  # 1. Already migrated?
  local claude_link="$project_path/.claude/skills"
  local agents_link="$project_path/.agents/skills"
  local canonical="$project_path/skills"

  local claude_real agents_real canonical_real
  claude_real="$(real_path "$claude_link")"
  agents_real="$(real_path "$agents_link")"
  canonical_real="$(real_path "$canonical")"

  if [ -L "$claude_link" ] && [ -L "$agents_link" ] \
     && [ -n "$canonical_real" ] \
     && [ "$claude_real" = "$canonical_real" ] \
     && [ "$agents_real" = "$canonical_real" ]; then
    echo "  already migrated — no changes"
    return 0
  fi

  # 2. Sanity: canonical mustn't be a symlink to elsewhere.
  if [ -L "$canonical" ]; then
    echo "  REFUSE: <project>/skills is itself a symlink — manual review required"
    return 1
  fi

  # 3. Survey what's under .claude/skills and .agents/skills.
  local claude_dir_is_real=no
  local agents_dir_is_real=no
  if [ -d "$claude_link" ] && [ ! -L "$claude_link" ]; then claude_dir_is_real=yes; fi
  if [ -d "$agents_link" ] && [ ! -L "$agents_link" ]; then agents_dir_is_real=yes; fi

  if [ "$claude_dir_is_real" = "no" ] && [ "$agents_dir_is_real" = "no" ]; then
    # Neither side has real content. Just create the canonical dir + symlinks.
    if [ "$DRY_RUN" = true ]; then
      echo "  Would create skills/, link .claude/skills and .agents/skills to it"
      return 0
    fi
    mkdir -p "$canonical"
    rm -f "$claude_link"  # in case it's a broken symlink
    mkdir -p "$project_path/.claude"
    ( cd "$project_path/.claude" && ln -s ../skills skills )
    rm -f "$agents_link"
    mkdir -p "$project_path/.agents"
    ( cd "$project_path/.agents" && ln -s ../skills skills )
    echo "  fresh layout created (no entries to move)"
    return 0
  fi

  # 4. Conflict detection: any entry name appearing in BOTH .claude/skills AND
  # canonical skills/ that isn't trivially the same content. We keep this
  # simple: refuse if the destination already has a non-AEP entry the source
  # also has, or any conflicting AEP entry.
  local conflicts=()
  if [ -d "$canonical" ]; then
    if [ "$claude_dir_is_real" = "yes" ]; then
      while IFS= read -r entry; do
        [ -z "$entry" ] && continue
        local dst="$canonical/$entry"
        local src="$claude_link/$entry"
        if [ -e "$dst" ] || [ -L "$dst" ]; then
          # Same target after resolution? If the existing canonical entry
          # already equals the source, that's not a conflict (already moved).
          local d_real s_real
          d_real="$(real_path "$dst")"
          s_real="$(real_path "$src")"
          if [ -n "$d_real" ] && [ "$d_real" = "$s_real" ]; then
            continue
          fi
          conflicts+=("$entry")
        fi
      done < <(ls -A "$claude_link" 2>/dev/null)
    fi
  fi

  if [ "${#conflicts[@]}" -gt 0 ]; then
    echo "  REFUSE: conflicting entries already exist under skills/:"
    for c in "${conflicts[@]}"; do
      echo "    - $c"
    done
    echo "  Resolve manually (move or delete the conflicting skills/ entries) and re-run."
    return 1
  fi

  # 5. Sanity check .agents/skills: every entry must either be an aep-*
  # duplicate (we'll diff-confirm before dropping it) or be unique to the
  # codex side (we'll move it into skills/ alongside the others). If an
  # entry exists in both .claude/skills/ AND .agents/skills/ but their
  # content differs, we cannot safely choose a winner — refuse.
  if [ "$agents_dir_is_real" = "yes" ] && [ "$claude_dir_is_real" = "yes" ]; then
    local divergent=()
    while IFS= read -r entry; do
      [ -z "$entry" ] && continue
      local a_src="$agents_link/$entry"
      local c_src="$claude_link/$entry"
      if [ -e "$c_src" ] || [ -L "$c_src" ]; then
        # Entry exists in both. Confirm same content.
        if ! diff -rq "$c_src" "$a_src" >/dev/null 2>&1; then
          divergent+=("$entry")
        fi
      fi
    done < <(ls -A "$agents_link" 2>/dev/null)

    if [ "${#divergent[@]}" -gt 0 ]; then
      echo "  REFUSE: .claude/skills/ and .agents/skills/ disagree on these entries:"
      for d in "${divergent[@]}"; do
        echo "    - $d (different content on each side)"
      done
      echo "  Resolve manually, then re-run."
      return 1
    fi
  fi

  # 6. Plan and (optionally) execute the moves.
  if [ "$DRY_RUN" = true ]; then
    echo "  Would create $canonical/"
    # Track which entry names will end up at skills/ via the .claude pass,
    # so the .agents pass can preview drop-vs-move accurately.
    local will_be_at_skills=" "
    if [ "$claude_dir_is_real" = "yes" ]; then
      while IFS= read -r entry; do
        [ -z "$entry" ] && continue
        local src="$claude_link/$entry"
        if [ -L "$src" ]; then
          local target_abs
          target_abs="$(cd "$src" 2>/dev/null && pwd -P || echo '')"
          if [[ "$target_abs" == "$project_path/.agents/skills/"* ]] \
             || [[ "$target_abs" == "$project_path/skills/"* ]]; then
            echo "    Would remove redundant symlink .claude/skills/$entry → $target_abs"
            # If target is in skills/ (e.g. ai-creator-structured-prompt) the
            # entry already resides at canonical, so mark it.
            if [[ "$target_abs" == "$project_path/skills/$entry" ]]; then
              will_be_at_skills="$will_be_at_skills$entry "
            fi
            continue
          fi
          echo "    Would move .claude/skills/$entry → skills/$entry (symlink with external target)"
          will_be_at_skills="$will_be_at_skills$entry "
          continue
        fi
        local tracked
        tracked="$(git_tracked "$project_path" "$entry")"
        echo "    Would move .claude/skills/$entry → skills/$entry (real-dir, tracked=$tracked)"
        will_be_at_skills="$will_be_at_skills$entry "
      done < <(ls -A "$claude_link" 2>/dev/null)
    fi
    if [ "$agents_dir_is_real" = "yes" ]; then
      while IFS= read -r entry; do
        [ -z "$entry" ] && continue
        if [[ "$will_be_at_skills" == *" $entry "* ]]; then
          echo "    Would drop duplicate .agents/skills/$entry (covered by skills/$entry)"
        else
          echo "    Would move .agents/skills/$entry → skills/$entry (codex-only or canonical home)"
        fi
      done < <(ls -A "$agents_link" 2>/dev/null)
    fi
    echo "  Would replace .claude/skills with symlink → ../skills"
    echo "  Would replace .agents/skills with symlink → ../skills"
    return 0
  fi

  # --- Real execution ---

  mkdir -p "$canonical"

  # 6a. Move every entry from .claude/skills/ to skills/.
  #
  # Special handling: a `.claude/skills/X` that's a symlink into either
  # `.agents/skills/X` or `skills/X` is just a discovery link — the real
  # content lives elsewhere. Moving the symlink to `skills/X` would break
  # its relative target, so instead we just remove the symlink. The real
  # content gets handled (moved or already-canonical) in its own loop pass.
  if [ "$claude_dir_is_real" = "yes" ]; then
    while IFS= read -r entry; do
      [ -z "$entry" ] && continue
      local src="$claude_link/$entry"
      local dst="$canonical/$entry"
      local tracked
      tracked="$(git_tracked "$project_path" "$entry")"

      if [ -L "$src" ]; then
        # Resolve the symlink target to an absolute path.
        local target_abs
        target_abs="$(cd "$src" 2>/dev/null && pwd -P || echo '')"
        if [[ "$target_abs" == "$project_path/.agents/skills/"* ]] \
           || [[ "$target_abs" == "$project_path/skills/"* ]]; then
          # Redundant link — remove it. Real content stays put (or will be
          # promoted from .agents/skills/ in step 6b).
          if [ "$tracked" = "yes" ]; then
            ( cd "$project_path" && git rm -q ".claude/skills/$entry" )
          else
            rm -f "$src"
          fi
          echo "    removed redundant symlink (claude): $entry → $target_abs"
          continue
        fi
        # Symlink points outside the project (e.g. to ~/.dotfiles/...).
        # Move it with `mv` — bash's mv preserves the symlink itself.
        # Caller is responsible for ensuring the target is reachable from
        # the new location (absolute targets always are; relative ones may
        # break, but those would've been broken pre-migration too if the
        # relative path didn't account for nesting).
        if [ "$tracked" = "yes" ]; then
          ( cd "$project_path" && git mv ".claude/skills/$entry" "skills/$entry" )
        else
          mv "$src" "$dst"
        fi
        echo "    moved (claude symlink): $entry"
        continue
      fi

      # Real dir. Skip if already at destination (idempotent).
      local d_real s_real
      d_real="$(real_path "$dst")"
      s_real="$(real_path "$src")"
      if [ -n "$d_real" ] && [ "$d_real" = "$s_real" ]; then
        continue
      fi

      if [ "$tracked" = "yes" ]; then
        ( cd "$project_path" && git mv ".claude/skills/$entry" "skills/$entry" )
      else
        mv "$src" "$dst"
      fi
      echo "    moved (claude): $entry"
    done < <(ls -A "$claude_link" 2>/dev/null)
  fi

  # 6b. Handle .agents/skills/ residue.
  # Each entry is either a confirmed-identical duplicate of one already in
  # skills/ (drop), or unique to the codex side (move into skills/).
  # Divergent entries were caught earlier in step 5 and refused.
  if [ "$agents_dir_is_real" = "yes" ]; then
    while IFS= read -r entry; do
      [ -z "$entry" ] && continue
      local a_src="$agents_link/$entry"
      local dst="$canonical/$entry"
      local a_tracked
      a_tracked="$(git_tracked_at "$project_path" ".agents/skills" "$entry")"

      if [ -e "$dst" ] || [ -L "$dst" ]; then
        # Confirmed identical earlier — drop.
        if [ "$a_tracked" = "yes" ]; then
          ( cd "$project_path" && git rm -rqf ".agents/skills/$entry" )
        else
          rm -rf "$a_src"
        fi
        echo "    dropped (codex duplicate): $entry"
      else
        # Codex-only (or codex was the canonical home, e.g. dotfile-style
        # skills surfaced via a now-removed claude-side symlink).
        if [ "$a_tracked" = "yes" ]; then
          ( cd "$project_path" && git mv ".agents/skills/$entry" "skills/$entry" )
        else
          mv "$a_src" "$dst"
        fi
        echo "    moved (codex-only): $entry"
      fi
    done < <(ls -A "$agents_link" 2>/dev/null)
  fi

  # Replace .claude/skills (now empty real dir) with a symlink.
  if [ -d "$claude_link" ] && [ ! -L "$claude_link" ]; then
    rmdir "$claude_link" 2>/dev/null || rm -rf "$claude_link"
  fi
  mkdir -p "$project_path/.claude"
  ( cd "$project_path/.claude" && ln -s ../skills skills )

  # Replace .agents/skills with a symlink.
  if [ -e "$agents_link" ] || [ -L "$agents_link" ]; then
    rm -rf "$agents_link"
  fi
  mkdir -p "$project_path/.agents"
  ( cd "$project_path/.agents" && ln -s ../skills skills )

  echo "  migrated"
}

# --- Read version (cosmetic) ---

VERSION=""
MARKETPLACE="$AEP_REPO/.claude-plugin/marketplace.json"
if [ -f "$MARKETPLACE" ]; then
  VERSION=$(grep -o '"version"[[:space:]]*:[[:space:]]*"[^"]*"' "$MARKETPLACE" | head -1 | grep -o '"[^"]*"$' | tr -d '"')
fi

echo "AEP Migrate Downstream Layout${VERSION:+ (v$VERSION)}"
if [ -n "$NAME_FILTER" ]; then
  echo "  filter: *${NAME_FILTER}*"
fi
if [ "$DRY_RUN" = true ]; then
  echo "  mode: dry-run"
fi
echo ""

# --- Iterate downstreams ---

TOTAL=0
MIGRATED=0
SKIPPED=0
REFUSED=0
FAILED=0

while IFS='|' read -r name path; do
  if [ -n "$NAME_FILTER" ]; then
    [[ ! "$name" == *"$NAME_FILTER"* ]] && continue
  fi

  TOTAL=$((TOTAL + 1))

  path="${path/#\~/$HOME}"
  path="$(echo "$path" | xargs)"
  name="$(echo "$name" | xargs)"

  echo "━━━ $name ━━━"
  echo "  path: $path"

  if [ ! -d "$path" ]; then
    echo "  ERROR: directory not found"
    FAILED=$((FAILED + 1))
    echo ""
    continue
  fi

  # Pre-flight: git repo, on main, clean tree.
  if ! git -C "$path" rev-parse --git-dir >/dev/null 2>&1; then
    echo "  SKIP — not a git repository"
    SKIPPED=$((SKIPPED + 1))
    echo ""
    continue
  fi
  current_branch="$(git -C "$path" symbolic-ref --short HEAD 2>/dev/null || echo '')"
  if [ -z "$current_branch" ]; then
    echo "  SKIP — detached HEAD, expected branch 'main'"
    SKIPPED=$((SKIPPED + 1))
    echo ""
    continue
  fi
  if [ "$current_branch" != "main" ]; then
    echo "  SKIP — on '$current_branch', expected 'main'"
    SKIPPED=$((SKIPPED + 1))
    echo ""
    continue
  fi
  if [ -n "$(git -C "$path" status --porcelain 2>/dev/null)" ]; then
    echo "  SKIP — working tree has uncommitted changes (commit Round 1 sync first)"
    SKIPPED=$((SKIPPED + 1))
    echo ""
    continue
  fi

  if migrate_project "$path"; then
    MIGRATED=$((MIGRATED + 1))
  else
    REFUSED=$((REFUSED + 1))
  fi

  echo ""
done < <(parse_entries)

# --- Summary ---

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
if [ "$TOTAL" -eq 0 ]; then
  if [ -n "$NAME_FILTER" ]; then
    echo "No projects matched '$NAME_FILTER'."
  else
    echo "No downstream projects configured."
  fi
  exit 1
fi

if [ "$DRY_RUN" = true ]; then
  echo "Dry run complete: $TOTAL projects checked"
else
  echo "Migration complete: $MIGRATED/$TOTAL projects migrated"
fi
if [ "$SKIPPED" -gt 0 ]; then echo "  $SKIPPED skipped (pre-flight)"; fi
if [ "$REFUSED" -gt 0 ]; then echo "  $REFUSED refused (conflicts — resolve manually)"; fi
if [ "$FAILED"  -gt 0 ]; then echo "  $FAILED failed (see errors above)"; fi
exit 0
