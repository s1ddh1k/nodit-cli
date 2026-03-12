# Simulate transaction

**`POST /transactions/simulate`**

API that simulates a transaction without actually executing it.

The simulation result includes the same transaction output and events as when executing an actual signed transaction. However, the state hash is not included because storage is not updated. You can use this API to estimate the maximum gas amount required when submitting a transaction.

How to use:
  - Create a transaction with zero-padded signatures.
  - Submit the created transaction to this API.

To use this endpoint in BCS format, you must submit a signed transaction encoded in BCS.

## Parameters

| Name | In | Type | Required | Description |
|------|----|------|----------|-------------|
| estimate_gas_price | query | boolean |  | When set to True, the transaction's gas unit price is ignored and the estimated value is used.  |
| estimate_max_gas | query | boolean |  | When set to True, the transaction's max gas amount is ignored and the estimated value is used.  |
| estimate_prioritized_gas_uint_price | query | boolean |  | When set to True, the transaction is executed at a higher price than the estimate.  |

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
| sender | string | ✓ | A field representing the address of the account that sent the transaction. When expressed as a string, it is displayed as a 64-character hexadecimal string, and may be abbreviated by omitting leading zeros and adding 0x.  |
| sequence_number | string | ✓ | A field representing the transaction occurrence order. Represents a 64-bit unsigned integer as a string.  |
| max_gas_amount | string | ✓ | A field representing the maximum amount of gas to use when the transaction occurs. Represents a 64-bit unsigned integer as a string.  |
| gas_unit_price | string | ✓ | A field representing the gas price to use when the transaction occurs. Represents a 64-bit unsigned integer as a string.  |
| expiration_timestamp_secs | string | ✓ | A field representing the transaction expiration time. Represents a 64-bit unsigned integer as a string.  |
| payload | object | ✓ | A field representing the payload to use when the transaction occurs.  |
| replay_protection_nonce | string | ✓ | A field representing the nonce used to prevent transaction replay attacks. Represents a 64-bit unsigned integer as a string.  |
| signature | object | ✓ | A field representing the transaction signature. One of Ed25519, MultiEd25519, MultiAgent, FeePayer, Account, or NoAccount can be selected.  |
