# getBlockProduction

**`POST /`**

Returns recent block production information from the current or previous epoch.

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
| jsonrpc | string | ✓ |  |
| id | integer | ✓ |  |
| method | string | ✓ |  |
| params | array | ✓ | It contains following items: 1. ConfigurationObject (`optional`)  |
| params[].commitment | string |  | The commitment describes how finalized a block is at that point in time. - finalized - the node will query the most recent block confirmed by supermajority of the cluster as having reached maximum lockout, meaning the cluster has recognized this block as finalized - confirmed - the node will query the most recent block that has been voted on by supermajority of the cluster. - processed - the node will query its most recent block. Note that the block may still be skipped by the cluster.  |
| params[].identity | string |  | Only return results for this validator identity (base-58 encoded). |
| params[].range | object |  | Slot range to return block production for. If parameter not provided, defaults to current epoch. |
| params[].range.firstSlot | integer | ✓ | first slot to return block production information for (inclusive) |
| params[].range.lastSlot | integer |  | last slot to return block production information for (inclusive). If parameter not provided, defaults to the highest slot |

### Example

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "getBlockProduction",
  "params": [
    {
      "commitment": "finalized",
      "identity": "12i8gndWWWMTRzJBFhnYkobNgZB3XMUUJq75HeUrshrk",
      "range": {
        "firstSlot": 363067597,
        "lastSlot": 364115359
      }
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
| result.value | integer |  | The lamport balance of the account |
