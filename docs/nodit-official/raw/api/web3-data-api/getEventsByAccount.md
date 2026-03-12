# Get Events by Account

**`POST /{chain}/{network}/blockchain/getEventsByAccount`**

Retrieves the list of events emitted by a specific account.

## Parameters

| Name | In | Type | Required | Description |
|------|----|------|----------|-------------|
| chain | path | string | ✓ | Parameter to specify the target chain for the query. |
| network | path | string | ✓ | Parameter to specify the network of the target chain. Supported networks may vary depending on the chain. |

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
| accountAddress | string | ✓ | Parameter specifying the account address to query. The account address must be entered as a 64-character hexadecimal string (including "0x"). |
| eventTypes | array |  |  |
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
| items[].transactionHash | string | ✓ | A field that represents the hash of the transaction containing the event. A cryptographic hash computed from all transaction data, provided as a 64-character hexadecimal string prefixed with 0x. This hash enables retrieval of full information for the transaction related to the event.  |
| items[].transactionVersion | integer | ✓ | A field that represents the version of the transaction. A unique sequence ID indicating the order of the transaction on the chain, incrementing sequentially from 0. Used to ensure transaction execution order and to query the state of a specific transaction.  |
| items[].blockHeight | integer | ✓ | A field that represents the height of the block containing the transaction. Provided as an integer that increments sequentially from the genesis block (0). Used to identify which block the transaction was included in and to verify if the transaction has been finalized.  |
| items[].blockTimestamp | integer | ✓ | A field that represents the block creation time. Provided as a UNIX timestamp, used for block creation order and time tracking. Block timestamps are used to ensure transaction order and measure inter-block creation intervals.  |
