# Get Token Allowance

**`POST /{chain}/{network}/token/getTokenAllowance`**

Retrieves the amount of ERC20 tokens that the owner has approved for the spender.

## Parameters

| Name | In | Type | Required | Description |
|------|----|------|----------|-------------|
| chain | path | string | ✓ | Parameter to specify the target chain for the query. |
| network | path | string | ✓ | Parameter to specify the network of the target chain. Supported networks may vary depending on the chain. |

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
| contractAddress | string | ✓ | Parameter specifying the contract address to query. Can be entered as a 40-character hexadecimal string starting with 0x. |
| ownerAddress | string | ✓ | Parameter specifying the account address to query. Can be entered as a 40-character hexadecimal string starting with 0x. |
| spenderAddress | string | ✓ | Parameter specifying the account address to query. Can be entered as a 40-character hexadecimal string starting with 0x. |

## Response

| Name | Type | Required | Description |
|------|------|----------|-------------|
| allowance | string |  | Returns the remaining number of tokens that the spender can use on behalf of the owner through transferFrom. This value changes when approve or transferFrom is called. |
