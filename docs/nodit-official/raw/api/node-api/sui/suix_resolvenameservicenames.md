# suix_resolveNameServiceNames

**`POST /`**

Return the resolved names given address, if multiple names are resolved, the first one is the primary name.

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
  "method": "suix_resolveNameServiceNames",
  "params": [
    "0x5cd6fa76ed1d18f05f15e35075252ddec4fb83621d55952d9172fcfcb72feae2",
    "0xd22bbb46f892c42d9ec0ae4de93e02c75973a51c17180798237326a58694a2cf",
    3
  ]
}
```

## Response

| Name | Type | Required | Description |
|------|------|----------|-------------|
| jsonrpc | string |  |  |
| id | integer |  |  |
| result | object |  | `next_cursor` points to the last item in the page; Reading with `next_cursor` will start from the next item after `next_cursor` if `next_cursor` is `Some`, otherwise it will start from the first item. |
| result.data | array | ✓ |  |
| result.hasNextPage | boolean | ✓ |  |
| result.nextCursor | string |  | Hex string encoding. |

### Example

```json
{
  "jsonrpc": "2.0",
  "result": {
    "data": [
      "example.sui"
    ],
    "nextCursor": "0xd22bbb46f892c42d9ec0ae4de93e02c75973a51c17180798237326a58694a2cf",
    "hasNextPage": false
  },
  "id": 1
}
```
