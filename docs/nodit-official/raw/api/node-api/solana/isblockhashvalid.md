# isBlockhashValid

**`POST /`**

Returns whether a blockhash is still valid or not

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
| jsonrpc | string | ✓ |  |
| id | integer | ✓ |  |
| method | string | ✓ |  |
| params | array | ✓ | It contains following items: 1. Blockhash (`required`) 2. ConfigurationObject (`optional`)  |

### Example

```json
{
  "jsonrpc": "2.0",
  "id": 45,
  "method": "isBlockhashValid",
  "params": [
    "J7rBdM6AecPDEZp8aPq5iPSNKVkU5Q76F3oAV4eW5wsW",
    {
      "commitment": "processed"
    }
  ]
}
```

## Response

| Name | Type | Required | Description |
|------|------|----------|-------------|
| id | integer |  | The request ID |
| jsonrpc | string |  | The JSON-RPC version |
| result | object |  |  |
| result.context | object |  | An object containing metadata about the current state of the Solana network at the time of the request. |
| result.context.apiVersion | string |  | The API version used for the request |
| result.context.slot | integer |  | The slot number at which the request was evaluated |
| result.value | boolean |  | Whether the blockhash is still valid or not |
