# Get NFT Contract Metadata by Contracts

**`POST /{chain}/{network}/nft/getNftContractMetadataByContracts`**

Retrieves metadata for specific NFT contracts. Multiple contracts can be queried, up to a maximum of 100 contracts.

## Parameters

| Name | In | Type | Required | Description |
|------|----|------|----------|-------------|
| chain | path | string | ✓ | Parameter to specify the target chain for the query. |
| network | path | string | ✓ | Parameter to specify the network of the target chain. Supported networks may vary depending on the chain. |

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
| contractAddresses | array | ✓ | Parameter specifying an array of contract addresses to query. Contract addresses can be entered as 40-character hexadecimal strings starting with 0x. |
