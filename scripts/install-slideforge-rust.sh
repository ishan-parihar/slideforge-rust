#!/usr/bin/env bash
#
# SlideForge Rust — Installer
#
# Usage:
#   curl -fsSL https://raw.githubusercontent.com/ishan-parihar/slideforge-rust/master/scripts/install-slideforge-rust.sh | bash
#
# Or with options:
#   curl -fsSL https://raw.githubusercontent.com/ishan-parihar/slideforge-rust/master/scripts/install-slideforge-rust.sh | bash -s -- --bin-dir /usr/local/bin
#
set -euo pipefail

REPO="ishan-parihar/slideforge-rust"
VERSION="v0.2.0"
BIN_DIR="${HOME}/.local/bin"
SKILL_DIR="${HOME}/.agents/skills"
INSTALL_NAME="slideforge"
LOCAL_BINARY=""
SKIP_SKILL="false"

usage() {
  cat <<'USAGE'
Install SlideForge Rust CLI/MCP server + AI agent skill.

Usage:
  install-slideforge-rust.sh [OPTIONS]

Options:
  --bin-dir DIR     Install directory (default: ~/.local/bin)
  --version VER     Release version to download (default: v0.2.0)
  --local PATH      Install from a local binary instead of downloading
  --skip-skill      Skip installing the AI agent skill to ~/.agents/skills/
  --help            Show this help

Examples:
  # One-liner install (binary + skill)
  curl -fsSL https://raw.githubusercontent.com/ishan-parihar/slideforge-rust/master/scripts/install-slideforge-rust.sh | bash

  # Install to /usr/local/bin (needs sudo), skip skill
  curl -fsSL https://raw.githubusercontent.com/ishan-parihar/slideforge-rust/master/scripts/install-slideforge-rust.sh | sudo bash -s -- --bin-dir /usr/local/bin --skip-skill

  # Install a specific version
  curl -fsSL https://raw.githubusercontent.com/ishan-parihar/slideforge-rust/master/scripts/install-slideforge-rust.sh | bash -s -- --version v0.1.0
USAGE
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --bin-dir)
      BIN_DIR="${2:-}"
      shift 2
      ;;
    --version)
      VERSION="${2:-}"
      shift 2
      ;;
    --local)
      LOCAL_BINARY="${2:-}"
      shift 2
      ;;
    --skip-skill)
      SKIP_SKILL="true"
      shift
      ;;
    --help|-h)
      usage
      exit 0
      ;;
    *)
      echo "error: unknown argument: $1" >&2
      usage >&2
      exit 1
      ;;
  esac
done

# If no local binary, download from GitHub releases
if [[ -z "$LOCAL_BINARY" ]]; then
  # Detect OS and architecture
  OS="$(uname -s)"
  ARCH="$(uname -m)"

  # Map to release asset name
  case "$OS" in
    Linux|linux)
      case "$ARCH" in
        x86_64|amd64)
          ASSET="slideforge-x86_64-linux-gnu"
          ;;
        *)
          echo "error: unsupported architecture: $ARCH" >&2
          echo "  Only x86_64 Linux is currently available." >&2
          echo "  Build from source: https://github.com/$REPO#build-from-source" >&2
          exit 1
          ;;
      esac
      ;;
    Darwin|macos)
      echo "error: macOS builds are not yet available." >&2
      echo "  Build from source: https://github.com/$REPO#build-from-source" >&2
      exit 1
      ;;
    *)
      echo "error: unsupported OS: $OS" >&2
      exit 1
      ;;
  esac

  DOWNLOAD_URL="https://github.com/$REPO/releases/download/$VERSION/$ASSET"
  echo "Downloading SlideForge $VERSION for $OS/$ARCH..."
  tmp="$(mktemp)"

  # Use curl with -L to follow redirects (GitHub releases redirect to CDN)
  if command -v curl >/dev/null 2>&1; then
    if ! curl -fsSL "$DOWNLOAD_URL" -o "$tmp"; then
      echo "error: failed to download binary from $DOWNLOAD_URL" >&2
      rm -f "$tmp"
      exit 1
    fi
  elif command -v wget >/dev/null 2>&1; then
    if ! wget -qO "$tmp" "$DOWNLOAD_URL"; then
      echo "error: failed to download binary from $DOWNLOAD_URL" >&2
      rm -f "$tmp"
      exit 1
    fi
  else
    echo "error: curl or wget is required" >&2
    exit 1
  fi

  # Verify the download is actually a binary (not an HTML error page)
  if [[ $(wc -c < "$tmp") -lt 1000000 ]]; then
    echo "error: downloaded file is too small — likely an error page" >&2
    echo "  URL: $DOWNLOAD_URL" >&2
    echo "  Size: $(wc -c < "$tmp") bytes" >&2
    rm -f "$tmp"
    exit 1
  fi

  # Check file type — should be an ELF binary, not HTML/text
  file_type="$(file "$tmp" 2>/dev/null || echo "unknown")"
  if echo "$file_type" | grep -qi "html\|text\|ascii"; then
    echo "error: downloaded file is not a binary — likely an error page" >&2
    echo "  File type: $file_type" >&2
    rm -f "$tmp"
    exit 1
  fi

  LOCAL_BINARY="$tmp"
fi

# Verify the local binary exists
if [[ ! -f "$LOCAL_BINARY" ]]; then
  echo "error: binary not found: $LOCAL_BINARY" >&2
  exit 1
fi

# Install binary
mkdir -p "$BIN_DIR"
install_path="${BIN_DIR}/${INSTALL_NAME}"
cp "$LOCAL_BINARY" "$install_path"
chmod +x "$install_path"

# Clean up temp file
if [[ -n "${tmp:-}" ]] && [[ -f "$tmp" ]]; then
  rm -f "$tmp"
fi

# Verify the binary works — try --version, fall back to --help, then list-slides
verify_ok="false"
if "$install_path" --version >/dev/null 2>&1; then
  verify_ok="true"
elif "$install_path" --help >/dev/null 2>&1; then
  verify_ok="true"
elif "$install_path" list-slides >/dev/null 2>&1; then
  verify_ok="true"
fi

if [[ "$verify_ok" != "true" ]]; then
  echo "error: binary verification failed — the binary may be incompatible with this system" >&2
  echo "  Binary path: $install_path" >&2
  echo "  Try running it manually: $install_path --version" >&2
  echo "  If it fails with 'cannot execute binary file', you may need to build from source:" >&2
  echo "    https://github.com/$REPO#build-from-source" >&2
  exit 1
fi

echo ""
echo "✓ SlideForge binary installed to: $install_path"
echo ""

# Check if BIN_DIR is in PATH
if [[ ":$PATH:" != *":$BIN_DIR:"* ]]; then
  echo "⚠ $BIN_DIR is not in your PATH. Add it:"
  echo "  echo 'export PATH=\"$BIN_DIR:\$PATH\"' >> ~/.bashrc"
  echo "  source ~/.bashrc"
  echo ""
fi

# Install the AI agent skill
if [[ "$SKIP_SKILL" != "true" ]]; then
  # Resolve HOME if not set (can happen in some CI/sudo environments)
  if [[ -z "${HOME:-}" ]]; then
    HOME="$(eval echo ~$(whoami))"
  fi
  SKILL_DIR="${HOME}/.agents/skills"

  echo "Installing SlideForge AI agent skill to ${SKILL_DIR}/slideforge/..."

  # Download the SKILL.md from the repo
  SKILL_URL="https://raw.githubusercontent.com/$REPO/master/skill/slideforge/SKILL.md"
  mkdir -p "${SKILL_DIR}/slideforge"

  skill_ok="false"
  if command -v curl >/dev/null 2>&1; then
    if curl -fsSL "$SKILL_URL" -o "${SKILL_DIR}/slideforge/SKILL.md"; then
      skill_ok="true"
    fi
  elif command -v wget >/dev/null 2>&1; then
    if wget -qO "${SKILL_DIR}/slideforge/SKILL.md" "$SKILL_URL"; then
      skill_ok="true"
    fi
  fi

  if [[ "$skill_ok" == "true" ]]; then
    echo "✓ Skill installed to: ${SKILL_DIR}/slideforge/SKILL.md"
  else
    echo "⚠ Could not download skill file (non-fatal) — the CLI/MCP tool still works"
    echo "  Manual download: curl -fsSL $SKILL_URL -o ~/.agents/skills/slideforge/SKILL.md"
  fi
  echo ""
fi

echo "Quick start:"
echo "  slideforge --help              # See all 18 CLI commands"
echo "  slideforge list-slides         # List 47 slide types"
echo "  slideforge skill-guide         # Print the design guide"
echo "  slideforge mcp                 # Start MCP server"
echo ""

echo "MCP configuration (for Claude, Cursor, etc.):"
cat <<EOF
{
  "mcpServers": {
    "slideforge": {
      "command": "$install_path",
      "args": ["mcp"]
    }
  }
}
EOF

echo ""
echo "AI agent skill location: $SKILL_DIR/slideforge/"
