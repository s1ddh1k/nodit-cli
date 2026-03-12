# Get NFT Metadata by Token IDs

**`POST /{chain}/{network}/nft/getNftMetadataByTokenIds`**

Retrieves metadata for specific NFTs. Multiple NFTs can be queried, up to a maximum of 100 NFTs.


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
| tokens | array | ✓ | List of NFTs to query. Up to 100 NFTs can be queried. |
| tokens[].contractAddress | string | ✓ | Parameter specifying the contract address to query. Can be entered as a 40-character hexadecimal string starting with 0x. |
| tokens[].tokenId | string | ✓ | Parameter specifying the NFT token ID to query. Can be entered as a decimal string. |
