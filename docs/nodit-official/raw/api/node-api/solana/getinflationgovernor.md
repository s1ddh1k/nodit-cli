# getInflationGovernor

**`POST /`**

Returns the current inflation governor

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
| jsonrpc | string | ✓ |  |
| id | integer | ✓ |  |
| method | string | ✓ |  |
| params | array | ✓ | It contains following items: 1. ConfigurationObject (`optional`)  |
| params[].commitment | string |  | The commitment describes how finalized a block is at that point in time. - finalized - the node will query the most recent block confirmed by supermajority of the cluster as having reached maximum lockout, meaning the cluster has recognized this block as finalized - confirmed - the node will query the most recent block that has been voted on by supermajority of the cluster. - processed - the node will query its most recent block. Note that the block may still be skipped by the cluster.  |

### Example

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "getInflationGovernor",
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
| result.foundation | number |  | Percentage of total inflation allocated to the foundation |
| result.foundationTerm | integer |  | Duration of foundation pool inflation in years |
| result.initial | number |  | Initial inflation percentage from time 0 |
| result.taper | number |  | Rate per year at which inflation is lowered. (Rate reduction is derived using the target slot time in genesis config) |
| result.terminal | number |  | Terminal inflation percentage |
