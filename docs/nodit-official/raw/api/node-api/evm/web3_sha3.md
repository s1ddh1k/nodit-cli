# web3_sha3

**`POST /`**

Applies the SHA3 (Keccak-256) hash function to the input data and returns the result. This method is primarily used to verify data integrity or generate unique identifiers for specific data. For example, it can be used to hash input data for storage on the blockchain or to compute unique hash values.

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
| id | integer | ✓ |  |
| jsonrpc | string | ✓ |  |
| method | string | ✓ |  |
| params | array |  | Enter the following parameters as an array with the appropriate types.  1. data in hex: Data to apply the SHA3 hash to. This value must be a hex-encoded string with a '0x' prefix. |

### Example

```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "method": "web3_sha3",
  "params": [
    "0x68656c6c6f204e4f444954"
  ]
}
```

## Response

### Example

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": "0x274020d2b9b210b079f3898f94123f710f5ab65525f23c7ad9e04e425d3648bd"
}
```
