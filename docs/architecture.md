# Architecture

## Why this shape

Nodit is not a single API. It is a product family with different transports:

- JSON-RPC for node access
- REST for data and webhook management
- GraphQL for Aptos indexer queries
- WebSocket for stream subscriptions

Trying to model everything as strongly typed Rust structs too early will slow the first implementation down and create churn every time the upstream API changes. This scaffold keeps the transport typed and the payloads untyped JSON.

## Command map

- `nodit rpc`: Elastic Node JSON-RPC calls
- `nodit data`: generic REST access for Web3 Data API
- `nodit webhook`: common webhook lifecycle commands plus raw access
- `nodit aptos graphql`: Aptos indexer GraphQL queries
- `nodit aptos rest`: generic Aptos REST access if needed
- `nodit stream`: WebSocket connection and subscription

## Recommended next milestones

1. Add endpoint presets by chain and network.
2. Add profile management in `~/.config/nodit-cli`.
3. Generate typed clients where Nodit publishes stable OpenAPI or GraphQL schemas.
4. Add pagination helpers and table output for list commands.
5. Add integration tests against a mock server before hitting live Nodit environments.
