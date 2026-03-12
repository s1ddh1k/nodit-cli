# Get Ledgers Within Range

**`POST /{chain}/{network}/blockchain/getLedgersWithinRange`**

Retrieves the list of Ledgers within a specific range.

## Parameters

| Name | In | Type | Required | Description |
|------|----|------|----------|-------------|
| chain | path | string | ✓ | Parameter to specify the target chain for the query. |
| network | path | string | ✓ | Parameter to specify the network of the target chain. Supported networks may vary depending on the chain. |

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
| fromLedger | string | ✓ | Parameter specifying the starting ledger for the query. You can enter one of the following formats: - **ledger index**: Enter ledger number as a decimal string - **ledger hash**: 64-character hexadecimal string (excluding 0x prefix) - **ledger tag**: Enter "earliest" or "latest" (oldest or most recent ledger)  Notes: - If provided without toLedger, results are queried from this ledger to the latest ledger. - fromLedger value cannot be greater than toLedger value. - If fromLedger and toLedger have the same value, only that ledger is queried. - "latest" for fromLedger is only allowed when toLedger is "latest". |
| toLedger | string | ✓ | Parameter specifying the ending ledger for the query. You can enter one of the following formats: - **ledger index**: Enter ledger number as a decimal string - **ledger hash**: 64-character hexadecimal string (excluding 0x prefix) - **ledger tag**: Enter "earliest" or "latest" (oldest or most recent ledger)  Notes: - If provided without fromLedger, results are queried from genesis ledger to this ledger. - toLedger value cannot be less than fromLedger value. - If fromLedger and toLedger have the same value, only that ledger is queried. - "earliest" for toLedger is only allowed when fromLedger is "earliest". |
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
| items[].ledgerHash | string | ✓ | Unique identifier of the current ledger, a 256-bit (64-digit hexadecimal) cryptographic hash. Used for ledger data integrity verification and identification. |
| items[].ledgerIndex | integer | ✓ | Integer value indicating the creation order of the ledger, used to identify its position (sequence number) on the network. |
| items[].ledgerTimestamp | integer | ✓ | The ledger's close time converted to a Unix timestamp. The close time recorded in the ledger header is expressed in seconds since the Ripple Epoch (January 1, 2000 00:00 UTC), which differs from the Unix Epoch (January 1, 1970 00:00 UTC) by 946,684,800 seconds. Thus ledgerTimestamp is the recorded close time plus 946,684,800 seconds. |
| items[].parentLedgerHash | string | ✓ | 256-bit (64-digit hexadecimal) hash identifying the ledger immediately preceding the current ledger. References the previous ledger's hash to maintain chain integrity. |
| items[].accountHash | string | ✓ | 256-bit (64-digit hexadecimal) hash summarizing all account state information recorded in the ledger. Used to verify consistency and integrity of account data. |
| items[].totalCoins | string | ✓ | Total quantity of XRP (in XRP units) owned by accounts recorded in the ledger. This value excludes XRP burned as transaction fees. The actual circulating XRP may be lower than this value because some accounts may be in a "black hole" state (accounts with no key). |
| items[].transactionHash | string | ✓ | 256-bit (64-digit hexadecimal) Merkle tree root hash summarizing all transaction data included in the ledger. Used for transaction history integrity verification. |
| items[].transactionCount | integer | ✓ | Integer value indicating the total number of transactions included in the ledger. Used to assess the scale and activity of ledger transaction history. |
| items[].transactions | array | ✓ | Array of transaction hashes included in the ledger. Each hash is a 256-bit (64-digit hexadecimal) cryptographic hash uniquely identifying a transaction, used for transaction data integrity and identification. |
