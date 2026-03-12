# Submit transaction

**`POST /transactions`**

This endpoint supports transaction submission in two formats.

1. **Submitting in JSON format:**
  - First encode the transaction in BCS. If your language has a BCS-supporting library, use it.
  - If you cannot encode BCS directly, you can use the Encode Submission API. When using this API, you must only send requests to a trusted node, as the node could tamper with the request content.
  - Sign the encoded transaction.
  - Submit the signed transaction with "application/json" Content-Type.

2. **Submitting in BCS format:**
  - Submit the BCS-encoded signed transaction with "application/x.aptos.signed_transaction+bcs" Content-Type.

## Parameters

| Name | In | Type | Required | Description |
|------|----|------|----------|-------------|
| accept | header | string |  | The content type that the client can receive. The API works correctly only when application/x-bcs is specified.  |

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
