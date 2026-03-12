# Roadmap

## Status

This repository is past the scaffold stage but not yet at full Nodit feature coverage.

Current state:

- Product-level CLI structure exists for `node`, `data`, `webhook`, and `stream`
- Interface direction is documented in `docs/interface.md`
- Core Web3 Data lookups exist for a subset of account, token, NFT, block, and transaction queries
- Webhook CRUD exists
- `webhook serve` exists and validates Nodit `x-signature` using `signingKey`
- CI and release workflows exist for Linux, macOS, and Windows

Missing state:

- Full endpoint coverage across Web3 Data domains and supported networks
- Typed `webhook create/update` flows for common event types
- Typed `stream subscribe` flows using Nodit event model
- Better chain/network presets and saved profiles
- Live integration verification against real Nodit environments

## Goal

Build a practical Rust CLI that covers the public Nodit developer surface with a bias toward:

1. Fast time-to-first-use
2. Typed commands for common flows
3. Raw escape hatches for less-common or newly released APIs
4. Cross-platform distribution

## Product Checklist

### Elastic Node

- [x] Generic JSON-RPC call
- [x] First-class `node` command surface
- [x] Chain-family-oriented `node` interface direction
- [x] Common RPC shortcuts
- [x] Header-auth default endpoint strategy
- [x] Batch JSON-RPC call
- [ ] Network preset catalog
- [ ] Node WebSocket helpers

### Web3 Data API

- [x] Native balance
- [x] Native transfers by account
- [x] Transaction by hash
- [x] Transactions by account
- [x] Transactions in block
- [x] Transactions by hashes
- [x] Total transaction count by account
- [x] Internal transactions by account
- [x] Internal transactions by transaction hash
- [x] Next nonce by account
- [x] Tokens owned by account
- [x] Token holders by contract
- [x] Token transfers by contract
- [x] Token transfers within range
- [x] Token transfers by account
- [x] NFTs owned by account
- [x] NFT contracts by account
- [x] NFT holders by contract
- [x] NFT metadata by contract
- [x] NFT metadata by token IDs
- [x] NFT transfers by contract
- [x] NFT transfers by token ID
- [x] NFT transfers within range
- [x] NFT transfers by account
- [x] Block by number
- [x] Block by hash
- [x] Blocks within range
- [x] Gas price
- [x] Contract check
- [ ] Remaining account/token/NFT/block/transaction endpoints
- [ ] Pagination/sorting helpers

### Webhook

- [x] List/get/delete/history
- [x] Raw create/update
- [x] Local webhook receiver
- [x] Nodit `x-signature` verification with `signingKey`
- [x] Typed create/update flags
- [ ] Event-type-specific condition builders
- [ ] Signing key persistence helpers
- [ ] Reactivation helpers

### Stream

- [x] Generic WebSocket connect and subscribe
- [x] Typed subscribe command using Nodit event model
- [ ] Event-type-specific condition builders
- [ ] Reconnect and resume strategy

### Aptos

- [x] Basic node endpoints
- [ ] Event API coverage
- [ ] Aptos webhook typed flows
- [ ] Aptos-specific stream/webhook filters

### Multichain

- [x] `lookupEntities`
- [ ] Additional multichain queries if public API grows

## Execution Phases

### Phase 1

Typed webhook create/update commands with raw JSON fallback.

Success criteria:

- Create webhook without writing raw JSON for common cases
- Update webhook activation and notification settings without raw JSON
- Preserve `--body` or `--condition-json` escape hatch

### Phase 2

Typed stream subscriptions using the same event model as webhook.

Success criteria:

- Subscribe with `eventType`, `protocol`, `network`, and common conditions
- Reuse condition-building logic from webhook

Status: complete

### Phase 3

Expand Web3 Data domain coverage and normalize pagination/filter flags.

Status: in progress

### Phase 4

Elastic Node quality-of-life features: presets, batches, profiles.

### Phase 5

Live integration tests and end-to-end examples with real Nodit credentials.

## Rules For Further Work

- Keep the primary surface limited to `node`, `data`, `webhook`, and `stream`.
- Keep `node` grouped by chain family and `data` grouped by domain.
- Prefer typed commands for repeated workflows.
- Keep raw JSON escape hatches where the upstream API changes often.
- Verify Nodit-specific behavior against official docs before freezing the CLI surface.
