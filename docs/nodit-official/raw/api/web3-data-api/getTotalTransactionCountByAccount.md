# Get Total Transaction Count By Account

**`POST /{chain}/{network}/blockchain/getTotalTransactionCountByAccount`**

This API returns the total count of transactions in which a specific account address is included as the sender or recipient.

## Parameters

| Name | In | Type | Required | Description |
|------|----|------|----------|-------------|
| chain | path | string | ✓ | Parameter to specify the target chain for the query. |
| network | path | string | ✓ | Parameter to specify the network of the target chain. Supported networks may vary depending on the chain. |

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
| accountAddress | string | ✓ | Parameter specifying the account address to query. The account address must be entered as a 64-character hexadecimal string (including "0x"). |

## Response

| Name | Type | Required | Description |
|------|------|----------|-------------|
| transactionCount | integer |  | Indicates the total count of transactions in which the account participated as sender or recipient. For XRPL and Aptos, it indicates the total count of transactions created by the specific account. |
