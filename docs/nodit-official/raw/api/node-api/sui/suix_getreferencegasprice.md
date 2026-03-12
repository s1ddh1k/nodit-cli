# suix_getReferenceGasPrice

**`POST /`**

Return the reference gas price for the network

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
| id | integer | ✓ |  |
| jsonrpc | string | ✓ |  |
| method | string | ✓ |  |
| params | array | ✓ |  |

### Example

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "suix_getReferenceGasPrice",
  "params": []
}
```

## Response

| Name | Type | Required | Description |
|------|------|----------|-------------|
| jsonrpc | string |  |  |
| id | integer |  |  |
| result | string |  |  |

### Example

```json
{
  "jsonrpc": "2.0",
  "result": 1000,
  "id": 1
}
```
