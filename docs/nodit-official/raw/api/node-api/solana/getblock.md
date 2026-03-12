# getBlock

**`POST /`**

Returns identity and transaction information about a confirmed block in the ledger

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
| jsonrpc | string | ✓ |  |
| id | integer | ✓ |  |
| method | string | ✓ |  |
| params | array | ✓ | It contains following items: 1. Slot number (`required`) 2. ConfigurationObject (`optional`)  |

### Example

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "getBlock",
  "params": [
    378967388,
    {
      "commitment": "finalized",
      "encoding": "json",
      "transactionDetails": "full",
      "maxSupportedTransactionVersion": 0,
      "rewards": false
    }
  ]
}
```

## Response

| Name | Type | Required | Description |
|------|------|----------|-------------|
| id | integer |  | The request ID |
| jsonrpc | string |  | The JSON-RPC version |
| result | object |  |  |
| result.blockHeight | integer |  | The number of blocks beneath this block. |
| result.blockTime | integer |  | Estimated production time, as Unix timestamp (seconds since the Unix epoch). null if not available. |
| result.blockhash | string |  | The blockhash of the block, as base-58 encoded string |
| result.parentSlot | integer |  | The slot index of the parent block. |
| result.previousBlockhash | string |  | The blockhash of this block's parent, as base-58 encoded string; if the parent block is not available due to ledger cleanup, this field will return "11111111111111111111111111111111" |
| result.transactions | array |  |  |
| result.transactions[].transaction | object | ✓ |  |
| result.transactions[].meta | object | ✓ |  |
