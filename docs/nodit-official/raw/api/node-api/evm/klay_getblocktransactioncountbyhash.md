# klay_getBlockTransactionCountByHash

**`POST /`**

Returns the number of transactions in a block by block hash.

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
| id | integer | ✓ |  |
| jsonrpc | string | ✓ |  |
| method | string | ✓ |  |
| params | array | ✓ | Enter the following parameters as an array with the appropriate types. 1. `block hash`: Enter the block hash to query as a 64-digit hexadecimal string. |

### Example

```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "method": "klay_getBlockTransactionCountByHash",
  "params": [
    "0x59f63e3840e0f4a1659074c1a4021e881a268a52d31958688da1d66bfbf6d2ca"
  ]
}
```
