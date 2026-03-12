# Get Token Holders By Contract

**`POST /{chain}/{network}/token/getTokenHoldersByContract`**

Retrieves the holder list for a specific ERC20 token contract. The holder list includes holder addresses and the quantity of tokens held by each holder.

## Parameters

| Name | In | Type | Required | Description |
|------|----|------|----------|-------------|
| chain | path | string | ✓ | Parameter to specify the target chain for the query. |
| network | path | string | ✓ | Parameter to specify the network of the target chain. Supported networks may vary depending on the chain. |

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
| contractAddress | string | ✓ | Parameter specifying the contract address to query. Can be entered as a 40-character hexadecimal string starting with 0x. |
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
| items[].ownerAddress | string | ✓ | A field representing the owner address. |
| items[].balance | string | ✓ | A field representing the balance. |
| items[].lastTransferredAt | string | ✓ | A field representing the time when the token or NFT was last transferred to the owner in ISO 8601 format. Only reflects transfers made via standard token Transfer events (ERC20, ERC721, ERC1155); transfers via non-standard events may not be reflected. |
