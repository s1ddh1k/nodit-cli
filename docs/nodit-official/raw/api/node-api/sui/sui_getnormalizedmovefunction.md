# sui_getNormalizedMoveFunction

**`POST /`**

Return a structured representation of Move function

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
  "method": "sui_getNormalizedMoveFunction",
  "params": [
    "0x9c4eb6769ca8b6a23efeb7298cf0a8d0b837b78749c2cfc711c42036cc6b7621",
    "moduleName",
    "functionName"
  ]
}
```

## Response

| Name | Type | Required | Description |
|------|------|----------|-------------|
| jsonrpc | string |  |  |
| id | integer |  |  |
| result | object |  |  |
| result.isEntry | boolean | ✓ |  |
| result.parameters | array | ✓ |  |
| result.return | array | ✓ |  |
| result.typeParameters | array | ✓ |  |
| result.typeParameters[].abilities | array | ✓ |  |
| result.visibility | string | ✓ |  |

### Example

```json
{
  "jsonrpc": "2.0",
  "result": {
    "visibility": "Public",
    "isEntry": false,
    "typeParameters": [
      {
        "abilities": [
          "Store",
          "Key"
        ]
      }
    ],
    "parameters": [
      "U64"
    ],
    "return": [
      "U64"
    ]
  },
  "id": 1
}
```
