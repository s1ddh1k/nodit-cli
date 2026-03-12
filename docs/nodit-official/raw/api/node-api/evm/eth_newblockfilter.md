# eth_newBlockFilter

**`POST /`**

Creates a filter to receive notifications when new blocks are mined, and returns a filter ID. The filter ID is used with eth_getFilterChanges to poll for filter results.

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
  "method": "eth_newBlockFilter"
}
```

## Response

### Example

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": "0x0200000000000000cb91b496c6fee6fb"
}
```
