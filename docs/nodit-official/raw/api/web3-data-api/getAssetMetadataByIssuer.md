# Get Asset Metadata By Issuer

**`POST /{chain}/{network}/asset/getAssetMetadataByIssuer`**

Retrieves metadata for TRC-10 assets issued by a specific issuer.

> 🚧 What is TRC10?
>
> TRC10 is the native token standard supported by Tron. Tokens issued under this standard are created according to the specification built into the Tron chain itself without using smart contracts. Thanks to this structural characteristic, tokens can be issued easily without complex processes, and each token is identified by a unique Asset ID.

## Parameters

| Name | In | Type | Required | Description |
|------|----|------|----------|-------------|
| chain | path | string | ✓ | Parameter to specify the target chain for the query. |
| network | path | string | ✓ | Parameter to specify the network of the target chain. Supported networks may vary depending on the chain. |

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
| issuer | string | ✓ | Parameter specifying the issuer address to query. This field uses a 34-character base58 string format starting with "T". |

## Response

| Name | Type | Required | Description |
|------|------|----------|-------------|
| id | integer | ✓ | Represents the unique ID of the TRC10 token. This ID is used to identify a specific TRC10 asset issued on the TRON network. |
| name | string | ✓ | Represents the name of the contract. Returns an empty string if the contract does not follow the standard or if the name ("name") was not provided during contract creation. |
| symbol | string | ✓ | Represents the symbol of the contract. Returns an empty string if the contract does not follow the standard or if the symbol ("symbol") was not provided during contract creation. |
| totalSupply | string | ✓ | Represents the total supply of the contract. This value is the total token issuance and is displayed as a decimal string. |
| decimals | integer | ✓ | Represents the number of decimal places for the token. Before the TIP10 standard was introduced, the "precision" value was set to 0. |
| startTime | integer | ✓ | Represents the ICO (Initial Coin Offering) start time. This value is recorded as a UNIX timestamp in milliseconds. |
| endTime | integer | ✓ | Represents the ICO end time. This value is also recorded as a UNIX timestamp in milliseconds. |
| description | string | ✓ | Represents the description of the TRC10 token. This value is set by the issuer and may be subject to change. |
| url | string | ✓ | Represents the URL for the TRC10 token. This value is also set by the issuer and may be subject to change. |
| blockNumber | integer | ✓ | Represents the block number in which the TRC10 token was created. This is the block number containing the token creation transaction, used to trace the transaction on the blockchain. |
| blockTimestamp | integer | ✓ | Represents when the TRC10 token was created. Recorded as a UNIX timestamp in milliseconds, used to determine when the asset was created. |
| transactionHash | string | ✓ | Represents the hash of the transaction that issued the TRC10 token. Used to identify and query information about the specific TRC10 issuance transaction. |
| deployer | string | ✓ | Represents the account or address that issued the TRC10 token. Identifies the issuer's owning account and serves as the basis for all related asset management and information queries. |
| trxNum | integer | ✓ | Represents the quantity of TRX used to determine the value of the TRC10 asset. This value is calculated as the "trx_num/num" ratio, with the unit in sun. |
| num | integer | ✓ | Represents the quantity of TRC10 used to determine the value of the TRC10 asset. The TRC10 asset value is calculated as the "trx_num/num" ratio. |
