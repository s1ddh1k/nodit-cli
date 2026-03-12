# Get Token Metadata by Asset Types

**`POST /{chain}/{network}/token/getTokenMetadataByAssetTypes`**

Retrieves token metadata for the specified asset types.

## Parameters

| Name | In | Type | Required | Description |
|------|----|------|----------|-------------|
| chain | path | string | ✓ | Parameter to specify the target chain for the query. |
| network | path | string | ✓ | Parameter to specify the network of the target chain. Supported networks may vary depending on the chain. |

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
| assetTypes | array | ✓ | Enter the Asset Types to query as an array. |
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
| items[].assetType | string |  | A field that represents the unique identifier of the asset. Provided in one of two formats: - Coin: Asset ID in Move struct format (e.g., 0x1::aptos_coin::AptosCoin) - Fungible Asset: Object address that owns the metadata of the asset  |
| items[].tokenStandard | string |  | A field that represents the standard the asset follows. - v1: Assets following the Coin standard - v2: Assets following the Fungible Asset standard  |
| items[].name | string | ✓ | A field that represents the name of the asset. The official name of the asset displayed in the user interface; used to identify and distinguish assets.  |
| items[].symbol | string | ✓ | A field that represents the symbol of the asset. The abbreviated identifier of the asset used in exchanges and wallets.  |
| items[].decimals | integer | ✓ | A field that represents the decimal places of the asset. Defines the number of decimal places applied when displaying the actual asset quantity. For example, if decimals is 8, 100000000 (10^8) is displayed as 1.0.  |
| items[].totalSupply | string | ✓ | A field that represents the total circulating supply. Provided as an integer value before decimals are applied. The asset's decimals must be considered when calculating the actual circulating supply.  |
| items[].maximumSupply | string | ✓ | A field that represents the maximum supply that can be minted. Provided as an integer value before decimals are applied. An empty string ("") means there is no maximum supply limit.  |
| items[].creatorAddress | string | ✓ | A field that represents the address of the account that created the asset. Provided as a 64-character hexadecimal string prefixed with 0x; identifies the asset creator. Used to track asset management authority and ownership.  |
| items[].projectURI | string | ✓ | A field that represents the URI that provides project-related information. A URL is provided where you can find detailed information about the project such as website, whitepaper, and documentation. Used to obtain additional information related to the asset.  |
| items[].iconURI | string | ✓ | A field that represents the icon image URL of the asset. The URL of an image that allows visual identification of the asset is provided.  |
