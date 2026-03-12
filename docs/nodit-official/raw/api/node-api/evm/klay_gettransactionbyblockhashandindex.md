# klay_getTransactionByBlockHashAndIndex

**`POST /`**

Returns transaction information by block hash and transaction index.

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
| id | integer | ✓ |  |
| jsonrpc | string | ✓ |  |
| method | string | ✓ |  |
| params | array | ✓ | Enter the following parameters as an array with the appropriate types. 1. `block hash`: Enter the block hash to query as a 64-digit hexadecimal string. 2. `transaction index`: Enter the transaction index as a hexadecimal string. |

### Example

```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "method": "klay_getTransactionByBlockHashAndIndex",
  "params": [
    "0x59f63e3840e0f4a1659074c1a4021e881a268a52d31958688da1d66bfbf6d2ca",
    "0x0"
  ]
}
```

## Response

| Name | Type | Required | Description |
|------|------|----------|-------------|
| blockHash | string |  | Hash of the block containing this transaction. Null for pending transactions |
| blockNumber | string |  | Block number containing this transaction (hex). Null for pending transactions |
| from | string |  | Sender address of the transaction |
| gas | string |  | Gas limit set for the transaction (hex) |
| gasPrice | string |  | Gas price (hex, in peb) |
| hash | string |  | Transaction hash |
| input | string |  | Transaction input data (hex) |
| nonce | string |  | Number of transactions sent by the sender prior to this one (hex) |
| to | string |  | Recipient address. Null for contract creation transactions |
| transactionIndex | string |  | Transaction position index within the block (hex). Null for pending transactions |
| value | string |  | Amount of KAIA transferred (hex, in peb) |
| type | string |  | Transaction type (hex string) |
| typeInt | integer |  | Transaction type as integer |
| signatures | array |  | Array of transaction signatures |
| signatures[].V | string |  | ECDSA signature v value (hex) |
| signatures[].R | string |  | ECDSA signature r value (hex) |
| signatures[].S | string |  | ECDSA signature s value (hex) |
| chainId | string |  | Chain ID of the network this transaction belongs to (hex) |
