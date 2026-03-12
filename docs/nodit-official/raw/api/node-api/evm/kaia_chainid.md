# kaia_chainID

**`POST /`**

Returns the chain ID of the network the current node is connected to, as a hexadecimal string. Chain ID is the network identifier defined in EIP-155, used in transaction signing to prevent replay attacks.

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
  "method": "kaia_chainID"
}
```
