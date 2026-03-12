# getRecentPerformanceSamples

**`POST /`**

Returns a list of recent performance samples, in reverse slot order. Performance samples are taken every 60 seconds and include the number of transactions and slots that occur in a given time window.

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
| jsonrpc | string | ✓ |  |
| id | integer | ✓ |  |
| method | string | ✓ |  |
| params | array | ✓ | It contains following items: 1. Number of samples (`optional`)  |

### Example

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "getRecentPerformanceSamples",
  "params": [
    2
  ]
}
```

## Response

| Name | Type | Required | Description |
|------|------|----------|-------------|
| id | integer |  | The request ID |
| jsonrpc | string |  | The JSON-RPC version |
| result | array |  |  |
| result[].slot | integer |  | Slot in which sample was taken at |
| result[].numTransactions | integer |  | Number of transactions processed during the sample period |
| result[].numSlots | integer |  | Number of slots completed during the sample period |
| result[].samplePeriodSecs | integer |  | Number of seconds in a sample window |
| result[].numNonVoteTransactions | integer |  | Number of non-vote transactions processed during the sample period |
