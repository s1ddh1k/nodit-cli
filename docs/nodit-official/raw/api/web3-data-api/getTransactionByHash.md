# Get Transaction By Hash

**`POST /{chain}/{network}/blockchain/getTransactionByHash`**

Retrieves information about a specific transaction.

## Parameters

| Name | In | Type | Required | Description |
|------|----|------|----------|-------------|
| chain | path | string | ✓ | Parameter to specify the target chain for the query. |
| network | path | string | ✓ | Parameter to specify the network of the target chain. Supported networks may vary depending on the chain. |

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
| transactionHash | string | ✓ | Parameter specifying the transaction hash to query. The transaction hash must be entered as a 64-character hexadecimal string (including "0x"). |
| withBalanceChanges | boolean |  | Parameter specifying whether to include the balanceChanges field in the response.  - balanceChanges is a field containing balance changes for Native token (APT) and Tokens. - Response speed may be slower when true is set for this parameter. |

## Response

| Name | Type | Required | Description |
|------|------|----------|-------------|
| transactionHash | string | ✓ | A field that represents the unique identifier of the transaction. A cryptographic hash computed from all transaction data (signed payload), provided as a 64-character hexadecimal string prefixed with 0x. Used to verify transaction integrity and to uniquely identify the transaction.  |
| transactionVersion | integer | ✓ | A field that represents the version of the transaction. A unique sequence ID indicating the order of the transaction on the chain, incrementing sequentially from 0. Used to ensure transaction execution order and to query the state of a specific transaction.  |
| blockHeight | integer | ✓ | A field that represents the height of the block containing the transaction. Provided as an integer that increments sequentially from the genesis block (0). Used to identify which block the transaction was included in and to verify if the transaction has been finalized.  |
| blockTimestamp | integer | ✓ | A field that represents the creation time of the block containing the transaction. Provided as a UNIX timestamp; this value means both when the block was created and when the transactions in that block were processed.  |
| expirationTimestampSecs | integer | ✓ | A field that represents the expiration time of the transaction. Provided as a UNIX timestamp in seconds; the transaction cannot be included in the chain after this time. Sets the validity period of the transaction to prevent old transactions from being processed.  |
| gasUnitPrice | string | ✓ | A field that represents the gas unit price of the transaction. Provided in Octas (1 APT = 10^8 Octas); indicates the amount to pay per gas unit. Used to determine transaction processing priority and to calculate transaction fees.  |
| gasUsed | string | ✓ | A field that represents the amount of gas actually consumed during transaction execution. A unit that measures the amount of computing resources consumed for transaction processing. Actual transaction fee is calculated as gasUsed * gasUnitPrice.  |
| maxGasAmount | string | ✓ | A field that represents the maximum amount of gas available for the transaction. The transaction fails if this value is exceeded during execution; serves as a cap on gas cost. Prevents users from paying unexpectedly high fees.  |
| secondarySigners | array | ✓ | A field that represents the list of addresses of additional signers in a multi-signature transaction. Contains the addresses of accounts that signed the transaction in addition to the primary signer (sender). Used for complex permission management or transactions requiring multiple approvals.  |
| sender | string | ✓ | A field that represents the address of the account that initiated the transaction. Provided as a 64-character hexadecimal string prefixed with 0x; identifies the principal of the transaction. The account against which transaction fee payment and sequence number management are applied.  |
| sequenceNumber | integer | ✓ | A field that represents the current sequence number of the sender account. Starts at 0 and increments by 1 per transaction per account; prevents replay attacks and ensures transaction order. Transactions with the same sequence number cannot be executed redundantly.  |
| success | boolean | ✓ | A field that indicates whether the transaction executed successfully. true means the transaction was executed successfully; false means an error occurred during execution.  |
| vmStatus | string | ✓ | A field that represents the execution result status of the transaction. Returns 'Executed successfully' on success; includes specific error messages on failure. Helps analyze the cause of transaction failures and troubleshoot issues.  |
| events | array | ✓ | A field that represents the list of events emitted during transaction execution. Each event records state changes or important operations caused by the transaction. Provides information to track transaction impact and for integration with external systems.  |
| events[].eventIndex | integer | ✓ | A field that represents the order index of the event within the transaction. Indicates the order of the current event among multiple events emitted in a single transaction. Starts at 0 and increments in event emission order; used to identify the order of events within the same transaction.  |
| events[].eventType | string | ✓ | A field that represents the type of the event. Provided as the full path (address::module::struct) of the event struct defined in the Move module. Used to identify the type of operation that caused the balance change.  |
| events[].accountAddress | string | ✓ | A field that represents the address of the resource owner that emitted the event. Provided as a 64-character hexadecimal string prefixed with 0x; identifies the owner of the resource where the event occurred. The account that owns the event handle; can be used as a filter condition when querying events.  |
| events[].data | object | ✓ | A field that represents the specific data of the event. The fields of the event struct defined in eventType are provided as a JSON object. Contains the event details and may have different structures depending on the event type.  |
| events[].creationNumber | integer | ✓ | A field that represents the creation number of the event handle. A unique identifier to distinguish event handles created by an account, incrementing sequentially from 0. Used to distinguish different types of events emitted from the same account.  |
| events[].sequenceNumber | integer | ✓ | A field that represents the sequence number of the event emitted from the event handle. Indicates the order of events emitted from the same event handle. Starts at 0 and increments sequentially; ensures the time order of events emitted from a specific event handle.  |
| events[].objectAddress | string | ✓ | A field that represents the address of the Object when the event is for an Object resource. Provided as a 64-character hexadecimal string prefixed with 0x; the unique identifier for Object-based resources. Used to track and query events related to Object resources.  |
| events[].objectOwnerAddress | string | ✓ | A field that represents the owner address of the Object identified by objectAddress. Provided as a 64-character hexadecimal string prefixed with 0x; identifies the current owner of the Object. Used for Object ownership tracking and permission verification.  |
| balanceInAccounts | array | ✓ | A field that represents the list of addresses of accounts that received assets in the transaction. Contains addresses of accounts with positive values (deposits) in balanceChanges. Used to track asset transfer recipients and analyze deposit history.  |
| balanceOutAccounts | array | ✓ | A field that represents the list of addresses of accounts that sent assets in the transaction. Contains addresses of accounts with negative values (withdrawals) in balanceChanges. Used to track asset transfer senders and analyze withdrawal history.  |
| balanceChangedTokens | array | ✓ | A field that represents the list of token types whose balances changed due to the transaction. Each token is provided in 'moduleAddress::moduleName::structName' format. Used to identify the types of assets affected by the transaction and to track changes per asset.  |
| balanceChanges | object |  |  |
| balanceChanges.blockHeight | integer |  | A field that represents the height of the block containing this balance change. An integer that increments sequentially starting from the genesis block (0), used as an indicator for the blockchain length and the position of a specific block.  |
| balanceChanges.blockTimestamp | integer |  | A field that represents the block creation time. Provided as a UNIX timestamp, used for block creation order and time tracking. Block timestamps are used to ensure transaction order and measure inter-block creation intervals.  |
| balanceChanges.transactionHash | string |  | A field that represents the hash of the transaction that caused the balance change. A cryptographic hash computed from all transaction data, provided as a 64-character hexadecimal string prefixed with 0x. This hash enables retrieval of full information for the transaction related to the balance change.  |
| balanceChanges.transactionVersion | integer |  | A field that represents the version of the transaction where the balance change occurred. A unique sequence ID indicating the order of the transaction on the chain, incrementing sequentially from 0.  |
| balanceChanges.eventIndex | integer | ✓ | A field that represents the order index of the event within the transaction. Indicates the order of the current event among multiple events emitted in a single transaction. Starts at 0 and increments in event emission order; used to identify the order of events within the same transaction.  |
| balanceChanges.eventType | string | ✓ | A field that represents the type of the event. Provided as the full path (address::module::struct) of the event struct defined in the Move module. Used to identify the type of operation that caused the balance change.  |
| balanceChanges.subEventIndex | integer | ✓ | A field that represents the order of asset changes when multiple asset changes occur within a single event. Identifies the order of multiple asset changes within an event sharing the same eventIndex. Starts at 0 and increments sequentially; plays an important role in complex asset operations (e.g., swaps).  |
| balanceChanges.accountAddress | string | ✓ | A field that represents the address of the account where the balance change occurred. Provided as a 64-character hexadecimal string prefixed with 0x; identifies the account whose asset balance changed. This address is either the asset owner or the destination account the asset was transferred to.  |
| balanceChanges.assetType | string | ✓ | A field that represents the type of the asset whose balance changed. Provided in one of two formats: - Coin: Asset ID in Move struct format (e.g., 0x1::aptos_coin::AptosCoin) - Fungible Asset: Object address that owns the metadata of the asset Used to uniquely identify the type of asset.  |
| balanceChanges.linkedAssetType | string | ✓ | A common identifier that links Coin and Fungible Asset when they have been migrated. This field serves as a unified key that groups Coin/Fungible Asset into a single asset unit for identification and retrieval, providing the Object address where the migrated Fungible Asset's Metadata is stored.  |
| balanceChanges.changeValue | string | ✓ | A field that represents the balance change amount. Provided as a string in integer form. - Positive: indicates the account balance increased - Negative: indicates the account balance decreased The actual value must be interpreted considering decimals.  |
| balanceChanges.tokenStandard | string | ✓ | A field that represents the standard type of the asset. - v1: Assets following the Coin standard - v2: Assets following the Fungible Asset standard  |
| balanceChanges.transferType | string | ✓ | A field that represents the transfer direction or action type. Indicates the specific type of balance change; the following values are possible: - deposit: asset deposit - withdraw: asset withdrawal - swap_in: deposit from swap - swap_out: withdrawal from swap - gas: gas fee payment - refund_gas: gas fee refund  |
