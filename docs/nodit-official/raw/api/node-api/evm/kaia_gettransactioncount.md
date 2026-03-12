# kaia_getTransactionCount

**`POST /`**

Returns the number of transactions sent from a specific address (nonce).

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
| id | integer | ✓ |  |
| jsonrpc | string | ✓ |  |
| method | string | ✓ |  |
| params | array | ✓ | Enter the following parameters as an array with the appropriate types. 1. `address` - Enter the address to query as a 40-digit hexadecimal string. 2. `block identifier` - Block number, block hash, or block tag for the block to query. |

### Example

```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "method": "kaia_getTransactionCount",
  "params": [
    "0xc90d3Ac75D1D36dF0b0a229E73D8409FB7F3c4ab",
    "latest"
  ]
}
```
