# eth_blockNumber

**`POST /`**

Returns the number of the most recent block.

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
  "method": "eth_blockNumber"
}
```

## Response

### Example

```json
{
  "jsonrpc": "2.0",
  "id": "1",
  "result": "0x1075788"
}
```
