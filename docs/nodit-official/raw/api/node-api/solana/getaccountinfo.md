# getAccountInfo

**`POST /`**

Returns all information associated with the account of provided Pubkey

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
  "method": "getAccountInfo",
  "params": [
    "3fJ7AiixCoHhaYzaNn1nNoLZMQnrGSMDNmMN4ZNUMpEa",
    {
      "commitment": "processed",
      "encoding": "base58",
      "dataSlice": {
        "offset": 0,
        "length": 100
      },
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
| result | object |  | If the requested account doesn't exist result will be null. Otherwise, an object containing: 1. context 2. value  |
| result.context | object |  | An object containing metadata about the current state of the Solana network at the time of the request. |
| result.context.apiVersion | string |  | The API version used for the request |
| result.context.slot | integer |  | The slot number at which the request was evaluated |
| result.value | object |  |  |
| result.value.data | object |  | Data associated with the account, either as encoded binary data or JSON format {<program>: <state>} - depending on encoding parameter  |
| result.value.executable | boolean |  | Boolean indicating if the account contains a program (and is strictly read-only) |
| result.value.lamports | integer |  | Number of lamports assigned to this account |
| result.value.owner | string |  | base-58 encoded Pubkey of the program this account has been assigned to |
| result.value.rentEpoch | integer |  | The epoch at which this account will next owe rent, as u64 |
| result.value.space | integer |  | The data size of the account |
