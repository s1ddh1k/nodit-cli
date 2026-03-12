# getVoteAccounts

**`POST /`**

Returns the current Solana version running on the node

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
| jsonrpc | string | ✓ |  |
| id | integer | ✓ |  |
| method | string | ✓ |  |
| params | array | ✓ | It contains following items: 1. ConfigurationObject (`optional`)  |
| params[].commitment | string |  | The commitment describes how finalized a block is at that point in time. - finalized - the node will query the most recent block confirmed by supermajority of the cluster as having reached maximum lockout, meaning the cluster has recognized this block as finalized - confirmed - the node will query the most recent block that has been voted on by supermajority of the cluster. - processed - the node will query its most recent block. Note that the block may still be skipped by the cluster.  |
| params[].votePubkey | string |  | Only return results for this validator vote address (base-58 encoded) |
| params[].keepUnstakedDelinquents | boolean |  | Do not filter out delinquent validators with no stake |
| params[].delinquentSlotDistance | integer |  | Specify the number of slots behind the tip that a validator must fall to be considered delinquent. NOTE: For the sake of consistency between ecosystem products, it is not recommended that this argument be specified.  |

### Example

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "getVoteAccounts",
  "params": [
    {
      "commitment": "finalized",
      "votePubkey": "i7NyKBMJCA9bLM2nsGyAGCKHECuR2L5eh4GqFciuwNT"
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
| result.current | array |  | Array of all vote accounts currently participating in the validator set  |
| result.current[].activatedStake | integer |  | The stake, in lamports, delegated to this vote account and active in this epoch |
| result.current[].commission | integer |  | Percentage (0-100) of rewards payout owed to the vote account |
| result.current[].epochCredits | array |  | Latest history of earned credits for up to five epochs, as an array of arrays containing: [epoch, credits, previousCredits]  |
| result.current[].epochVoteAccount | boolean |  | Whether the vote account is staked for this epoch |
| result.current[].lastVote | integer |  | Most recent slot voted on by this vote account |
| result.current[].nodePubkey | string |  | Validator identity, as base-58 encoded string |
| result.current[].rootSlot | integer |  | Current root slot for this vote account |
| result.current[].votePubkey | string |  | Vote account address, as base-58 encoded string |
| result.delinquent | array |  | Array of all vote accounts that are delinquent  |
| result.delinquent[].activatedStake | integer |  | The stake, in lamports, delegated to this vote account and active in this epoch |
| result.delinquent[].commission | integer |  | Percentage (0-100) of rewards payout owed to the vote account |
| result.delinquent[].epochCredits | array |  | Latest history of earned credits for up to five epochs, as an array of arrays containing: [epoch, credits, previousCredits]  |
| result.delinquent[].epochVoteAccount | boolean |  | Whether the vote account is staked for this epoch |
| result.delinquent[].lastVote | integer |  | Most recent slot voted on by this vote account |
| result.delinquent[].nodePubkey | string |  | Validator identity, as base-58 encoded string |
| result.delinquent[].rootSlot | integer |  | Current root slot for this vote account |
| result.delinquent[].votePubkey | string |  | Vote account address, as base-58 encoded string |
