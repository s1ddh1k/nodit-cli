# getBlockCommitment

**`POST /`**

Returns commitment for particular block.

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
| jsonrpc | string | ✓ |  |
| id | integer | ✓ |  |
| method | string | ✓ |  |
| params | array | ✓ | It contains following items: 1. Block number (`required`)  |

### Example

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "getBlockCommitment",
  "params": [
    5
  ]
}
```

## Response

| Name | Type | Required | Description |
|------|------|----------|-------------|
| id | integer |  | The request ID |
| jsonrpc | string |  | The JSON-RPC version |
| result | object |  |  |
| result.commitment | array |  | Array of u64 integers logging the amount of cluster stake in lamports that has voted on the block at each depth from 0 to `MAX_LOCKOUT_HISTORY`. |
| result.totalStake | integer |  | Total active stake, in lamports, of the current epoch. |

### Example

```json
{
  "jsonrpc": "2.0",
  "result": {
    "commitment": [
      0,
      0,
      0,
      0,
      0,
      0,
      0,
      0,
      0,
      0,
      0,
      0,
      0,
      0,
      0,
      0,
      0,
      0,
      0,
      0,
      0,
      0,
      0,
      0,
      0,
      0,
      0,
      0,
      0,
      0,
      10,
      32
    ],
    "totalStake": 42
  },
  "id": 1
}
```
