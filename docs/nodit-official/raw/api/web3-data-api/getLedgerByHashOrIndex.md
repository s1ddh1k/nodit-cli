# Get Ledger By Hash Or Index

**`POST /{chain}/{network}/blockchain/getLedgerByHashOrIndex`**

Returns information about a Ledger queried by Ledger hash or index.

## Parameters

| Name | In | Type | Required | Description |
|------|----|------|----------|-------------|
| chain | path | string | ✓ | Parameter to specify the target chain for the query. |
| network | path | string | ✓ | Parameter to specify the network of the target chain. Supported networks may vary depending on the chain. |

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
| ledger | string | ✓ | Parameter specifying the ledger to query. You can enter one of the following formats: - ledger index: Ledger number as a decimal string - ledger hash: 64-character hexadecimal string (excluding 0x prefix) - ledger tag: "earliest" or "latest" (oldest or most recent ledger)  |

## Response

| Name | Type | Required | Description |
|------|------|----------|-------------|
| ledgerHash | string | ✓ | Unique identifier of the current ledger, a 256-bit (64-digit hexadecimal) cryptographic hash. Used for ledger data integrity verification and identification. |
| ledgerIndex | integer | ✓ | Integer value indicating the creation order of the ledger, used to identify its position (sequence number) on the network. |
| ledgerTimestamp | integer | ✓ | The ledger's close time converted to a Unix timestamp. The close time recorded in the ledger header is expressed in seconds since the Ripple Epoch (January 1, 2000 00:00 UTC), which differs from the Unix Epoch (January 1, 1970 00:00 UTC) by 946,684,800 seconds. Thus ledgerTimestamp is the recorded close time plus 946,684,800 seconds. |
| parentLedgerHash | string | ✓ | 256-bit (64-digit hexadecimal) hash identifying the ledger immediately preceding the current ledger. References the previous ledger's hash to maintain chain integrity. |
| accountHash | string | ✓ | 256-bit (64-digit hexadecimal) hash summarizing all account state information recorded in the ledger. Used to verify consistency and integrity of account data. |
| totalCoins | string | ✓ | Total quantity of XRP (in XRP units) owned by accounts recorded in the ledger. This value excludes XRP burned as transaction fees. The actual circulating XRP may be lower than this value because some accounts may be in a "black hole" state (accounts with no key). |
| transactionHash | string | ✓ | 256-bit (64-digit hexadecimal) Merkle tree root hash summarizing all transaction data included in the ledger. Used for transaction history integrity verification. |
| transactionCount | integer | ✓ | Integer value indicating the total number of transactions included in the ledger. Used to assess the scale and activity of ledger transaction history. |
| transactions | array | ✓ | Array of transaction hashes included in the ledger. Each hash is a 256-bit (64-digit hexadecimal) cryptographic hash uniquely identifying a transaction, used for transaction data integrity and identification. |
