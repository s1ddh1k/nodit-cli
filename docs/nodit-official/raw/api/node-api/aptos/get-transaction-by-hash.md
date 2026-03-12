# Get transaction by hash

**`GET /transactions/by_hash/{txn_hash}`**

Retrieves a transaction by its hash. This hash is the same value returned by the API when submitting a transaction (Pending Transaction).

How to generate a transaction hash directly:
1. Create message bytes: concatenate "RawTransaction" bytes + the transaction's BCS bytes
2. Apply the SHA3-256 hash algorithm to the message bytes
3. Encode the hashed bytes as a hex string with the 0x prefix

## Parameters

| Name | In | Type | Required | Description |
|------|----|------|----------|-------------|
| txn_hash | path | string | ✓ | Hash of the transaction to query |

## Response

| Name | Type | Required | Description |
|------|------|----------|-------------|
| type | string | ✓ | The type of the transaction |
| hash | string | ✓ | A field representing the unique identifier of the transaction, provided as a 64-character hexadecimal string starting with 0x.  |
| sender | string | ✓ | A field representing the address of the account that sent the transaction. When expressed as a string, it is displayed as a 64-character hexadecimal string, and may be abbreviated by omitting leading zeros and adding 0x.  |
| sequence_number | string | ✓ | A field representing the transaction occurrence order. Represents a 64-bit unsigned integer as a string.  |
| max_gas_amount | string | ✓ | A field representing the maximum amount of gas to use when the transaction occurs. Represents a 64-bit unsigned integer as a string.  |
| gas_unit_price | string | ✓ | A field representing the gas price to use when the transaction occurs. Represents a 64-bit unsigned integer as a string.  |
| expiration_timestamp_secs | string | ✓ | A field representing the transaction expiration time. Represents a 64-bit unsigned integer as a string.  |
| payload | object | ✓ |  |
| signature | object |  |  |
| replay_protection_nonce | string |  | A field representing the nonce used to prevent transaction replay attacks. Represents a 64-bit unsigned integer as a string.  |
