# kaia_estimateGas

**`POST /`**

Estimates the gas required to execute a transaction. Estimates gas without actually submitting the transaction.

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
| id | integer | ✓ |  |
| jsonrpc | string | ✓ |  |
| method | string | ✓ |  |
| params | array | ✓ | Enter the following parameters as an array with the appropriate types. 1. `call object` - Enter the contract address to call and the function data. |
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
  "method": "kaia_estimateGas",
  "params": [
    {
      "from": "0xc90d3Ac75D1D36dF0b0a229E73D8409FB7F3c4ab",
      "to": "0xd3CdA913deB6f67967B99D67aCDFa1712C293601",
      "value": "0x186a0"
    }
  ]
}
```
