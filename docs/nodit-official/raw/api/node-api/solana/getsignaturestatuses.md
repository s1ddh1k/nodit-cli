# getSignatureStatuses

**`POST /`**

Returns the statuses of a list of signatures. Each signature must be a txid, the first signature of a transaction.
Unless the `searchTransactionHistory` configuration parameter is included, this method only searches the recent status cache of signatures, which retains statuses for all active slots plus `MAX_RECENT_BLOCKHASHES` rooted slots.

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
| jsonrpc | string | ✓ |  |
| id | integer | ✓ |  |
| method | string | ✓ |  |
| params | array | ✓ | It contains following items: 1. Transaction signatures (`required`) 2. ConfigurationObject (`optional`)  |

### Example

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "getSignatureStatuses",
  "params": [
    [
      "5VERv8NMvzbJMEkV8xnrLkEaWRtSz9CosKDYjCJjBRnbJLgp8uirBgmQpjKhoR4tjF3ZpRzrFmBV6UjKdiSZkQUW"
    ],
    {
      "searchTransactionHistory": true
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
| result.context | object |  | An object containing metadata about the current state of the Solana network at the time of the request. |
| result.context.apiVersion | string |  | The API version used for the request |
| result.context.slot | integer |  | The slot number at which the request was evaluated |
| result.value | array |  |  |
| result.value[].slot | integer |  | The slot the transaction was processed |
| result.value[].confirmations | integer |  | Number of blocks since signature confirmation, null if rooted, as well as finalized by a supermajority of the cluster |
| result.value[].err | object |  | Error if transaction failed, null if transaction succeeded. |
| result.value[].status | object |  | `DEPRECATED` Transaction status  "Ok": <null> - Transaction was successful "Err": <ERR> - Transaction failed with TransactionError  |
| result.value[].confirmationStatus | string |  | The transaction's cluster confirmation status; Either processed, confirmed, or finalized. - processed - the transaction has been processed by the cluster - confirmed - the transaction has been confirmed by the cluster - finalized - the transaction has been finalized by the cluster  |
