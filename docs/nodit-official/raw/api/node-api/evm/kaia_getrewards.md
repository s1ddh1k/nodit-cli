# kaia_getRewards

**`POST /`**

Retrieves reward distribution for a block by block number. Includes reward recipients and their share. If the parameter is omitted, returns reward distribution for the latest block.

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
| id | integer | ✓ |  |
| jsonrpc | string | ✓ |  |
| method | string | ✓ |  |
| params | array |  | Enter the following parameters as an array with the appropriate types. When omitted, returns reward distribution for the latest block. 1. `block number or block tag` - Block number or block tag for the block to query. - Block number: Integer or hexadecimal string (e.g. "0x1") - Block tag: Enum string (e.g. "latest", "earliest", "pending") |

### Example

```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "method": "kaia_getRewards",
  "params": []
}
```

## Response

| Name | Type | Required | Description |
|------|------|----------|-------------|
| minted | string |  | Total KAIA newly minted in this block (hex, in peb) |
| totalFee | string |  | Total transaction fees in this block (hex, in peb) |
| burntFee | string |  | Total fees burned (hex, in peb) |
| proposer | string |  | Reward received by the block proposer (hex, in peb) |
| stakers | string |  | Total rewards received by staking participants (hex, in peb) |
| kif | string |  | Reward received by the Kaia Infrastructure Fund (hex, in peb) |
| kef | string |  | Reward received by the Kaia Ecosystem Fund (hex, in peb) |
| rewards | object |  | Map of rewards per beneficiary address (key=address, value=amount in hex) |
