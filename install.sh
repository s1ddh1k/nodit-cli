#!/usr/bin/env bash

set -euo pipefail

DEFAULT_REPO_URL="https://github.com/s1ddh1k/nodit-cli.git"
REPO_URL="${1:-${NODIT_CLI_REPO_URL:-$DEFAULT_REPO_URL}}"
REF="${NODIT_CLI_REF:-}"
BIN_DIR="${NODIT_CLI_BIN_DIR:-$HOME/.local/bin}"
INSTALL_NAME="${NODIT_CLI_INSTALL_NAME:-nodit-cli}"
TMP_DIR=""

usage() {
  cat <<'EOF'
Usage:
  curl -fsSL https://raw.githubusercontent.com/s1ddh1k/nodit-cli/main/install.sh | bash
  curl -fsSL https://raw.githubusercontent.com/s1ddh1k/nodit-cli/main/install.sh | bash -s -- <github-repo-url>

Optional environment variables:
  NODIT_CLI_REF=<branch-or-tag>
  NODIT_CLI_BIN_DIR=$HOME/.local/bin
  NODIT_CLI_INSTALL_NAME=nodit-cli
EOF
}

cleanup() {
  if [[ -n "${TMP_DIR}" && -d "${TMP_DIR}" ]]; then
    rm -rf "${TMP_DIR}"
  fi
}

fail() {
  echo "Error: $*" >&2
  exit 1
}

need_cmd() {
  command -v "$1" >/dev/null 2>&1 || fail "required command not found: $1"
}

ensure_rust_toolchain() {
  if command -v cargo >/dev/null 2>&1 && command -v rustc >/dev/null 2>&1; then
    return
  fi

  need_cmd curl
  echo "Rust toolchain not found. Installing via rustup."
  curl --proto '=https' --tlsv1.2 -fsSL https://sh.rustup.rs | sh -s -- -y

  export PATH="$HOME/.cargo/bin:$PATH"
  command -v cargo >/dev/null 2>&1 || fail "cargo was not installed correctly"
  command -v rustc >/dev/null 2>&1 || fail "rustc was not installed correctly"
}

clear_quarantine() {
  local target="$1"
  if [[ "$(uname -s)" != "Darwin" ]]; then
    return
  fi

  if command -v xattr >/dev/null 2>&1; then
    xattr -d com.apple.quarantine "$target" 2>/dev/null || true
  fi
}

main() {
  if [[ "$REPO_URL" == "-h" || "$REPO_URL" == "--help" ]]; then
    usage
    exit 0
  fi

  need_cmd git
  need_cmd mktemp
  ensure_rust_toolchain

  mkdir -p "$BIN_DIR"
  TMP_DIR="$(mktemp -d)"
  trap cleanup EXIT

  echo "Cloning $REPO_URL"
  if [[ -n "$REF" ]]; then
    git clone --depth 1 --branch "$REF" "$REPO_URL" "$TMP_DIR/repo"
  else
    git clone --depth 1 "$REPO_URL" "$TMP_DIR/repo"
  fi

  cd "$TMP_DIR/repo"

  echo "Building release binary"
  cargo build --release

  local source_bin="$TMP_DIR/repo/target/release/nodit-cli"
  local target_bin="$BIN_DIR/$INSTALL_NAME"

  [[ -f "$source_bin" ]] || fail "release binary not found: $source_bin"

  cp "$source_bin" "$target_bin"
  chmod 755 "$target_bin"
  clear_quarantine "$target_bin"
  clear_quarantine "$BIN_DIR"

  cat <<EOF

Installed: $target_bin

Add this directory to PATH if needed:
  export PATH="$BIN_DIR:\$PATH"

Run:
  $INSTALL_NAME --help
EOF
}

main "$@"
