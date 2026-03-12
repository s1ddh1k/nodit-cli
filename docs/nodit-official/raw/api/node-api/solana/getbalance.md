# getBalance

**`POST /`**

Returns the lamport balance of the account of provided Pubkey

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
| jsonrpc | string | ✓ |  |
| id | integer | ✓ |  |
| method | string | ✓ |  |
| params | array | ✓ | It contains following items: 1. Pubkey (`required`) 2. ConfigurationObject (`optional`)  |

### Example

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "getBalance",
  "params": [
    "3fJ7AiixCoHhaYzaNn1nNoLZMQnrGSMDNmMN4ZNUMpEa",
    {
      "commitment": "processed",
      "minContextSlot": "1234567890"
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
| result.value | integer |  | The lamport balance of the account |
