# Get Internal Transactions By Transaction Hash

**`POST /{chain}/{network}/blockchain/getInternalTransactionsByTransactionHash`**

Retrieves the list of internal transactions that occurred in a specific transaction.

## Parameters

| Name | In | Type | Required | Description |
|------|----|------|----------|-------------|
| chain | path | string | ✓ | Parameter to specify the target chain for the query. |
| network | path | string | ✓ | Parameter to specify the network of the target chain. Supported networks may vary depending on the chain. |

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
| transactionHash | string | ✓ | Parameter specifying the transaction hash to query. Can be entered as a 64-character hexadecimal string starting with 0x. |
| page | integer |  | The page parameter specifies the data page to query. It accepts values up to 100; for pages exceeding 100, use the cursor pagination method.  The page and cursor parameters cannot be used together. If both page and cursor are empty, cursor pagination is used. |
| rpp | integer |  | rpp stands for results per page and specifies the page size. You can specify a number greater than 0 and up to 1000. |
| cursor | string |  | The cursor parameter supports pagination and data navigation between pages. Provide the cursor value from the previous page in the next request to load the next page of data.  The page and cursor parameters cannot be used together. If both page and cursor are empty, cursor pagination is used. |
| withCount | boolean |  | Parameter specifying whether to include the count field in the response. The count field indicates the total number of requested data items. When set to true, the count field is included and response speed may be slower. |
| withZeroValue | boolean |  | Parameter specifying whether to include results with value 0 in the response. Set to true for faster responses. |
| withExternalTransaction | boolean |  | Parameter determining whether to include external transactions in the response. External transactions are returned in the same format as internal transactions with trace index 0. When set to true, external transactions are included and response speed may be slower. |

## Response

| Name | Type | Required | Description |
|------|------|----------|-------------|
| page | integer |  | Field representing the page number specified in the page parameter. This field is only included in the response when a value greater than 0 is specified for the page parameter. |
| rpp | integer |  | Field representing the number of results per page specified in the rpp parameter. |
| cursor | string |  | A field for cursor pagination. This value must be provided in the next request to load the next page of data. |
| count | integer |  | Field representing the total count of requested data. This field is only included in the response when true is set for the withCount parameter. |
| items | array | ✓ |  |
| items[].traceId | string |  | A field representing the trace ID of the transaction. Has a unique value based on the depth and call order of the transaction.  Naming convention: `call_{blockNumber}_{transactionIndex}_{depth1's callOrder}_{depth2's callOrder}_{...}` |
| items[].traceIndex | integer |  | A field representing the trace index of the transaction. Returns 0 for external transactions. |
| items[].transactionHash | string |  | A field representing the hash of the transaction. Provided as a 64-character hexadecimal string prefixed with 0x. |
| items[].transactionIndex | integer |  | A field representing the index of the transaction. Indicates the order within the block. Provided in decimal string format. |
| items[].from | string |  | A field representing the address that initiated the transaction. Provided as a 40-character hexadecimal string prefixed with 0x. |
| items[].to | string |  | A field representing the recipient address of the transaction. For contract creation transactions, this field returns the created contract address. Provided as a 40-character hexadecimal string prefixed with 0x. |
| items[].value | string |  | A field representing the amount transferred in the transaction. Provided in decimal string format. |
| items[].traceType | string |  | A field representing the trace type of the transaction. Includes call, create, etc. |
| items[].callType | string |  | A field representing the call type of the transaction. Includes call, delegatecall, staticcall, etc. |
| items[].gas | string |  | A field representing the amount of gas allocated to the transaction. Provided in decimal string format. |
| items[].gasUsed | string |  | A field representing the amount of gas actually used in transaction execution. Provided in decimal string format. |
| items[].status | string |  | A field representing the status of the transaction. 1 indicates success; 0 indicates failure. Provided in decimal string format. |
| items[].blockNumber | integer |  | A field representing the block number that contains the transaction. |
| items[].timestamp | integer |  | 트랜잭션이 생성된 시간을 나타내는 필드입니다. 이 필드는 UNIX 타임스탬프로 제공됩니다. |
