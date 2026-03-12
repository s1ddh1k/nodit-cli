# kaia_getTransactionReceipt

**`POST /`**

Returns the receipt for a transaction by transaction hash.
					

> 🚧 Is the response taking a long time? Response time may vary based on transaction status!
>
> When querying a specific transaction, response time may increase depending on the volume of event logs in the transaction, transaction order within the block, etc.
> For faster responses, request without including the transaction's event logs.

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
| id | integer | ✓ |  |
| jsonrpc | string | ✓ |  |
| method | string | ✓ |  |
| params | array | ✓ | Enter the following parameters as an array with the appropriate types. 1. `transaction hash`: Enter the transaction hash to query as a string. |

### Example

```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "method": "kaia_getTransactionReceipt",
  "params": [
    "0xda148d856aef6d77d0b76c90ef1091ffe77afe9ee9b1c6cc23f28f042f198bd8"
  ]
}
```

## Response

| Name | Type | Required | Description |
|------|------|----------|-------------|
| blockHash | string |  | Hash of the block containing this transaction |
| blockNumber | string |  | Block number containing this transaction (hex) |
| contractAddress | string |  | Contract address created if this was a contract creation transaction, otherwise null |
| effectiveGasPrice | string |  | Actual gas price paid (hex, in peb) |
| from | string |  | Sender address of the transaction |
| gas | string |  | Gas limit set for the transaction (hex) |
| gasPrice | string |  | Gas price (hex, in peb) |
| gasUsed | string |  | Gas used by this specific transaction (hex) |
| logs | array |  | List of event logs emitted during transaction execution |
| logs[].address | string |  | The address of the contract that emitted this log |
| logs[].topics | array |  | Array of event signature hash and indexed parameters (up to 4 entries) |
| logs[].data | string |  | Non-indexed event parameter data (hex encoded) |
| logs[].blockNumber | string |  | Block number containing this log (hex) |
| logs[].transactionHash | string |  | Hash of the transaction that created this log |
| logs[].transactionIndex | string |  | Transaction position index within the block (hex) |
| logs[].blockHash | string |  | Hash of the block containing this log |
| logs[].logIndex | string |  | Log position index within the block (hex) |
| logs[].removed | boolean |  | True if the log was removed due to a chain reorganization |
| logsBloom | string |  | Bloom filter for event log search (2048 bits, hex) |
| nonce | string |  | Sender nonce (hex) |
| senderTxHash | string |  | Transaction hash containing only the sender signature, for fee-delegated transactions |
| signatures | array |  | Array of transaction signatures |
| signatures[].V | string |  | ECDSA signature v value (hex) |
| signatures[].R | string |  | ECDSA signature r value (hex) |
| signatures[].S | string |  | ECDSA signature s value (hex) |
| status | string |  | Transaction execution result (0x1=success, 0x0=failure) |
| to | string |  | Recipient address. Null for contract creation transactions |
| transactionHash | string |  | Transaction hash |
| transactionIndex | string |  | Transaction position index within the block (hex) |
| type | string |  | Transaction type (hex string) |
| typeInt | integer |  | Transaction type as integer |
