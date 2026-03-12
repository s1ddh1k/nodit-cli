# Interface

## Primary Model

The CLI uses a product-first interface.

Primary top-level commands:

- `node`
- `data`
- `webhook`
- `stream`

These are the commands that should appear first in documentation and examples.

## Rules

1. New public functionality should be added under the relevant product command first.
2. `raw` or low-level escape hatches must be available, but they should not dominate the primary docs.
3. `node` and `data` are the highest-priority surfaces and should remain stable.

## Shape

### Node

`node` is organized by chain family, not by a single flat RPC model.

Examples:

- `node evm ...`
- `node aptos ...`
- `node solana ...`
- `node bitcoin ...`
- `node dogecoin ...`
- `node xrpl ...`

Current implementation priority:

- `node evm` first
- reserve stable slots for non-EVM families

For `node evm`, use task-oriented subcommands for common JSON-RPC reads:

- `block-number`
- `chain-id`
- `balance`
- `transaction`
- `transaction-receipt`
- `transaction-count`
- `gas-price`
- `code`
- `logs`
- `call`
- `batch`
- `raw`

### Data

`data` should use domain subcommands:

- `native`
- `account`
- `tx`
- `block`
- `token`
- `nft`
- `multichain`

### Webhook / Stream

These remain product-first and should mirror Nodit concepts:

- `webhook list|get|create|update|delete|history|serve`
- `stream` as a typed subscribe/watch flow without alias commands

## Documentation Priority

README examples should prioritize:

1. `node`
2. `data`
3. `webhook`
4. `stream`
