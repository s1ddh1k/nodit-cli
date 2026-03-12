# Get NFTs Owned By Account

**`POST /{chain}/{network}/nft/getNftsOwnedByAccount`**

Retrieves the list of NFTs held by a specific account. Results include the quantity held per token and token metadata.


> 📘 What are NFT Metadata and Token URI?
>
> NFTs are a technology that ensures ownership and uniqueness of digital assets. NFT metadata is a critical component that stores information about these assets. Metadata plays an important role in describing the essence of an NFT.
> 1. NFT Metadata
> Includes the NFT's creator, issuance date, attributes, and media files (e.g., images, videos). This metadata makes an NFT more than just a digital file—it defines a unique asset and establishes the NFT's scarcity or distinctive characteristics.
> 2. Token URI
> Storing data on the blockchain is costly. Therefore, metadata and media files are not stored on the blockchain but are kept in external storage such as IPFS or S3. Token URI is a field that points to the location of the metadata file in such external storage. Metadata is accessed via this URI. In other words, Token URI links metadata and media files, serving as the bridge between on-chain information and actual content.

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
| withMetadata | boolean |  | Parameter specifying whether to include NFT token metadata fields (rawMetadata, metadataSyncedAt) in the response. Response speed may be slower when set to true. |

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
| items[].tokenId | string | ✓ | A field representing the unique ID of the NFT token. This ID is used to identify the NFT. |
| items[].tokenUri | string | ✓ | A field representing the URI where the original metadata of the NFT is located. Format may vary by contract implementation, but is typically provided as an IPFS address or web URL. |
| items[].tokenUriSyncedAt | string | ✓ | A field representing when the tokenUri of the NFT was synchronized. |
| items[].rawMetadata | string |  | A field representing the rawMetadata of the NFT. Included in the response only when the withMetadata parameter is set to true. |
| items[].metadataSyncedAt | string |  | A field representing when the metadata of the NFT was synchronized. Included in the response only when the withMetadata parameter is set to true. |
| items[].contract | object |  |  |
