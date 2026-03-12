# getSupply

**`POST /`**

Returns information about the current supply.

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
| jsonrpc | string | ✓ |  |
| id | integer | ✓ |  |
| method | string | ✓ |  |
| params | array | ✓ | It contains following items: 1. ConfigurationObject (`optional`)  |
| params[].commitment | string |  | The commitment describes how finalized a block is at that point in time. - finalized - the node will query the most recent block confirmed by supermajority of the cluster as having reached maximum lockout, meaning the cluster has recognized this block as finalized - confirmed - the node will query the most recent block that has been voted on by supermajority of the cluster. - processed - the node will query its most recent block. Note that the block may still be skipped by the cluster.  |
| params[].excludeNonCirculatingAccountsList | boolean |  | Exclude non circulating accounts list from response |

### Example

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "getSupply",
  "params": [
    {
      "commitment": "finalized",
      "excludeNonCirculatingAccountsList": true
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
| result.value | object |  |  |
| result.value.total | integer |  | Total supply in lamports |
| result.value.circulating | integer |  | Circulating supply in lamports |
| result.value.nonCirculating | integer |  | Non-circulating supply in lamports |
| result.value.nonCirculatingAccounts | array |  |  |
