# Get Transaction By Transaction ID

**`POST /{chain}/{network}/blockchain/getTransactionByTransactionId`**

Retrieves information about a specific transaction.

## Parameters

| Name | In | Type | Required | Description |
|------|----|------|----------|-------------|
| chain | path | string | ✓ | Parameter to specify the target chain for the query. |
| network | path | string | ✓ | Parameter to specify the network of the target chain. Supported networks may vary depending on the chain. |

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
| transactionId | string | ✓ | Parameter specifying the transaction ID to query. Can be entered as a 64-character hexadecimal string (excluding 0x). |

## Response

| Name | Type | Required | Description |
|------|------|----------|-------------|
| id | string | ✓ | Unique identifier of the transaction; a value that uniquely identifies each transaction. |
| index | integer | ✓ | Indicates the index of the transaction within the block. Starts at 0; represents the order of transactions included in the block. |
| hash | string | ✓ | Hash value of the transaction; a unique identifier calculated from the transaction data. Computed including SegWit data. |
| version | integer | ✓ | Value indicating the transaction version; defines the transaction format. |
| lockTime | integer | ✓ | Lock time of the transaction; indicates the condition (block height or UNIX timestamp) under which the transaction becomes valid. A value of `0` means the transaction is immediately valid. |
| size | integer | ✓ | Actual size of the transaction in bytes; represents the length of the transaction data. |
| vsize | integer | ✓ | Virtual size of the transaction in bytes; represents the effective size of the transaction as included in a block when SegWit data is included. Calculated by dividing weight by 4 and rounding up, per BIP-141. |
| weight | integer | ✓ | Weight of the transaction; represents the network cost of the transaction as calculated per BIP-141. Calculated as `(non-witness size * 3) + total size`. |
| fee | integer | ✓ | Transaction fee (in BTC units); the remainder when output value is subtracted from input value. |
| vinCount | integer | ✓ | Indicates the number of input (Vin) transactions. |
| voutCount | integer | ✓ | Indicates the number of output (Vout) transactions. |
| blockHeight | integer | ✓ | Indicates the height of the block containing the transaction. |
| blockHash | string | ✓ | Hash of the block containing the transaction. |
| blockTimestamp | integer | ✓ | Indicates when the block containing the transaction was created; displayed in UNIX timestamp format. |
| vin | object |  |  |
| vin.index | integer | ✓ | Indicates the index of the Vin (input) within the containing transaction. |
| vin.address | string | ✓ | Address of the previous Vout (output) referenced by the Vin (input); represents the sender address that transferred the Bitcoin. |
| vin.value | string | ✓ | Output value of the previous Vout (output) referenced by the Vin (input); represents the amount of Bitcoin being transferred (in BTC units). |
| vin.coinbase | string | ✓ | Used in coinbase transactions (mining reward transactions); contains arbitrary data defined by the miner. Set to null for regular transactions. |
| vin.voutTransactionId | string | ✓ | ID of the previous transaction (Vout) referenced by the Vin (input). This field is null for coinbase transactions. |
| vin.voutIndex | integer | ✓ | Index of the previous Vout (output) referenced by the Vin (input). Null for coinbase transactions. |
| vin.scriptSig | object | ✓ | Signature script of the Vin (input); used to unlock the lock script of the referenced previous Vout (output). |
| vin.scriptSig.hex | string | ✓ | Signature script of the Vin (input) in hexadecimal format. Provided as an empty string for SegWit transactions. |
| vin.scriptSig.asm | string | ✓ | Signature script of the Vin (input) in assembly format. Provided as an empty string for SegWit transactions. |
| vin.scriptSig.type | string | ✓ | Indicates the signature script type of the Vin (input) (e.g., `p2pkh`, `p2sh`). |
| vin.sequence | integer | ✓ | Sequence number of the Vin (input); controls transaction validity or works with `locktime`. Also used for RBF (Replace-By-Fee) control. |
| vin.witness | array | ✓ | Array of script signature data for the Vin (input), included for SegWit transactions. |
| vout | object |  |  |
| vout.index | integer | ✓ | Indicates the index of the Vout (output) within the containing transaction. |
| vout.address | string | ✓ | Represents the recipient address that will receive the Bitcoin from the Vout (output). |
| vout.value | string | ✓ | Represents the amount of Bitcoin transferred in the Vout (output) (in BTC units). |
| vout.scriptPubKey | object | ✓ | Script of the Vout (output); defines the conditions (lock script) required to spend the Bitcoin. |
| vout.scriptPubKey.hex | string | ✓ | Script of the Vout (output) in hexadecimal format. |
| vout.scriptPubKey.asm | string | ✓ | Script of the Vout (output) in assembly format. |
| vout.scriptPubKey.type | string | ✓ | Indicates the script type of the Vout (output) (e.g., pubkey, p2sh, p2pkh). |
