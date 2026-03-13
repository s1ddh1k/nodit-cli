#!/usr/bin/env bash

set -euo pipefail

REPO_SLUG="${NODIT_CLI_REPO_SLUG:-s1ddh1k/nodit-cli}"
BIN_DIR="${NODIT_CLI_BIN_DIR:-$HOME/.local/bin}"
INSTALL_NAME="${NODIT_CLI_INSTALL_NAME:-nodit-cli}"
VERSION="${NODIT_CLI_VERSION:-latest}"
TMP_DIR=""

usage() {
  cat <<'EOF'
Usage:
  curl -fsSL https://raw.githubusercontent.com/s1ddh1k/nodit-cli/main/install.sh | bash

Optional environment variables:
  NODIT_CLI_VERSION=latest
  NODIT_CLI_BIN_DIR=$HOME/.local/bin
  NODIT_CLI_INSTALL_NAME=nodit-cli
  NODIT_CLI_REPO_SLUG=s1ddh1k/nodit-cli
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

detect_asset_name() {
  local os arch
  os="$(uname -s)"
  arch="$(uname -m)"

  case "$os" in
    Darwin)
      case "$arch" in
        arm64|x86_64) echo "nodit-cli-macos.tar.gz" ;;
        *) fail "unsupported macOS architecture: $arch" ;;
      esac
      ;;
    Linux)
      case "$arch" in
        x86_64|amd64) echo "nodit-cli-linux.tar.gz" ;;
        *) fail "unsupported Linux architecture: $arch" ;;
      esac
      ;;
    MINGW*|MSYS*|CYGWIN*|Windows_NT)
      echo "nodit-cli-windows.zip"
      ;;
    *)
      fail "unsupported operating system: $os"
      ;;
  esac
}

release_api_url() {
  if [[ "$VERSION" == "latest" ]]; then
    echo "https://api.github.com/repos/$REPO_SLUG/releases/latest"
  else
    echo "https://api.github.com/repos/$REPO_SLUG/releases/tags/$VERSION"
  fi
}

extract_tag_name() {
  local json="$1"
  printf '%s' "$json" | sed -n 's/.*"tag_name"[[:space:]]*:[[:space:]]*"\([^"]*\)".*/\1/p' | head -n1
}

extract_asset_url() {
  local json="$1"
  local asset_name="$2"
  awk -v asset="$asset_name" '
    index($0, "\"name\": \"" asset "\"") { found=1 }
    found && /"browser_download_url":/ {
      line = $0
      sub(/.*"browser_download_url":[[:space:]]*"/, "", line)
      sub(/".*/, "", line)
      print line
      exit
    }
  ' <<<"$json"
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

install_unix_archive() {
  local archive_path="$1"
  local extract_dir="$2"
  local target_bin="$3"

  tar -xzf "$archive_path" -C "$extract_dir"

  local source_bin
  source_bin="$(find "$extract_dir" -type f -name nodit-cli | head -n1)"
  [[ -n "$source_bin" ]] || fail "nodit-cli binary not found in archive"

  cp "$source_bin" "$target_bin"
  chmod 755 "$target_bin"
}

main() {
  if [[ "${1:-}" == "-h" || "${1:-}" == "--help" ]]; then
    usage
    exit 0
  fi

  need_cmd curl
  need_cmd tar
  need_cmd mktemp

  local asset_name api_url release_json tag_name asset_url archive_path extract_dir target_bin

  asset_name="$(detect_asset_name)"
  api_url="$(release_api_url)"

  echo "Resolving release metadata from $api_url"
  release_json="$(curl -fsSL "$api_url")" || fail "failed to fetch release metadata"

  tag_name="$(extract_tag_name "$release_json")"
  [[ -n "$tag_name" ]] || fail "failed to resolve release tag"

  asset_url="$(extract_asset_url "$release_json" "$asset_name")"
  [[ -n "$asset_url" ]] || fail "asset not found for this platform: $asset_name"

  mkdir -p "$BIN_DIR"
  TMP_DIR="$(mktemp -d)"
  trap cleanup EXIT

  archive_path="$TMP_DIR/$asset_name"
  extract_dir="$TMP_DIR/extracted"
  target_bin="$BIN_DIR/$INSTALL_NAME"
  mkdir -p "$extract_dir"

  echo "Downloading $asset_name from release $tag_name"
  curl -fL "$asset_url" -o "$archive_path"

  case "$asset_name" in
    *.tar.gz)
      install_unix_archive "$archive_path" "$extract_dir" "$target_bin"
      ;;
    *.zip)
      fail "Windows zip installation is not supported by this shell script"
      ;;
    *)
      fail "unsupported asset format: $asset_name"
      ;;
  esac

  clear_quarantine "$target_bin"
  clear_quarantine "$BIN_DIR"

  cat <<EOF

Installed $INSTALL_NAME from release $tag_name:
  $target_bin

Add this directory to PATH if needed:
  export PATH="$BIN_DIR:\$PATH"

Run:
  $INSTALL_NAME --help
EOF
}

main "$@"
