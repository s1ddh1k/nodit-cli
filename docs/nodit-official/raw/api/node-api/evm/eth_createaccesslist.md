# eth_createAccessList

**`POST /`**

Generates an Access List for a transaction according to EIP-2930. The Access List contains only the minimum information required for execution, reducing network load and gas costs.

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
| id | integer | ✓ |  |
| jsonrpc | string | ✓ |  |
| method | string | ✓ |  |
| params | array | ✓ | Pass the following parameters as an array with the appropriate types. 1. `call object` - Contract address and call data. 2. `block identifier` - Block number, block hash, or block tag. |
| params[].from | string |  | The transaction sender (from) address as a string. |
| params[].to | string |  | The transaction recipient (to) address as a string. |
| params[].gas | string |  | The gas limit for the transaction as a hex string. For smart contract calls, use 0x0 since no gas is consumed. |
| params[].gasPrice | string |  | The gas price per unit as a hex string. |
| params[].value | string |  | The value (amount) for the transaction. |
| params[].data | string |  | The method signature hash of the call. See ABI for reference. |

### Example

```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "method": "eth_createAccessList",
  "params": [
    {
      "from": null,
      "to": "0xdAC17F958D2ee523a2206206994597C13D831ec7",
      "data": "0x70a0823100000000000000000000000047ac0Fb4F2D84898e4D9E7b4DaB3C24507a6D503"
    },
    "latest"
  ]
}
```

## Response

| Name | Type | Required | Description |
|------|------|----------|-------------|
| accessList | array |  | The generated access list |
| accessList[].address | string |  | Contract address to be accessed |
| accessList[].storageKeys | array |  | Storage keys to be accessed |
| gasUsed | string |  | Estimated gas used when applying the access list (hex) |

### Example

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "accessList": [
      {
        "address": "0xdac17f958d2ee523a2206206994597c13d831ec7",
        "storageKeys": [
          "0x000000000000000000000000000000000000000000000000000000000000000a",
          "0xb6a3055d770d1f33e0d6a6d5bdda21882d967a5d6d17933aa7c73780c5899db9"
        ]
      }
    ],
    "gasUsed": "0x6ff7"
  }
}
```
