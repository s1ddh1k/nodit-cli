# getSignaturesForAddress

**`POST /`**

Returns signatures for confirmed transactions that include the given address in their accountKeys list. Returns signatures backwards in time from the provided signature or most recent confirmed block

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
| jsonrpc | string | ✓ |  |
| id | integer | ✓ |  |
| method | string | ✓ |  |
| params | array | ✓ | It contains following items: 1. Account address (`required`) 2. ConfigurationObject (`optional`)  |

### Example

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "getSignaturesForAddress",
  "params": [
    "Vote111111111111111111111111111111111111111",
    {
      "commitment": "finalized",
      "limit": 1
    }
  ]
}
```

## Response

| Name | Type | Required | Description |
|------|------|----------|-------------|
| id | integer |  | The request ID |
| jsonrpc | string |  | The JSON-RPC version |
| result | array |  |  |
| result[].signature | string |  | Transaction signature as base-58 encoded string |
| result[].slot | integer |  | The slot that contains the block with the transaction |
| result[].err | object |  | Error if transaction failed, null if transaction succeeded. |
| result[].memo | string |  | Memo associated with the transaction, null if no memo is present |
| result[].blockTime | integer |  | Estimated production time, as Unix timestamp (seconds since the Unix epoch) of when transaction was processed. null if not available. |
| result[].confirmationStatus | string |  | The transaction's cluster confirmation status; Either processed, confirmed, or finalized. - processed - the transaction has been processed by the cluster - confirmed - the transaction has been confirmed by the cluster - finalized - the transaction has been finalized by the cluster  |
