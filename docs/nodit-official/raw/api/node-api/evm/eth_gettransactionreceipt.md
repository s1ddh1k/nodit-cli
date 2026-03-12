# eth_getTransactionReceipt

**`POST /`**

Returns the transaction receipt for the specified transaction hash.
					

> 🚧 Response taking too long? Response time may vary based on transaction complexity!
>
> When querying a specific transaction, response time can increase depending on the amount of event logs, transaction order in the block, etc.
> For faster responses, request without including the transaction's event logs.

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
| id | integer | ✓ |  |
| jsonrpc | string | ✓ |  |
| method | string | ✓ |  |
| params | array | ✓ | Pass the following parameters as an array with the appropriate types. 1. `transaction hash`: The transaction hash to query as a string. |

### Example

```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "method": "eth_getTransactionReceipt",
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
| cumulativeGasUsed | string |  | Total gas used in the block up to and including this transaction (hex) |
| effectiveGasPrice | string |  | Actual gas price paid (hex, in wei) |
| from | string |  | Sender address of the transaction |
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
| status | string |  | Transaction execution result (0x1=success, 0x0=failure) |
| to | string |  | Recipient address. Null for contract creation transactions |
| transactionHash | string |  | Transaction hash |
| transactionIndex | string |  | Transaction position index within the block (hex) |
| type | string |  | Transaction type (0x0=Legacy, 0x1=EIP-2930, 0x2=EIP-1559) |

### Example

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "blockHash": "0x7ce837c7672f469b85b0fbad0d1cc650a37e955efa43c018efb2f8624af6a772",
    "blockNumber": "0x1076b5f",
    "contractAddress": null,
    "cumulativeGasUsed": "0x125392e",
    "effectiveGasPrice": "0xa44661dc3",
    "from": "0x1f9090aae28b8a3dceadf281b0f12828e676c326",
    "gasUsed": "0x565f",
    "logs": [
      {
        "address": "0x388c818ca8b9251b393131c08a736a67ccb19297",
        "topics": [
          "0x27f12abfe35860a9a927b465bb3d4a9c23c8428174b83f278fe45ed7b4da2662"
        ],
        "data": "0x00000000000000000000000000000000000000000000000001162a2c94f37cd6",
        "blockNumber": "0x1076b5f",
        "transactionHash": "0xda148d856aef6d77d0b76c90ef1091ffe77afe9ee9b1c6cc23f28f042f198bd8",
        "transactionIndex": "0xd7",
        "blockHash": "0x7ce837c7672f469b85b0fbad0d1cc650a37e955efa43c018efb2f8624af6a772",
        "logIndex": "0x21d",
        "removed": false
      }
    ],
    "logsBloom": "0x00000000000000000000000000000000000100004000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000008000000000000000000000000000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000080000000000000000000000000000000000000000000000000000000000000000000000000800000000000000000000000000000000000000000000000000000000000",
    "status": "0x1",
    "to": "0x388c818ca8b9251b393131c08a736a67ccb19297",
    "transactionHash": "0xda148d856aef6d77d0b76c90ef1091ffe77afe9ee9b1c6cc23f28f042f198bd8",
    "transactionIndex": "0xd7",
    "type": "0x2"
  }
}
```
