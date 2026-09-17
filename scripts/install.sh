#!/usr/bin/env bash
# Install the aep CLI from a GitHub release archive.
#
#   curl -fsSL https://raw.githubusercontent.com/memorysaver/agentic-engineering-patterns/main/scripts/install.sh | bash
#
# Environment:
#   AEP_VERSION   Release tag to install (default: newest v5 release, prereleases included).
#   AEP_BASE_URL  Directory URL holding aep-<target>.tar.gz and its .sha256 (default: the tag's GitHub release assets).
#   AEP_HOME      Install root (default: ~/.local/share/aep). Builds live under $AEP_HOME/builds/<sha16>/bin/aep.
#   AEP_BIN_DIR   Directory for the `aep` link (default: ~/.local/bin).
#
# Targets: Linux x86_64 (x86_64-unknown-linux-gnu) and macOS Apple Silicon (aarch64-apple-darwin).
set -euo pipefail

REPO="memorysaver/agentic-engineering-patterns"
AEP_HOME="${AEP_HOME:-$HOME/.local/share/aep}"
AEP_BIN_DIR="${AEP_BIN_DIR:-$HOME/.local/bin}"

log() { printf 'aep-install: %s\n' "$*" >&2; }
fail() { log "error: $*"; exit 1; }

need() { command -v "$1" >/dev/null 2>&1 || fail "$1 is required"; }
need curl
need tar
need git

sha256_of() {
  if command -v sha256sum >/dev/null 2>&1; then sha256sum "$1" | cut -d' ' -f1
  elif command -v shasum >/dev/null 2>&1; then shasum -a 256 "$1" | cut -d' ' -f1
  else fail "sha256sum or shasum is required"; fi
}

os="$(uname -s)"; arch="$(uname -m)"
case "$os/$arch" in
  Linux/x86_64) target="x86_64-unknown-linux-gnu" ;;
  Darwin/arm64) target="aarch64-apple-darwin" ;;
  Darwin/x86_64) fail "macOS Intel is not a supported target; build from source with cargo" ;;
  *) fail "unsupported platform $os/$arch" ;;
esac

version="${AEP_VERSION:-}"
if [[ -z "$version" ]]; then
  # /releases/latest excludes prereleases and may point at the legacy v4 line; pick the newest v5 tag instead.
  version="$(curl -fsSL "https://api.github.com/repos/$REPO/releases?per_page=30" \
    | grep -o '"tag_name": *"v5[^"]*"' | head -n1 | sed 's/.*"\(v5[^"]*\)"/\1/')" || true
  [[ -n "$version" ]] || fail "no v5 release found; set AEP_VERSION or build from source"
fi
base="${AEP_BASE_URL:-https://github.com/$REPO/releases/download/$version}"
archive="aep-$target.tar.gz"

tmp="$(mktemp -d)"; trap 'rm -rf "$tmp"' EXIT
log "downloading $version for $target"
curl -fsSL -o "$tmp/$archive" "$base/$archive" || fail "download failed: $base/$archive"
curl -fsSL -o "$tmp/$archive.sha256" "$base/$archive.sha256" || fail "checksum download failed: $base/$archive.sha256"
expected="$(cut -d' ' -f1 "$tmp/$archive.sha256")"
actual="$(sha256_of "$tmp/$archive")"
[[ "$expected" == "$actual" ]] || fail "checksum mismatch for $archive (expected $expected, got $actual)"

tar -xzf "$tmp/$archive" -C "$tmp"
[[ -f "$tmp/aep" ]] || fail "archive did not contain aep"
chmod 755 "$tmp/aep"
"$tmp/aep" --version >/dev/null || fail "downloaded binary does not run on this system"

binary_sha="$(sha256_of "$tmp/aep")"
build_dir="$AEP_HOME/builds/${binary_sha:0:16}"
mkdir -p "$build_dir/bin" "$AEP_BIN_DIR"
cp "$tmp/aep" "$build_dir/bin/aep"
[[ -f "$tmp/LICENSE" ]] && cp "$tmp/LICENSE" "$build_dir/LICENSE"
previous="$(readlink "$AEP_BIN_DIR/aep" 2>/dev/null || true)"
printf '{\n  "binary": "%s",\n  "sha256": "%s",\n  "version_tag": "%s",\n  "target": "%s",\n  "archive_sha256": "%s",\n  "source": "%s",\n  "installed_at": "%s",\n  "previous_binary": "%s"\n}\n' \
  "$build_dir/bin/aep" "$binary_sha" "$version" "$target" "$actual" "$base/$archive" \
  "$(date -u +%Y-%m-%dT%H:%M:%SZ)" "$previous" > "$build_dir/manifest.json"

# Switch the link atomically so a running agent sees either the old or the new binary.
ln -sfn "$build_dir/bin/aep" "$AEP_BIN_DIR/aep.new"
mv -f "$AEP_BIN_DIR/aep.new" "$AEP_BIN_DIR/aep"

log "installed $("$AEP_BIN_DIR/aep" --version) at $AEP_BIN_DIR/aep -> $build_dir/bin/aep"
case ":$PATH:" in
  *":$AEP_BIN_DIR:"*) ;;
  *) log "add $AEP_BIN_DIR to PATH, for example: export PATH=\"$AEP_BIN_DIR:\$PATH\"" ;;
esac
log "next: cd <project> && aep init && aep doctor"
