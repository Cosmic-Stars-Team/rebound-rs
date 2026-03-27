#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
BIND_DIR="$ROOT_DIR/bind/rebound"
TARGET_DIR="$BIND_DIR/c_src"
VERSION_FILE="$TARGET_DIR/.version"
WORKSPACE_CARGO_TOML="$ROOT_DIR/Cargo.toml"

REBOUND_VERSION="4.6.0"
FORCE=false

usage() {
  cat <<'EOF'
Usage:
  scripts/vendor.sh [--force] [--version <x.y.z>]

Options:
  --force             Force re-vendor even when local version already matches.
  --version <x.y.z>   Rebound tag to vendor (also syncs Cargo workspace version).
  -h, --help          Show this message.

Result in bind/rebound/c_src:
  - src/
  - LICENSE
EOF
}

require_cmd() {
  local cmd="$1"
  if ! command -v "$cmd" >/dev/null 2>&1; then
    echo "Error: required command not found: $cmd" >&2
    exit 1
  fi
}

sync_workspace_version() {
  local version="$1"
  local tmp_file
  tmp_file="$(mktemp)"

  awk -v ver="$version" '
    BEGIN { in_workspace_pkg = 0; changed = 0 }
    /^\[workspace\.package\]/ { in_workspace_pkg = 1; print; next }
    /^\[/ { in_workspace_pkg = 0 }
    {
      if (in_workspace_pkg && $0 ~ /^version[[:space:]]*=/) {
        print "version = \"" ver "\""
        changed = 1
        next
      }
      print
    }
    END {
      if (!changed) {
        exit 2
      }
    }
  ' "$WORKSPACE_CARGO_TOML" > "$tmp_file" || {
    rm -f "$tmp_file"
    echo "Error: failed to update [workspace.package].version in $WORKSPACE_CARGO_TOML" >&2
    exit 1
  }

  mv "$tmp_file" "$WORKSPACE_CARGO_TOML"
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --force)
      FORCE=true
      shift
      ;;
    --version)
      if [[ $# -lt 2 ]]; then
        echo "Error: --version requires a value" >&2
        usage
        exit 1
      fi
      REBOUND_VERSION="$2"
      shift 2
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "Error: unknown option: $1" >&2
      usage
      exit 1
      ;;
  esac
done

require_cmd curl
require_cmd tar
require_cmd awk

echo "Rebound version: $REBOUND_VERSION"
echo "Syncing workspace version in Cargo.toml..."
sync_workspace_version "$REBOUND_VERSION"

CURRENT_LOCAL_VERSION="none"
if [[ -f "$VERSION_FILE" ]]; then
  CURRENT_LOCAL_VERSION="$(cat "$VERSION_FILE")"
fi

if [[ "$FORCE" == false && -d "$TARGET_DIR/src" && -f "$TARGET_DIR/LICENSE" && "$REBOUND_VERSION" == "$CURRENT_LOCAL_VERSION" ]]; then
  echo "Local vendor already at v$REBOUND_VERSION, skipping (use --force to refresh)."
  exit 0
fi

URL="https://github.com/hannorein/rebound/archive/refs/tags/$REBOUND_VERSION.tar.gz"
echo "Downloading: $URL"

TEMP_TAR="$(mktemp)"
cleanup() {
  rm -f "$TEMP_TAR"
}
trap cleanup EXIT

curl -L -f "$URL" -o "$TEMP_TAR"

echo "Refreshing vendored source: $TARGET_DIR"
mkdir -p "$(dirname "$TARGET_DIR")"
rm -rf "$TARGET_DIR"
mkdir -p "$TARGET_DIR"

ARCHIVE_ROOT="rebound-$REBOUND_VERSION"
tar -xzf "$TEMP_TAR" --strip-components=1 -C "$TARGET_DIR" \
  "$ARCHIVE_ROOT/src" \
  "$ARCHIVE_ROOT/LICENSE"

echo "$REBOUND_VERSION" > "$VERSION_FILE"

echo "Vendor sync complete."
