# unsafe_payAllSui

**`POST /`**

Send all SUI coins to one recipient. This is for SUI coin only and does not require a separate gas coin object. Specifically, what pay_all_sui does are:
1. accumulate all SUI from input coins and deposit all SUI to the first input coin
2. transfer the updated first coin to the recipient and also use this first coin as gas coin object.
3. the balance of the first input coin after tx is sum(input_coins) - actual_gas_cost.
4. all other input coins other than the first are deleted.

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
