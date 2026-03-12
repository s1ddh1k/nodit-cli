# eth_feeHistory

**`POST /`**

Returns gas fee history for the requested block range. Use this to set appropriate values for maxFeePerGas and maxPriorityFeePerGas when creating transactions.

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
| id | integer | ✓ |  |
| jsonrpc | string | ✓ |  |
| method | string | ✓ |  |
| params | array | ✓ | Pass the following parameters as an array with the appropriate types. 1. `block count`: Number of blocks to query (1-1024). Fewer blocks may be returned if not all are available. 2. `newest block`: The reference block for the query. Block number as hex string or "latest". 3. `reward percentiles`: Integer array for sampling priority fee percentiles. Percentiles are weighted by gas used per block. Values 0-100 in ascending order. |

### Example

```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "method": "eth_feeHistory",
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

### Example

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "oldestBlock": "0x1249315",
    "reward": [
      [
        "0x186a0",
        "0x61ac6361c"
      ],
      [
        "0x0",
        "0xae908e1c8"
      ]
    ],
    "baseFeePerGas": [
      "0x589753de4",
      "0x5634a5a38",
      "0x5a0918fd1"
    ],
    "gasUsedRatio": [
      0.392293,
      0.6777092333333333
    ]
  }
}
```
