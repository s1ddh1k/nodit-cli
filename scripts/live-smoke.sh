#!/usr/bin/env bash

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

CONFIG_PATH="${XDG_CONFIG_HOME:-$HOME/.config}/nodit-cli/config.toml"
HAS_AUTH_SOURCE=0

if [[ -n "${NODIT_API_KEY:-}" ]]; then
  HAS_AUTH_SOURCE=1
elif [[ -f .env ]] && rg -q '^NODIT_API_KEY=' .env; then
  HAS_AUTH_SOURCE=1
elif [[ -f "$CONFIG_PATH" ]] && rg -q '^api_key[[:space:]]*=' "$CONFIG_PATH"; then
  HAS_AUTH_SOURCE=1
fi

if [[ "$HAS_AUTH_SOURCE" -ne 1 ]]; then
  echo "No Nodit API key source found." >&2
  echo "Use one of: --api-key, NODIT_API_KEY, ./.env, or $CONFIG_PATH" >&2
  exit 1
fi

RUNNER=()
if command -v mise >/dev/null 2>&1; then
  RUNNER=(mise x rust@stable -- cargo run --quiet --)
elif command -v cargo >/dev/null 2>&1; then
  RUNNER=(cargo run --quiet --)
else
  echo "cargo or mise is required" >&2
  exit 1
fi

run_case() {
  local name="$1"
  shift

  echo "==> $name"
  "${RUNNER[@]}" --json "$@" >/tmp/nodit-cli-smoke.out
  echo "ok"
}

run_optional_case() {
  local name="$1"
  shift

  echo "==> $name"
  if "${RUNNER[@]}" --json "$@" >/tmp/nodit-cli-smoke.out 2>/tmp/nodit-cli-smoke.err; then
    echo "ok"
    return 0
  fi

  if rg -q "plan|429|forbidden|not supported|unauthorized" /tmp/nodit-cli-smoke.out /tmp/nodit-cli-smoke.err 2>/dev/null; then
    echo "plan-limited"
    return 0
  fi

  echo "failed"
  cat /tmp/nodit-cli-smoke.out /tmp/nodit-cli-smoke.err
  return 1
}

run_case "EVM block-number" \
  node evm block-number --protocol ethereum --network mainnet

run_optional_case "Aptos ledger-info" \
  node aptos ledger-info

run_optional_case "Solana slot" \
  node solana slot --protocol solana --network mainnet

run_optional_case "Sui chain-identifier" \
  node sui chain-identifier --protocol sui --network mainnet

run_optional_case "Data native balance" \
  data native balance \
  --protocol ethereum \
  --network mainnet \
  --account 0x0000000000000000000000000000000000000000

if [[ "${NODIT_INCLUDE_STREAM:-0}" == "1" ]]; then
  echo "==> Stream smoke"
  if timeout 45s "${RUNNER[@]}" --json stream \
    --protocol ethereum \
    --network mainnet \
    --event-type BLOCK_PERIOD \
    --period 1 \
    --messages 1 >/tmp/nodit-cli-smoke.out 2>/tmp/nodit-cli-smoke.err; then
    echo "ok"
  elif rg -q "plan|429|forbidden|unauthorized|authentication|connection" /tmp/nodit-cli-smoke.out /tmp/nodit-cli-smoke.err 2>/dev/null; then
    echo "plan-limited-or-config-needed"
  else
    echo "failed"
    cat /tmp/nodit-cli-smoke.out /tmp/nodit-cli-smoke.err
    exit 1
  fi
else
  echo "==> Stream smoke"
  echo "skipped (set NODIT_INCLUDE_STREAM=1 to include)"
fi
