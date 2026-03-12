# kaia_createAccessList

**`POST /`**

Generates an Access List for a transaction according to EIP-2930. The Access List contains only the minimal information required for transaction execution, reducing network load and gas costs.

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
| id | integer | ✓ |  |
| jsonrpc | string | ✓ |  |
| method | string | ✓ |  |
| params | array | ✓ | Enter the following parameters as an array with the appropriate types. 1. `call object` - Enter the contract address to call and the function data. 2. `block identifier` - Block number, block hash, or block tag for the block to query. |
| params[].from | string |  | Enter the transaction's from address as a string. |
| params[].to | string |  | Enter the transaction's to address as a string. |
| params[].gas | string |  | Enter the gas limit for the transaction as a hexadecimal string. For smart contract calls, you can enter 0x0 since no gas is consumed. |
| params[].gasPrice | string |  | Enter the gas price per unit as a hexadecimal string. |
| params[].value | string |  | The value field of the transaction. |
| params[].data | string |  | The method signature hash of the transaction to execute. Can be referenced from the ABI. |

### Example

```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "method": "kaia_createAccessList",
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
