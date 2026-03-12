# sui_tryGetPastObject

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
  "method": "sui_tryGetPastObject",
  "params": [
    "0x11af4b844ff94b3fbef6e36b518da3ad4c5856fa686464524a876b463d129760",
    4,
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
| result | object |  | The object exists and is found with this version |
| result.details | object | ✓ |  |
| result.details.bcs | object |  |  |
| result.details.content | object |  |  |
| result.details.digest | object | ✓ | Base64 string representing the object digest |
| result.details.display | object |  |  |
| result.details.objectId | string | ✓ | Hex string encoding. |
| result.details.owner | object |  | Object is exclusively owned by a single address, and is mutable. |
| result.details.previousTransaction | string |  |  |
| result.details.storageRebate | string |  |  |
| result.details.type | string |  | The type of the object. Default to be None unless SuiObjectDataOptions.showType is set to true |
| result.details.version | object | ✓ | Object version. |
| result.status | string | ✓ |  |

### Example

```json
{
  "jsonrpc": "2.0",
  "result": {
    "status": "VersionFound",
    "details": {
      "objectId": "0x11af4b844ff94b3fbef6e36b518da3ad4c5856fa686464524a876b463d129760",
      "version": "4",
      "digest": "5VPAwDXy3BL72ehFc7gSJoz27ahMd6spUg5YwYc4ibcv",
      "type": "0x2::coin::Coin<0x2::sui::SUI>",
      "owner": {
        "AddressOwner": "0x3568c40e814d9d5396d23087a0fd641e91e0e00df6c012cded9ef9ba5e5bf042"
      },
      "previousTransaction": "5jQByoouHBwaico5pQB73GdbzerC2StjTiHh5garBjiV",
      "storageRebate": "100",
      "content": {
        "dataType": "moveObject",
        "type": "0x2::coin::Coin<0x2::sui::SUI>",
        "hasPublicTransfer": true,
        "fields": {
          "balance": "10000",
          "id": {
            "id": "0x11af4b844ff94b3fbef6e36b518da3ad4c5856fa686464524a876b463d129760"
          }
        }
      }
    }
  },
  "id": 1
}
```
