# getMinimumBalanceForRentExemption

**`POST /`**

Returns minimum balance required to make account rent exempt.

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
| jsonrpc | string | ✓ |  |
| id | integer | ✓ |  |
| method | string | ✓ |  |
| params | array | ✓ | It contains following items: 1. Account's Data Length (`required`) 2. ConfigurationObject (`optional`)  |

### Example

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "getMinimumBalanceForRentExemption",
  "params": [
    50,
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
| result | integer |  | Minimum lamports required in the Account to remain rent free |
