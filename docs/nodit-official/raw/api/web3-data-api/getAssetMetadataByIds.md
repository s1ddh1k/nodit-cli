# Get Asset Metadata By IDs

**`POST /{chain}/{network}/asset/getAssetMetadataByIds`**

Retrieves metadata for a specific TRC-10 asset.

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
| assetIds | array | ✓ | Enter the list of Asset IDs to query. Each Asset ID must be an integer greater than 0. |
| page | integer |  | The page parameter specifies the data page to query. It accepts values up to 100; for pages exceeding 100, use the cursor pagination method.  The page and cursor parameters cannot be used together. If both page and cursor are empty, cursor pagination is used. |
| rpp | integer |  | rpp stands for results per page and specifies the page size. You can specify a number greater than 0 and up to 1000. |
| cursor | string |  | The cursor parameter supports pagination and data navigation between pages. Provide the cursor value from the previous page in the next request to load the next page of data.  The page and cursor parameters cannot be used together. If both page and cursor are empty, cursor pagination is used. |
| withCount | boolean |  | Parameter specifying whether to include the count field in the response. The count field indicates the total number of requested data items. When set to true, the count field is included and response speed may be slower. |
