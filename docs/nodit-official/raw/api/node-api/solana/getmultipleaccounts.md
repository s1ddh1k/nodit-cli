# getMultipleAccounts

**`POST /`**

Returns the account information for a list of Pubkeys.

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
| jsonrpc | string | ✓ |  |
| id | integer | ✓ |  |
| method | string | ✓ |  |
| params | array | ✓ | It contains following items: 1. Pubkeys (`required`) 2. ConfigurationObject (`optional`)  |

### Example

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "getMultipleAccounts",
  "params": [
    [
      "vines1vzrYbzLMRdu58ou5XTby4qAqVRLmqo36NKPTg",
      "4fYNw3dojWmQ4dXtSGE9epjRGy9pFSx62YypT7avPYvA"
    ],
    {
      "encoding": "base58",
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
| result.value | array |  |  |
| result.value[].data | array |  | Data associated with the account, either as encoded binary data or JSON format {<program>: <state>} - depending on encoding parameter  |
| result.value[].executable | boolean |  | Boolean indicating if the account contains a program (and is strictly read-only) |
| result.value[].lamports | integer |  | Number of lamports assigned to this account |
| result.value[].owner | string |  | Base-58 encoded Pubkey of the program this account has been assigned to |
| result.value[].rentEpoch | integer |  | The epoch at which this account will next owe rent |
| result.value[].space | integer |  | The data size of the account |
