# suix_resolveNameServiceAddress

**`POST /`**

Return the resolved address given resolver and name

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
  "method": "suix_resolveNameServiceAddress",
  "params": [
    "example.sui"
  ]
}
```

## Response

| Name | Type | Required | Description |
|------|------|----------|-------------|
| jsonrpc | string |  |  |
| id | integer |  |  |
| result | string |  | Hex string encoding. |

### Example

```json
{
  "jsonrpc": "2.0",
  "result": "0x6710024f81dd33ab6833482ee8034e779a48e6ef635c7f856df4905022458bfb",
  "id": 1
}
```
