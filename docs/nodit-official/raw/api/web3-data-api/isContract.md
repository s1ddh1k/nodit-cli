# Is Contract

**`POST /{chain}/{network}/blockchain/isContract`**

Checks whether the input address is a contract address or not.

## Parameters

| Name | In | Type | Required | Description |
|------|----|------|----------|-------------|
| chain | path | string | ✓ | Parameter to specify the target chain for the query. |
| network | path | string | ✓ | Parameter to specify the network of the target chain. Supported networks may vary depending on the chain. |

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
| address | string | ✓ | Parameter specifying the address to query. Can be entered as a 40-character hexadecimal string starting with 0x. |

## Response

| Name | Type | Required | Description |
|------|------|----------|-------------|
| result | boolean |  | Returns true if the input address is a contract, false otherwise. |
