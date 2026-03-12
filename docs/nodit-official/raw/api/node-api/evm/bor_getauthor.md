# bor_getAuthor

**`POST /`**

Returns the address of the validator (or block producer) that created the current block. This method is used to identify the node that produced a block in a PoA network. The block producer is selected according to the network's consensus protocol, and this method reveals the address of the block's "author" or "miner".

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
| id | integer | ✓ |  |
| jsonrpc | string | ✓ |  |
| method | string | ✓ |  |
| params | array | ✓ | Enter the following parameters as an array with the appropriate types. 1. `block number or block tag`: Enter the block number in hexadecimal string format. You can also enter block tags such as "earliest", "latest", or "pending". |

### Example

```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "method": "bor_getAuthor",
  "params": [
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
  "result": "0x7c7379531b2aee82e4ca06d4175d13b9cbeafd49"
}
```
