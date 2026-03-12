# unsafe_paySui

**`POST /`**

Send SUI coins to a list of addresses, following a list of amounts. This is for SUI coin only and does not require a separate gas coin object. Specifically, what pay_sui does are:
1. debit each input_coin to create new coin following the order of amounts and assign it to the corresponding recipient.
2. accumulate all residual SUI from input coins left and deposit all SUI to the first input coin, then use the first input coin as the gas coin object.
3. the balance of the first input coin after tx is sum(input_coins) - sum(amounts) - actual_gas_cost
4. all other input coints other than the first one are deleted.

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
