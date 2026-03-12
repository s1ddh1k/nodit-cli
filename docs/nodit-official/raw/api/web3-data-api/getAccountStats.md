# Get Account Stats

**`POST /{chain}/{network}/stats/getAccountStats`**

Retrieves statistics for a specific address. Account activity can be analyzed through transaction data, transfer events, assets, and other information for the account.

## Parameters

| Name | In | Type | Required | Description |
|------|----|------|----------|-------------|
| chain | path | string | ✓ | Parameter to specify the target chain for the query. |
| network | path | string | ✓ | Parameter to specify the network of the target chain. Supported networks may vary depending on the chain. |

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
| address | string | ✓ | Parameter specifying the address to query. Can be entered as a 40-character hexadecimal string starting with 0x. |

## Response

| Name | Type | Required | Description |
|------|------|----------|-------------|
| transactionCounts | object |  | An object containing transaction count information associated with the queried address. |
| transactionCounts.external | number |  | The number of external transactions where the queried address is included in the from or to field. |
| transactionCounts.internal | number |  | The number of internal transactions where the queried address is included in the from or to field. |
| transferCounts | object |  | An object containing transfer event count information associated with the queried address. |
| transferCounts.tokens | number |  | The number of ERC20 token transfer events where the queried address is included in the from or to field. |
| transferCounts.nfts | number |  | The number of NFT (ERC721, ERC1155) transfer events where the queried address is included in the from or to field. |
| assets | object |  | An object containing asset information owned by the queried address. |
| assets.tokens | number |  | The number of ERC20 token types held by the queried address. |
| assets.nfts | number |  | The number of NFTs (ERC721, ERC1155) held by the queried address. |
| assets.nftContracts | number |  | The number of NFT contract types (ERC721, ERC1155) owned by the queried address. |
