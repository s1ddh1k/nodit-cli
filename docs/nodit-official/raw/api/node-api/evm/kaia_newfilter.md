# kaia_newFilter

**`POST /`**

Creates a filter to query logs matching the specified criteria and returns the filter ID. The filter ID is used with kaia_getFilterLogs and kaia_uninstallFilter.


> 🚧 Is the response taking a long time? Response time may vary based on block range!
>
> Specifying a wide block range or including blocks with many events may cause response delays or timeouts.
>
> For faster responses, consider the following recommendations:
>
> - Keep the fromBlock ~ toBlock range to 1000 blocks or fewer.
> - When using latest for blockTag, the exact block number may not be known internally, which can increase response time.
  In that case, query the latest block number with kaia_blockNumber first, then specify it explicitly.

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
| id | integer | ✓ |  |
| jsonrpc | string | ✓ |  |
| method | string | ✓ |  |
| params | array | ✓ |  |
| params[].fromBlock | string |  | Use one of block number or block tag to specify the block.  * Block number: Hexadecimal string (e.g. "0x1") * Block tag: Enum string (e.g. "latest", "earliest", "pending")   * `earliest`: The oldest available block on the chain.   * `finalized`: A recently finalized block that can no longer be changed. Primarily used in proof-of-stake (PoS) blockchains.   * `safe`: A recent block considered safe by the network (immune to reorgs).   * `latest`: The most recent block on the chain (may be subject to reorgs).   * `pending`: The next block to be mined, including pending transactions in the mempool. |
| params[].toBlock | string |  | Use one of block number or block tag to specify the block.  * Block number: Hexadecimal string (e.g. "0x1") * Block tag: Enum string (e.g. "latest", "earliest", "pending")   * `earliest`: The oldest available block on the chain.   * `finalized`: A recently finalized block that can no longer be changed. Primarily used in proof-of-stake (PoS) blockchains.   * `safe`: A recent block considered safe by the network (immune to reorgs).   * `latest`: The most recent block on the chain (may be subject to reorgs).   * `pending`: The next block to be mined, including pending transactions in the mempool. |
| params[].address | array |  | Enter addresses to filter as an array. |
| params[].topics | array |  | Topics is an array of topics for filtering transaction logs. Each topic is expressed as a 32-byte hexadecimal string. |

### Example

```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "method": "kaia_newFilter",
  "params": [
    {
      "fromBlock": "latest"
    }
  ]
}
```
