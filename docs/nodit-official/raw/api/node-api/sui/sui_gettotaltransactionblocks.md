# sui_getTotalTransactionBlocks

**`POST /`**

Return the total number of transaction blocks known to the server.

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
  "method": "sui_getTotalTransactionBlocks",
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
  "result": "2451485",
  "id": 1
}
```
