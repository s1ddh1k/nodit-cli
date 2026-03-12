# getMaxShredInsertSlot

**`POST /`**

Get the max slot seen from after shred insert.

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
  "method": "getMaxShredInsertSlot"
}
```

## Response

| Name | Type | Required | Description |
|------|------|----------|-------------|
| id | integer |  | The request ID |
| jsonrpc | string |  | The JSON-RPC version |
| result | integer |  | The max slot seen from after shred insert |
