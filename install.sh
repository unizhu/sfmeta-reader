#!/usr/bin/env bash
# sfmeta-reader skill installer
# Usage: curl -fsSL https://raw.githubusercontent.com/<OWNER>/sfmeta-reader/main/install.sh | bash
set -euo pipefail

REPO="unizhu/sfmeta-reader"
INSTALL_DIR="${SFMETA_INSTALL_DIR:-$HOME/.claude/skills/sfmeta-reader}"
API_LATEST_TAG="https://api.github.com/repos/${REPO}/releases/tags/latest"
API_LATEST="https://api.github.com/repos/${REPO}/releases/latest"

# ── Detect platform ────────────────────────────────────────────
case "$(uname -s)" in
  Darwin*)  OS="darwin"  ;;
  Linux*)   OS="linux"   ;;
  MINGW*|MSYS*|CYGWIN*) OS="windows" ;;
  *) echo "Error: unsupported OS '$(uname -s)'" >&2; exit 1 ;;
esac

case "$(uname -m)" in
  x86_64|amd64)  ARCH="x86_64"  ;;
  arm64|aarch64) ARCH="aarch64" ;;
  *) echo "Error: unsupported architecture '$(uname -m)'" >&2; exit 1 ;;
esac

if [ "$OS" = "windows" ]; then
  BINARY="sfmeta-reader-${OS}-${ARCH}.exe"
else
  BINARY="sfmeta-reader-${OS}-${ARCH}"
fi

echo "🔍  Detected platform: ${OS}/${ARCH}"
echo "📦  Binary: ${BINARY}"
echo "📂  Install directory: ${INSTALL_DIR}"
echo ""

# ── Resolve download URL from latest release ───────────────────
echo "🌐  Fetching latest release from ${REPO}..."

# Try rolling "latest" tag first (pre-release from CI), then fall back to latest stable
RELEASE_JSON=$(curl -fsSL "$API_LATEST_TAG" 2>/dev/null || true)
if [ -z "$RELEASE_JSON" ] || echo "$RELEASE_JSON" | grep -q '"message"'; then
  RELEASE_JSON=$(curl -fsSL "$API_LATEST")
fi
DOWNLOAD_URL=$(echo "$RELEASE_JSON" | grep -o "\"browser_download_url\":[[:space:]]*\"[^\"]*${BINARY}\"" | head -1 | cut -d'"' -f4)
TAG=$(echo "$RELEASE_JSON" | grep -o '"tag_name":[[:space:]]*"[^"]*"' | head -1 | cut -d'"' -f4)

if [ -z "$DOWNLOAD_URL" ]; then
  echo "Error: could not find binary '${BINARY}' in the latest release." >&2
  echo "Available assets:" >&2
  echo "$RELEASE_JSON" | grep -o '"name":[[:space:]]*"sfmeta-reader-[^"]*"' | cut -d'"' -f4 >&2
  exit 1
fi

echo "📥  Downloading ${TAG} → ${BINARY}..."

# ── Create skill directory structure ────────────────────────────
mkdir -p "${INSTALL_DIR}/bin"
mkdir -p "${INSTALL_DIR}/scripts"
mkdir -p "${INSTALL_DIR}/resources"

# ── Download binary ─────────────────────────────────────────────
curl -fsSL -o "${INSTALL_DIR}/bin/${BINARY}" "$DOWNLOAD_URL"
chmod +x "${INSTALL_DIR}/bin/${BINARY}"

# ── Download skill files from main branch ───────────────────────
RAW="https://raw.githubusercontent.com/${REPO}/main"
curl -fsSL -o "${INSTALL_DIR}/SKILL.md"               "${RAW}/skills/sfmeta-reader/SKILL.md"
curl -fsSL -o "${INSTALL_DIR}/scripts/run.sh"          "${RAW}/skills/sfmeta-reader/scripts/run.sh"
curl -fsSL -o "${INSTALL_DIR}/scripts/run.ps1"         "${RAW}/skills/sfmeta-reader/scripts/run.ps1"
curl -fsSL -o "${INSTALL_DIR}/resources/reference.md"  "${RAW}/skills/sfmeta-reader/resources/reference.md"
chmod +x "${INSTALL_DIR}/scripts/run.sh"

echo ""
echo "✅  Installed sfmeta-reader ${TAG} to ${INSTALL_DIR}"
echo ""
echo "   Skill:   ${INSTALL_DIR}/SKILL.md"
echo "   Binary:  ${INSTALL_DIR}/bin/${BINARY}"
echo "   Runner:  ${INSTALL_DIR}/scripts/run.sh"
echo ""
echo "🚀  Ready! LLM agents will auto-discover the skill from ${INSTALL_DIR}"
