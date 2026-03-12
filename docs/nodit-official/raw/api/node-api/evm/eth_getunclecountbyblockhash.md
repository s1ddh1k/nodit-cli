# eth_getUncleCountByBlockHash

**`POST /`**

Returns the number of uncle blocks in the block specified by its block hash.

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
| id | integer | ✓ |  |
| jsonrpc | string | ✓ |  |
| method | string | ✓ |  |
| params | array | ✓ | Pass the following parameters as an array with the appropriate types. 1. `block hash`: The block hash to query as a 64-character hex string. |

### Example

```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "method": "eth_getUncleCountByBlockHash",
  "params": [
    "0x61a8ad530a8a43e3583f8ec163f773ad370329b2375d66433eb82f005e1d6202"
  ]
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
