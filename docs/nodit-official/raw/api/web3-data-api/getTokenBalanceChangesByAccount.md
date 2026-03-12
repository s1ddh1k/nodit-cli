# Get Token Balance Changes by Account

**`POST /{chain}/{network}/token/getTokenBalanceChangesByAccount`**

Retrieves the IOU token balance change history for a specific account.

## Parameters

| Name | In | Type | Required | Description |
|------|----|------|----------|-------------|
| chain | path | string | ✓ | Parameter to specify the target chain for the query. |
| network | path | string | ✓ | Parameter to specify the network of the target chain. Supported networks may vary depending on the chain. |

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
| accountAddress | string | ✓ | Parameter specifying the account address to query. The account address must be entered as a 64-character hexadecimal string (including "0x"). |
| assetTypes | array |  | Specifies the type of asset to query. The asset type must be a 64-character hexadecimal value (including "0x") in one of the following formats:  * Coin: A Struct-format asset ID such as "0x1::aptos_coin::AptosCoin" * Fungible Asset: The Object address that owns the Token Metadata  ⚠️ assetTypes and linkedAssetTypes cannot be used at the same time.  |
| linkedAssetTypes | array |  | A common identifier used to query Coin and FA (Fungible Asset) as a single asset unit.  The linkedAssetType must be the Object address of a migrated FA, entered in 64-character hexadecimal format (including "0x").  ⚠️ assetTypes and linkedAssetTypes cannot be used at the same time.  |
| fromBlock | string |  | Parameter specifying the starting block for the query. You can enter one of the following formats: - block number: Enter block number as a decimal string. - block hash: 64-character hexadecimal (including "0x"). - block tag: Use "earliest" or "latest" to specify the first block or the most recent block.  Notes: - If provided without toBlock, results are queried from this block to the latest block. - fromBlock value cannot be greater than toBlock value. - If fromBlock and toBlock have the same value, only that block is queried. - "latest" for fromBlock is only allowed when toBlock is "latest".  |
| toBlock | string |  | Parameter specifying the ending block for the query. You can enter one of the following formats: - block number: Enter block number as a decimal string. - block hash: 64-character hexadecimal (including "0x"). - block tag: Use "earliest" or "latest" to specify the first block or the most recent block.  Notes: - If provided without fromBlock, results are queried from genesis block to this block. - toBlock value cannot be less than fromBlock value. - If fromBlock and toBlock have the same value, only that block is queried. - "earliest" for toBlock is only allowed when fromBlock is "earliest". |
| fromDate | string |  | Parameter specifying the start date for the query. Date must follow ISO 8601 format (YYYY-MM-DDThh:mm:ss{time zone}) including seconds.  Notes: - If provided without toDate, results are queried from this date to the latest block. - fromDate value must be equal to or earlier than toDate value. - If fromDate and toDate have the same value, only blocks from that date are queried. - This field cannot be used together with fromBlock and toBlock.   |
| toDate | string |  | Parameter specifying the end date for the query. Date must follow ISO 8601 format (YYYY-MM-DDThh:mm:ss{time zone}) including seconds.  Notes: - If provided without fromDate, results are queried from genesis block to this date. - toDate value must be equal to or later than fromDate value. - If fromDate and toDate have the same value, only blocks from that date are queried. - This field cannot be used together with fromBlock and toBlock.   |
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
| items[].blockHeight | integer |  | A field that represents the height of the block containing this balance change. An integer that increments sequentially starting from the genesis block (0), used as an indicator for the blockchain length and the position of a specific block.  |
| items[].blockTimestamp | integer |  | A field that represents the block creation time. Provided as a UNIX timestamp, used for block creation order and time tracking. Block timestamps are used to ensure transaction order and measure inter-block creation intervals.  |
| items[].transactionHash | string |  | A field that represents the hash of the transaction that caused the balance change. A cryptographic hash computed from all transaction data, provided as a 64-character hexadecimal string prefixed with 0x. This hash enables retrieval of full information for the transaction related to the balance change.  |
| items[].transactionVersion | integer |  | A field that represents the version of the transaction where the balance change occurred. A unique sequence ID indicating the order of the transaction on the chain, incrementing sequentially from 0.  |
| items[].eventIndex | integer | ✓ | A field that represents the order index of the event within the transaction. Indicates the order of the current event among multiple events emitted in a single transaction. Starts at 0 and increments in event emission order; used to identify the order of events within the same transaction.  |
| items[].eventType | string | ✓ | A field that represents the type of the event. Provided as the full path (address::module::struct) of the event struct defined in the Move module. Used to identify the type of operation that caused the balance change.  |
| items[].subEventIndex | integer | ✓ | A field that represents the order of asset changes when multiple asset changes occur within a single event. Identifies the order of multiple asset changes within an event sharing the same eventIndex. Starts at 0 and increments sequentially; plays an important role in complex asset operations (e.g., swaps).  |
| items[].accountAddress | string | ✓ | A field that represents the address of the account where the balance change occurred. Provided as a 64-character hexadecimal string prefixed with 0x; identifies the account whose asset balance changed. This address is either the asset owner or the destination account the asset was transferred to.  |
| items[].assetType | string | ✓ | A field that represents the type of the asset whose balance changed. Provided in one of two formats: - Coin: Asset ID in Move struct format (e.g., 0x1::aptos_coin::AptosCoin) - Fungible Asset: Object address that owns the metadata of the asset Used to uniquely identify the type of asset.  |
| items[].linkedAssetType | string | ✓ | A common identifier that links Coin and Fungible Asset when they have been migrated. This field serves as a unified key that groups Coin/Fungible Asset into a single asset unit for identification and retrieval, providing the Object address where the migrated Fungible Asset's Metadata is stored.  |
| items[].changeValue | string | ✓ | A field that represents the balance change amount. Provided as a string in integer form. - Positive: indicates the account balance increased - Negative: indicates the account balance decreased The actual value must be interpreted considering decimals.  |
| items[].tokenStandard | string | ✓ | A field that represents the standard type of the asset. - v1: Assets following the Coin standard - v2: Assets following the Fungible Asset standard  |
| items[].transferType | string | ✓ | A field that represents the transfer direction or action type. Indicates the specific type of balance change; the following values are possible: - deposit: asset deposit - withdraw: asset withdrawal - swap_in: deposit from swap - swap_out: withdrawal from swap - gas: gas fee payment - refund_gas: gas fee refund  |
