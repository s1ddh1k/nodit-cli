# sui_getNormalizedMoveStruct

**`POST /`**

Return a structured representation of Move struct

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
  "method": "sui_getNormalizedMoveStruct",
  "params": [
    "0xc95b9e341bc3aba1654bdbad707dcd773bd6309363447ef3fe58a960de92aa93",
    "module",
    "StructName"
  ]
}
```

## Response

| Name | Type | Required | Description |
|------|------|----------|-------------|
| jsonrpc | string |  |  |
| id | integer |  |  |
| result | object |  |  |
| result.abilities | object | ✓ |  |
| result.abilities.abilities | array | ✓ |  |
| result.fields | array | ✓ |  |
| result.fields[].name | string | ✓ |  |
| result.fields[].type | string | ✓ |  |
| result.typeParameters | array | ✓ |  |
| result.typeParameters[].constraints | object | ✓ |  |
| result.typeParameters[].isPhantom | boolean | ✓ |  |

### Example

```json
{
  "jsonrpc": "2.0",
  "result": {
    "abilities": {
      "abilities": [
        "Store",
        "Key"
      ]
    },
    "typeParameters": [],
    "fields": []
  },
  "id": 1
}
```
