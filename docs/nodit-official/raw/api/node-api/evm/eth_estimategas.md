# eth_estimateGas

**`POST /`**

Estimates the gas required to execute a transaction. Estimates gas only; does not broadcast the transaction.

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
| id | integer | ✓ |  |
| jsonrpc | string | ✓ |  |
| method | string | ✓ |  |
| params | array | ✓ | Pass the following parameters as an array with the appropriate types. 1. `call object` - Contract address and call data. |
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
  "method": "eth_estimateGas",
  "params": [
    {
      "from": "0xc90d3Ac75D1D36dF0b0a229E73D8409FB7F3c4ab",
      "to": "0xd3CdA913deB6f67967B99D67aCDFa1712C293601",
      "value": "0x186a0"
    }
  ]
}
```

## Response

### Example

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": "0x5208"
}
```
