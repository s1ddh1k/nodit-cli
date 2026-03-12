# nodit-cli

Rust CLI for Nodit services, with native builds for Linux, macOS, and Windows.

## Scope

This project is structured to cover the public Nodit surfaces that are documented today:

- Elastic Node / JSON-RPC
- Web3 Data API
- Webhook API
- Stream API over WebSocket
- Aptos Node API

`Datasquare`, `Dedicated Node`, and console-only workflows are not exposed here as first-class commands yet because their public automation surface is either limited or product-specific. The transport layer is generic enough to add them when the API surface is clarified.

## Environment

```bash
export NODIT_API_KEY=your_api_key
export NODIT_API_BASE_URL=https://web3.nodit.io
export NODIT_RPC_URL=https://web3.nodit.io
export NODIT_STREAM_URL=wss://web3.nodit.io
export NODIT_APTOS_API_BASE_URL=https://aptos-mainnet.nodit.io/v1
```

Override any endpoint with flags when a chain or network needs a more specific URL.

Configuration sources are loaded in this order:

1. CLI flags
2. Process environment variables
3. Local `.env`
4. `~/.config/nodit-cli/config.toml`
5. Built-in defaults

Example config files:

- [config.example.toml](/home/eugene/git/nodit-cli/config.example.toml)
- [.env.example](/home/eugene/git/nodit-cli/.env.example)

## Build

```bash
cargo build
```

Release build:

```bash
cargo build --release
```

The repository includes GitHub Actions workflows for:

- CI on `ubuntu-latest`, `macos-latest`, and `windows-latest`
- Release artifacts for tagged versions on all three operating systems

## Roadmap

See [docs/roadmap.md](/home/eugene/git/nodit-cli/docs/roadmap.md) for current status, remaining scope, and execution phases.
See [docs/interface.md](/home/eugene/git/nodit-cli/docs/interface.md) for the intended CLI surface.
See [docs/coverage.md](/home/eugene/git/nodit-cli/docs/coverage.md) for endpoint coverage tracking.

## Command examples

### Node API

```bash
nodit node evm block-number \
  --protocol ethereum \
  --network mainnet
```

Get chain ID:

```bash
nodit node evm chain-id \
  --protocol ethereum \
  --network mainnet
```

Get transaction receipt:

```bash
nodit node evm transaction-receipt \
  --protocol ethereum \
  --network mainnet \
  --hash 0xYOUR_TX_HASH
```

Get transaction count (nonce):

```bash
nodit node evm transaction-count \
  --protocol ethereum \
  --network mainnet \
  --account 0x0000000000000000000000000000000000000000
```

Get code at an address:

```bash
nodit node evm code \
  --protocol ethereum \
  --network mainnet \
  --account 0x0000000000000000000000000000000000000000
```

Raw JSON-RPC:

```bash
nodit node evm raw \
  --protocol ethereum \
  --network mainnet \
  --method eth_blockNumber \
  --params '[]'
```

Batch JSON-RPC:

```bash
nodit node evm batch \
  --protocol ethereum \
  --network mainnet \
  --body '[{"jsonrpc":"2.0","id":1,"method":"eth_blockNumber","params":[]},{"jsonrpc":"2.0","id":2,"method":"eth_chainId","params":[]}]'
```

Aptos node API:

```bash
nodit node aptos ledger-info
```

Solana node API:

```bash
nodit node solana slot \
  --protocol solana \
  --network mainnet
```

```bash
nodit node solana latest-blockhash \
  --protocol solana \
  --network mainnet
```

```bash
nodit node solana block-height \
  --protocol solana \
  --network mainnet
```

```bash
nodit node solana version \
  --protocol solana \
  --network mainnet
```

```bash
nodit node solana epoch-info \
  --protocol solana \
  --network mainnet
```

```bash
nodit node solana minimum-balance-for-rent-exemption \
  --protocol solana \
  --network mainnet \
  --data-length 165
```

### Web3 Data API

```bash
nodit data native balance \
  --protocol ethereum \
  --network mainnet \
  --account 0x0000000000000000000000000000000000000000
```

Get token holders:

```bash
nodit data token holders-by-contract \
  --protocol ethereum \
  --network mainnet \
  --contract 0x0000000000000000000000000000000000000000
```

Get NFT transfers by account:

```bash
nodit data nft transfers-by-account \
  --protocol ethereum \
  --network mainnet \
  --account 0x0000000000000000000000000000000000000000
```

Resolve ENS name by address:

```bash
nodit data ens name-by-address \
  --protocol ethereum \
  --network mainnet \
  --address 0x0000000000000000000000000000000000000000
```

Get account stats:

```bash
nodit data stats account \
  --protocol ethereum \
  --network mainnet \
  --account 0x0000000000000000000000000000000000000000
```

Bitcoin account stats via Web3 Data API:

```bash
nodit data account total-transaction-count \
  --protocol bitcoin \
  --network mainnet \
  --account 1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa
```

```bash
nodit data account unspent-transaction-outputs \
  --protocol bitcoin \
  --network mainnet \
  --account 1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa
```

```bash
nodit data tx by-account \
  --protocol bitcoin \
  --network mainnet \
  --account 1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa
```

Bitcoin block-style queries are not assumed to share the same EVM-style path shape. Use only the verified `data account` and `data tx` flows until the Bitcoin-specific data endpoints are mapped from the official docs.

Verified Bitcoin block lookup:

```bash
nodit data block by-hash-or-number \
  --protocol bitcoin \
  --network mainnet \
  --block 0
```

### Webhook and Stream

Webhook for `BLOCK_PERIOD`:

```bash
nodit webhook create \
  --protocol ethereum \
  --network mainnet \
  --event-type BLOCK_PERIOD \
  --webhook-url https://example.com/hook \
  --period 1
```

Webhook for `ADDRESS_ACTIVITY`:

```bash
nodit webhook create \
  --protocol ethereum \
  --network mainnet \
  --event-type ADDRESS_ACTIVITY \
  --webhook-url https://example.com/hook \
  --address 0xdAC17F958D2ee523a2206206994597C13D831ec7,0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045
```

Stream for `MINED_TRANSACTION`:

```bash
nodit stream \
  --protocol ethereum \
  --network mainnet \
  --event-type MINED_TRANSACTION \
  --address 0xc1f18552bD0dc6D4f5A2dccc756F10C4882E2F4A \
  --messages 3
```

## Design notes

- `node` and `data` are the primary surfaces.
- Product commands are separated at the CLI layer.
- `node` is grouped by chain family such as `evm` and `aptos`.
- `data` is grouped by domains such as `native`, `account`, `tx`, `block`, `token`, `nft`, `ens`, `stats`, `asset`, and `multichain`.
- `solana` is currently verified on the node surface, while `bitcoin` is currently verified on the data surface.
- `raw` commands remain available where a stable typed command has not been added yet.
- `webhook serve` gives you a local receiver for Nodit callback development and can validate Nodit `x-signature` headers with a `signingKey`.
- Responses are kept as raw JSON to avoid premature schema lock-in across chains.
- Native CI and release packaging are configured for Linux, macOS, and Windows runners.
