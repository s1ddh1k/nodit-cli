# Get transaction summaries

**`GET /accounts/{address}/transaction_summaries`**

Retrieves summary information for an account's on-chain committed transactions (both sequence-number-based and unordered transactions). Each transaction summary includes sender address, transaction hash, version, and replay protection nonce.

- If start_version is provided, transaction summaries starting from that version are returned.
- If start_version is not provided and only end_version is provided, transaction summaries ending at end_version are returned.
- If both start_version and end_version are not provided, the account's most recent committed transaction summary is returned.
- Output is always composed of transaction summaries sorted in ascending order by version.
- To query pending transactions, use the Get Transaction by Hash API.

## Parameters

| Name | In | Type | Required | Description |
|------|----|------|----------|-------------|
| address | path | string | ✓ | The address of the account to retrieve. Account addresses without the hexadecimal prefix can also be queried.  |
| limit | query | integer |  | The maximum number of account modules to fetch. If no value is provided, the default page size is used.  |
| start | query | string |  | A cursor that specifies the starting position for pagination.  This cursor cannot be arbitrarily generated on the client side. You must first call this endpoint without specifying this query parameter, then use the cursor returned in the X-Aptos-Cursor header of the response.  |

## Response

### Example

```json
[
  {
    "version": "223509841",
    "hash": "0x67c6d8698c3f41dc4fee191668301b753b57f8c4f2368c4795ee28ef1f283b81",
    "state_change_hash": "0xafb6e14fe47d850fd0a7395bcfb997ffacf4715e0f895cc162c218e4a7564bc6",
    "event_root_hash": "0x414343554d554c41544f525f504c414345484f4c4445525f4841534800000000",
    "state_checkpoint_hash": "0x4bbcd8d72e714d2e019f669e119d73169f7d378950ba34b27900fb9199cc73ec",
    "gas_used": "0",
    "success": true,
    "vm_status": "Executed successfully",
    "accumulator_root_hash": "0x5232557c436793e73468483a5488d0d3825b08792050ef06e3a03dfb002d2b40",
    "changes": [],
    "timestamp": "1691977636550021",
    "type": "state_checkpoint_transaction"
  }
]
```
