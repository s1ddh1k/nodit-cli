# klay_getBlockReceipts

**`POST /`**

Retrieves receipts for a block by block hash.
					

> 🚧 Is the response taking a long time? Response time may vary based on the number of transactions in the block!
>
> klay_getBlockReceipts retrieves execution results (receipts) for all transactions in a block at once. When there are many transactions or complex execution results, response time may be delayed.
>
> For faster responses, consider the following recommendations:
>
> - If you need only specific transactions, use klay_getTransactionReceipt to query individual transaction execution results.

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
| id | integer | ✓ |  |
| jsonrpc | string | ✓ |  |
| method | string | ✓ |  |
| params | array | ✓ | Enter the following parameters as an array with the appropriate types. 1. `block identifier` - Block number, block hash, or block tag for the block to query. 	- Block number: Hexadecimal string (e.g. "0x1") 	- Block hash: 64-digit hexadecimal string (e.g. "0x39008d07edf93c03bb9d1cfc80598fcf63f441ec86e9de3733fa6a484980ca48")] 	- Block tag: Enum string (e.g. "latest", "earliest", "pending") |

### Example

```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "method": "klay_getBlockReceipts",
  "params": [
    "latest"
  ]
}
```
