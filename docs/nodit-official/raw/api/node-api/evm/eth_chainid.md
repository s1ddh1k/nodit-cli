# eth_chainId

**`POST /`**

Returns the chain ID of the network the node is connected to as a hexadecimal string. The chain ID is the network identifier defined in EIP-155, used for transaction signing to prevent replay attacks.

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
  "method": "eth_chainId"
}
```

## Response

### Example

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": "0x1"
}
```
