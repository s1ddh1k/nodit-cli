# Search Asset Metadata By Keyword

**`POST /{chain}/{network}/asset/searchAssetMetadataByKeyword`**

Searches for TRC-10 asset metadata by keyword.

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
| items[].id | integer | ✓ | Represents the unique ID of the TRC10 token. This ID is used to identify a specific TRC10 asset issued on the TRON network. |
| items[].name | string | ✓ | Represents the name of the contract. Returns an empty string if the contract does not follow the standard or if the name ("name") was not provided during contract creation. |
| items[].symbol | string | ✓ | Represents the symbol of the contract. Returns an empty string if the contract does not follow the standard or if the symbol ("symbol") was not provided during contract creation. |
| items[].totalSupply | string | ✓ | Represents the total supply of the contract. This value is the total token issuance and is displayed as a decimal string. |
| items[].decimals | integer | ✓ | Represents the number of decimal places for the token. Before the TIP10 standard was introduced, the "precision" value was set to 0. |
| items[].startTime | integer | ✓ | Represents the ICO (Initial Coin Offering) start time. This value is recorded as a UNIX timestamp in milliseconds. |
| items[].endTime | integer | ✓ | Represents the ICO end time. This value is also recorded as a UNIX timestamp in milliseconds. |
| items[].description | string | ✓ | Represents the description of the TRC10 token. This value is set by the issuer and may be subject to change. |
| items[].url | string | ✓ | Represents the URL for the TRC10 token. This value is also set by the issuer and may be subject to change. |
| items[].blockNumber | integer | ✓ | Represents the block number in which the TRC10 token was created. This is the block number containing the token creation transaction, used to trace the transaction on the blockchain. |
| items[].blockTimestamp | integer | ✓ | Represents when the TRC10 token was created. Recorded as a UNIX timestamp in milliseconds, used to determine when the asset was created. |
| items[].transactionHash | string | ✓ | Represents the hash of the transaction that issued the TRC10 token. Used to identify and query information about the specific TRC10 issuance transaction. |
| items[].deployer | string | ✓ | Represents the account or address that issued the TRC10 token. Identifies the issuer's owning account and serves as the basis for all related asset management and information queries. |
| items[].trxNum | integer | ✓ | Represents the quantity of TRX used to determine the value of the TRC10 asset. This value is calculated as the "trx_num/num" ratio, with the unit in sun. |
| items[].num | integer | ✓ | Represents the quantity of TRC10 used to determine the value of the TRC10 asset. The TRC10 asset value is calculated as the "trx_num/num" ratio. |
