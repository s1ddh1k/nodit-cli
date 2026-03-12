# Get account

**`GET /accounts/{address}`**

Returns the authentication key and sequence number for an account address. You can specify the ledger version of the transaction to query; if no ledger version is specified, the latest ledger version is returned.

## Parameters

| Name | In | Type | Required | Description |
|------|----|------|----------|-------------|
| address | path | string | ✓ | The address of the account to retrieve. Account addresses without the hexadecimal prefix can also be queried.  |
| ledger_version | query | integer |  | The ledger version of the transaction to fetch account state from. If no value is provided, the latest version is used.  |

## Response

| Name | Type | Required | Description |
|------|------|----------|-------------|
| sequence_number | string |  | A field representing the transaction occurrence order. Represents a 64-bit unsigned integer as a string.  |
| authentication_key | string |  | All byte (Vec) data is expressed as a hex-encoded string starting with 0x, with two hex digits per byte. Unlike the Address type, HexEncodedBytes does not omit any zeros.  |

### Example

```json
{
  "sequence_number": 692283,
  "authentication_key": "0xb3e73f93ff4077d27c61d6c4f364cfffeac97a9ab0c5a874758150389697a97f"
}
```
