# Get account resources

**`GET /accounts/{address}/resources`**

Retrieves all resources for a given account at the ledger version of a specific transaction. If no ledger version is specified, the latest ledger version is used.
Aptos nodes prune account state history according to a configurable retention period. If the requested ledger version has been pruned, the server responds with a 410 status code.

> ⚠️ Notice regarding some API calls
> 
> As the latest version of the Aptos node client excludes the Legacy Indexer, Indexer-related errors may occur when making some API calls. 
> We are currently reviewing options to restore this functionality or provide alternative APIs, and will provide updates as related measures are completed.
> We apologize for any inconvenience.

## Parameters

| Name | In | Type | Required | Description |
|------|----|------|----------|-------------|
| address | path | string | ✓ | The address of the account to retrieve. Account addresses without the hexadecimal prefix can also be queried.  |
| ledger_version | query | integer |  | The ledger version of the transaction to fetch account state from. If no value is provided, the latest version is used.  |
| limit | query | integer |  | The maximum number of account modules to fetch. If no value is provided, the default page size is used.  |
| start | query | string |  | A cursor that specifies the starting position for pagination.  This cursor cannot be arbitrarily generated on the client side. You must first call this endpoint without specifying this query parameter, then use the cursor returned in the X-Aptos-Cursor header of the response.  |

## Response

### Example

```json
[
  {
    "type": "0x1::account::Account",
    "data": {
      "authentication_key": "0xf92bc956b9e25f38a2e4829b58f03ca9724233985cdda3f818bc3e62d6ed7d9c",
      "coin_register_events": {
        "counter": "1",
        "guid": {
          "id": {
            "addr": "0xf92bc956b9e25f38a2e4829b58f03ca9724233985cdda3f818bc3e62d6ed7d9c",
            "creation_num": "0"
          }
        }
      },
      "guid_creation_num": "4",
      "key_rotation_events": {
        "counter": "0",
        "guid": {
          "id": {
            "addr": "0xf92bc956b9e25f38a2e4829b58f03ca9724233985cdda3f818bc3e62d6ed7d9c",
            "creation_num": "1"
          }
        }
      },
      "rotation_capability_offer": {
        "for": {
          "vec": []
        }
      },
      "sequence_number": "1645002",
      "signer_capability_offer": {
        "for": {
          "vec": []
        }
      }
    }
  },
  {
    "type": "0x1::coin::CoinStore<0x1::aptos_coin::AptosCoin>",
    "data": {
      "coin": {
        "value": "1767711144"
      },
      "deposit_events": {
        "counter": "137",
        "guid": {
          "id": {
            "addr": "0xf92bc956b9e25f38a2e4829b58f03ca9724233985cdda3f818bc3e62d6ed7d9c",
            "creation_num": "2"
          }
        }
      },
      "frozen": false,
      "withdraw_events": {
        "counter": "13",
        "guid": {
          "id": {
            "addr": "0xf92bc956b9e25f38a2e4829b58f03ca9724233985cdda3f818bc3e62d6ed7d9c",
            "creation_num": "3"
          }
        }
      }
    }
  }
]
```
