# klay_maxPriorityFeePerGas

**`POST /`**

Returns the maximum priority fee in wei for the network the current node is connected to. Maximum priority fee is used in the new transaction format defined in EIP-1559.

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
| id | integer | ✓ |  |
| jsonrpc | string | ✓ |  |
| method | string | ✓ |  |

### Example

```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "method": "klay_maxPriorityFeePerGas"
}
```
