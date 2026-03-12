# suix_subscribeTransaction

**`POST /`**

Subscribe to a stream of Sui transaction effects

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
| id | integer | ✓ |  |
| jsonrpc | string | ✓ |  |
| method | string | ✓ |  |
| params | array | ✓ |  |
| params[].Checkpoint | string | ✓ |  |

## Response

| Name | Type | Required | Description |
|------|------|----------|-------------|
| jsonrpc | string |  |  |
| id | integer |  |  |
| result | object |  | The response from processing a transaction or a certified transaction |
| result.abortError | object |  |  |
| result.abortError.error_code | integer,null |  |  |
| result.abortError.function | string,null |  |  |
| result.abortError.line | integer,null |  |  |
| result.abortError.module_id | string,null |  |  |
| result.created | array |  | ObjectRef and owner of new objects created. |
| result.created[].owner | object | ✓ | Object is exclusively owned by a single address, and is mutable. |
| result.created[].reference | object | ✓ |  |
| result.deleted | array |  | Object Refs of objects now deleted (the old refs). |
| result.deleted[].digest | object | ✓ | Base64 string representing the object digest |
| result.deleted[].objectId | object | ✓ | Hex code as string representing the object id |
| result.deleted[].version | object | ✓ | Object version. |
| result.dependencies | array |  | The set of transaction digests this transaction depends on. |
| result.eventsDigest | string |  |  |
| result.executedEpoch | object | ✓ | The epoch when this transaction was executed. |
| result.gasObject | object | ✓ | The updated gas object reference. Have a dedicated field for convenient access. It's also included in mutated. |
| result.gasUsed | object | ✓ | Summary of the charges in a transaction. Storage is charged independently of computation. There are 3 parts to the storage charges: `storage_cost`: it is the charge of storage at the time the transaction is executed. The cost of storage is the number of bytes of the objects being mutated multiplied by a variable storage cost per byte `storage_rebate`: this is the amount a user gets back when manipulating an object. The `storage_rebate` is the `storage_cost` for an object minus fees. `non_refundable_storage_fee`: not all the value of the object storage cost is given back to user and there is a small fraction that is kept by the system. This value tracks that charge.  When looking at a gas cost summary the amount charged to the user is `computation_cost + storage_cost - storage_rebate` and that is the amount that is deducted from the gas coins. `non_refundable_storage_fee` is collected from the objects being mutated/deleted and it is tracked by the system in storage funds.  Objects deleted, including the older versions of objects mutated, have the storage field on the objects added up to a pool of "potential rebate". This rebate then is reduced by the "nonrefundable rate" such that: `potential_rebate(storage cost of deleted/mutated objects) = storage_rebate + non_refundable_storage_fee`  |
| result.gasUsed.computationCost | object | ✓ | Cost of computation/execution |
| result.gasUsed.nonRefundableStorageFee | object | ✓ | The fee for the rebate. The portion of the storage rebate kept by the system. |
| result.gasUsed.storageCost | object | ✓ | Storage cost, it's the sum of all storage cost for all objects created or mutated. |
| result.gasUsed.storageRebate | object | ✓ | The amount of storage cost refunded to the user for all objects deleted or mutated in the transaction. |
| result.messageVersion | string | ✓ |  |
| result.modifiedAtVersions | array |  | The version that every modified (mutated or deleted) object had before it was modified by this transaction. |
| result.modifiedAtVersions[].objectId | string | ✓ | Hex string encoding. |
| result.modifiedAtVersions[].sequenceNumber | integer | ✓ |  |
| result.mutated | array |  | ObjectRef and owner of mutated objects, including gas object. |
| result.mutated[].owner | object | ✓ | Object is exclusively owned by a single address, and is mutable. |
| result.mutated[].reference | object | ✓ |  |
| result.sharedObjects | array |  | The object references of the shared objects used in this transaction. Empty if no shared objects were used. |
| result.sharedObjects[].digest | object | ✓ | Base64 string representing the object digest |
| result.sharedObjects[].objectId | object | ✓ | Hex code as string representing the object id |
| result.sharedObjects[].version | object | ✓ | Object version. |
| result.status | object | ✓ | The status of the execution |
| result.transactionDigest | object | ✓ | The transaction digest |
| result.unwrapped | array |  | ObjectRef and owner of objects that are unwrapped in this transaction. Unwrapped objects are objects that were wrapped into other objects in the past, and just got extracted out. |
| result.unwrapped[].owner | object | ✓ | Object is exclusively owned by a single address, and is mutable. |
| result.unwrapped[].reference | object | ✓ |  |
| result.unwrappedThenDeleted | array |  | Object refs of objects previously wrapped in other objects but now deleted. |
| result.unwrappedThenDeleted[].digest | object | ✓ | Base64 string representing the object digest |
| result.unwrappedThenDeleted[].objectId | object | ✓ | Hex code as string representing the object id |
| result.unwrappedThenDeleted[].version | object | ✓ | Object version. |
| result.wrapped | array |  | Object refs of objects now wrapped in other objects. |
| result.wrapped[].digest | object | ✓ | Base64 string representing the object digest |
| result.wrapped[].objectId | object | ✓ | Hex code as string representing the object id |
| result.wrapped[].version | object | ✓ | Object version. |
