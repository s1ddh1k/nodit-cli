# getEpochSchedule

**`POST /`**

Returns the epoch schedule information from this cluster

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
| jsonrpc | string | ✓ |  |
| id | integer | ✓ |  |
| method | string | ✓ |  |

### Example

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "getEpochSchedule"
}
```

## Response

| Name | Type | Required | Description |
|------|------|----------|-------------|
| id | integer |  | The request ID |
| jsonrpc | string |  | The JSON-RPC version |
| result | object |  |  |
| result.firstNormalEpoch | integer |  | First normal-length epoch, log2(slotsPerEpoch) - log2(MINIMUM_SLOTS_PER_EPOCH) |
| result.firstNormalSlot | integer |  | Minimum number of slots in an epoch, MINIMUM_SLOTS_PER_EPOCH * (2.pow(firstNormalEpoch) - 1) |
| result.leaderScheduleSlotOffset | integer |  | The number of slots before beginning of an epoch to calculate a leader schedule for that epoch. |
| result.slotsPerEpoch | integer |  | The maximum number of slots in each epoch. |
| result.warmup | boolean |  | Whether epochs start short and grow. |
