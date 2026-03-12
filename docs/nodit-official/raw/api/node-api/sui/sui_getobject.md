# sui_getObject

**`POST /`**

Return the object information for a specified object

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
  "method": "sui_getObject",
  "params": [
    "0x53e4567ccafa5f36ce84c80aa8bc9be64e0d5ae796884274aef3005ae6733809",
    {
      "showType": true,
      "showOwner": true,
      "showPreviousTransaction": true,
      "showDisplay": false,
      "showContent": true,
      "showBcs": false,
      "showStorageRebate": true
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
      "objectId": "0x53e4567ccafa5f36ce84c80aa8bc9be64e0d5ae796884274aef3005ae6733809",
      "version": "1",
      "digest": "33K5ZXJ3RyubvYaHuEnQ1QXmmbhgtrFwp199dnEbL4n7",
      "type": "0x2::coin::Coin<0x2::sui::SUI>",
      "owner": {
        "AddressOwner": "0xc8ec1d5b84dd6289e193b9f88de4a994358c9f856135236c3e75a925e1c77ac3"
      },
      "previousTransaction": "5PLgmQye6rraDYqpV3npV6H1cUXoJZgJh1dPCyRa3WCv",
      "storageRebate": "100",
      "content": {
        "dataType": "moveObject",
        "type": "0x2::coin::Coin<0x2::sui::SUI>",
        "hasPublicTransfer": true,
        "fields": {
          "balance": "100000000",
          "id": {
            "id": "0x53e4567ccafa5f36ce84c80aa8bc9be64e0d5ae796884274aef3005ae6733809"
          }
        }
      }
    }
  },
  "id": 1
}
```
