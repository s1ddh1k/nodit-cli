# sui_executeTransactionBlock

**`POST /`**

Execute the transaction and wait for results if desired. Request types:
1. WaitForEffectsCert: waits for TransactionEffectsCert and then return to client. This mode is a proxy for transaction finality.
2. WaitForLocalExecution: waits for TransactionEffectsCert and make sure the node executed the transaction locally before returning the client.
The local execution makes sure this node is aware of this transaction when client fires subsequent queries.
However if the node fails to execute the transaction locally in a timely manner, a bool type in the response is set to false to indicated the case.
request_type is default to be WaitForEffectsCert unless options.show_events or options.show_effects is true

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
  "method": "sui_executeTransactionBlock",
  "params": [
    "AAACACBqEB6aOvXIBwES+Ahkizbvv43uihqC3kbZUE6WoRCKFwEAjvdvVsOZYzousxC8qRJOXy84znOeqsu2YAaIgE4HhEgCAAAAAAAAACB9w3+ufZMpihJFwxtCBojBaGy00TVtFxgN2C6TpIPFqwEBAQEBAAEAAAS0l6kWtGVmCaf6gnoJGE1vR2gdO6dM4NejbGSysfiHAZ+Q9/hmzCnfsdpjc86U+dldylpA9OF2mRjuv5+64AvTAgAAAAAAAAAgjleHL0UiRGjh/BfIFHCJ3EMY/dQA22c2TvNQyVJnbYUEtJepFrRlZgmn+oJ6CRhNb0doHTunTODXo2xksrH4hwoAAAAAAAAAoIYBAAAAAAAA",
    [
      "AKD4XdltkCyBi1Heb4EJJ3lzuV3F4u7+CYeaE+Fd7qXpaT17yd4tHWjMf4CWq3TuXBLxTpkc2MV39P6p7eMV8QnqvbuA0Q1Bqu4RHV3JPpqmH+C527hWJGUBOZN1j9sg8w=="
    ],
    {
      "showInput": true,
      "showRawInput": true,
      "showEffects": true,
      "showEvents": true,
      "showObjectChanges": true,
      "showBalanceChanges": true,
      "showRawEffects": false
    },
    "WaitForLocalExecution"
  ]
}
```

## Response

| Name | Type | Required | Description |
|------|------|----------|-------------|
| id | integer |  |  |
| jsonrpc | string |  |  |
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
    "digest": "Gx7V6EteEAqSsptc1kbi1nUEMP7mhr4qV2cYS7JjzbBd",
    "transaction": {
      "data": {
        "messageVersion": "v1",
        "transaction": {
          "kind": "ProgrammableTransaction",
          "inputs": [
            {
              "type": "pure",
              "valueType": "address",
              "value": "0x6a101e9a3af5c8070112f808648b36efbf8dee8a1a82de46d9504e96a1108a17"
            },
            {
              "type": "object",
              "objectType": "immOrOwnedObject",
              "objectId": "0x8ef76f56c399633a2eb310bca9124e5f2f38ce739eaacbb6600688804e078448",
              "version": "2",
              "digest": "9Tvs1pGrMbNv7kkr1PoKLsWamyQpaFz5UWbL2AQ1ezk2"
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
        "sender": "0x04b497a916b4656609a7fa827a09184d6f47681d3ba74ce0d7a36c64b2b1f887",
        "gasData": {
          "payment": [
            {
              "objectId": "0x9f90f7f866cc29dfb1da6373ce94f9d95dca5a40f4e1769918eebf9fbae00bd3",
              "version": 2,
              "digest": "AaeJbTYkUuyromsivxzkoxSkHt7pCESTyQG7xz6nbQ2G"
            }
          ],
          "owner": "0x04b497a916b4656609a7fa827a09184d6f47681d3ba74ce0d7a36c64b2b1f887",
          "price": "10",
          "budget": "100000"
        }
      },
      "txSignatures": [
        "AKD4XdltkCyBi1Heb4EJJ3lzuV3F4u7+CYeaE+Fd7qXpaT17yd4tHWjMf4CWq3TuXBLxTpkc2MV39P6p7eMV8QnqvbuA0Q1Bqu4RHV3JPpqmH+C527hWJGUBOZN1j9sg8w=="
      ]
    },
    "rawTransaction": "AQAAAAAAAgAgahAemjr1yAcBEvgIZIs277+N7ooagt5G2VBOlqEQihcBAI73b1bDmWM6LrMQvKkSTl8vOM5znqrLtmAGiIBOB4RIAgAAAAAAAAAgfcN/rn2TKYoSRcMbQgaIwWhstNE1bRcYDdguk6SDxasBAQEBAQABAAAEtJepFrRlZgmn+oJ6CRhNb0doHTunTODXo2xksrH4hwGfkPf4Zswp37HaY3POlPnZXcpaQPThdpkY7r+fuuAL0wIAAAAAAAAAII5Xhy9FIkRo4fwXyBRwidxDGP3UANtnNk7zUMlSZ22FBLSXqRa0ZWYJp/qCegkYTW9HaB07p0zg16NsZLKx+IcKAAAAAAAAAKCGAQAAAAAAAAFhAKD4XdltkCyBi1Heb4EJJ3lzuV3F4u7+CYeaE+Fd7qXpaT17yd4tHWjMf4CWq3TuXBLxTpkc2MV39P6p7eMV8QnqvbuA0Q1Bqu4RHV3JPpqmH+C527hWJGUBOZN1j9sg8w==",
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
      "transactionDigest": "9agZ3azEMgMqxrDVG8P4GddELfWag2HhimEkpjixHhGE",
      "mutated": [
        {
          "owner": {
            "AddressOwner": "0x04b497a916b4656609a7fa827a09184d6f47681d3ba74ce0d7a36c64b2b1f887"
          },
          "reference": {
            "objectId": "0x9f90f7f866cc29dfb1da6373ce94f9d95dca5a40f4e1769918eebf9fbae00bd3",
            "version": 2,
            "digest": "AaeJbTYkUuyromsivxzkoxSkHt7pCESTyQG7xz6nbQ2G"
          }
        },
        {
          "owner": {
            "AddressOwner": "0x6a101e9a3af5c8070112f808648b36efbf8dee8a1a82de46d9504e96a1108a17"
          },
          "reference": {
            "objectId": "0x8ef76f56c399633a2eb310bca9124e5f2f38ce739eaacbb6600688804e078448",
            "version": 2,
            "digest": "9Tvs1pGrMbNv7kkr1PoKLsWamyQpaFz5UWbL2AQ1ezk2"
          }
        }
      ],
      "gasObject": {
        "owner": {
          "ObjectOwner": "0x04b497a916b4656609a7fa827a09184d6f47681d3ba74ce0d7a36c64b2b1f887"
        },
        "reference": {
          "objectId": "0x9f90f7f866cc29dfb1da6373ce94f9d95dca5a40f4e1769918eebf9fbae00bd3",
          "version": 2,
          "digest": "AaeJbTYkUuyromsivxzkoxSkHt7pCESTyQG7xz6nbQ2G"
        }
      },
      "eventsDigest": "816hEv4WAW2reK9xkf11PeHiaZJrp7PQT9oGJZhdf9TN"
    },
    "objectChanges": [
      {
        "type": "transferred",
        "sender": "0x04b497a916b4656609a7fa827a09184d6f47681d3ba74ce0d7a36c64b2b1f887",
        "recipient": {
          "AddressOwner": "0x6a101e9a3af5c8070112f808648b36efbf8dee8a1a82de46d9504e96a1108a17"
        },
        "objectType": "0x2::example::Object",
        "objectId": "0x8ef76f56c399633a2eb310bca9124e5f2f38ce739eaacbb6600688804e078448",
        "version": "2",
        "digest": "7PsBHpUW6yfGNov2WrbVafLjgT9nYziQ3gVDbRq6zTbF"
      }
    ]
  },
  "id": 1
}
```
