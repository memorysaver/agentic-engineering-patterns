#!/usr/bin/env bash
# Build self-contained AEP skills
#
# Some skills share resources. To stay portable under the open Agent Skills
# format (https://skills.sh) — where the `skills` CLI installs each skill
# directory independently (symlink or copy) — every skill must be
# self-contained. This script materializes the canonical shared resources in
# skills/product-context/_shared/ into each skill that references them, so the
# committed tree has no cross-skill file dependencies.
#
# Authoring model (per-file materialization):
#   - Edit shared resources ONCE in skills/product-context/_shared/{references,templates}/
#   - Run this script to materialize copies into each consuming skill
#   - The .aep-generated marker in a consumer dir LISTS the managed files —
#     only those are ever removed/overwritten. Any other file in the same dir
#     is skill-owned and never touched, so authored and generated files coexist.
#
# Selection rules:
#   - references: a skill receives exactly the _shared/references/ files whose
#     basename its SKILL.md mentions as "references/<name>".
#   - templates: a skill that mentions "templates/" receives the full shared
#     template kit (templates cross-reference each other, so they ship whole).
#
# Usage:
#   bash scripts/build-skills.sh            # materialize (idempotent)
#   bash scripts/build-skills.sh --check    # verify in sync; non-zero if stale
#   bash scripts/build-skills.sh --help

set -euo pipefail

MARKER=".aep-generated"
MARKER_HEADER="# Managed by scripts/build-skills.sh — the files listed below are copies of skills/product-context/_shared/. Edit _shared/ and rebuild; every other file in this directory is skill-owned and never touched."

show_help() {
  sed -n '2,28p' "$0" | sed 's/^# \{0,1\}//'
}

# --- Resolve repo + paths ---

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
REPO="$(dirname "$SCRIPT_DIR")"
PC="$REPO/skills/product-context"
SHARED="$PC/_shared"

CHECK=false
for arg in "$@"; do
  case "$arg" in
    --check) CHECK=true ;;
    --help|-h) show_help; exit 0 ;;
    *) echo "Unknown argument: $arg (try --help)"; exit 1 ;;
  esac
done

if [ ! -d "$SHARED" ]; then
  echo "ERROR: shared source not found: $SHARED"
  exit 1
fi

# Desired managed files (relative paths within the shared dir) for one consumer.
desired_files() {
  local skillmd="$1" shared="$2"
  if [ "$shared" = "templates" ]; then
    if grep -qE '(^|[^A-Za-z0-9_/])templates/' "$skillmd"; then
      (cd "$SHARED/templates" && find . -type f ! -name "$MARKER" | sed 's|^\./||' | sort)
    fi
    return 0
  fi
  local f bn
  for f in "$SHARED/references"/*; do
    [ -f "$f" ] || continue
    bn="$(basename "$f")"
    if grep -qF "references/$bn" "$skillmd"; then
      printf '%s\n' "$bn"
    fi
  done | sort
}

# Managed files recorded in an existing marker. Legacy dir-level markers (the
# pre-v3 format, whose first line is not MARKER_HEADER) list no files; for
# those, treat as managed every file in the dir that mirrors a _shared path —
# exactly the set the old dir-level build could have written.
recorded_files() {
  local dst="$1" shared="$2"
  [ -f "$dst/$MARKER" ] || return 0
  if [ "$(head -n1 "$dst/$MARKER")" = "$MARKER_HEADER" ]; then
    tail -n +2 "$dst/$MARKER"
    return 0
  fi
  # legacy marker
  (cd "$dst" && find . -type f ! -name "$MARKER" | sed 's|^\./||') | while IFS= read -r rel; do
    [ -f "$SHARED/$shared/$rel" ] && printf '%s\n' "$rel"
  done | sort
}

STALE=0
WROTE=0
CONFLICT=0

for skill_dir in "$PC"/*/; do
  name="$(basename "$skill_dir")"
  [ "$name" = "_shared" ] && continue
  skillmd="$skill_dir/SKILL.md"
  [ -f "$skillmd" ] || continue

  for shared in references templates; do
    src="$SHARED/$shared"
    [ -d "$src" ] || continue
    dst="$skill_dir$shared"

    desired="$(desired_files "$skillmd" "$shared")"
    recorded="$(recorded_files "$dst" "$shared")"
    legacy=false
    if [ -f "$dst/$MARKER" ] && [ "$(head -n1 "$dst/$MARKER")" != "$MARKER_HEADER" ]; then
      legacy=true
    fi

    [ -z "$desired" ] && [ -z "$recorded" ] && continue

    # Conflict: a desired managed file already exists but is skill-owned.
    while IFS= read -r rel; do
      [ -n "$rel" ] || continue
      if [ -f "$dst/$rel" ] && ! printf '%s\n' "$recorded" | grep -qxF "$rel"; then
        echo "CONFLICT: $name/$shared/$rel exists as a skill-owned file but _shared wants to manage that name"
        CONFLICT=$((CONFLICT + 1))
      fi
    done <<< "$desired"

    if [ "$CHECK" = true ]; then
      if [ "$legacy" = true ]; then
        echo "STALE: $name/$shared/ carries a legacy dir-level marker (rebuild to migrate)"
        STALE=$((STALE + 1))
      fi
      if [ "$desired" != "$recorded" ]; then
        echo "STALE: $name/$shared/ managed-file set is out of date"
        STALE=$((STALE + 1))
      else
        while IFS= read -r rel; do
          [ -n "$rel" ] || continue
          if [ ! -f "$dst/$rel" ] || ! diff -q "$src/$rel" "$dst/$rel" >/dev/null 2>&1; then
            echo "STALE: $name/$shared/$rel is missing or out of date"
            STALE=$((STALE + 1))
          fi
        done <<< "$desired"
      fi
      continue
    fi

    changed=false

    # Remove managed files no longer desired.
    while IFS= read -r rel; do
      [ -n "$rel" ] || continue
      if ! printf '%s\n' "$desired" | grep -qxF "$rel"; then
        rm -f "$dst/$rel"
        changed=true
      fi
    done <<< "$recorded"

    # Copy each desired file that is missing or differs.
    while IFS= read -r rel; do
      [ -n "$rel" ] || continue
      if [ ! -f "$dst/$rel" ] || ! diff -q "$src/$rel" "$dst/$rel" >/dev/null 2>&1; then
        mkdir -p "$(dirname "$dst/$rel")"
        cp "$src/$rel" "$dst/$rel"
        changed=true
      fi
    done <<< "$desired"

    # Marker: manifest of managed files, or gone when nothing is managed.
    if [ -n "$desired" ]; then
      new_marker="$MARKER_HEADER
$desired"
      if [ ! -f "$dst/$MARKER" ] || [ "$(cat "$dst/$MARKER")" != "$new_marker" ]; then
        printf '%s\n' "$new_marker" > "$dst/$MARKER"
        changed=true
      fi
    elif [ -f "$dst/$MARKER" ]; then
      rm -f "$dst/$MARKER"
      changed=true
    fi

    # Prune dirs emptied by removals (keeps dirs that still hold owned files).
    [ -d "$dst" ] && find "$dst" -type d -empty -delete 2>/dev/null || true

    if [ "$changed" = true ]; then
      echo "  → $name/$shared/ (synced)"
      WROTE=$((WROTE + 1))
    fi
  done
done

if [ "$CONFLICT" -gt 0 ]; then
  echo ""
  echo "ERROR: $CONFLICT name conflict(s) between skill-owned files and _shared. Rename one side."
  exit 1
fi

if [ "$CHECK" = true ]; then
  if [ "$STALE" -gt 0 ]; then
    echo ""
    echo "ERROR: $STALE generated skill resource(s) are out of sync."
    echo "Run 'bash scripts/build-skills.sh' and commit the result."
    exit 1
  fi
  echo "build-skills: generated resources are in sync"
else
  echo "build-skills: done ($WROTE change(s))"
fi
