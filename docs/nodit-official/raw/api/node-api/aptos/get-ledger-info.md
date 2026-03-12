# Get ledger info

**`GET /`**

Retrieves the latest ledger information. Includes data such as chain ID, role type, ledger version, and epoch.

## Response

| Name | Type | Required | Description |
|------|------|----------|-------------|
| chain_id | integer | ✓ | Identifier of the current chain. |
| epoch | string | ✓ | Current consensus epoch number, based on the validator set for that period. |
| ledger_version | string | ✓ | Ledger version of the latest transaction. |
| oldest_ledger_version | string | ✓ | Ledger version of the oldest transaction. |
| ledger_timestamp | string | ✓ | Ledger timestamp indicating when the current ledger was created. |
| node_role | string | ✓ | Role of the node (full_node, validator). |
| oldest_block_height | string | ✓ | Oldest block height. |
| block_height | string | ✓ | Latest block height. |
| git_hash | string |  | Git hash of the built API endpoint. Used to identify the exact software version used by the API endpoint. |

### Example

```json
{
  "chain_id": 1,
  "epoch": "3671",
  "ledger_version": "223505345",
  "oldest_ledger_version": "0",
  "ledger_timestamp": "1691977290941952",
  "node_role": "full_node",
  "oldest_block_height": "0",
  "block_height": "80602020",
  "git_hash": "ebfec6c8296848d00b69cc7bfa284d820a3c3566"
}
```
