# Get account resource

**`GET /accounts/{address}/resources/{resource_type}`**

Returns the resource for an account address. You can specify the ledger version of the transaction; if no ledger version is specified, the latest ledger version is returned.
Aptos nodes prune account state history according to a configurable retention period. If the requested ledger version has been pruned, the server responds with a 410 status code.

## Parameters

| Name | In | Type | Required | Description |
|------|----|------|----------|-------------|
| address | path | string | ✓ | The address of the account to retrieve. Account addresses without the hexadecimal prefix can also be queried.  |
| resource_type | path | string | ✓ | The type name of the target resource (struct) to retrieve.  |
| ledger_version | query | integer |  | The ledger version of the transaction to fetch account state from. If no value is provided, the latest version is used.  |

## Response

| Name | Type | Required | Description |
|------|------|----------|-------------|
| type | string |  | String representation of a Move struct tag. This type exists to allow MoveStructTag to be specified as a path or query parameter (e.g. used in get_events_by_event_handle). This value is composed of: 1. move_module_address, module_name, struct_name joined by two colons (::) 2. Generic type parameters of the struct specified as comma-separated values Examples: - `0x1::coin::CoinStore<0x1::aptos_coin::aptosCoin>` - `0x1::account::Account`  |
| data | object |  | Partial data from within an account resource, displayed in JSON representation. Specifically, a map with the top-level field keys of the given resource as keys and arbitrary JSON values/objects as values.  - Move bool type: Converted to boolean. - u8~i32: Converted to integer. - u64~i256: Converted to string. - address: Converted to HexEncodedBytes string (e.g. 0x1). - Struct: Converted to object. - vector: Converted to array.   - However, vector<u8> is converted to a HexEncodedBytes string starting with 0x. - 0x1::string::String: Converted to regular string.  |

### Example

```json
{
  "type": "0x1::account::Account",
  "data": {
    "authentication_key": "0xca62eccbbdb22b5de18165d0bdf2d7127569b91498f0a7f6944028793cef8137",
    "coin_register_events": {
      "counter": "1",
      "guid": {
        "id": {
          "addr": "0xca62eccbbdb22b5de18165d0bdf2d7127569b91498f0a7f6944028793cef8137",
          "creation_num": "0"
        }
      }
    },
    "guid_creation_num": "4",
    "key_rotation_events": {
      "counter": "0",
      "guid": {
        "id": {
          "addr": "0xca62eccbbdb22b5de18165d0bdf2d7127569b91498f0a7f6944028793cef8137",
          "creation_num": "1"
        }
      }
    },
    "rotation_capability_offer": {
      "for": {
        "vec": []
      }
    },
    "sequence_number": "2581041",
    "signer_capability_offer": {
      "for": {
        "vec": []
      }
    }
  }
}
```
