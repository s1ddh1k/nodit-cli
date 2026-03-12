# Get Native Balance by Account

**`POST /{chain}/{network}/native/getNativeBalanceByAccount`**

Retrieves the native token balance held by a specific account. The token type may vary depending on the selected chain.

## Parameters

| Name | In | Type | Required | Description |
|------|----|------|----------|-------------|
| chain | path | string | ✓ | Parameter to specify the target chain for the query. |
| network | path | string | ✓ | Parameter to specify the network of the target chain. Supported networks may vary depending on the chain. |

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
| accountAddress | string | ✓ | Parameter specifying the account address to query. Can be entered as a 40-character hexadecimal string starting with 0x. |

## Response

| Name | Type | Required | Description |
|------|------|----------|-------------|
| ownerAddress | string | ✓ | A field representing the owner address. |
| balance | string | ✓ | A field representing the balance. |
