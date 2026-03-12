# eth_getUncleCountByBlockNumber

**`POST /`**

Returns the number of uncle blocks in the block specified by its block number.

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
| id | integer | ✓ |  |
| jsonrpc | string | ✓ |  |
| method | string | ✓ |  |
| params | array | ✓ | Pass the following parameters as an array with the appropriate types. 1. `block number or block tag`: Block number as hex string, or block tag such as "earliest", "latest", "pending". |

### Example

```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "method": "eth_getUncleCountByBlockNumber",
  "params": [
    "0x5BAD54"
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
