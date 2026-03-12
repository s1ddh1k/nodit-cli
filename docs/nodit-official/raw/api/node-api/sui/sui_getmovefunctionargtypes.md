# sui_getMoveFunctionArgTypes

**`POST /`**

Return the argument types of a Move function, based on normalized Type.

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
  "method": "sui_getMoveFunctionArgTypes",
  "params": [
    "0xa0a7b108f5023b7356f2c6a4be6f058e267aae38e08260c7d519d8641897490c",
    "suifrens",
    "mint"
  ]
}
```

## Response

| Name | Type | Required | Description |
|------|------|----------|-------------|
| jsonrpc | string |  |  |
| id | integer |  |  |
| result | array |  |  |

### Example

```json
{
  "jsonrpc": "2.0",
  "result": [
    {
      "Object": "ByMutableReference"
    },
    "Pure",
    "Pure",
    {
      "Object": "ByValue"
    },
    {
      "Object": "ByImmutableReference"
    },
    {
      "Object": "ByValue"
    },
    {
      "Object": "ByMutableReference"
    }
  ],
  "id": 1
}
```
