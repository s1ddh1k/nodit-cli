# suix_getDynamicFieldObject

**`POST /`**

Return the dynamic field object information for a specified object

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
  "method": "suix_getDynamicFieldObject",
  "params": [
    "0x3ddea0f8c3da994d9ead562ce76e36fdef6a382da344930c73d1298b0e9644b8",
    {
      "type": "0x0000000000000000000000000000000000000000000000000000000000000009::test::TestField",
      "value": "some_value"
    }
  ]
}
```

## Response

| Name | Type | Required | Description |
|------|------|----------|-------------|
| jsonrpc | string |  |  |
| id | integer |  |  |
| result | object |  |  |
| result.data | object |  |  |
| result.data.bcs | object |  |  |
| result.data.content | object |  |  |
| result.data.digest | object | ✓ | Base64 string representing the object digest |
| result.data.display | object |  |  |
| result.data.objectId | string | ✓ | Hex string encoding. |
| result.data.owner | object |  | Object is exclusively owned by a single address, and is mutable. |
| result.data.previousTransaction | string |  |  |
| result.data.storageRebate | string |  |  |
| result.data.type | string |  | The type of the object. Default to be None unless SuiObjectDataOptions.showType is set to true |
| result.data.version | object | ✓ | Object version. |
| result.error | object |  |  |
| result.error.code | string | ✓ |  |
| result.error.object_id | string | ✓ | Hex string encoding. |

### Example

```json
{
  "jsonrpc": "2.0",
  "result": {
    "data": {
      "objectId": "0x3ddea0f8c3da994d9ead562ce76e36fdef6a382da344930c73d1298b0e9644b8",
      "version": "1",
      "digest": "Faiv4yqGR4HjAW8WhMN1NHHNStxXgP3u22dVPyvLad2z",
      "type": "0x0000000000000000000000000000000000000000000000000000000000000009::test::TestField",
      "owner": {
        "AddressOwner": "0x5ea6f7a348f4a7bd1a9ab069eb7f63865de3075cc5a4e62432f634b50fd2bb2b"
      },
      "previousTransaction": "5qTpesGST3v9NmMTkzV7HHNZRJh52BSqUTErc6L6XGm",
      "storageRebate": "100",
      "content": {
        "dataType": "moveObject",
        "type": "0x0000000000000000000000000000000000000000000000000000000000000009::test::TestField",
        "hasPublicTransfer": true,
        "fields": {}
      }
    }
  },
  "id": 1
}
```
