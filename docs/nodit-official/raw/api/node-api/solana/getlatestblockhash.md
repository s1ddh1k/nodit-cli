# getLatestBlockhash

**`POST /`**

Returns the latest blockhash

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
| jsonrpc | string | ✓ |  |
| id | integer | ✓ |  |
| method | string | ✓ |  |
| params | array | ✓ | It contains following items: 1. ConfigurationObject (`optional`)  |
| params[].commitment | string |  | The commitment describes how finalized a block is at that point in time. - finalized - the node will query the most recent block confirmed by supermajority of the cluster as having reached maximum lockout, meaning the cluster has recognized this block as finalized - confirmed - the node will query the most recent block that has been voted on by supermajority of the cluster. - processed - the node will query its most recent block. Note that the block may still be skipped by the cluster.  |
| params[].minContextSlot | string |  | The minimum slot that the request can be evaluated at. |

### Example

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "getLatestBlockhash",
  "params": [
    {
      "commitment": "processed"
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
| result.value.blockhash | string |  | The blockhash of the block, as base-58 encoded string |
| result.value.lastValidBlockHeight | integer |  | Last block height at which the blockhash will be valid |
