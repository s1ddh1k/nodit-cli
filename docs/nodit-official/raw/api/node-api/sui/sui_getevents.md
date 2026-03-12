# sui_getEvents

**`POST /`**

Return transaction events.

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
  "method": "sui_getEvents",
  "params": [
    "11a72GCQ5hGNpWGh2QhQkkusTEGS6EDqifJqxr7nSYX"
  ]
}
```

## Response

| Name | Type | Required | Description |
|------|------|----------|-------------|
| jsonrpc | string |  |  |
| id | integer |  |  |
| result | array |  |  |
| result[].bcs | string | ✓ | Base64 encoding |
| result[].bcsEncoding | string | ✓ |  |

### Example

```json
{
  "jsonrpc": "2.0",
  "result": {
    "data": [
      {
        "id": {
          "txDigest": "11a72GCQ5hGNpWGh2QhQkkusTEGS6EDqifJqxr7nSYX",
          "eventSeq": "0"
        },
        "packageId": "0xc54ab30a3d9adc07c1429c4d6bbecaf9457c9af77a91f631760853934d383634",
        "transactionModule": "test_module",
        "sender": "0xbcf7c32655009a61f1de0eae420a2e4ae1bb772ab2dd5d5a7dfa949c0ef06908",
        "type": "0x0000000000000000000000000000000000000000000000000000000000000009::test::TestEvent",
        "parsedJson": {
          "test": "example value"
        },
        "bcsEncoding": "base64",
        "bcs": ""
      }
    ],
    "nextCursor": {
      "txDigest": "11a72GCQ5hGNpWGh2QhQkkusTEGS6EDqifJqxr7nSYX",
      "eventSeq": "5"
    },
    "hasNextPage": false
  },
  "id": 1
}
```
