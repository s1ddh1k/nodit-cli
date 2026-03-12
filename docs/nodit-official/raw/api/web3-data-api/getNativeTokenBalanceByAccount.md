# Get Native Token Balance by Account

**`POST /{chain}/{network}/native/getNativeTokenBalanceByAccount`**

Retrieves the native token balance held by a specific account. The token type may vary depending on the selected chain. (e.g.,
For Bitcoin, you can query the BTC balance.)

## Parameters

| Name | In | Type | Required | Description |
|------|----|------|----------|-------------|
| chain | path | string | ✓ | Parameter to specify the target chain for the query. |
| network | path | string | ✓ | Parameter to specify the network of the target chain. Supported networks may vary depending on the chain. |

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
| accountAddress | string | ✓ | Parameter specifying the account address to query. |

## Response

| Name | Type | Required | Description |
|------|------|----------|-------------|
| ownerAddress | string | ✓ | Field representing the owner address. |
| balance | string | ✓ | Field representing the balance. |
