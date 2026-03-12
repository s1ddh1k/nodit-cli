# eth_getLogs

**`POST /`**

Returns logs matching the specified filter criteria. Unlike filter-based methods, filter conditions are passed directly in the request without creating a filter first.


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
  "method": "eth_getLogs",
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
  "result": [
    {
      "address": "0xb59f67a8bff5d8cd03f6ac17265c550ed8f33907",
      "topics": [
        "0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef",
        "0x00000000000000000000000000b46c2526e227482e2ebb8f4c69e4674d262e75",
        "0x00000000000000000000000054a2d42a40f51259dedd1978f6c118a0f0eff078"
      ],
      "data": "0x000000000000000000000000000000000000000000000000000000012a05f200",
      "blockNumber": "0x429d3b",
      "transactionHash": "0xab059a62e22e230fe0f56d8555340a29b2e9532360368f810595453f6fdd213b",
      "transactionIndex": "0xac",
      "blockHash": "0x8243343df08b9751f5ca0c5f8c9c0460d8a9b6351066fae0acbd4d3e776de8bb",
      "logIndex": "0x56",
      "removed": false
    }
  ]
}
```
