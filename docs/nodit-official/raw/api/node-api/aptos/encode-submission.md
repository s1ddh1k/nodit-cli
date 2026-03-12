# Encode submission

**`POST /transactions/encode_submission`**

This API accepts unsigned transaction information in JSON format and converts it to BCS (Binary Canonical Serialization) format. You can use the converted result to sign the transaction and then submit it via the Submit Transaction API.

That is, this endpoint enables you to submit transaction requests to the API even in languages that do not have BCS-supporting libraries. You do not need to use this endpoint when using an SDK that supports BCS, such as the official Rust, TypeScript, or Python SDK.

To sign a transaction using this endpoint's response:
- Decode the hex-encoded string from the response into bytes.
- Sign the decoded bytes.
- Convert the signed bytes to Ed25519 signature format and use it to create the final transaction signature.

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
