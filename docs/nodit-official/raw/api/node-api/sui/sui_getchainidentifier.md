# sui_getChainIdentifier

**`POST /`**

Return the first four bytes of the chain's genesis checkpoint digest.

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
| id | integer | ✓ |  |
| jsonrpc | string | ✓ |  |
| method | string | ✓ |  |
| params | array | ✓ |  |

### Example

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "sui_getChainIdentifier",
  "params": []
}
```

## Response

| Name | Type | Required | Description |
|------|------|----------|-------------|
| jsonrpc | string |  |  |
| id | integer |  |  |
| result | string |  |  |

### Example

```json
{
  "jsonrpc": "2.0",
  "result": "4c78adac",
  "id": 1
}
```
