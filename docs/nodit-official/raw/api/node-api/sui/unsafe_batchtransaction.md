# unsafe_batchTransaction

**`POST /`**

Create an unsigned batched transaction.

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
| id | integer | ✓ |  |
| jsonrpc | string | ✓ |  |
| method | string | ✓ |  |
| params | array | ✓ |  |

## Response

| Name | Type | Required | Description |
|------|------|----------|-------------|
| jsonrpc | string |  |  |
| id | integer |  |  |
| result | object |  |  |
| result.gas | array | ✓ | the gas objects to be used |
| result.gas[].digest | object | ✓ | Base64 string representing the object digest |
| result.gas[].objectId | object | ✓ | Hex code as string representing the object id |
| result.gas[].version | object | ✓ | Object version. |
| result.inputObjects | array | ✓ | objects to be used in this transaction |
| result.inputObjects[].MovePackage | string | ✓ | Hex string encoding. |
| result.txBytes | string | ✓ | BCS serialized transaction data bytes without its type tag, as base-64 encoded string. |
