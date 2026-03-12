# Get Token Accounts by Asset Type

**`POST /{chain}/{network}/token/getTokenAccountsByAssetType`**

Retrieves the list of accounts holding a specific asset type.

## Parameters

| Name | In | Type | Required | Description |
|------|----|------|----------|-------------|
| chain | path | string | ✓ | Parameter to specify the target chain for the query. |
| network | path | string | ✓ | Parameter to specify the network of the target chain. Supported networks may vary depending on the chain. |

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
| assetType | string |  | Parameter specifying the asset type to query. The asset type must be entered as a 64-character hexadecimal string (including "0x") using one of the following formats:  - Coin: Asset ID in Struct format such as "0x1::aptos_coin::AptosCoin" - Fungible Asset: Object address that owns the Token Metadata |
| linkedAssetType | string |  | Common identifier for querying Coin and FA (Fungible Asset) as a single asset unit.  linkedAssetType must contain the Object address of the migrated FA, entered in 64-character hexadecimal format (including "0x"). |
| page | integer |  | The page parameter specifies the data page to query. It accepts values up to 100; for pages exceeding 100, use the cursor pagination method.  The page and cursor parameters cannot be used together. If both page and cursor are empty, cursor pagination is used. |
| rpp | integer |  | rpp stands for results per page and specifies the page size. You can specify a number greater than 0 and up to 1000. |
| cursor | string |  | The cursor parameter supports pagination and data navigation between pages. Provide the cursor value from the previous page in the next request to load the next page of data.  The page and cursor parameters cannot be used together. If both page and cursor are empty, cursor pagination is used. |
| withCount | boolean |  | Parameter specifying whether to include the count field in the response. The count field indicates the total number of requested data items. When set to true, the count field is included and response speed may be slower. |

## Response

| Name | Type | Required | Description |
|------|------|----------|-------------|
| page | integer |  | Field representing the page number specified in the page parameter. This field is only included in the response when a value greater than 0 is specified for the page parameter. |
| rpp | integer |  | Field representing the number of results per page specified in the rpp parameter. |
| cursor | string |  | A field for cursor pagination. This value must be provided in the next request to load the next page of data. |
| count | integer |  | Field representing the total count of requested data. This field is only included in the response when true is set for the withCount parameter. |
| items | array | ✓ |  |
| items[].ownerAddress | string | ✓ | A field that represents the address of the account holding the asset. Provided as a 64-character hexadecimal string prefixed with 0x; identifies the actual owner of the asset. For Coin: the holding account address; for Fungible Asset: the Object owner's address.  |
| items[].objectAddress | string | ✓ | A field that represents the address of the Object where the asset is stored. Provided depending on storage location: - Coin: empty string ("") as it is stored directly in the account - Fungible Asset: Object address where the token is stored (64-character hexadecimal string prefixed with 0x)  |
| items[].value | string | ✓ | A field that represents the quantity of the held asset. Provided as a string in integer form; decimals are not included. The asset's decimals must be considered when calculating the actual value.  |
| items[].isFrozen | boolean | ✓ | A field that indicates whether the asset can be transferred. true means the asset is frozen and cannot be transferred. Used to check the transfer restriction status of the asset.  |
| items[].isPrimary | boolean | ✓ | A field that indicates whether this Object is the representative Object for the Owner address. true means this Object is the representative Object for the Owner address.  |
| items[].assetType | string | ✓ | A field that represents the unique identifier of the asset. Provided in one of two formats: - Coin: Asset ID in Move struct format (e.g., 0x1::aptos_coin::AptosCoin) - Fungible Asset: Object address that owns the metadata of the asset  |
| items[].tokenStandard | string | ✓ | A field that represents the standard the asset follows. - v1: Assets following the Coin standard - v2: Assets following the Fungible Asset standard  |
| items[].linkedAssetType | string | ✓ | A common identifier that links Coin and Fungible Asset when they have been migrated. This field serves as a unified key that groups Coin/Fungible Asset into a single asset unit for identification and retrieval, providing the Object address where the migrated Fungible Asset's Metadata is stored.  |
