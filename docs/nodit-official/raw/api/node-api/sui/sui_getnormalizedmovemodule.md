# sui_getNormalizedMoveModule

**`POST /`**

Return a structured representation of Move module

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
  "method": "sui_getNormalizedMoveModule",
  "params": [
    "0x0047d5fa0a823e7d0ff4d55c32b09995a0ae1eedfee9c7b1354e805ed10ee3d0",
    "module"
  ]
}
```

## Response

| Name | Type | Required | Description |
|------|------|----------|-------------|
| jsonrpc | string |  |  |
| id | integer |  |  |
| result | object |  |  |
| result.address | string | ✓ |  |
| result.enums | object |  |  |
| result.exposedFunctions | object | ✓ |  |
| result.fileFormatVersion | integer | ✓ |  |
| result.friends | array | ✓ |  |
| result.friends[].address | string | ✓ |  |
| result.friends[].name | string | ✓ |  |
| result.name | string | ✓ |  |
| result.structs | object | ✓ |  |

### Example

```json
{
  "jsonrpc": "2.0",
  "result": {
    "fileFormatVersion": 6,
    "address": "0x1639f3606a53f61f3a566963b3eac49fe3bb57d304a454ed2f4859b44f4e4918",
    "name": "module",
    "friends": [],
    "structs": {},
    "exposedFunctions": {}
  },
  "id": 1
}
```
