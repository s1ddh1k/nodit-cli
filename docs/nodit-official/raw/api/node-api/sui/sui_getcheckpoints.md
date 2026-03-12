# sui_getCheckpoints

**`POST /`**

Return paginated list of checkpoints

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
| id | integer | ✓ |  |
| jsonrpc | string | ✓ |  |
| method | string | ✓ |  |
| params | array | ✓ |  |

### Example

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "sui_getCheckpoints",
  "params": [
    "1004",
    4,
    false
  ]
}
```

## Response

| Name | Type | Required | Description |
|------|------|----------|-------------|
| jsonrpc | string |  |  |
| id | integer |  |  |
| result | object |  | `next_cursor` points to the last item in the page; Reading with `next_cursor` will start from the next item after `next_cursor` if `next_cursor` is `Some`, otherwise it will start from the first item. |
| result.data | array | ✓ |  |
| result.data[].checkpointCommitments | array | ✓ | Commitments to checkpoint state |
| result.data[].digest | object | ✓ | Checkpoint digest |
| result.data[].endOfEpochData | object |  |  |
| result.data[].epoch | object | ✓ | Checkpoint's epoch ID |
| result.data[].epochRollingGasCostSummary | object | ✓ | The running total gas costs of all transactions included in the current epoch so far until this checkpoint. |
| result.data[].networkTotalTransactions | object | ✓ | Total number of transactions committed since genesis, including those in this checkpoint. |
| result.data[].previousDigest | string |  |  |
| result.data[].sequenceNumber | object | ✓ | Checkpoint sequence number |
| result.data[].timestampMs | object | ✓ | Timestamp of the checkpoint - number of milliseconds from the Unix epoch Checkpoint timestamps are monotonic, but not strongly monotonic - subsequent checkpoints can have same timestamp if they originate from the same underlining consensus commit |
| result.data[].transactions | array | ✓ | Transaction digests |
| result.data[].validatorSignature | object | ✓ | Validator Signature |
| result.hasNextPage | boolean | ✓ |  |
| result.nextCursor | string |  |  |

### Example

```json
{
  "jsonrpc": "2.0",
  "result": {
    "data": [
      {
        "epoch": "5000",
        "sequenceNumber": "1005",
        "digest": "9zA7Q9Ka1ykvYjSQGhQCdCf32FZkcWNWx7L22JczXGsk",
        "networkTotalTransactions": "792385",
        "previousDigest": "8BLFxLTjWZ2KqaGc3FjR1o9aL6kbyYrmhuNfJLU1ehYt",
        "epochRollingGasCostSummary": {
          "computationCost": "0",
          "storageCost": "0",
          "storageRebate": "0",
          "nonRefundableStorageFee": "0"
        },
        "timestampMs": "1676911928",
        "transactions": [
          "7RudGLkQDBNJyqrptkrNU66Zd3pvq8MHVAHYz9WpBm59"
        ],
        "checkpointCommitments": [],
        "validatorSignature": "wAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA"
      },
      {
        "epoch": "5000",
        "sequenceNumber": "1006",
        "digest": "FAUWHyWacmb4Vg4QGi9a6gqeVb7ixAZiL73FaGd6WpoV",
        "networkTotalTransactions": "792385",
        "previousDigest": "6Pn25cieaE62AT6BwCeBoca13AGZuneucaaTGqt3gNCo",
        "epochRollingGasCostSummary": {
          "computationCost": "0",
          "storageCost": "0",
          "storageRebate": "0",
          "nonRefundableStorageFee": "0"
        },
        "timestampMs": "1676911928",
        "transactions": [
          "7r7tmP5hzgrusiN6cucFwfTveqDb7K75tMJ7oNCyoDmy"
        ],
        "checkpointCommitments": [],
        "validatorSignature": "wAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA"
      },
      {
        "epoch": "5000",
        "sequenceNumber": "1007",
        "digest": "B3mzC6gy87SomUQwPsmVY7mtwkZLxfm5WwNi3kKyEb3x",
        "networkTotalTransactions": "792385",
        "previousDigest": "CnHTfdUJr1UUqwXkYUhbQjXeM16xR33UR62jE72toCis",
        "epochRollingGasCostSummary": {
          "computationCost": "0",
          "storageCost": "0",
          "storageRebate": "0",
          "nonRefundableStorageFee": "0"
        },
        "timestampMs": "1676911928",
        "transactions": [
          "Gb1UDqhmKMzMJ5FL37kBqCcuy4TtBL2ay3qec8tEUBLj"
        ],
        "checkpointCommitments": [],
        "validatorSignature": "wAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA"
      },
      {
        "epoch": "5000",
        "sequenceNumber": "1008",
        "digest": "HunuJWKu7azBfS47rJTq9FHTMvUDNVo2SK4hQeh5brXp",
        "networkTotalTransactions": "792385",
        "previousDigest": "38fLUfuigyzLPEDrsmRhcQmhKtbEUohuFBP9NDcWBmFz",
        "epochRollingGasCostSummary": {
          "computationCost": "0",
          "storageCost": "0",
          "storageRebate": "0",
          "nonRefundableStorageFee": "0"
        },
        "timestampMs": "1676911928",
        "transactions": [
          "GWTS9QR7mjNz9fBWGkk4JZU3mrzMXrmj74uS59Cd5und"
        ],
        "checkpointCommitments": [],
        "validatorSignature": "wAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA"
      }
    ],
    "nextCursor": "1008",
    "hasNextPage": true
  },
  "id": 1
}
```
