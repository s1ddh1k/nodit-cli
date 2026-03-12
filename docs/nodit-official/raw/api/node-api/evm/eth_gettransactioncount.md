# eth_getTransactionCount

**`POST /`**

Returns the number of transactions sent from the specified address.

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
| id | integer | ✓ |  |
| jsonrpc | string | ✓ |  |
| method | string | ✓ |  |
| params | array | ✓ | Pass the following parameters as an array with the appropriate types. 1. `address` - The address to query as a 40-character hex string. 2. `block identifier` - Block number, block hash, or block tag. |

### Example

```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "method": "eth_getTransactionCount",
  "params": [
    "0xc90d3Ac75D1D36dF0b0a229E73D8409FB7F3c4ab",
    "latest"
  ]
}
```

## Response

### Example

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": "0x2"
}
```
