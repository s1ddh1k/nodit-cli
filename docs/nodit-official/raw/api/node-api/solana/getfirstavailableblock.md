# getFirstAvailableBlock

**`POST /`**

Returns the slot of the lowest confirmed block that has not been purged from the ledger

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
  "method": "getFirstAvailableBlock"
}
```

## Response

| Name | Type | Required | Description |
|------|------|----------|-------------|
| id | integer |  | The request ID |
| jsonrpc | string |  | The JSON-RPC version |
| result | integer |  | The slot of the lowest confirmed block that has not been purged from the ledger |
