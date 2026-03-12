# kaia_call

**`POST /`**

Executes a contract's read-only method without creating or submitting a transaction. Primarily used to read the current state of a specific smart contract. State changes from the call do not occur.
					

> 🚧 Is the response taking a long time? Response time may vary based on function complexity!
>
> When executing a smart contract function at a specific state, response time may increase depending on the function's complexity.
> If you want faster responses, optimize the function being executed and minimize the function's execution result.

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
| id | integer | ✓ |  |
| jsonrpc | string | ✓ |  |
| method | string | ✓ |  |
| params | array | ✓ | Enter the following parameters as an array with the appropriate types. 1. `call object` - Enter the contract address to call and the function data. 2. `block identifier` - Block number, block hash, or block tag for the block to query. 3. `state override set` - State override set for the transaction (optional). |
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
  "method": "kaia_call",
  "params": [
    {
      "to": "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48",
      "data": "0x70a08231000000000000000000000000d8dA6BF26964aF9D7eEd9e03E53415D37aA96045"
    },
    "latest"
  ]
}
```
