# Coverage

This document tracks endpoint coverage against the public Nodit developer docs.

## Source Scope

Primary references used for this checklist:

- Web3 Data API usage page
- Web3 Data API reference pages
- Elastic Node reference pages

The current CLI target is:

- `node`: chain-family-based command surface with typed helpers plus raw escape hatches
- `data`: domain-based command surface with typed coverage by endpoint family

## Node

### EVM

- [x] Raw JSON-RPC
- [x] Batch JSON-RPC
- [x] `eth_blockNumber`
- [x] `eth_chainId`
- [x] `eth_getBalance`
- [x] `eth_getTransactionCount`
- [x] `eth_getCode`
- [x] `eth_getLogs`
- [x] `eth_getTransactionByHash`
- [x] `eth_getTransactionReceipt`
- [x] `eth_gasPrice`
- [x] `eth_call`

### Non-EVM families

- [x] `node aptos` basic REST coverage
- [x] `node aptos` raw fallback
- [x] `node solana` raw fallback
- [ ] `node bitcoin` default Nodit endpoint support not confirmed
- [x] `node dogecoin` raw fallback
- [x] `node xrpl` raw fallback

Verified live on `node solana`:

- [x] `getSlot`
- [x] `getLatestBlockhash`
- [x] `getBlockHeight`
- [x] `getEpochInfo`
- [x] `getVersion`
- [x] `getGenesisHash`
- [x] `getMinimumBalanceForRentExemption`
- [x] `getAccountInfo`
- [x] `getTokenSupply`

## Web3 Data API

### Native

- [x] `getNativeBalanceByAccount`
- [x] `getNativeTokenBalanceByAccount`
- [x] `getNativeTransfersByAccount`
- [x] `getNativeTransfersWithinRange`
- [x] `getNativeHolders`

Bitcoin status:

- [ ] `native` category is not supported on Bitcoin for `getNativeBalanceByAccount`

### Account / Blockchain

- [x] `getTotalTransactionCountByAccount`
- [x] `getInternalTransactionsByAccount`
- [x] `getNextNonceByAccount`
- [x] `isContract`
- [x] `getUnspentTransactionOutputsByAccount`

### Transaction / Blockchain

- [x] `getTransactionByHash`
- [x] `getTransactionByTransactionId`
- [x] `getTransactionsByHashes`
- [x] `getTransactionsByTransactionIds`
- [x] `getTransactionsByAccount`
- [x] `getTransactionsInBlock`
- [x] `getInternalTransactionsByTransactionHash`
- [x] `searchEvents`

### Block / Blockchain

- [x] `getBlockByNumber`
- [x] `getBlockByHash`
- [x] `getBlockByHashOrNumber`
- [x] `getBlocksWithinRange`
- [x] `getGasPrice`

### Token

- [x] `getTokensOwnedByAccount`
- [x] `getTokenAllowance`
- [x] `getTokenContractMetadataByContracts`
- [x] `getTokenHoldersByContract`
- [x] `getTokenPricesByContracts`
- [x] `getTokenTransfersByContract`
- [x] `getTokenTransfersWithinRange`
- [x] `getTokenTransfersByAccount`
- [x] `searchTokenContractMetadataByKeyword`

### NFT

- [x] `getNftsOwnedByAccount`
- [x] `getNftContractsByAccount`
- [x] `getNftContractMetadataByContracts`
- [x] `getNftHoldersByContract`
- [x] `getNftHoldersByTokenId`
- [x] `getNftMetadataByContract`
- [x] `getNftMetadataByTokenIds`
- [x] `getNftTransfersByContract`
- [x] `getNftTransfersByTokenId`
- [x] `getNftTransfersWithinRange`
- [x] `getNftTransfersByAccount`
- [x] `searchNftContractMetadataByKeyword`
- [x] `syncNftMetadata`

### ENS

- [x] `getAddressByEnsName`
- [x] `getEnsNameByAddress`
- [x] `getEnsRecordByName`
- [x] `getEnsRecordsByAccount`

### Stats

- [x] `getAccountStats`
- [x] `getDailyActiveAccountsStats`
- [x] `getDailyActiveAccountsStatsByContract`
- [x] `getDailyTransactionsStats`
- [x] `getDailyTransactionsStatsByContract`
- [x] `getHourlyActiveAccountsStats`
- [x] `getHourlyActiveAccountsStatsByContract`
- [x] `getHourlyTransactionsStats`
- [x] `getHourlyTransactionsStatsByContract`

### Asset

- [x] `getAssetMetadataByIssuer`
- [x] `getAssetMetadataByIds`
- [x] `searchAssetMetadataByKeyword`
- [x] `getAssetHoldersById`
- [x] `getAssetTransfersById`
- [x] `getAssetTransfersWithinRange`
- [x] `getAssetTransfersByAccount`
- [x] `getAssetsOwnedByAccount`

### Multichain

- [x] `lookupEntities`

## Notes

- A checked item means there is at least one direct CLI command path to that endpoint.
- Some commands still rely on `--body` for the less-stable payload shapes.
- Live verification status still needs to be expanded endpoint by endpoint because free-plan rate limits make full-session verification noisy.
- As of March 13, 2026, `solana` has been live-verified on the `node` surface, while `bitcoin` has been live-verified on the `data` surface.
- As of March 13, 2026, Bitcoin `data account`, `data tx by-account`, and `data block by-hash-or-number` are live-verified.
- As of March 13, 2026, Bitcoin `native` balance is confirmed unsupported on Nodit's `native` category.
