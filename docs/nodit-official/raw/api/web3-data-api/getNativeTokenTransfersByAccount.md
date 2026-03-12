# Get Native Token Transfers By Account

**`POST /{chain}/{network}/native/getNativeTokenTransfersByAccount`**

Retrieves the native token transfer history associated with a specific address. The token type may vary depending on the selected chain. (e.g.,
For Bitcoin, you can query BTC transfer history.)

## Parameters

| Name | In | Type | Required | Description |
|------|----|------|----------|-------------|
| chain | path | string | ✓ | Parameter to specify the target chain for the query. |
| network | path | string | ✓ | Parameter to specify the network of the target chain. Supported networks may vary depending on the chain. |

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
| accountAddress | string | ✓ | Parameter specifying the account address to query. |
| relation | string |  | Parameter for filtering transactions where the queried account address is the sender or recipient. - from: Filter by sender only. - to: Filter by recipient only. - both (default): Returns all transactions where the queried address appears in from or to. |
| fromBlock | string |  | Parameter specifying the starting block for the query. You can enter one of the following formats: - block number: Enter block number as a decimal string. - block hash: 64-character hexadecimal (excluding "0x"). - block tag: Use "earliest" or "latest" to specify the first block or the most recent block.  Notes: - If provided without toBlock, results are queried from this block to the latest block. - fromBlock value cannot be greater than toBlock value. - If fromBlock and toBlock have the same value, only that block is queried. - "latest" for fromBlock is only allowed when toBlock is "latest".  |
| toBlock | string |  | Parameter specifying the ending block for the query. You can enter one of the following formats: - block number: Enter block number as a decimal string. - block hash: 64-character hexadecimal (excluding "0x"). - block tag: Use "earliest" or "latest" to specify the first block or the most recent block.  Notes: - If provided without fromBlock, results are queried from genesis block to this block. - toBlock value cannot be less than fromBlock value. - If fromBlock and toBlock have the same value, only that block is queried. - "earliest" for toBlock is only allowed when fromBlock is "earliest".  |
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
| items[].senders | array | ✓ | Array containing detailed information for each sender in the transaction. Each object includes the sender's address, amount sent, and sender index. |
| items[].senders[].index | integer | ✓ | Index of the sender; uniquely identifies the sender in the transaction. Starts at 0; assigned sequentially when multiple senders exist. |
| items[].senders[].address | string | ✓ | Address of the sender. The address used by the sender to send Bitcoin in the transaction; follows P2PKH, P2SH, Bech32, or Bech32m format. |
| items[].senders[].value | string | ✓ | Amount sent by the sender in the transaction, displayed in BTC units. Amount allows up to 8 decimal places. |
| items[].recipients | array | ✓ | Array containing detailed information for each recipient in the transaction. Each object includes the recipient's address, amount received, and recipient index. |
| items[].recipients[].index | integer | ✓ | Index of the recipient; uniquely identifies the recipient in the transaction. Starts at 0; assigned sequentially when multiple recipients exist. |
| items[].recipients[].address | string | ✓ | Address of the recipient. The address used by the recipient to receive Bitcoin; follows P2PKH, P2SH, Bech32, or Bech32m format. |
| items[].recipients[].value | string | ✓ | Amount received by the recipient in the transaction, displayed in BTC units. Amount allows up to 8 decimal places. |
| items[].fee | string | ✓ | Transaction fee; the amount paid by the sender to include the transaction in the blockchain network. Amount is displayed in BTC units. |
| items[].transactionHash | string | ✓ | Unique hash value of the transaction. A 64-character hexadecimal string generated from transaction data; uniquely identifies the transaction. |
| items[].transactionId | string | ✓ | Unique ID of the transaction. A value that uniquely identifies each transaction; displayed as a 64-character hexadecimal string. |
| items[].blockHeight | integer | ✓ | Indicates the height of the block containing the transaction. Integer value representing the order in the blockchain. |
| items[].blockHash | string | ✓ | Unique hash value of the block containing the transaction. A 64-character hexadecimal string generated using the SHA-256 algorithm; uniquely identifies the block. |
| items[].blockTimestamp | integer | ✓ | Creation time of the block containing the transaction. Value in UNIX timestamp format; represents when the block was created in UTC, in seconds. |
