# sui_getTransactionBlock

**`POST /`**

Return the transaction response object.

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
  "method": "sui_getTransactionBlock",
  "params": [
    "Hay2tj3GcDYcE3AMHrej5WDsHGPVAYsegcubixLUvXUF",
    {
      "showInput": true,
      "showRawInput": false,
      "showEffects": true,
      "showEvents": true,
      "showObjectChanges": false,
      "showBalanceChanges": false,
      "showRawEffects": false
    }
  ]
}
```

## Response

| Name | Type | Required | Description |
|------|------|----------|-------------|
| jsonrpc | string |  |  |
| id | integer |  |  |
| result | object |  |  |
| result.balanceChanges | array,null |  |  |
| result.checkpoint | string |  |  |
| result.confirmedLocalExecution | boolean,null |  |  |
| result.digest | string | ✓ |  |
| result.effects | object |  | The response from processing a transaction or a certified transaction |
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
| result.errors | array |  |  |
| result.events | array,null |  |  |
| result.objectChanges | array,null |  |  |
| result.rawEffects | array |  |  |
| result.rawTransaction | object |  | BCS encoded [SenderSignedData] that includes input object references returns empty array if `show_raw_transaction` is false |
| result.timestampMs | string |  |  |
| result.transaction | object |  |  |
| result.transaction.data | object | ✓ |  |
| result.transaction.txSignatures | array | ✓ |  |

### Example

```json
{
  "jsonrpc": "2.0",
  "result": {
    "digest": "Hay2tj3GcDYcE3AMHrej5WDsHGPVAYsegcubixLUvXUF",
    "transaction": {
      "data": {
        "messageVersion": "v1",
        "transaction": {
          "kind": "ProgrammableTransaction",
          "inputs": [
            {
              "type": "pure",
              "valueType": "address",
              "value": "0x8196d048b7a6d04c8edc89579d86fd3fc90c52f9a14c6b812b94fe613c5bcebb"
            },
            {
              "type": "object",
              "objectType": "immOrOwnedObject",
              "objectId": "0x5eeb1d449e2516166d57d71fdeb154d0dc9ecdb7b30057d0a932684cac352cdc",
              "version": "2",
              "digest": "GK4NxEKSrK88XkPNeuBqtJYPmU9yMTWMD7K9TdU4ybKN"
            }
          ],
          "transactions": [
            {
              "TransferObjects": [
                [
                  {
                    "Input": 1
                  }
                ],
                {
                  "Input": 0
                }
              ]
            }
          ]
        },
        "sender": "0x82179c57d5895babfb655cd62e8e886a53334b5e7be9be658eb759cc35e3fc66",
        "gasData": {
          "payment": [
            {
              "objectId": "0x1a3e898029d024eec1d44c6af5e2facded84d03b5373514f16e3d66e00081051",
              "version": 2,
              "digest": "7nDZ5J4VyvYGUbX2f6mQdhkr3RFrb3vZqui1ogoyApD9"
            }
          ],
          "owner": "0x82179c57d5895babfb655cd62e8e886a53334b5e7be9be658eb759cc35e3fc66",
          "price": "10",
          "budget": "100000"
        }
      },
      "txSignatures": [
        "AMU7cJTEsJ5WoVlKZ2zsVuGMk9linDuNqLV9eGIIrqarP2x4R9riuvmmMgXfdxMm7jTzYxbHrsDNMwlxpTbbFghtCxWrqsEEHAdxoMDwblU5hyWJ8H3zFvk20E2fO5bzHA=="
      ]
    },
    "rawTransaction": "AQAAAAAAAgAggZbQSLem0EyO3IlXnYb9P8kMUvmhTGuBK5T+YTxbzrsBAF7rHUSeJRYWbVfXH96xVNDcns23swBX0KkyaEysNSzcAgAAAAAAAAAg43+UGkUe+CCaD7+/G1SbK7Jrjq7giJUUbfJ7w88mEMEBAQEBAQABAACCF5xX1Ylbq/tlXNYujohqUzNLXnvpvmWOt1nMNeP8ZgEaPomAKdAk7sHUTGr14vrN7YTQO1NzUU8W49ZuAAgQUQIAAAAAAAAAIGS7c6HtWLLBiwy/N3eS4gbmuA1NXupk4ucFY7FYkCbEghecV9WJW6v7ZVzWLo6IalMzS1576b5ljrdZzDXj/GYKAAAAAAAAAKCGAQAAAAAAAAFhAMU7cJTEsJ5WoVlKZ2zsVuGMk9linDuNqLV9eGIIrqarP2x4R9riuvmmMgXfdxMm7jTzYxbHrsDNMwlxpTbbFghtCxWrqsEEHAdxoMDwblU5hyWJ8H3zFvk20E2fO5bzHA==",
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
      "transactionDigest": "6AyFnAuKAKCqm1cD94EyGzBqJCDDJ716ojjmsKF2rqoi",
      "mutated": [
        {
          "owner": {
            "AddressOwner": "0x82179c57d5895babfb655cd62e8e886a53334b5e7be9be658eb759cc35e3fc66"
          },
          "reference": {
            "objectId": "0x1a3e898029d024eec1d44c6af5e2facded84d03b5373514f16e3d66e00081051",
            "version": 2,
            "digest": "7nDZ5J4VyvYGUbX2f6mQdhkr3RFrb3vZqui1ogoyApD9"
          }
        },
        {
          "owner": {
            "AddressOwner": "0x8196d048b7a6d04c8edc89579d86fd3fc90c52f9a14c6b812b94fe613c5bcebb"
          },
          "reference": {
            "objectId": "0x5eeb1d449e2516166d57d71fdeb154d0dc9ecdb7b30057d0a932684cac352cdc",
            "version": 2,
            "digest": "GK4NxEKSrK88XkPNeuBqtJYPmU9yMTWMD7K9TdU4ybKN"
          }
        }
      ],
      "gasObject": {
        "owner": {
          "ObjectOwner": "0x82179c57d5895babfb655cd62e8e886a53334b5e7be9be658eb759cc35e3fc66"
        },
        "reference": {
          "objectId": "0x1a3e898029d024eec1d44c6af5e2facded84d03b5373514f16e3d66e00081051",
          "version": 2,
          "digest": "7nDZ5J4VyvYGUbX2f6mQdhkr3RFrb3vZqui1ogoyApD9"
        }
      },
      "eventsDigest": "9BQobwxQvJ1JxSXNn8v8htZPTu8FEzJJGgcD4kgLUuMd"
    },
    "objectChanges": [
      {
        "type": "transferred",
        "sender": "0x82179c57d5895babfb655cd62e8e886a53334b5e7be9be658eb759cc35e3fc66",
        "recipient": {
          "AddressOwner": "0x8196d048b7a6d04c8edc89579d86fd3fc90c52f9a14c6b812b94fe613c5bcebb"
        },
        "objectType": "0x2::example::Object",
        "objectId": "0x5eeb1d449e2516166d57d71fdeb154d0dc9ecdb7b30057d0a932684cac352cdc",
        "version": "2",
        "digest": "64UQ3a7m1mjWuzgyGoH8RnMyPGDN4XYTC9dS4qiSfdK4"
      }
    ]
  },
  "id": 1
}
```
