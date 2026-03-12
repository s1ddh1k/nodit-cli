# Get Block by Hash or Number

**`POST /{chain}/{network}/blockchain/getBlockByHashOrNumber`**

Returns specific information about a block queried by block hash or block number.

## Parameters

| Name | In | Type | Required | Description |
|------|----|------|----------|-------------|
| chain | path | string | ✓ | Parameter to specify the target chain for the query. |
| network | path | string | ✓ | Parameter to specify the network of the target chain. Supported networks may vary depending on the chain. |

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
| block | string | ✓ | Parameter specifying the block to query. The default value is latest. You can input block number (decimal string), block hash (64-character hexadecimal string starting with 0x), or block tag (earliest, latest). "earliest" refers to the first block, "latest" refers to the most recent block. |

## Response

| Name | Type | Required | Description |
|------|------|----------|-------------|
| hash | string | ✓ | A field that represents the unique hash identifier of the block. It is a cryptographic hash computed from all data of the block, provided as a 64-character hexadecimal string prefixed with 0x.  |
| height | integer | ✓ | A field that represents the block height. An integer that increments sequentially starting from the genesis block (0), used as an indicator for the blockchain length and the position of a specific block.  |
| timestamp | integer | ✓ | A field that represents when the block was created. Provided as a UNIX timestamp, used for block creation order and time tracking. Block timestamps are used to ensure transaction order and measure inter-block creation intervals.  |
| firstVersion | integer | ✓ | A field that represents the version number of the first transaction processed in the block. In Aptos, each transaction has a unique version number that indicates the global execution order of the transaction. firstVersion and lastVersion allow you to determine the version range of transactions within the block.  |
| lastVersion | integer | ✓ | A field that represents the version number of the last transaction processed in the block. Together with firstVersion, it defines the version range of transactions within the block; this enables sequential retrieval of all transactions in the block.  |
| transactionsCount | integer | ✓ | A field that represents the total number of transactions included in the block. An important indicator for understanding block size and throughput; it includes all transaction types (user transactions, system transactions, etc.). This value equals lastVersion - firstVersion + 1.  |
| transactions | array | ✓ | A field that represents the list of version numbers for all transactions included in the block. Each version number serves as a unique identifier for the transaction, sorted sequentially from firstVersion to lastVersion. This list enables retrieval of detailed information for individual transactions in the block.  |
