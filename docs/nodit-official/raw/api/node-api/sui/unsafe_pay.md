# unsafe_pay

**`POST /`**

Send Coin<T> to a list of addresses, where T can be any coin type, following a list of amounts, The object specified in the gas field will be used to pay the gas fee for the transaction. The gas object can not appear in input_coins. If the gas object is not specified, the RPC server will auto-select one.

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
