# getTokenAccountBalance

**`POST /`**

Returns the token balance of an SPL Token account.

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
| jsonrpc | string | ✓ |  |
| id | integer | ✓ |  |
| method | string | ✓ |  |
| params | array | ✓ | It contains following items: 1. Pubkey of Token Account (`required`) 2. ConfigurationObject (`optional`)  |

### Example

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "getTokenAccountBalance",
  "params": [
    "7fUAJdStEuGbc3sM84cKRL6yYaaSstyLSU4ve5oovLS7",
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
| result | object |  |  |
| result.context | object |  | An object containing metadata about the current state of the Solana network at the time of the request. |
| result.context.apiVersion | string |  | The API version used for the request |
| result.context.slot | integer |  | The slot number at which the request was evaluated |
| result.value | object |  |  |
| result.value.amount | string |  | The raw balance without decimals, a string representation of u64 |
| result.value.decimals | number |  | Number of base 10 digits to the right of the decimal place |
| result.value.uiAmount | number |  | `DEPRECATED` The balance, using mint-prescribed decimals  |
| result.value.uiAmountString | string |  | The balance as a string, using mint-prescribed decimals |
