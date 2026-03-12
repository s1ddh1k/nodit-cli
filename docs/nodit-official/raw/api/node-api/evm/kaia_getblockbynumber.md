# kaia_getBlockByNumber

**`POST /`**

Retrieves block information by block number.
					

> 🚧 Is the response taking a long time? Response time may increase when requesting transactions!
>
> When requesting all information for a specific block, response time may increase if the block contains many transactions.
> For faster responses, request only the block header without transactions (include transactions=false).

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
| id | integer | ✓ |  |
| jsonrpc | string | ✓ |  |
| method | string | ✓ |  |
| params | array | ✓ | Enter the following parameters as an array with the appropriate types. 1. `block number or block tag`: Enter the block number to query as a hexadecimal string. You may also use block tags such as "earliest", "latest", "pending". 2. `includeTransactions` : Boolean indicating whether to include all transaction information in the block when querying. true includes all transactions, false excludes them. |

### Example

```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "method": "kaia_getBlockByNumber",
  "params": [
    "0x1076B5A",
    false
  ]
}
```

## Response

| Name | Type | Required | Description |
|------|------|----------|-------------|
| blockScore | string |  | Former difficulty field (hex) |
| extraData | string |  | Arbitrary extra data added by the proposer (hex) |
| gasUsed | string |  | Total gas used by all transactions in this block (hex) |
| governanceData | string |  | RLP-encoded governance configuration data (hex) |
| hash | string |  | Block hash |
| logsBloom | string |  | Bloom filter for event log search (2048 bits, hex) |
| number | string |  | Block number (hex) |
| parentHash | string |  | Hash of the parent block |
| proposer | string |  | Address of the node that proposed this block |
| receiptsRoot | string |  | Root hash of the transaction receipts trie |
| reward | string |  | Address of the reward beneficiary |
| size | string |  | Block size in bytes (hex) |
| stateRoot | string |  | Root hash of the state trie |
| timestamp | string |  | Block creation time (Unix timestamp, hex) |
| timestampFoS | string |  | Fractional seconds of block creation time (1/100 second, hex) |
| totalBlockScore | string |  | Cumulative blockScore from genesis to this block (hex) |
| transactions | object |  | Transactions in this block |
| transactionsRoot | string |  | Root hash of the transactions trie |
| voteData | string |  | RLP-encoded governance vote data (hex) |
