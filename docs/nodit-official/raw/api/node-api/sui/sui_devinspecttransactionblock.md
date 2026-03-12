# sui_devInspectTransactionBlock

**`POST /`**

Runs the transaction in dev-inspect mode. Which allows for nearly any transaction (or Move call) with any arguments. Detailed results are provided, including both the transaction effects and any return values.

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
  "method": "sui_devInspectTransactionBlock",
  "params": [
    "0xd70420418b84502e506794227f897237764dde8d79a01ab2104bf742a277a2ab",
    "AAACACBnxtMcbJcOVn8D72fYEaT4Q2ZbjePygvpIs+AQO6m77QEAagYVO5/EhuEB8OnicDrIZm0GrsxN3355JqNhlwxlpbECAAAAAAAAACDoQ3EipycU+/EOvBcDPFtMkZiSbdzWAw3CwdmQCAtBWAEBAQEBAAEAAC9cVD1xauQ9RT3rOxmbva8bxwMMdoL4dwPc5DEkj+3gASxDgF0Nb1QCp60Npb3sVJx83qBrxKHTOaIlIe6pM7iJAgAAAAAAAAAgnvsgc1pPauyCE27/c+aBnHN3fSsxRAWdEJYzYFOryNAvXFQ9cWrkPUU96zsZm72vG8cDDHaC+HcD3OQxJI/t4AoAAAAAAAAAoIYBAAAAAAAA",
    1000,
    8888,
    null
  ]
}
```

## Response

| Name | Type | Required | Description |
|------|------|----------|-------------|
| id | integer |  |  |
| jsonrpc | string |  |  |
| result | object |  | The response from processing a dev inspect transaction |
| result.effects | object | ✓ | The response from processing a transaction or a certified transaction |
| result.effects.abortError | object |  |  |
| result.effects.created | array |  | ObjectRef and owner of new objects created. |
| result.effects.deleted | array |  | Object Refs of objects now deleted (the old refs). |
| result.effects.dependencies | array |  | The set of transaction digests this transaction depends on. |
| result.effects.eventsDigest | string |  |  |
| result.effects.executedEpoch | object | ✓ | The epoch when this transaction was executed. |
| result.effects.gasObject | object | ✓ | The updated gas object reference. Have a dedicated field for convenient access. It's also included in mutated. |
| result.effects.gasUsed | object | ✓ | Summary of the charges in a transaction. Storage is charged independently of computation. There are 3 parts to the storage charges: `storage_cost`: it is the charge of storage at the time the transaction is executed. The cost of storage is the number of bytes of the objects being mutated multiplied by a variable storage cost per byte `storage_rebate`: this is the amount a user gets back when manipulating an object. The `storage_rebate` is the `storage_cost` for an object minus fees. `non_refundable_storage_fee`: not all the value of the object storage cost is given back to user and there is a small fraction that is kept by the system. This value tracks that charge.  When looking at a gas cost summary the amount charged to the user is `computation_cost + storage_cost - storage_rebate` and that is the amount that is deducted from the gas coins. `non_refundable_storage_fee` is collected from the objects being mutated/deleted and it is tracked by the system in storage funds.  Objects deleted, including the older versions of objects mutated, have the storage field on the objects added up to a pool of "potential rebate". This rebate then is reduced by the "nonrefundable rate" such that: `potential_rebate(storage cost of deleted/mutated objects) = storage_rebate + non_refundable_storage_fee`  |
| result.effects.messageVersion | string | ✓ |  |
| result.effects.modifiedAtVersions | array |  | The version that every modified (mutated or deleted) object had before it was modified by this transaction. |
| result.effects.mutated | array |  | ObjectRef and owner of mutated objects, including gas object. |
| result.effects.sharedObjects | array |  | The object references of the shared objects used in this transaction. Empty if no shared objects were used. |
| result.effects.status | object | ✓ | The status of the execution |
| result.effects.transactionDigest | object | ✓ | The transaction digest |
| result.effects.unwrapped | array |  | ObjectRef and owner of objects that are unwrapped in this transaction. Unwrapped objects are objects that were wrapped into other objects in the past, and just got extracted out. |
| result.effects.unwrappedThenDeleted | array |  | Object refs of objects previously wrapped in other objects but now deleted. |
| result.effects.wrapped | array |  | Object refs of objects now wrapped in other objects. |
| result.error | string |  | Execution error from executing the transactions |
| result.events | array | ✓ | Events that likely would be generated if the transaction is actually run. |
| result.rawEffects | array |  | The raw effects of the transaction that was dev inspected. |
| result.rawTxnData | array |  | The raw transaction data of the transaction that was dev inspected. |
| result.results | array |  | Execution results (including return values) from executing the transactions |
| result.results[].abortError | object |  |  |
| result.results[].created | array |  | ObjectRef and owner of new objects created. |
| result.results[].deleted | array |  | Object Refs of objects now deleted (the old refs). |
| result.results[].dependencies | array |  | The set of transaction digests this transaction depends on. |
| result.results[].eventsDigest | string |  |  |
| result.results[].executedEpoch | object | ✓ | The epoch when this transaction was executed. |
| result.results[].gasObject | object | ✓ | The updated gas object reference. Have a dedicated field for convenient access. It's also included in mutated. |
| result.results[].gasUsed | object | ✓ | Summary of the charges in a transaction. Storage is charged independently of computation. There are 3 parts to the storage charges: `storage_cost`: it is the charge of storage at the time the transaction is executed. The cost of storage is the number of bytes of the objects being mutated multiplied by a variable storage cost per byte `storage_rebate`: this is the amount a user gets back when manipulating an object. The `storage_rebate` is the `storage_cost` for an object minus fees. `non_refundable_storage_fee`: not all the value of the object storage cost is given back to user and there is a small fraction that is kept by the system. This value tracks that charge.  When looking at a gas cost summary the amount charged to the user is `computation_cost + storage_cost - storage_rebate` and that is the amount that is deducted from the gas coins. `non_refundable_storage_fee` is collected from the objects being mutated/deleted and it is tracked by the system in storage funds.  Objects deleted, including the older versions of objects mutated, have the storage field on the objects added up to a pool of "potential rebate". This rebate then is reduced by the "nonrefundable rate" such that: `potential_rebate(storage cost of deleted/mutated objects) = storage_rebate + non_refundable_storage_fee`  |
| result.results[].messageVersion | string | ✓ |  |
| result.results[].modifiedAtVersions | array |  | The version that every modified (mutated or deleted) object had before it was modified by this transaction. |
| result.results[].mutated | array |  | ObjectRef and owner of mutated objects, including gas object. |
| result.results[].sharedObjects | array |  | The object references of the shared objects used in this transaction. Empty if no shared objects were used. |
| result.results[].status | object | ✓ | The status of the execution |
| result.results[].transactionDigest | object | ✓ | The transaction digest |
| result.results[].unwrapped | array |  | ObjectRef and owner of objects that are unwrapped in this transaction. Unwrapped objects are objects that were wrapped into other objects in the past, and just got extracted out. |
| result.results[].unwrappedThenDeleted | array |  | Object refs of objects previously wrapped in other objects but now deleted. |
| result.results[].wrapped | array |  | Object refs of objects now wrapped in other objects. |

### Example

```json
{
  "jsonrpc": "2.0",
  "result": {
    "effects": {
      "messageVersion": "v1",
      "status": {
        "status": "success"
      },
      "executedEpoch": "0",
      "gasUsed": {
        "computationCost": "100",
        "storageCost": "100",
        "storageRebate": "10",
        "nonRefundableStorageFee": "0"
      },
      "transactionDigest": "76gyHCk7FRrGACRqXM7Ybj5uJLtAzgEMJ5P9CeEzxZSG",
      "mutated": [
        {
          "owner": {
            "AddressOwner": "0x2f5c543d716ae43d453deb3b199bbdaf1bc7030c7682f87703dce431248fede0"
          },
          "reference": {
            "objectId": "0x2c43805d0d6f5402a7ad0da5bdec549c7cdea06bc4a1d339a22521eea933b889",
            "version": 2,
            "digest": "BhbWpBeESxuRWvmvLMyb2JNUuFa6j4aG1T4WUiPgKAHm"
          }
        },
        {
          "owner": {
            "AddressOwner": "0x67c6d31c6c970e567f03ef67d811a4f843665b8de3f282fa48b3e0103ba9bbed"
          },
          "reference": {
            "objectId": "0x6a06153b9fc486e101f0e9e2703ac8666d06aecc4ddf7e7926a361970c65a5b1",
            "version": 2,
            "digest": "GdfET1avZReDftpJNB8LSuHJ2cKUheSbEaLMzuPVXHsM"
          }
        }
      ],
      "gasObject": {
        "owner": {
          "ObjectOwner": "0x2f5c543d716ae43d453deb3b199bbdaf1bc7030c7682f87703dce431248fede0"
        },
        "reference": {
          "objectId": "0x2c43805d0d6f5402a7ad0da5bdec549c7cdea06bc4a1d339a22521eea933b889",
          "version": 2,
          "digest": "BhbWpBeESxuRWvmvLMyb2JNUuFa6j4aG1T4WUiPgKAHm"
        }
      },
      "eventsDigest": "6kerMphN4S5QTfd9TAhwMiFq1q9c2YwfpheBfWm85vUq"
    },
    "events": []
  },
  "id": 1
}
```
