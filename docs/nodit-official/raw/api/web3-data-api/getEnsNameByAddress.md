# Get ENS Name By Address

**`POST /{chain}/{network}/ens/getEnsNameByAddress`**

Returns the mapped ENS domain name by entering an address.


> 🚧 Seeing null even though the address has an ENS domain name?
>
> Multiple domain names can point to a single address. This API returns only the name registered as the Primary Name. If you want to check all names, try [Get Ens Records By Account](ref:getensrecordsbyaccount)!


> 🚧 Please verify the network when using!
>
> This API is supported only on Ethereum Mainnet and cannot be used on other networks. Please verify the network when using.

## Parameters

| Name | In | Type | Required | Description |
|------|----|------|----------|-------------|
| chain | path | string | ✓ | Parameter to specify the target chain for the query. |
| network | path | string | ✓ | Parameter to specify the network of the target chain. Supported networks may vary depending on the chain. |

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
| address | string | ✓ | Parameter specifying the account address to query. Can be entered as a 40-character hexadecimal string starting with 0x. |

## Response

| Name | Type | Required | Description |
|------|------|----------|-------------|
| name | string |  | The ENS domain name mapped to the address. Returns null if no ENS domain is mapped to the address. If domain renewal or new registration does not occur after the expiry date, the previous domain name is displayed. |
| expiryDate | string |  | Indicates the expiry date of the ENS domain. If domain renewal or new registration does not occur after the expiry date, the previous expiry date is returned. |
