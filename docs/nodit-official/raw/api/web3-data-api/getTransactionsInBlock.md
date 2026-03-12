# Get Transactions In Block

**`POST /{chain}/{network}/blockchain/getTransactionsInBlock`**

Retrieves the list of transactions within a specific block.


> 🚧 Caution when using decodeInput
>
> The decodeInput field decodes the transaction's input field and provides the result. However, different functions can use the same function selector, so the provided result may not match the actually called function. Therefore, additional verification is recommended for functions that differ from ERC standard specifications.

## Parameters

| Name | In | Type | Required | Description |
|------|----|------|----------|-------------|
| chain | path | string | ✓ | Parameter to specify the target chain for the query. |
| network | path | string | ✓ | Parameter to specify the network of the target chain. Supported networks may vary depending on the chain. |

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
| block | string | ✓ | Parameter specifying the block to query. The default value is latest. You can input block number (decimal string), block hash (64-character hexadecimal string starting with 0x), or block tag (earliest, latest). "earliest" refers to the first block, "latest" refers to the most recent block.   |
| page | integer |  | The page parameter specifies the data page to query. It accepts values up to 100; for pages exceeding 100, use the cursor pagination method.  The page and cursor parameters cannot be used together. If both page and cursor are empty, cursor pagination is used. |
| rpp | integer |  | rpp stands for results per page and specifies the page size. You can specify a number greater than 0 and up to 1000. |
| cursor | string |  | The cursor parameter supports pagination and data navigation between pages. Provide the cursor value from the previous page in the next request to load the next page of data.  The page and cursor parameters cannot be used together. If both page and cursor are empty, cursor pagination is used. |
| withCount | boolean |  | Parameter specifying whether to include the count field in the response. The count field indicates the total number of requested data items. When set to true, the count field is included and response speed may be slower. |
| withLogs | boolean |  | Parameter specifying whether to include the logs field in the response. Response speed may be slower when set to true. |
| withDecode | boolean |  | Parameter specifying whether to include decodedInput and decodedLog fields in the response. Response speed may be slower when set to true.  decodedLog is part of logs, so it is not included when withLogs is false even if withDecode is true. decodedInput interprets the transaction input field and provides the result. However, different functions may use the same function selector, so the result may not match the actually called function. Additional verification is recommended for non-standard ERC functions. |

## Response

| Name | Type | Required | Description |
|------|------|----------|-------------|
| page | integer |  | Field representing the page number specified in the page parameter. This field is only included in the response when a value greater than 0 is specified for the page parameter. |
| rpp | integer |  | Field representing the number of results per page specified in the rpp parameter. |
| cursor | string |  | A field for cursor pagination. This value must be provided in the next request to load the next page of data. |
| count | integer |  | Field representing the total count of requested data. This field is only included in the response when true is set for the withCount parameter. |
| items | array | ✓ |  |
| items[].transactionHash | string | ✓ | A field representing the hash of the transaction. Provided as a 64-character hexadecimal string prefixed with 0x. |
| items[].transactionIndex | integer | ✓ | A field representing the index of the transaction. Indicates the order within the block. Provided in decimal string format. |
| items[].blockHash | string | ✓ | A field representing the hash of the block. Provided as a 64-character hexadecimal string prefixed with 0x. |
| items[].blockNumber | integer | ✓ | A field representing the block number. |
| items[].from | string | ✓ | A field representing the address that initiated the transaction. Provided as a 40-character hexadecimal string prefixed with 0x. |
| items[].to | string | ✓ | A field representing the recipient address of the transaction. For contract creation transactions, this field returns null. Provided as a 40-character hexadecimal string prefixed with 0x. |
| items[].value | string | ✓ | A field representing the amount of tokens transferred in the transaction. Provided in decimal string format. |
| items[].input | string | ✓ | A field representing the data of the transaction. For Native Token (ETH, MATIC, etc.) transfers, this field is empty. Provided as a hexadecimal string prefixed with 0x.  |
| items[].decodedInput | object |  | A field representing the decoded result of the transaction input. Included in the response only when the withDecode parameter is set to true, providing function data executed based on the decoded input. This field is not provided even when withDecode is true if the function cannot be identified. |
| items[].decodedInput.type | string |  | A field representing the function type of the transaction. |
| items[].decodedInput.name | string |  | A field representing the function name of the transaction. |
| items[].decodedInput.signature | string |  | A field representing the function signature of the transaction. |
| items[].decodedInput.args | array |  |  |
| items[].functionSelector | string | ✓ | A 4-byte function selector value that identifies the called function. The function selector is the first 4 bytes of the Keccak-256 hash of the function signature. Provided as an 8-character hexadecimal string prefixed with 0x. |
| items[].nonce | string | ✓ | A field representing the nonce value of the transaction. Used for transaction deduplication and ordering. Provided in decimal string format. |
| items[].gas | string | ✓ | A field representing the maximum amount of gas the user intends to allocate for transaction execution. Provided in decimal string format. |
| items[].gasPrice | string | ✓ | A field representing the amount the user is willing to pay per unit of gas. Used in the transaction model before EIP-1559 (London Hard fork) with fixed fees; users can manually set the price based on network congestion. Provided in decimal string format. |
| items[].maxFeePerGas | string |  | A field representing the maximum fee the user is willing to pay per unit of gas. Must be greater than or equal to the sum of baseFeePerGas and maxPriorityFeePerGas. Part of the variable fee model introduced in EIP-1559 (London Hard fork). This field is not provided for transactions before this introduction. Provided in decimal string format. |
| items[].maxPriorityFeePerGas | string |  | The maximum fee per unit of gas that the user wants to pay directly to the block builder. Higher values cause the block builder to prioritize the transaction. Part of the variable fee model introduced in EIP-1559 (London Hard fork). This field is not provided for transactions before this introduction. Provided in decimal string format. |
| items[].gasUsed | string | ✓ | A field representing the amount of gas actually used in transaction execution. Provided in decimal string format. |
| items[].cumulativeGasUsed | string | ✓ | A field representing the total gas used by all transactions in the block up to and including the current transaction. Provided in decimal string format. |
| items[].effectGasPrice | string |  | A field representing the average price actually paid per unit of gas by the transaction. Part of the variable fee model introduced in EIP-1559 (London Hard fork). This field is not provided for transactions before this introduction. Provided in decimal string format. |
| items[].contractAddress | string | ✓ | A field representing the created contract address when the transaction is a contract creation transaction. Provided as a 40-character hexadecimal string prefixed with 0x. |
| items[].type | string |  | A field representing the type of the transaction. This field may not be provided depending on the block context. Provided in decimal string format. 0: Represents a standard Ethereum transaction. This is the traditional transaction format before EIP-2718. 1: Represents the 'Access List' transaction introduced in EIP-2930. This type includes an access list for specific addresses and storage slots to optimize gas costs. 2: Represents the 'Fee Market' transaction introduced in EIP-1559. This type uses a variable gas cost model and includes maxFeePerGas and maxPriorityFeePerGas fields. 3: Represents the 'Blob Transaction' introduced in EIP-4844. This type includes additional blob data separate from the body, improving data availability and supporting scalability in conjunction with Layer2 solutions such as Rollups. 4: An extended transaction type introduced in EIP-7702, containing an authorizationList field with signature/authorization information. Used when multi-signature or additional authentication procedures are required.  |
| items[].status | string |  | A field representing the status of the transaction. 1 indicates success; 0 indicates failure. This field is not provided for transactions before the Byzantium Hard Fork. Provided in decimal string format. |
| items[].logsBloom | string | ✓ | A field representing all logs bloom of the transaction. Provided as a 512-character hexadecimal string prefixed with 0x. |
| items[].accessList | array |  | A field representing the access list of the transaction. |
| items[].accessList[].address | string | ✓ | A field representing the contract address accessed in the transaction. Provided as a 40-character hexadecimal string prefixed with 0x. |
| items[].accessList[].storageKeys | array | ✓ | A field representing the list of storage keys of the contract accessed in the transaction. |
| items[].timeStamp | integer |  | A field representing when the transaction was created. This field is provided as a UNIX timestamp. |
| items[].authorizationList | array |  | This field is a list containing the transaction's signature and authorization information. Each item includes chain ID, nonce, signature values, etc. required for signature verification. Provided after EIP-7702. Support and adoption may vary by network, and this field may not be present depending on the transaction type (e.g., type 4 or higher). |
| items[].authorizationList[].chainId | string |  | A field representing the ID of the chain for which the authorization is valid. Used for signature verification and replay attack prevention. Expressed in decimal string format. |
| items[].authorizationList[].nonce | string |  | A nonce value to ensure the uniqueness of this authorization item and prevent replay attacks. Expressed in decimal string format. |
| items[].authorizationList[].address | string |  | The address of the account that provided the authorization (signature). |
| items[].authorizationList[].yParity | string |  | A 1-bit parity value used to select the y-coordinate of the signed public key. Typically 0 or 1, used for public key recovery. Expressed in decimal string format. |
| items[].authorizationList[].r | string |  | The first value of the ECDSA signature, generated through the nonce used for signing and curve operations. Used for public key recovery and signature verification. |
| items[].authorizationList[].s | string |  | The second value of the ECDSA signature, calculated based on the message hash and private key. Essential for verifying signature validity. |
| items[].logs | array |  |  |
| items[].logs[].contractAddress | string | ✓ | A field representing the contract address that generated the log. Provided as a 40-character hexadecimal string prefixed with 0x. |
| items[].logs[].transactionHash | string | ✓ | A field representing the hash of the transaction that generated the log. Provided as a 64-character hexadecimal string prefixed with 0x. |
| items[].logs[].transactionIndex | integer | ✓ | A field representing the index of the transaction that generated the log. Indicates the order within the block. Provided in decimal string format. |
| items[].logs[].blockHash | string | ✓ | A field representing the hash of the block that generated the log. Provided as a 64-character hexadecimal string prefixed with 0x. |
| items[].logs[].blockNumber | integer | ✓ | A field representing the block number that generated the log. |
| items[].logs[].data | string | ✓ | A field representing the data of the log. Provided as a hexadecimal string prefixed with 0x. This field represents the event data of the log. |
| items[].logs[].logIndex | integer | ✓ | A field representing the index of the log. Indicates the order of events in the transaction. Provided in decimal string format. |
| items[].logs[].removed | boolean | ✓ | A field indicating whether the log was removed. Returns true if the log was removed due to chain reorganization (Reorg). |
| items[].logs[].topics | array | ✓ | A field representing the indexed arguments of the log. Can have up to 4 topics. |
| items[].logs[].decodedLog | object |  | A field representing the event data of the log. Included in the response only when the withDecode parameter is set to true. |
