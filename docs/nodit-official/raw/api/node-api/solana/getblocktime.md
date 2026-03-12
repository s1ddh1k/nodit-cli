# getBlockTime

**`POST /`**

Returns the estimated production time of a block.

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
| jsonrpc | string | ✓ |  |
| id | integer | ✓ |  |
| method | string | ✓ |  |
| params | array | ✓ | It contains following items: 1. Block number (`required`)  |

### Example

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "getBlockTime",
  "params": [
    338967300
  ]
}
```

## Response

| Name | Type | Required | Description |
|------|------|----------|-------------|
| id | integer |  | The request ID |
| jsonrpc | string |  | The JSON-RPC version |
| result | object |  |  |
