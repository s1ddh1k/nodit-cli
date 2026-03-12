# getSlotLeaders

**`POST /`**

Returns the slot leaders for a given slot range

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
| jsonrpc | string | ✓ |  |
| id | integer | ✓ |  |
| method | string | ✓ |  |
| params | array | ✓ | It contains following items: 1. StartSlot (`required`) 2. Limit (`required`)  |

### Example

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "getSlotLeaders",
  "params": [
    100,
    10
  ]
}
```

## Response

| Name | Type | Required | Description |
|------|------|----------|-------------|
| id | integer |  | The request ID |
| jsonrpc | string |  | The JSON-RPC version |
| result | array |  | Array of Node identity public keys as base-58 encoded strings. |
