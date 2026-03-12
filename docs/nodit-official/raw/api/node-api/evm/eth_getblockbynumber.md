# eth_getBlockByNumber

**`POST /`**

Returns block information for the specified block number.
					

> 🚧 Response taking too long? Response time may increase when including transactions!
>
> When requesting full block information, response time can increase if the block contains many transactions.
> For faster responses, exclude transactions and request only the block header (include transactions=false).

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
| id | integer | ✓ |  |
| jsonrpc | string | ✓ |  |
| method | string | ✓ |  |
| params | array | ✓ | Pass the following parameters as an array with the appropriate types. 1. `block number or block tag`: Block number as hex string, or block tag such as "earliest", "latest", "pending". 2. `includeTransactions`: Boolean indicating whether to include full transaction data. true includes all transactions, false excludes them. |

### Example

```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "method": "eth_getBlockByNumber",
  "params": [
    "latest",
    false
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
    "baseFeePerGas": "0xb25ce488f",
    "difficulty": "0x0",
    "extraData": "0x6265617665726275696c642e6f7267",
    "gasLimit": "0x1c9c380",
    "gasUsed": "0x9d6057",
    "hash": "0x59f63e3840e0f4a1659074c1a4021e881a268a52d31958688da1d66bfbf6d2ca",
    "logsBloom": "0x44ac510200a6014110057376a14d5485001541a84893289ee0c31013e08c8464d59145a90c002333650031500911057202a28300888128d11e40fc13c7b419010850000879cf8eac3dc3c2c8484ec630585188043a440c502c2455e692d21410d2b8af22f2814c8110a430318402884502181e0c083105a5920e63d8100e070a2e22a214556c080240cd8050056083276d208c8591011dbc443530f2d6f8bf044f9843da708065196e9a63d488a985480f940a44ae821a8b26ab0e0a38993041c98d868a0715a8f1000741c102019441610000a8e3a45a312814712a01436b5f90b6a0087581d581c204649a10184c350332821d9a780b47188f12f4d0791c4c",
    "miner": "0x388c818ca8b9251b393131c08a736a67ccb19297",
    "mixHash": "0xece1eb0b690ac889dcc9827dc80c87be317b50092ba0d5853619a004176393aa",
    "nonce": "0x0000000000000000",
    "number": "0x1076b5a",
    "parentHash": "0x78e316d93c059503a17ac843d1d4a99ad638f4d312e22d5eb4f88845d675982b",
    "receiptsRoot": "0xcf8924b0179ce08d6a1ae59ed4fa840ef8744dba0c249e476b7080d2c5af974e",
    "sha3Uncles": "0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347",
    "size": "0x25c46",
    "stateRoot": "0x58fe9dd8d65183497e88f7a5e5603721806563ed41999bbff153d2c358df603e",
    "timestamp": "0x6461d323",
    "totalDifficulty": "0xc70d815d562d3cfa955",
    "transactions": [
      "0xf3b9303a4149fed1a61438c737264d7d2ca9c33a94c2174506b84192998d9000",
      "0xfe5c51e3c60b41d1f4e58914356c895923cee931096bd1474b04ab7fd8e93be7",
      "0xd5fbb985d3d72f990c0e25a37d065c180c4c22f51e1be6304bf85463b11cf02b",
      "0xc654141dad3b63c4fcf015ed573ff6310d4b0395a9ccdde8a08a27cedb638626",
      "0xf0dfb4b0ee42f47bcfc641f47b1df323d73c14ab8f5e26022f56ca8352b4804c",
      "0xf00b3b2fb2f58a4fea4e52477dda6785392f93bdd3a535300d02e0426c7d33bf"
    ],
    "transactionsRoot": "0x33ea85c37a76ea540ac2c4f5cff2245c08ad5b57a9fe3a4bda3d5159a4a8549d",
    "uncles": [],
    "withdrawals": [
      {
        "index": "0x37cda6",
        "validatorIndex": "0x7387b",
        "address": "0x48bbf1c68037bf35b0eb090f1b5e0fa52f690502",
        "amount": "0xbc27d7"
      },
      {
        "index": "0x37cda7",
        "validatorIndex": "0x7387c",
        "address": "0x48bbf1c68037bf35b0eb090f1b5e0fa52f690502",
        "amount": "0xbd4f74"
      }
    ],
    "withdrawalsRoot": "0xfc59eb2b12a8b3b8f5f8cfb03f1c335734883d17edd226cf10d68e2b32b908df"
  }
}
```
