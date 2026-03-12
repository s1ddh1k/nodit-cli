# Get NFT Contracts by Account

**`POST /{chain}/{network}/nft/getNftContractsByAccount`**

Retrieves NFTs held by a specific account, grouped by contract. Results include the quantity of NFTs per contract and contract metadata.

## Parameters

| Name | In | Type | Required | Description |
|------|----|------|----------|-------------|
| chain | path | string | ✓ | Parameter to specify the target chain for the query. |
| network | path | string | ✓ | Parameter to specify the network of the target chain. Supported networks may vary depending on the chain. |

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
| accountAddress | string | ✓ | Parameter specifying the account address to query. Can be entered as a 40-character hexadecimal string starting with 0x. |
| contractAddresses | array |  | Parameter specifying an array of contract addresses to query. Contract addresses can be entered as 40-character hexadecimal strings starting with 0x. |
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
| items[].totalBalance | string | ✓ | Returns the sum of all balance quantities for the contract.   [Note]  The quantity calculation method may vary depending on the contract type. For ERC721, it equals uniqueBalance; for ERC1155, it returns the sum of all quantities.   (Example)  ERC721: Returns 3 if holding #100, #200, #300  ERC1155: Returns 600 if holding 100 of #1, 200 each of #200 and #300 |
| items[].uniqueBalance | string | ✓ | Returns the number of distinct token IDs among the NFTs held by the contract.   [Note]  The quantity calculation method may vary depending on the contract type. For ERC721, it equals totalBalance; for ERC1155, it returns the number of unique token IDs.   (Example)  ERC721: Returns 3 if holding #100, #200, #300  ERC1155: Returns 3 if holding 100 of #1, 200 each of #200 and #300 |
| items[].contract | object |  |  |
