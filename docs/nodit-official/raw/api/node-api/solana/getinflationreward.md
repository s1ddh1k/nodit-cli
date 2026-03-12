# getInflationReward

**`POST /`**

Returns the specific inflation values for the current epoch

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
| jsonrpc | string | ✓ |  |
| id | integer | ✓ |  |
| method | string | ✓ |  |
| params | array | ✓ | It contains following items: 1. Addresses (`optional`) 2. ConfigurationObject (`optional`)  |

### Example

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "getInflationReward",
  "params": [
    [
      "6dmNQ5jwLeLk5REvio1JcMshcbvkYMwy26sJ8pbkvStu",
      "BGsqMegLpV6n6Ve146sSX2dTjUMj3M92HnU8BbNRMhF2"
    ],
    {
      "epoch": 800,
      "commitment": "finalized"
    }
  ]
}
```

## Response

| Name | Type | Required | Description |
|------|------|----------|-------------|
| id | integer |  | The request ID |
| jsonrpc | string |  | The JSON-RPC version |
| result | object |  |  |
| result.epoch | integer |  | Epoch for which reward occurred |
| result.effectiveSlot | integer |  | The slot in which the rewards are effective |
| result.amount | integer |  | Reward amount in lamports |
| result.postBalance | integer |  | Post balance of the account in lamports |
| result.commission | integer |  | Vote account commission when the reward was credited |
