# getBlocks

**`POST /`**

Returns a list of confirmed blocks between two slots.

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
| jsonrpc | string | ✓ |  |
| id | integer | ✓ |  |
| method | string | ✓ |  |
| params | array | ✓ | It contains following items: 1. Start slot (`required`) 2. End slot (`optional`) 3. ConfigurationObject (`optional`)  |

### Example

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "getBlocks",
  "params": [
    338967300,
    338967310,
    {
      "commitment": "finalized"
    }
  ]
}
```

## Response

| Name | Type | Required | Description |
|------|------|----------|-------------|
| id | integer |  | The request ID |
| jsonrpc | string |  | The JSON-RPC version |
| result | array |  | An array of u64 integers listing confirmed blocks between `start_slot` and either `end_slot` - if provided, or latest confirmed slot, inclusive. Max range allowed is 500,000 slots. |
