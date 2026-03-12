# eth_uninstallFilter

**`POST /`**

Uninstalls the filter with the specified filter ID. Use a filter ID created via eth_newFilter. Returns `false` if the filter was already removed or does not exist.

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
| id | integer | ✓ |  |
| jsonrpc | string | ✓ |  |
| method | string | ✓ |  |
| params | array | ✓ | Pass the following parameters as an array with the appropriate types. 1. `filter ID`: The pre-created filter ID. |

### Example

```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "method": "eth_uninstallFilter",
  "params": [
    "0xaf35d60b70eb3b54018456a0d365ea49"
  ]
}
```

## Response

### Example

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": true
}
```
