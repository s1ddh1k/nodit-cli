# getLeaderSchedule

**`POST /`**

Returns the leader schedule for an epoch

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
| jsonrpc | string | ✓ |  |
| id | integer | ✓ |  |
| method | string | ✓ |  |
| params | array | ✓ | It contains following items: 1. Epoch (`optional`) 2. ConfigurationObject (`optional`)  |
| params[].commitment | string |  | The commitment describes how finalized a block is at that point in time. - finalized - the node will query the most recent block confirmed by supermajority of the cluster as having reached maximum lockout, meaning the cluster has recognized this block as finalized - confirmed - the node will query the most recent block that has been voted on by supermajority of the cluster. - processed - the node will query its most recent block. Note that the block may still be skipped by the cluster.  |
| params[].identity | string |  | Only return results for this validator identity (base-58 encoded). |

### Example

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "getLeaderSchedule",
  "params": [
    null,
    {
      "commitment": "processed",
      "identity": "dv2eQHeP4RFrJZ6UeiZWoc3XTtmtZCUKxxCApCDcRNV"
    }
  ]
}
```

## Response

| Name | Type | Required | Description |
|------|------|----------|-------------|
| id | integer |  | The request ID |
| jsonrpc | string |  | The JSON-RPC version |
| result | object |  | Returns null if requested epoch is not found, otherwise returns an object where: - Keys are validator identities (as base-58 encoded strings) - Values are arrays of leader slot indices relative to the first slot in the requested epoch  |
