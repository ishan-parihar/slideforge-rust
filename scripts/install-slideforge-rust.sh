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
INSTALL_NAME="slideforge"
LOCAL_BINARY=""

usage() {
  cat <<'USAGE'
Install SlideForge Rust CLI/MCP server.

Usage:
  install-slideforge-rust.sh [OPTIONS]

Options:
  --bin-dir DIR     Install directory (default: ~/.local/bin)
  --version VER     Release version to download (default: v0.2.0)
  --local PATH      Install from a local binary instead of downloading
  --help            Show this help

Examples:
  # One-liner install
  curl -fsSL https://raw.githubusercontent.com/ishan-parihar/slideforge-rust/master/scripts/install-slideforge-rust.sh | bash

  # Install to /usr/local/bin (needs sudo)
  curl -fsSL https://raw.githubusercontent.com/ishan-parihar/slideforge-rust/master/scripts/install-slideforge-rust.sh | sudo bash -s -- --bin-dir /usr/local/bin

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
  if command -v curl >/dev/null 2>&1; then
    curl -fsSL "$DOWNLOAD_URL" -o "$tmp"
  elif command -v wget >/dev/null 2>&1; then
    wget -qO "$tmp" "$DOWNLOAD_URL"
  else
    echo "error: curl or wget is required" >&2
    exit 1
  fi
  LOCAL_BINARY="$tmp"
fi

# Verify the binary exists
if [[ ! -f "$LOCAL_BINARY" ]]; then
  echo "error: binary not found: $LOCAL_BINARY" >&2
  exit 1
fi

# Install
mkdir -p "$BIN_DIR"
install_path="${BIN_DIR}/${INSTALL_NAME}"
cp "$LOCAL_BINARY" "$install_path"
chmod +x "$install_path"

# Clean up temp file
if [[ -n "${tmp:-}" ]] && [[ -f "$tmp" ]]; then
  rm -f "$tmp"
fi

# Verify the binary works
if ! "$install_path" --version >/dev/null 2>&1; then
  echo "error: binary verification failed" >&2
  exit 1
fi

echo ""
echo "✓ SlideForge installed to: $install_path"
echo ""

# Check if BIN_DIR is in PATH
if [[ ":$PATH:" != *":$BIN_DIR:"* ]]; then
  echo "⚠ $BIN_DIR is not in your PATH. Add it:"
  echo "  echo 'export PATH=\"$BIN_DIR:\$PATH\"' >> ~/.bashrc"
  echo "  source ~/.bashrc"
  echo ""
fi

echo "Quick start:"
echo "  slideforge --help              # See all CLI commands"
echo "  slideforge list-slides         # List 47 slide types"
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
