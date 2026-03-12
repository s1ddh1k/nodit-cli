# eth_getStorageAt

**`POST /`**

Returns the value at the assigned storage slot of the specified address.

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
| id | integer | ✓ |  |
| jsonrpc | string | ✓ |  |
| method | string | ✓ |  |
| params | array | ✓ | Pass the following parameters as an array with the appropriate types. 1. `address` - The address to query as a 40-character hex string. 2. `position` - The storage position (hex string) 3. `block identifier` - Block number, block hash, or block tag. 	- Block number: hex string (ex. "0x1") 	- Block hash: 64-character hex string (ex. "0x39008d07edf93c03bb9d1cfc80598fcf63f441ec86e9de3733fa6a484980ca48")] 	- Block tag: enum string (ex. "latest", "earliest", "pending") |

### Example

```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "method": "eth_getStorageAt",
  "params": [
    "0xdac17f958d2ee523a2206206994597c13d831ec7",
    "0x0",
    "latest"
  ]
}
```

## Response

### Example

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": "0x000000000000000000000000c6cde7c39eb2f0f0095f41570af89efc2c1ea828"
}
```
