# kaia_feeHistory

**`POST /`**

Returns gas fee history for the requested block range. Use this information to set appropriate values for maxFeePerGas and maxPriorityFeePerGas when creating transactions.

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
| id | integer | ✓ |  |
| jsonrpc | string | ✓ |  |
| method | string | ✓ |  |
| params | array | ✓ | Enter the following parameters as an array with the appropriate types. 1. `block count`: Enter the block range to query. Block range must be an integer between 1 and 1024. Fewer blocks may be returned if not all requested blocks are available. 2. `newest block`: Enter the reference block for the query. Block number as hexadecimal string or "latest". 3. `reward percentiles`: Enter an array of integers for sampling priority fee percentiles. Percentiles are weighted by gas used per block. Percentiles must be integers from 0 to 100 in ascending order. |

### Example

```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "method": "kaia_feeHistory",
  "params": [
    2,
    "latest",
    [
      0,
      100
    ]
  ]
}
```

## Response

| Name | Type | Required | Description |
|------|------|----------|-------------|
| oldestBlock | string |  | Oldest block number in the returned range (hex) |
| reward | array |  | Array of priority fees at requested percentiles for each block (in wei, hex) |
| baseFeePerGas | array |  | Array of base fees per gas for each block (in wei, hex). Last element is the next block's expected value. |
| gasUsedRatio | array |  | Array of gas utilization ratios per block (0 to 1) |
