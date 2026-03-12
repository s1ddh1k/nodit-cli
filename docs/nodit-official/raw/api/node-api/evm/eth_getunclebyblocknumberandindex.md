# eth_getUncleByBlockNumberAndIndex

**`POST /`**

Returns the uncle block information for the specified block number and uncle index.

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
| id | integer | ✓ |  |
| jsonrpc | string | ✓ |  |
| method | string | ✓ |  |
| params | array | ✓ | Pass the following parameters as an array with the appropriate types. 1. `block number or block tag`: Block number as hex string, or block tag such as "earliest", "latest", "pending". 2. `uncle index`: The uncle index as a hex string. |

### Example

```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "method": "eth_getUncleByBlockNumberAndIndex",
  "params": [
    "0x5BAD54",
    "0x0"
  ]
}
```

## Response

| Name | Type | Required | Description |
|------|------|----------|-------------|
| baseFeePerGas | string |  | Base fee per gas (EIP-1559, hex in wei) |
| difficulty | string |  | Mining difficulty of the block (hex) |
| extraData | string |  | Arbitrary extra data added by the miner (hex) |
| gasLimit | string |  | Maximum total gas allowed in this block (hex) |
| gasUsed | string |  | Total gas used by all transactions in this block (hex) |
| hash | string |  | Block hash |
| logsBloom | string |  | Bloom filter for event log search (2048 bits, hex) |
| miner | string |  | Address of the block miner |
| mixHash | string |  | Mix hash used in proof-of-work |
| nonce | string |  | Nonce used in proof-of-work (hex) |
| number | string |  | Block number (hex) |
| parentHash | string |  | Hash of the parent block |
| receiptsRoot | string |  | Root hash of the transaction receipts trie |
| sha3Uncles | string |  | SHA3 hash of the uncles list |
| size | string |  | Block size in bytes (hex) |
| stateRoot | string |  | Root hash of the state trie |
| timestamp | string |  | Block creation time (Unix timestamp, hex) |
| totalDifficulty | string |  | Cumulative difficulty from genesis to this block (hex) |
| transactions | object |  | Transactions in this block. Returns hash array or full objects depending on includeTransactions parameter. |
| transactionsRoot | string |  | Root hash of the transactions trie |
| uncles | array |  | Array of uncle block hashes |
| withdrawals | array |  | Validator withdrawal list (EIP-4895) |
| withdrawals[].index | string |  | Withdrawal index (hex) |
| withdrawals[].validatorIndex | string |  | Validator index (hex) |
| withdrawals[].address | string |  | Withdrawal recipient address |
| withdrawals[].amount | string |  | Withdrawal amount (in Gwei, hex) |
| withdrawalsRoot | string |  | Root hash of the withdrawals trie (EIP-4895) |

### Example

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "difficulty": "0xbf93d9424b943",
    "extraData": "0x73656f33",
    "gasLimit": "0x7a121d",
    "gasUsed": "0x7936f4",
    "hash": "0x407116a0f58ad370ac6c45813665988903f4c60ecc2f112d55cbf43a8c33f48e",
    "logsBloom": "0x804e00000660061481081001000002000804080b1012106100a001010248430040000008104c0200200003020070104a00008404000000c2c00010010304410000148c2019288401020881090040243240312000b80c2084a111c2010221081042004400028c2880a000104810004a1000208001240000000088011002200000048024000140004894689004026980000588001040a02000828400480302010020008060208084084e20020054428d20800245122410000040000000020004000420000204101200620000010050080c1540000080008114210003cc0200610400801c0300640000630500880a40820400241010441000420a504001048008a5",
    "miner": "0xb2930b35844a230f00e51431acae96fe543a0347",
    "mixHash": "0xb1266ba0d3c68a5561e6f62a819d214d9eaa293312f674f3a44ad7fae5e6740d",
    "nonce": "0x6b5afb9416a32721",
    "number": "0x5bad53",
    "parentHash": "0xd2f7edf95b79d2a7e994f698d1ea1737642de0bd49138d183bcc3d1ecf9792e4",
    "receiptsRoot": "0xc2798ebd620a7dee020ba272d4396a60a68de6ce5f52b22770e9fee2e14ff15a",
    "sha3Uncles": "0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347",
    "size": "0x20d",
    "stateRoot": "0x9a57583eeb63556158cdda05f188ff1dffa1a1cfde194d026f2719e69edf2a9d",
    "timestamp": "0x5b541431",
    "transactionsRoot": "0x11447f3cc960e33ecca21d2b1ca08b6b5a7e478c35920ad1159c9332c0f9332c",
    "uncles": []
  }
}
```
