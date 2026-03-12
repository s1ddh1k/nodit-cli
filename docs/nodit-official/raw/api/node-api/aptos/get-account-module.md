# Get account module

**`GET /accounts/{address}/module/{module_name}`**

Returns the module for an account address. You can specify the ledger version of the transaction; if no ledger version is specified, the latest ledger version is returned.
Aptos nodes prune account state history according to a configurable retention period. If the requested ledger version has been pruned, the server responds with a 410 status code.

## Parameters

| Name | In | Type | Required | Description |
|------|----|------|----------|-------------|
| address | path | string | ✓ | The address of the account to retrieve. Account addresses without the hexadecimal prefix can also be queried.  |
| module_name | path | string | ✓ | The name of the specific module to query.  |
| ledger_version | query | integer |  | The ledger version of the transaction to fetch account state from. If no value is provided, the latest version is used.  |

## Response

| Name | Type | Required | Description |
|------|------|----------|-------------|
| bytecode | string |  | All byte (Vec) data is expressed as a hex-encoded string starting with 0x, with two hex digits per byte. Unlike the Address type, HexEncodedBytes does not omit any zeros.  |
| abi | object |  | A Move module  |
| abi.address | string | ✓ | A 32-byte Aptos account address encoded in hexadecimal. When expressed as a string, it is a 64-character hexadecimal string. It may be abbreviated by omitting leading zeros and adding 0x prefix. For example, the address 0x0000000000000000000000000000000000000000000000000000000000000001 can be expressed as 0x1.  |
| abi.name | string | ✓ |  |
| abi.friends | array | ✓ | Friends of the module |
| abi.exposed_functions | array | ✓ |  |
| abi.structs | array | ✓ |  |
| abi.structs[].name | string | ✓ |  |
| abi.structs[].is_native | boolean | ✓ |  |
| abi.structs[].is_event | boolean |  | Whether the struct is an event struct |
| abi.structs[].is_enum | boolean |  | Whether the struct is an enum |
| abi.structs[].abilities | array | ✓ | Ability associated with the Move struct |
| abi.structs[].generic_type_params | array | ✓ | Generic type parameters associated with the Move function. |
| abi.structs[].fields | array | ✓ |  |

### Example

```json
{
  "bytecode": "0xa11ceb0b0500000007010006030612041804051c4807646108c501200ce501980100000001000200030001010002040401010001040b010100010302030f0c050a050a030a010a040a020a010a0a020a040a020a010a040a020a010002030301090002060c0501050103010101040102010a020f060c050503010402010a02040201040201196f7261636c655f736176655f726573756c745f616374696f6e1d61676772656761746f725f736176655f726573756c745f616374696f6e176f7261636c655f6865617274626561745f616374696f6e0372756e0c72756e5f696e7465726e616c07d7e436f0b2aafde60774efb26ccc432cf881b677aca7faaf2a01879bd19fb800010400024a0e000a0138000e0241050c100600000000000000000c0f0a0f0a10230449050d0e000a010e020a0f4205140e030a0f4206140e040a0f4207140e050a0f4208140e060a0f4209140e070a0f4207140e080a0f420a140e090a0f4208140e0a0a0f4209140e0b0a0f4207140e0c0a0f4208140e0d0a0f4209140e0e0a0f42071438010b0f060100000000000000160c0f05080200",
  "abi": {
    "address": "0x7d7e436f0b2aafde60774efb26ccc432cf881b677aca7faaf2a01879bd19fb8",
    "name": "oracle_save_result_action",
    "friends": [],
    "exposed_functions": [
      {
        "name": "run",
        "visibility": "public",
        "is_entry": true,
        "is_view": false,
        "generic_type_params": [
          {
            "constraints": []
          }
        ],
        "params": [
          "signer",
          "address",
          "vector<address>",
          "vector<u64>",
          "vector<bool>",
          "vector<u128>",
          "vector<u8>",
          "vector<bool>",
          "vector<vector<u8>>",
          "vector<u128>",
          "vector<u8>",
          "vector<bool>",
          "vector<u128>",
          "vector<u8>",
          "vector<bool>"
        ],
        "return": []
      }
    ],
    "structs": []
  }
}
```
