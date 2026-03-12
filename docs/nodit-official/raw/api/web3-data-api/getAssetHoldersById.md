# Get Asset Holders By ID

**`POST /{chain}/{network}/asset/getAssetHoldersById`**

Retrieves the list of holders for a specific TRC-10 asset.

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
| assetId | integer | ✓ | Enter the Asset ID to query. Asset ID must be an integer greater than 0. |
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
| items[].ownerAddress | string | ✓ | TRON address of the account owner. The address is a 34-character Base58 string format starting with "T". |
| items[].balance | string | ✓ | Represents the current balance of the account. The value is a decimal string in SUN, the smallest unit of TRX (1 TRX = 1,000,000 SUN). |
