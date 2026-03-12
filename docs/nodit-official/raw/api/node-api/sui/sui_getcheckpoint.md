# sui_getCheckpoint

**`POST /`**

Return a checkpoint

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
  "method": "sui_getCheckpoint",
  "params": [
    "1000"
  ]
}
```

## Response

| Name | Type | Required | Description |
|------|------|----------|-------------|
| jsonrpc | string |  |  |
| id | integer |  |  |
| result | object |  |  |
| result.checkpointCommitments | array | ✓ | Commitments to checkpoint state |
| result.checkpointCommitments[].ECMHLiveObjectSetDigest | object | ✓ | The Sha256 digest of an EllipticCurveMultisetHash committing to the live object set. |
| result.digest | object | ✓ | Checkpoint digest |
| result.endOfEpochData | object |  |  |
| result.endOfEpochData.epochCommitments | array | ✓ | Commitments to epoch specific state (e.g. live object set) |
| result.endOfEpochData.nextEpochCommittee | array | ✓ | next_epoch_committee is `Some` if and only if the current checkpoint is the last checkpoint of an epoch. Therefore next_epoch_committee can be used to pick the last checkpoint of an epoch, which is often useful to get epoch level summary stats like total gas cost of an epoch, or the total number of transactions from genesis to the end of an epoch. The committee is stored as a vector of validator pub key and stake pairs. The vector should be sorted based on the Committee data structure. |
| result.endOfEpochData.nextEpochProtocolVersion | object | ✓ | The protocol version that is in effect during the epoch that starts immediately after this checkpoint. |
| result.epoch | object | ✓ | Checkpoint's epoch ID |
| result.epochRollingGasCostSummary | object | ✓ | The running total gas costs of all transactions included in the current epoch so far until this checkpoint. |
| result.networkTotalTransactions | object | ✓ | Total number of transactions committed since genesis, including those in this checkpoint. |
| result.previousDigest | string |  |  |
| result.sequenceNumber | object | ✓ | Checkpoint sequence number |
| result.timestampMs | object | ✓ | Timestamp of the checkpoint - number of milliseconds from the Unix epoch Checkpoint timestamps are monotonic, but not strongly monotonic - subsequent checkpoints can have same timestamp if they originate from the same underlining consensus commit |
| result.transactions | array | ✓ | Transaction digests |
| result.validatorSignature | object | ✓ | Validator Signature |

### Example

```json
{
  "jsonrpc": "2.0",
  "result": {
    "epoch": "5000",
    "sequenceNumber": "1000",
    "digest": "G6Dtzr1ZSfHFhotGsTE3cLENa7L1ooe1BBvknAUsARbV",
    "networkTotalTransactions": "792385",
    "previousDigest": "6tBy8RXZKrdrB4XkMQn7J3MNG4fQCo9XcRduFFvYrL5Z",
    "epochRollingGasCostSummary": {
      "computationCost": "0",
      "storageCost": "0",
      "storageRebate": "0",
      "nonRefundableStorageFee": "0"
    },
    "timestampMs": "1676911928",
    "transactions": [
      "mN8YNBgVR3wB7vfXmjVgDRF4oqxVRRjzmJ6U4mzbq77"
    ],
    "checkpointCommitments": [],
    "validatorSignature": "wAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA"
  },
  "id": 1
}
```
