# optimism_outputAtBlock

**`POST /`**

This method is used to retrieve the output root at a specific block. Users can call it by providing the block number in hexadecimal format, and a successful call returns the output root for that block. The output root is an important element representing the state of Optimism and can be used to verify the state of a specific block.

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
| id | integer | ✓ |  |
| jsonrpc | string | ✓ |  |
| method | string | ✓ |  |
| params | array | ✓ | Enter the following parameters as an array with the appropriate types. 1. `block number`: Enter the block number to query in hexadecimal string format. This method can query up to the latest 128 blocks. |

### Example

```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "method": "optimism_outputAtBlock"
}
```

## Response

### Example

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": [
    "0x0000000000000000000000000000000000000000000000000000000000000000",
    "0xabe711e34c1387c8c56d0def8ce77e454d6a0bfd26cef2396626202238442421"
  ]
}
```
