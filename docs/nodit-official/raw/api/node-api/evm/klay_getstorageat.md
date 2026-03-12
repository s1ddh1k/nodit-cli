# klay_getStorageAt

**`POST /`**

Returns the storage slot value at the given address.

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
| id | integer | ✓ |  |
| jsonrpc | string | ✓ |  |
| method | string | ✓ |  |
| params | array | ✓ | Enter the following parameters as an array with the appropriate types. 1. `address` - Enter the address to query as a 40-digit hexadecimal string. 2. `position` - Storage slot position to query (hexadecimal string) 3. `block identifier` - Block number, block hash, or block tag for the block to query. 	- Block number: Hexadecimal string (e.g. "0x1") 	- Block hash: 64-digit hexadecimal string (e.g. "0x39008d07edf93c03bb9d1cfc80598fcf63f441ec86e9de3733fa6a484980ca48")] 	- Block tag: Enum string (e.g. "latest", "earliest", "pending") |

### Example

```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "method": "klay_getStorageAt",
  "params": [
    "0xdac17f958d2ee523a2206206994597c13d831ec7",
    "0x0",
    "latest"
  ]
}
```
