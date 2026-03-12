# eth_getBlockReceipts

**`POST /`**

Returns all transaction receipts for the block specified by block hash or identifier.
					

> 🚧 Response taking too long? Response time may increase with the number of transactions in the block!
>
> eth_getBlockReceipts fetches all transaction receipts for a block at once. Response time can increase when the block has many transactions or complex execution results.
>
> For faster responses, consider:
>
> - Use eth_getTransactionReceipt to fetch individual transaction receipts when only specific transactions are needed.

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
| id | integer | ✓ |  |
| jsonrpc | string | ✓ |  |
| method | string | ✓ |  |
| params | array | ✓ | Pass the following parameters as an array with the appropriate types. 1. `block identifier` - Block number, block hash, or block tag. 	- Block number: hex string (ex. "0x1") 	- Block hash: 64-character hex string (ex. "0x39008d07edf93c03bb9d1cfc80598fcf63f441ec86e9de3733fa6a484980ca48")] 	- Block tag: enum string (ex. "latest", "earliest", "pending") |

### Example

```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "method": "eth_getBlockReceipts",
  "params": [
    "latest"
  ]
}
```

## Response

### Example

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": [
    {
      "blockHash": "0xf30beb3b775df15a6260eabffcb36829d5f9ced5a9c7ea2ca05be9164851d2a9",
      "blockNumber": "0x1076fc6",
      "contractAddress": null,
      "cumulativeGasUsed": "0x32b59",
      "effectiveGasPrice": "0x99a077c25",
      "from": "0xd99b5603addd1306b56e9fcc001ebe65956d1e31",
      "gasUsed": "0x32b59",
      "logs": [
        {
          "address": "0x000000000022d473030f116ddee9f6b43ac78ba3",
          "topics": [
            "0xc6a377bfc4eb120024a8ac08eef205be16b817020812c73223e81d1bdb9708ec",
            "0x000000000000000000000000d99b5603addd1306b56e9fcc001ebe65956d1e31",
            "0x0000000000000000000000000d58df0929b6baf8ed231f3fa672f0e5dcd665f7",
            "0x000000000000000000000000ef1c6e67703c7bd7107eed8303fbe6ec2554bf6b"
          ],
          "data": "0x000000000000000000000000ffffffffffffffffffffffffffffffffffffffff00000000000000000000000000000000000000000000000000000000648995c20000000000000000000000000000000000000000000000000000000000000000",
          "blockNumber": "0x1076fc6",
          "transactionHash": "0xe9bb07a140494db1d6f0a02f95e0804e520db1fd80bc541d95d4711e20e76e20",
          "transactionIndex": "0x0",
          "blockHash": "0xf30beb3b775df15a6260eabffcb36829d5f9ced5a9c7ea2ca05be9164851d2a9",
          "logIndex": "0x0",
          "removed": false
        },
        {
          "address": "0x0d58df0929b6baf8ed231f3fa672f0e5dcd665f7",
          "topics": [
            "0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef",
            "0x000000000000000000000000d99b5603addd1306b56e9fcc001ebe65956d1e31",
            "0x0000000000000000000000003bcfd7f139a4286a37aabff1cbde1447e90a6d57"
          ],
          "data": "0x0000000000000000000000000000000000000000000000656357415e66f0de16",
          "blockNumber": "0x1076fc6",
          "transactionHash": "0xe9bb07a140494db1d6f0a02f95e0804e520db1fd80bc541d95d4711e20e76e20",
          "transactionIndex": "0x0",
          "blockHash": "0xf30beb3b775df15a6260eabffcb36829d5f9ced5a9c7ea2ca05be9164851d2a9",
          "logIndex": "0x1",
          "removed": false
        }
      ],
      "logsBloom": "0x00210000000000000000000080000000000000000000000020001000000000000000020000000000800000000000000002014000880000000000000000280000000000000000000000004008000000201800000000400800000000000008040200000000000000000000000000000000400000000000040000000010000000000000004020000000000000000000000000000000000000080000004000001000020800000000000000000000000000000400000000000000400000100000000000000002000000000000000000000000400000000000001000000002000000000010200000000000000000000000000000000000000000000000000080000400",
      "status": "0x1",
      "to": "0xef1c6e67703c7bd7107eed8303fbe6ec2554bf6b",
      "transactionHash": "0xe9bb07a140494db1d6f0a02f95e0804e520db1fd80bc541d95d4711e20e76e20",
      "transactionIndex": "0x0",
      "type": "0x2"
    }
  ]
}
```
