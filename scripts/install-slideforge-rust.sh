#!/usr/bin/env bash
set -euo pipefail

BIN_DIR="${HOME}/.local/bin"
LOCAL_BINARY=""
INSTALL_NAME="slideforge"

usage() {
  cat <<'USAGE'
Install SlideForge Rust CLI/MCP.

Usage:
  install-slideforge-rust.sh --local <path-to-binary> [--bin-dir <dir>]
  install-slideforge-rust.sh --url <release-binary-url> [--bin-dir <dir>]

Options:
  --local PATH    Install from an existing local binary.
  --url URL       Download and install a release binary.
  --bin-dir DIR   Install directory. Defaults to ~/.local/bin.
  --help          Show this help.
USAGE
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --local)
      LOCAL_BINARY="${2:-}"
      shift 2
      ;;
    --url)
      url="${2:-}"
      tmp="$(mktemp)"
      if command -v curl >/dev/null 2>&1; then
        curl -fsSL "$url" -o "$tmp"
      elif command -v wget >/dev/null 2>&1; then
        wget -qO "$tmp" "$url"
      else
        echo "error: curl or wget is required for --url" >&2
        exit 1
      fi
      LOCAL_BINARY="$tmp"
      shift 2
      ;;
    --bin-dir)
      BIN_DIR="${2:-}"
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

if [[ -z "$LOCAL_BINARY" ]]; then
  echo "error: provide --local PATH or --url URL" >&2
  usage >&2
  exit 1
fi

if [[ ! -f "$LOCAL_BINARY" ]]; then
  echo "error: binary not found: $LOCAL_BINARY" >&2
  exit 1
fi

mkdir -p "$BIN_DIR"
install_path="${BIN_DIR}/${INSTALL_NAME}"
cp "$LOCAL_BINARY" "$install_path"
chmod +x "$install_path"

"$install_path" list-slides >/dev/null

cat <<EOF
Installed SlideForge Rust:
  $install_path

Add this directory to PATH if needed:
  export PATH="$BIN_DIR:\$PATH"

MCP configuration:
{
  "mcpServers": {
    "slideforge": {
      "command": "$install_path",
      "args": ["mcp"]
    }
  }
}
EOF
