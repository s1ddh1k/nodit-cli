# Get Blocks Within Range

**`POST /{chain}/{network}/blockchain/getBlocksWithinRange`**

Retrieves the list of blocks within a specific period or block range.

## Parameters

| Name | In | Type | Required | Description |
|------|----|------|----------|-------------|
| chain | path | string | ✓ | Parameter to specify the target chain for the query. |
| network | path | string | ✓ | Parameter to specify the network of the target chain. Supported networks may vary depending on the chain. |

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
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
| items[].hash | string | ✓ | A field that represents the unique hash identifier of the block. It is a cryptographic hash computed from all data of the block, provided as a 64-character hexadecimal string prefixed with 0x.  |
| items[].height | integer | ✓ | A field that represents the block height. An integer that increments sequentially starting from the genesis block (0), used as an indicator for the blockchain length and the position of a specific block.  |
| items[].timestamp | integer | ✓ | A field that represents when the block was created. Provided as a UNIX timestamp, used for block creation order and time tracking. Block timestamps are used to ensure transaction order and measure inter-block creation intervals.  |
| items[].firstVersion | integer | ✓ | A field that represents the version number of the first transaction processed in the block. In Aptos, each transaction has a unique version number that indicates the global execution order of the transaction. firstVersion and lastVersion allow you to determine the version range of transactions within the block.  |
| items[].lastVersion | integer | ✓ | A field that represents the version number of the last transaction processed in the block. Together with firstVersion, it defines the version range of transactions within the block; this enables sequential retrieval of all transactions in the block.  |
| items[].transactionsCount | integer | ✓ | A field that represents the total number of transactions included in the block. An important indicator for understanding block size and throughput; it includes all transaction types (user transactions, system transactions, etc.). This value equals lastVersion - firstVersion + 1.  |
| items[].transactions | array | ✓ | A field that represents the list of version numbers for all transactions included in the block. Each version number serves as a unique identifier for the transaction, sorted sequentially from firstVersion to lastVersion. This list enables retrieval of detailed information for individual transactions in the block.  |
