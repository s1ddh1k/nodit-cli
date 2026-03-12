# Get Address By ENS Name

**`POST /{chain}/{network}/ens/getAddressByEnsName`**

Retrieves the mapped address by entering the ENS domain name.


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
| name | string |  | Enter the ENS domain name. |

## Response

| Name | Type | Required | Description |
|------|------|----------|-------------|
| address | string |  | The address that the ENS domain points to. Returns null if the domain does not exist. If domain renewal or new registration does not occur after the expiry date, the address previously pointed to by the domain is returned. |
| expiryDate | string |  | Indicates the expiry date of the ENS domain. If domain renewal or new registration does not occur after the expiry date, the previous expiry date is returned. |
