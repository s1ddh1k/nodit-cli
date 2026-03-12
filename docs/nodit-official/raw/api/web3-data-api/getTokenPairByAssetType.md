# Get Token Pair by Asset Type

**`POST /{chain}/{network}/token/getTokenPairByAssetType`**

Retrieves token pair information for a specific asset type.

## Parameters

| Name | In | Type | Required | Description |
|------|----|------|----------|-------------|
| chain | path | string | ✓ | Parameter to specify the target chain for the query. |
| network | path | string | ✓ | Parameter to specify the network of the target chain. Supported networks may vary depending on the chain. |

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
| assetType | string | ✓ | Parameter specifying the asset type to query. The asset type must be entered as a 64-character hexadecimal string (including "0x") using one of the following formats:  - Coin: Asset ID in Struct format such as "0x1::aptos_coin::AptosCoin" - Fungible Asset: Object address that owns the Token Metadata |

## Response

| Name | Type | Required | Description |
|------|------|----------|-------------|
| coinAssetType | string | ✓ | A field that represents the type of the asset belonging to the Coin standard. The Coin standard is provided in 'moduleAddress::moduleName::structName' format.  |
| fungibleAssetType | string | ✓ | A field that represents the type of the asset belonging to the Fungible Asset standard. The Fungible Asset standard is provided as a 64-character hexadecimal string prefixed with 0x; the Object address that owns the metadata of the asset is provided.  |
| linkedAssetType | string | ✓ | A common identifier that links Coin and Fungible Asset when they have been migrated. This field serves as a unified key that groups Coin/Fungible Asset into a single asset unit for identification and retrieval, providing the Object address where the migrated Fungible Asset's Metadata is stored.  |
