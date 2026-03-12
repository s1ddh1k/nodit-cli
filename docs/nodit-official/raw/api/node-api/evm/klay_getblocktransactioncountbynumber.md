# klay_getBlockTransactionCountByNumber

**`POST /`**

Returns the number of transactions in a block by block number.

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
| id | integer | ✓ |  |
| jsonrpc | string | ✓ |  |
| method | string | ✓ |  |
| params | array | ✓ | Enter the following parameters as an array with the appropriate types. 1. `block number or block tag`: Enter the block number to query as a hexadecimal string. You may also use block tags such as "earliest", "latest", "pending". |

### Example

```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "method": "klay_getBlockTransactionCountByNumber",
  "params": [
    "0x1076B5A"
  ]
}
```
