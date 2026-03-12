# klay_getProof

**`POST /`**

Returns values stored in Storage for a specific address in a format including merkle-proof. The returned proof can be used to verify that the queried storage state has not been tampered with.

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
| id | integer | ✓ |  |
| jsonrpc | string | ✓ |  |
| method | string | ✓ |  |
| params | array | ✓ | Enter the following parameters as an array with the appropriate types. 1. `address` - Enter the address to query as a 40-digit hexadecimal string. 2. `storage hashes` - Array of storage slot positions to query (hexadecimal strings) 3. `block identifier` - Block number, block hash, or block tag for the block to query. 	- Block number: Hexadecimal string (e.g. "0x1") 	- Block hash: 64-digit hexadecimal string (e.g. "0x39008d07edf93c03bb9d1cfc80598fcf63f441ec86e9de3733fa6a484980ca48")] 	- Block tag: Enum string (e.g. "latest", "earliest", "pending") |

### Example

```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "method": "klay_getProof",
  "params": [
    "0xdac17f958d2ee523a2206206994597c13d831ec7",
    [
      "0x000000000000000000000000c6cde7c39eb2f0f0095f41570af89efc2c1ea828"
    ],
    "latest"
  ]
}
```

## Response

| Name | Type | Required | Description |
|------|------|----------|-------------|
| address | string |  | Address of the proven account |
| accountProof | array |  | Merkle proof nodes of the account state trie (RLP encoded, hex) |
| balance | string |  | Account balance (hex, in wei) |
| codeHash | string |  | Keccak-256 hash of the account code |
| nonce | string |  | Account transaction count (hex) |
| storageHash | string |  | Root hash of the storage trie |
| storageProof | array |  | Array of proofs for the requested storage keys |
| storageProof[].key | string |  | Storage key (hex) |
| storageProof[].value | string |  | Storage value (hex) |
| storageProof[].proof | array |  | Merkle proof nodes of the storage state trie (RLP encoded, hex) |
