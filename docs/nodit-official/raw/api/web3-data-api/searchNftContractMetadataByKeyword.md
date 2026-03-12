# Search NFT Contract Metadata By Keyword

**`POST /{chain}/{network}/nft/searchNftContractMetadataByKeyword`**

Retrieves the list of contracts matching the NFT contract name or symbol.


> 🚧 Getting a 429 error? Check your subscription plan!
> 429 errors may occur when the Throughput limit of your subscription plan is exceeded.
> For example, the Starter plan has a limit of 300 CU per second, so if you use the Starter plan and call an API that consumes 350 CU, you may get a 429 error.
> Check the CU consumption of the API you are using on the Compute Unit Costs page, and consider upgrading to a higher plan if you need more Throughput!
> 👉 [Go to Compute Unit Costs page](/guides/usage-measuringcu)

## Parameters

| Name | In | Type | Required | Description |
|------|----|------|----------|-------------|
| chain | path | string | ✓ | Parameter to specify the target chain for the query. |
| network | path | string | ✓ | Parameter to specify the network of the target chain. Supported networks may vary depending on the chain. |

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
| keyword | string | ✓ | Parameter specifying the name or symbol of the token to query. |
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
| items[].address | string | ✓ | A field representing the contract address. |
| items[].deployedTransactionHash | string | ✓ | A field representing the hash of the transaction in which the contract was deployed. |
| items[].deployedAt | string | ✓ | A field representing the time when the contract was deployed. This field is provided in ISO 8601 date and time format. |
| items[].deployerAddress | string | ✓ | A field representing the address that deployed the contract. |
| items[].logoUrl | string | ✓ | A field representing the logo URL of the contract. This URL can be used to visually identify the contract.   [Note]  Not all contracts have logo images available. If a contract has no logo, this field is provided as null. |
| items[].type | string | ✓ | A field representing the type of the contract. Contract types are denoted by standard specification names (e.g., ERC721, ERC1155, ERC20, ERC721, ERC1155, ERC20). |
| items[].name | string | ✓ | A field representing the name of the contract. Returns an empty string if the contract does not follow the standard or if name was not provided at contract creation. |
| items[].symbol | string | ✓ | A field representing the symbol of the contract. Returns an empty string if the contract does not follow the standard or if symbol was not provided at contract creation. |
| items[].totalSupply | string |  | A field representing the total supply of the contract. This value indicates the total token supply and is provided as a decimal string. Included in the response only when the contract type is ERC20. |
| items[].decimals | integer |  | A field representing the decimal places of the contract. Included in the response only when the contract type is ERC20 or ERC1155. |
