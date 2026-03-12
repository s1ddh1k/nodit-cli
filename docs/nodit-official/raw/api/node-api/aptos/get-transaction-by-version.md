# Get transaction by version

**`GET /transactions/by_version/{txn_version}`**

Retrieves a transaction by its version. Returns a 410 response if the version has been pruned.

## Parameters

| Name | In | Type | Required | Description |
|------|----|------|----------|-------------|
| txn_version | path | string | ✓ | Version of the transaction to query |

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
