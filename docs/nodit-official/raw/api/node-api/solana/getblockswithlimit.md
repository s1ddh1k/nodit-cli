# getBlocksWithLimit

**`POST /`**

Returns a list of confirmed blocks starting at the given slot

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
| jsonrpc | string | ✓ |  |
| id | integer | ✓ |  |
| method | string | ✓ |  |
| params | array | ✓ | It contains following items: 1. Start slot (`required`) 2. Limit (`required`) 3. ConfigurationObject (`optional`)  |

### Example

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "getBlocksWithLimit",
  "params": [
    338967300,
    10
  ],
  "[object Object]": null
}
```

## Response

| Name | Type | Required | Description |
|------|------|----------|-------------|
| id | integer |  | The request ID |
| jsonrpc | string |  | The JSON-RPC version |
| result | array |  | An array of u64 integers listing confirmed blocks starting at `start_slot` for up to `limit` blocks, inclusive. |
