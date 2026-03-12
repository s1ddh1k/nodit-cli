# getTokenSupply

**`POST /`**

Returns the total supply of an SPL Token type.

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
| jsonrpc | string | ✓ |  |
| id | integer | ✓ |  |
| method | string | ✓ |  |
| params | array | ✓ | It contains following items: 1. Pubkey of the token Mint (`required`) 2. ConfigurationObject (`optional`)  |

### Example

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "getTokenSupply",
  "params": [
    "3wyAj7Rt1TWVPZVteFJPLa26JmLvdb1CAKEFZm3NY75E",
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
| result.value.amount | string |  | The raw amount without decimals, a string representation of u64 |
| result.value.decimals | number |  | Number of base 10 digits to the right of the decimal place |
| result.value.uiAmount | number |  | The total token supply, using mint-prescribed decimals |
| result.value.uiAmountString | string |  | The total token supply as a string, using mint-prescribed decimals |
