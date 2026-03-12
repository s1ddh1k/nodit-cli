# getTokenAccountsByOwner

**`POST /`**

Returns all SPL Token accounts by token owner.

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
| jsonrpc | string | ✓ |  |
| id | integer | ✓ |  |
| method | string | ✓ |  |
| params | array | ✓ | It contains following items: 1. Pubkey of Token Account (`required`) 2. Program (`required`) 3. ConfigurationObject (`optional`)  |

### Example

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "getTokenAccountsByOwner",
  "params": [
    "A1TMhSGzQxMr1TboBKtgixKz1sS6REASMxPo1qsyTSJd",
    {
      "programId": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
    },
    {
      "commitment": "finalized",
      "encoding": "jsonParsed"
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
| result.value.pubkey | string |  | The account Pubkey as base-58 encoded string |
| result.value.account | object |  |  |
