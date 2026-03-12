# sui_tryMultiGetPastObjects

**`POST /`**

Note there is no software-level guarantee/SLA that objects with past versions can be retrieved by this API, even if the object and version exists/existed. The result may vary across nodes depending on their pruning policies. Return the object information for a specified version

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
  "method": "sui_tryMultiGetPastObjects",
  "params": [
    [
      {
        "objectId": "0x38b3186a7bb26a1ab2c982a0a9b482aa70f5a010fffc60f20194ef0f597474e8",
        "version": "4"
      },
      {
        "objectId": "0xceaf9ee4582d3a233101e322a22cb2a5bea2e681ea5af4e59bd1abb0bb4fcb27",
        "version": "12"
      }
    ],
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
| result | array |  |  |
| result[].details | object | ✓ |  |
| result[].details.bcs | object |  |  |
| result[].details.content | object |  |  |
| result[].details.digest | object | ✓ | Base64 string representing the object digest |
| result[].details.display | object |  |  |
| result[].details.objectId | string | ✓ | Hex string encoding. |
| result[].details.owner | object |  | Object is exclusively owned by a single address, and is mutable. |
| result[].details.previousTransaction | string |  |  |
| result[].details.storageRebate | string |  |  |
| result[].details.type | string |  | The type of the object. Default to be None unless SuiObjectDataOptions.showType is set to true |
| result[].details.version | object | ✓ | Object version. |
| result[].status | string | ✓ |  |

### Example

```json
{
  "jsonrpc": "2.0",
  "result": [
    {
      "status": "VersionFound",
      "details": {
        "objectId": "0x38b3186a7bb26a1ab2c982a0a9b482aa70f5a010fffc60f20194ef0f597474e8",
        "version": "4",
        "digest": "hvBGBXvKVhC7XYgVPujuiLjxASR6UGAkSFrCRtVxX1F",
        "type": "0x2::coin::Coin<0x2::sui::SUI>",
        "owner": {
          "AddressOwner": "0x47866ff92885a3c21a7703f564721c198308aa0c71b771ada6b96c16fc9c0fa7"
        },
        "previousTransaction": "6heEteheiLZcS8iVNXsNUnU7oVjzT7UHYzprGcuWQ4gG",
        "storageRebate": "100",
        "content": {
          "dataType": "moveObject",
          "type": "0x2::coin::Coin<0x2::sui::SUI>",
          "hasPublicTransfer": true,
          "fields": {
            "balance": "10000",
            "id": {
              "id": "0x38b3186a7bb26a1ab2c982a0a9b482aa70f5a010fffc60f20194ef0f597474e8"
            }
          }
        }
      }
    },
    {
      "status": "VersionFound",
      "details": {
        "objectId": "0xceaf9ee4582d3a233101e322a22cb2a5bea2e681ea5af4e59bd1abb0bb4fcb27",
        "version": "12",
        "digest": "B5z4YkAgTi78fdxMbxG3fv2V4YBkhpc8PRCPz8MzLtbf",
        "type": "0x2::coin::Coin<0x2::sui::SUI>",
        "owner": {
          "AddressOwner": "0xa6ced287081357950315a8842c3870f2d83f980fe0996a92d351d6749a0a0b47"
        },
        "previousTransaction": "BLN2oUCHmwmaAXXCxbojTcozUqZYfvXx4Bkgi7xcgyVc",
        "storageRebate": "100",
        "content": {
          "dataType": "moveObject",
          "type": "0x2::coin::Coin<0x2::sui::SUI>",
          "hasPublicTransfer": true,
          "fields": {
            "balance": "20000",
            "id": {
              "id": "0x38b3186a7bb26a1ab2c982a0a9b482aa70f5a010fffc60f20194ef0f597474e8"
            }
          }
        }
      }
    }
  ],
  "id": 1
}
```
