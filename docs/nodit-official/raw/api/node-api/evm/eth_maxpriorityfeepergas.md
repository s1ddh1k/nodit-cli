# eth_maxPriorityFeePerGas

**`POST /`**

Returns the maximum priority fee in wei for the network the node is connected to. The maximum priority fee is used in the new transaction format defined in EIP-1559.

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
  "method": "eth_maxPriorityFeePerGas"
}
```

## Response

### Example

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": "0x989680"
}
```
