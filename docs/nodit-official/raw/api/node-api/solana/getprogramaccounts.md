# getProgramAccounts

**`POST /`**

Returns all accounts owned by the provided program Pubkey

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
| jsonrpc | string | ✓ |  |
| id | integer | ✓ |  |
| method | string | ✓ |  |
| params | array | ✓ | It contains following items: 1. Pubkey of program (`required`) 2. ConfigurationObject (`optional`)  |

### Example

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "getProgramAccounts",
  "params": [
    "4Nd1mBQtrMJVYVfKf2PJy9NZUZdTAsp7D4xWLs4gDB4T",
    {
      "commitment": "finalized",
      "filters": [
        {
          "dataSize": 17
        },
        {
          "memcmp": {
            "offset": 4,
            "bytes": "3Mc6vR"
          }
        }
      ]
    }
  ]
}
```

## Response

| Name | Type | Required | Description |
|------|------|----------|-------------|
| id | integer |  | The request ID |
| jsonrpc | string |  | The JSON-RPC version |
| result | array |  |  |
| result[].pubkey | string |  | The account Pubkey as base-58 encoded string |
| result[].account | object |  |  |
| result[].account.data | array |  | Data associated with the account, either as encoded binary data or JSON format {<program>: <state>} - depending on encoding parameter  |
| result[].account.executable | boolean |  | Boolean indicating if the account contains a program (and is strictly read-only) |
| result[].account.lamports | integer |  | Number of lamports assigned to this account |
| result[].account.owner | string |  | Base-58 encoded Pubkey of the program this account has been assigned to |
| result[].account.rentEpoch | integer |  | The epoch at which this account will next owe rent |
| result[].account.space | integer |  | The data size of the account |
