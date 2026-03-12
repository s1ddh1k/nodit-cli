# sui_getNormalizedMoveModulesByPackage

**`POST /`**

Return structured representations of all modules in the given package

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
  "method": "sui_getNormalizedMoveModulesByPackage",
  "params": [
    "0x61630d3505f8905a0f4d42c6ff39a78a6ba2b28f68a3299ec3417bbabc6717dc"
  ]
}
```

## Response

| Name | Type | Required | Description |
|------|------|----------|-------------|
| jsonrpc | string |  |  |
| id | integer |  |  |
| result | object |  |  |

### Example

```json
{
  "jsonrpc": "2.0",
  "result": {
    "fileFormatVersion": 6,
    "address": "0x800105867da4655eca6d9eb1258bfd1ad92af329a07781ee71e60065e00f2de9",
    "name": "module",
    "friends": [],
    "structs": {},
    "exposedFunctions": {}
  },
  "id": 1
}
```
