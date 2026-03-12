# getInflationRate

**`POST /`**

Returns the specific inflation values for the current epoch

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
| jsonrpc | string | ✓ |  |
| id | integer | ✓ |  |
| method | string | ✓ |  |

### Example

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "getInflationRate"
}
```

## Response

| Name | Type | Required | Description |
|------|------|----------|-------------|
| id | integer |  | The request ID |
| jsonrpc | string |  | The JSON-RPC version |
| result | object |  |  |
| result.total | number |  | Total inflation |
| result.validator | number |  | Inflation allocated to validators |
| result.foundation | number |  | Inflation allocated to the foundation |
| result.epoch | integer |  | Epoch for which these values are valid |
