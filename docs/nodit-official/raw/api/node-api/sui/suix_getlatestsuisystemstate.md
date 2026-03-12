# suix_getLatestSuiSystemState

**`POST /`**

Return the latest SUI system state object on-chain.

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
  "method": "suix_getLatestSuiSystemState",
  "params": []
}
```

## Response

| Name | Type | Required | Description |
|------|------|----------|-------------|
| jsonrpc | string |  |  |
| id | integer |  |  |
| result | object |  | This is the JSON-RPC type for the SUI system state object. It flattens all fields to make them top-level fields such that it as minimum dependencies to the internal data structures of the SUI system state type. |
| result.activeValidators | array | ✓ | The list of active validators in the current epoch. |
| result.activeValidators[].commissionRate | string | ✓ |  |
| result.activeValidators[].description | string | ✓ |  |
| result.activeValidators[].exchangeRatesId | object | ✓ | ID of the exchange rate table object. |
| result.activeValidators[].exchangeRatesSize | object | ✓ | Number of exchange rates in the table. |
| result.activeValidators[].gasPrice | string | ✓ |  |
| result.activeValidators[].imageUrl | string | ✓ |  |
| result.activeValidators[].name | string | ✓ |  |
| result.activeValidators[].netAddress | string | ✓ |  |
| result.activeValidators[].networkPubkeyBytes | string | ✓ | Base64 encoding |
| result.activeValidators[].nextEpochCommissionRate | string | ✓ |  |
| result.activeValidators[].nextEpochGasPrice | string | ✓ |  |
| result.activeValidators[].nextEpochNetAddress | string,null |  |  |
| result.activeValidators[].nextEpochNetworkPubkeyBytes | string |  | Base64 encoding |
| result.activeValidators[].nextEpochP2pAddress | string,null |  |  |
| result.activeValidators[].nextEpochPrimaryAddress | string,null |  |  |
| result.activeValidators[].nextEpochProofOfPossession | string |  | Base64 encoding |
| result.activeValidators[].nextEpochProtocolPubkeyBytes | string |  | Base64 encoding |
| result.activeValidators[].nextEpochStake | string | ✓ |  |
| result.activeValidators[].nextEpochWorkerAddress | string,null |  |  |
| result.activeValidators[].nextEpochWorkerPubkeyBytes | string |  | Base64 encoding |
| result.activeValidators[].operationCapId | string | ✓ | Hex string encoding. |
| result.activeValidators[].p2pAddress | string | ✓ |  |
| result.activeValidators[].pendingPoolTokenWithdraw | object | ✓ | Pending pool token withdrawn during the current epoch, emptied at epoch boundaries. |
| result.activeValidators[].pendingStake | object | ✓ | Pending stake amount for this epoch. |
| result.activeValidators[].pendingTotalSuiWithdraw | object | ✓ | Pending stake withdrawn during the current epoch, emptied at epoch boundaries. |
| result.activeValidators[].poolTokenBalance | object | ✓ | Total number of pool tokens issued by the pool. |
| result.activeValidators[].primaryAddress | string | ✓ |  |
| result.activeValidators[].projectUrl | string | ✓ |  |
| result.activeValidators[].proofOfPossessionBytes | string | ✓ | Base64 encoding |
| result.activeValidators[].protocolPubkeyBytes | string | ✓ | Base64 encoding |
| result.activeValidators[].rewardsPool | object | ✓ | The epoch stake rewards will be added here at the end of each epoch. |
| result.activeValidators[].stakingPoolActivationEpoch | string |  |  |
| result.activeValidators[].stakingPoolDeactivationEpoch | string |  |  |
| result.activeValidators[].stakingPoolId | object | ✓ | ID of the staking pool object. |
| result.activeValidators[].stakingPoolSuiBalance | object | ✓ | The total number of SUI tokens in this pool. |
| result.activeValidators[].suiAddress | string | ✓ | Hex string encoding. |
| result.activeValidators[].votingPower | string | ✓ |  |
| result.activeValidators[].workerAddress | string | ✓ |  |
| result.activeValidators[].workerPubkeyBytes | string | ✓ | Base64 encoding |
| result.atRiskValidators | array | ✓ | Map storing the number of epochs for which each validator has been below the low stake threshold. |
| result.epoch | object | ✓ | The current epoch ID, starting from 0. |
| result.epochDurationMs | object | ✓ | The duration of an epoch, in milliseconds. |
| result.epochStartTimestampMs | object | ✓ | Unix timestamp of the current epoch start |
| result.inactivePoolsId | object | ✓ | ID of the object that maps from a staking pool ID to the inactive validator that has that pool as its staking pool. |
| result.inactivePoolsSize | object | ✓ | Number of inactive staking pools. |
| result.maxValidatorCount | object | ✓ | Maximum number of active validators at any moment. We do not allow the number of validators in any epoch to go above this. |
| result.minValidatorJoiningStake | object | ✓ | Lower-bound on the amount of stake required to become a validator. |
| result.pendingActiveValidatorsId | object | ✓ | ID of the object that contains the list of new validators that will join at the end of the epoch. |
| result.pendingActiveValidatorsSize | object | ✓ | Number of new validators that will join at the end of the epoch. |
| result.pendingRemovals | array | ✓ | Removal requests from the validators. Each element is an index pointing to `active_validators`. |
| result.protocolVersion | object | ✓ | The current protocol version, starting from 1. |
| result.referenceGasPrice | object | ✓ | The reference gas price for the current epoch. |
| result.safeMode | boolean | ✓ | Whether the system is running in a downgraded safe mode due to a non-recoverable bug. This is set whenever we failed to execute advance_epoch, and ended up executing advance_epoch_safe_mode. It can be reset once we are able to successfully execute advance_epoch. |
| result.safeModeComputationRewards | object | ✓ | Amount of computation rewards accumulated (and not yet distributed) during safe mode. |
| result.safeModeNonRefundableStorageFee | object | ✓ | Amount of non-refundable storage fee accumulated during safe mode. |
| result.safeModeStorageRebates | object | ✓ | Amount of storage rebates accumulated (and not yet burned) during safe mode. |
| result.safeModeStorageRewards | object | ✓ | Amount of storage rewards accumulated (and not yet distributed) during safe mode. |
| result.stakeSubsidyBalance | object | ✓ | Balance of SUI set aside for stake subsidies that will be drawn down over time. |
| result.stakeSubsidyCurrentDistributionAmount | object | ✓ | The amount of stake subsidy to be drawn down per epoch. This amount decays and decreases over time. |
| result.stakeSubsidyDecreaseRate | integer | ✓ | The rate at which the distribution amount decays at the end of each period. Expressed in basis points. |
| result.stakeSubsidyDistributionCounter | object | ✓ | This counter may be different from the current epoch number if in some epochs we decide to skip the subsidy. |
| result.stakeSubsidyPeriodLength | object | ✓ | Number of distributions to occur before the distribution amount decays. |
| result.stakeSubsidyStartEpoch | object | ✓ | The starting epoch in which stake subsidies start being paid out |
| result.stakingPoolMappingsId | object | ✓ | ID of the object that maps from staking pool's ID to the sui address of a validator. |
| result.stakingPoolMappingsSize | object | ✓ | Number of staking pool mappings. |
| result.storageFundNonRefundableBalance | object | ✓ | The non-refundable portion of the storage fund coming from storage reinvestment, non-refundable storage rebates and any leftover staking rewards. |
| result.storageFundTotalObjectStorageRebates | object | ✓ | The storage rebates of all the objects on-chain stored in the storage fund. |
| result.systemStateVersion | object | ✓ | The current version of the system state data structure type. |
| result.totalStake | object | ✓ | Total amount of stake from all active validators at the beginning of the epoch. |
| result.validatorCandidatesId | object | ✓ | ID of the object that stores preactive validators, mapping their addresses to their `Validator` structs. |
| result.validatorCandidatesSize | object | ✓ | Number of preactive validators. |
| result.validatorLowStakeGracePeriod | object | ✓ | A validator can have stake below `validator_low_stake_threshold` for this many epochs before being kicked out. |
| result.validatorLowStakeThreshold | object | ✓ | Validators with stake amount below `validator_low_stake_threshold` are considered to have low stake and will be escorted out of the validator set after being below this threshold for more than `validator_low_stake_grace_period` number of epochs. |
| result.validatorReportRecords | array | ✓ | A map storing the records of validator reporting each other. |
| result.validatorVeryLowStakeThreshold | object | ✓ | Validators with stake below `validator_very_low_stake_threshold` will be removed immediately at epoch change, no grace period. |

### Example

```json
{
  "jsonrpc": "2.0",
  "result": "some_system_state",
  "id": 1
}
```
