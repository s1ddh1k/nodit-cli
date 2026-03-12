# eth_newFilter

**`POST /`**

Creates a filter for logs matching the specified criteria and returns a filter ID. The filter ID is used with eth_getFilterLogs and eth_uninstallFilter.


> 🚧 Response taking too long? Response time may increase with block range!
>
> Specifying a wide block range or including blocks with many events can cause response delays or timeouts.
>
> For faster responses:
>
> - Keep the fromBlock ~ toBlock range to 1000 blocks or less.
> - When using blockTag "latest", the response may be slower since the exact block number is not known internally.
  In that case, use eth_blockNumber to get the latest block number and specify it explicitly.

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
| id | integer | ✓ |  |
| jsonrpc | string | ✓ |  |
| method | string | ✓ |  |
| params | array | ✓ |  |
| params[].fromBlock | string |  | Use one of block number or block tag to specify a block. * Block number: hex string (ex. "0x1") * Block tag: enum string (ex. "latest", "earliest", "pending") |
| params[].toBlock | string |  | Use one of block number or block tag to specify a block. * Block number: hex string (ex. "0x1") * Block tag: enum string (ex. "latest", "earliest", "pending") |
| params[].address | array |  | Addresses to filter by, as an array. |
| params[].topics | array |  | Topics array for filtering transaction logs. Each topic is a 32-byte hex string. |

### Example

```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "method": "eth_newFilter",
  "params": [
    {
      "fromBlock": "latest"
    }
  ]
}
```

## Response

### Example

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": "0x0100000000000000ee32c7a8d24aac1f"
}
```
