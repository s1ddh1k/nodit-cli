# Get Asset Transfers By Account

**`POST /{chain}/{network}/asset/getAssetTransfersByAccount`**

Retrieves the TRC-10 asset transfer history for a specific account.

## Parameters

| Name | In | Type | Required | Description |
|------|----|------|----------|-------------|
| chain | path | string | ✓ | Parameter to specify the target chain for the query. |
| network | path | string | ✓ | Parameter to specify the network of the target chain. Supported networks may vary depending on the chain. |

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
| accountAddress | string | ✓ | Parameter specifying the account address to query. This field uses a 34-character base58 string format starting with "T". |
| assetIds | array |  | Enter the list of Asset IDs to query. Each Asset ID must be an integer greater than 0. |
| fromBlock | string |  | Parameter specifying the starting point for the query. You can enter block number, block hash, or block tag. Default is 0. - block number: Enter as a decimal string. - block hash: Enter as a 64-character hexadecimal string, excluding the 0x prefix. - block tag: You can enter "earliest" for fromBlock.  Notes: - If only fromBlock is provided without toBlock, results are queried from fromBlock to the most recent block. - fromBlock's block number must be less than or equal to toBlock's block number. - If toBlock and fromBlock have the same value, only that single block's results are queried. - "latest" cannot be entered for fromBlock.  |
| toBlock | string |  | Parameter specifying the ending point for the query. You can enter block number, block hash, or block tag. Default is "latest". - block number: Enter as a decimal string. - block hash: Enter as a 64-character hexadecimal string, excluding the 0x prefix. - block tag: You can enter "latest".  Notes: - The blockNumber for toBlock must be greater than or equal to the blockNumber for fromBlock. - If toBlock and fromBlock have the same value, only that single block's results are queried. - If only toBlock is provided without fromBlock, results are queried from genesis block to the time specified in toBlock. - "earliest" cannot be entered for toBlock. |
| fromDate | string |  | Parameter specifying the start date for the query. Date must follow ISO 8601 format (YYYY-MM-DDThh:mm:ss{time zone}) including seconds.  Notes: - If provided without toDate, results are queried from this date to the latest block. - fromDate value must be equal to or earlier than toDate value. - If fromDate and toDate have the same value, only blocks from that date are queried. - This field cannot be used together with fromBlock and toBlock.   |
| toDate | string |  | Parameter specifying the end date for the query. Date must follow ISO 8601 format (YYYY-MM-DDThh:mm:ss{time zone}) including seconds.  Notes: - If provided without fromDate, results are queried from genesis block to this date. - toDate value must be equal to or later than fromDate value. - If fromDate and toDate have the same value, only blocks from that date are queried. - This field cannot be used together with fromBlock and toBlock.   |
| page | integer |  | The page parameter specifies the data page to query. It accepts values up to 100; for pages exceeding 100, use the cursor pagination method.  The page and cursor parameters cannot be used together. If both page and cursor are empty, cursor pagination is used. |
| rpp | integer |  | rpp stands for results per page and specifies the page size. You can specify a number greater than 0 and up to 1000. |
| cursor | string |  | The cursor parameter supports pagination and data navigation between pages. Provide the cursor value from the previous page in the next request to load the next page of data.  The page and cursor parameters cannot be used together. If both page and cursor are empty, cursor pagination is used. |
| withCount | boolean |  | Parameter specifying whether to include the count field in the response. The count field indicates the total number of requested data items. When set to true, the count field is included and response speed may be slower. |
| withZeroValue | boolean |  | Parameter specifying whether to include results with value 0 in the response. Set to true for faster responses. |

## Response

| Name | Type | Required | Description |
|------|------|----------|-------------|
| page | integer |  | Field representing the page number specified in the page parameter. This field is only included in the response when a value greater than 0 is specified for the page parameter. |
| rpp | integer |  | Field representing the number of results per page specified in the rpp parameter. |
| cursor | string |  | A field for cursor pagination. This value must be provided in the next request to load the next page of data. |
| count | integer |  | Field representing the total count of requested data. This field is only included in the response when true is set for the withCount parameter. |
| items | array | ✓ |  |
| items[].from | string | ✓ | Represents the address that initiated the transaction. This address refers to the account or smart contract that sends assets. This field has a 34-character base58 string format starting with "T". |
| items[].to | string | ✓ | Represents the recipient address of the transaction. This address refers to the account or smart contract that receives assets. This field has a 34-character base58 string format starting with "T". |
| items[].value | string | ✓ | Represents the amount of assets or data transferred. For TRC10 and TRC20, it indicates the amount transferred. |
| items[].blockTimestamp | integer | ✓ | Indicates when the block containing the transaction was created. Recorded in UNIX timestamp format in milliseconds, used to verify when the transaction occurred. |
| items[].blockNumber | integer | ✓ | Indicates the block number containing the transaction. Used to trace the transaction on the blockchain and identify the transaction's position within a specific block. |
| items[].transactionHash | string | ✓ | Represents the unique hash of the transaction. Used as the transaction identifier and serves as the key to trace all data related to the transaction. |
| items[].transactionIndex | integer | ✓ | Indicates the position of the transaction within the block containing it. Used to identify the order of the transaction among multiple transactions in a block. |
| items[].logIndex | integer |  | Indicates the index of the log containing this Transfer event. Used to identify a specific log when multiple logs exist within the same block. |
| items[].internalTransactionIndex | integer |  | Indicates the index of the internal transaction related to TRX or TRC10 transfer. Used to distinguish among multiple internal transactions within a block. <strong style='color: red;'>*</strong> When this value is "-1", it indicates a Root Transaction. |
| items[].exchangeId | integer |  | Unique ID that identifies a swap transaction on the decentralized exchange (DEX) provided by the TRON network. Used to distinguish specific swap transactions and track which asset pair was traded. Returns null when the transfer is not from a swap transaction. |
| items[].asset | object |  |  |
| items[].asset.id | integer | ✓ | Represents the unique ID of the TRC10 token. This ID is used to identify a specific TRC10 asset issued on the TRON network. |
| items[].asset.name | string | ✓ | Represents the name of the contract. Returns an empty string if the contract does not follow the standard or if the name ("name") was not provided during contract creation. |
| items[].asset.symbol | string | ✓ | Represents the symbol of the contract. Returns an empty string if the contract does not follow the standard or if the symbol ("symbol") was not provided during contract creation. |
| items[].asset.totalSupply | string | ✓ | Represents the total supply of the contract. This value is the total token issuance and is displayed as a decimal string. |
| items[].asset.decimals | integer | ✓ | Represents the number of decimal places for the token. Before the TIP10 standard was introduced, the "precision" value was set to 0. |
| items[].asset.startTime | integer | ✓ | Represents the ICO (Initial Coin Offering) start time. This value is recorded as a UNIX timestamp in milliseconds. |
| items[].asset.endTime | integer | ✓ | Represents the ICO end time. This value is also recorded as a UNIX timestamp in milliseconds. |
| items[].asset.description | string | ✓ | Represents the description of the TRC10 token. This value is set by the issuer and may be subject to change. |
| items[].asset.url | string | ✓ | Represents the URL for the TRC10 token. This value is also set by the issuer and may be subject to change. |
| items[].asset.blockNumber | integer | ✓ | Represents the block number in which the TRC10 token was created. This is the block number containing the token creation transaction, used to trace the transaction on the blockchain. |
| items[].asset.blockTimestamp | integer | ✓ | Represents when the TRC10 token was created. Recorded as a UNIX timestamp in milliseconds, used to determine when the asset was created. |
| items[].asset.transactionHash | string | ✓ | Represents the hash of the transaction that issued the TRC10 token. Used to identify and query information about the specific TRC10 issuance transaction. |
| items[].asset.deployer | string | ✓ | Represents the account or address that issued the TRC10 token. Identifies the issuer's owning account and serves as the basis for all related asset management and information queries. |
| items[].asset.trxNum | integer | ✓ | Represents the quantity of TRX used to determine the value of the TRC10 asset. This value is calculated as the "trx_num/num" ratio, with the unit in sun. |
| items[].asset.num | integer | ✓ | Represents the quantity of TRC10 used to determine the value of the TRC10 asset. The TRC10 asset value is calculated as the "trx_num/num" ratio. |
