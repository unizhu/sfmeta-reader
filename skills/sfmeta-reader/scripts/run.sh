#!/usr/bin/env bash
# sfmeta-reader wrapper — detects OS/arch + runs the correct binary.
# Usage: ./run.sh [sfmeta-reader arguments...]
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BIN_DIR="${SCRIPT_DIR}/../bin"

# Detect OS
case "$(uname -s)" in
    Darwin*)  OS="darwin"  ;;
    Linux*)   OS="linux"   ;;
    MINGW*|MSYS*|CYGWIN*) OS="windows" ;;
    *)
        echo "Error: unsupported OS '$(uname -s)'" >&2
        exit 1
        ;;
esac

# Detect architecture
case "$(uname -m)" in
    x86_64|amd64)   ARCH="x86_64"  ;;
    arm64|aarch64)   ARCH="aarch64" ;;
    *)
        echo "Error: unsupported architecture '$(uname -m)'" >&2
        exit 1
        ;;
esac

# Build binary name
if [ "$OS" = "windows" ]; then
    BINARY="${BIN_DIR}/sfmeta-reader-${OS}-${ARCH}.exe"
else
    BINARY="${BIN_DIR}/sfmeta-reader-${OS}-${ARCH}"
fi

# Check binary exists
if [ ! -f "$BINARY" ]; then
    echo "Error: binary not found at ${BINARY}" >&2
    echo "Available binaries:" >&2
    ls -1 "$BIN_DIR" 2>/dev/null || echo "  (bin/ directory is empty or missing)" >&2
    echo "" >&2
    echo "Download the correct binary from the GitHub Releases page" >&2
    echo "and place it in: ${BIN_DIR}/" >&2
    exit 1
fi

# Ensure executable
chmod +x "$BINARY" 2>/dev/null || true

# Run
exec "$BINARY" "$@"
