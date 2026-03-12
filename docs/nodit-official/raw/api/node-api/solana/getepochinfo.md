# getEpochInfo

**`POST /`**

Returns information about the current epoch

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
  "method": "getEpochInfo",
  "params": [
    {
      "commitment": "finalized"
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
| result.absoluteSlot | integer |  | The current slot |
| result.blockHeight | integer |  | The current block height |
| result.epoch | integer |  | The current epoch |
| result.slotIndex | integer |  | The current slot relative to the start of the current epoch |
| result.slotsInEpoch | integer |  | The number of slots in this epoch |
| result.transactionCount | integer |  | Total number of transactions processed without error since genesis |
