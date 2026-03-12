# Get Next Nonce by Account

**`POST /{chain}/{network}/blockchain/getNextNonceByAccount`**

Retrieves the next nonce for a specific account.

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
| nonce | string | ✓ | Returns the next nonce value for the queried account. This value increments by 1 for each transaction created. Before creating a new transaction, check this value and use it to specify the transaction's nonce.  |
