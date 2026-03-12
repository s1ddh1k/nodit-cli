# eth_getFilterChanges

**`POST /`**

Returns filter results for the specified filter ID. Use a filter ID created via eth_newFilter, eth_newBlockFilter, or eth_newPendingTransactionFilter.

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
| id | integer | ✓ |  |
| jsonrpc | string | ✓ |  |
| method | string | ✓ |  |
| params | array | ✓ | Pass the following parameters as an array with the appropriate types. 1. `filter ID`: The pre-created filter ID. |

### Example

```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "method": "eth_getFilterChanges",
  "params": [
    "0xaf35d60b70eb3b54018456a0d365ea49"
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
      "address": "0xa3e0dfbf8dbd86e039f7cdb37682a776d66dae4b",
      "topics": [
        "0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef",
        "0x000000000000000000000000a03167de1a56160e4647d77d81e9139af55b63d4",
        "0x00000000000000000000000038701e5945f578ccc8d0477d9a465254e9e95a09"
      ],
      "data": "0x000000000000000000000000000000000000000000000000000000000001d4c0",
      "blockNumber": "0x9e5983",
      "transactionHash": "0x987afc31410fe56a5b6c38a75b7338e1cd8eea5a11f626f2f6a5deeb764a64f9",
      "transactionIndex": "0xf",
      "blockHash": "0x7f1df48cad828de085f8e61235a184ac107b8a730691846fcc0acb74599743b9",
      "logIndex": "0x28",
      "removed": false
    }
  ]
}
```
