# Get Native Transfers By Account

**`POST /{chain}/{network}/native/getNativeTransfersByAccount`**

Retrieves the native token transfer history for a specific account.

## Parameters

| Name | In | Type | Required | Description |
|------|----|------|----------|-------------|
| chain | path | string | ✓ | Parameter to specify the target chain for the query. |
| network | path | string | ✓ | Parameter to specify the network of the target chain. Supported networks may vary depending on the chain. |

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
| accountAddress | string |  | Parameter specifying the account address to query. Can be entered as a 40-character hexadecimal string starting with 0x. |
| relation | string |  | Parameter for filtering transactions where the queried account address is the sender or recipient. - from: Filter by sender only. - to: Filter by recipient only. - both (default): Returns all transactions where the queried address appears in from or to. |
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
| items[].from | string | ✓ | A field representing the address from which the token was transferred. Provided as a 40-character hexadecimal string prefixed with 0x. |
| items[].to | string | ✓ | A field representing the address that received the token. Provided as a 40-character hexadecimal string prefixed with 0x. |
| items[].value | string | ✓ | A field representing the amount of tokens transferred. Provided in decimal string format. |
| items[].timestamp | integer | ✓ | A field representing when the token transfer occurred. This field is provided as a UNIX timestamp. |
| items[].blockNumber | integer | ✓ | A field representing the block number where the token transfer occurred. |
| items[].transactionHash | string | ✓ | A field representing the hash of the token transfer transaction. Provided as a 64-character hexadecimal string prefixed with 0x. |
| items[].logIndex | integer | ✓ | A field representing the log index of the token transfer transaction. Indicates the order of events in the transaction. |
| items[].batchIndex | integer |  | A field representing the batch index of the token transfer transaction. This field is included only in responses for ERC1155 token transfers. |
| items[].function | object |  | A field representing the information of the function called in a transaction containing a Transfer. This field is included in the response only when the withFunction parameter is set to true. |
| items[].function.selector | string | ✓ | A 4-byte function selector value that identifies the called function. The function selector is the first 4 bytes of the Keccak-256 hash of the function signature. Provided as an 8-character hexadecimal string prefixed with 0x. |
| items[].function.name | string |  | A field representing the name of the called function. |
| items[].function.signature | string |  | A field representing the signature of the called function.  |
| items[].function.args | array |  | A field representing the arguments of the called function. |
| items[].function.input | string | ✓ | A field representing the input data of the function. Provided as a hexadecimal string prefixed with 0x. |
