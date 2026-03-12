# eth_call

**`POST /`**

Execute a read-only contract method without creating or broadcasting a transaction. Primarily used to read the current state of a smart contract. No state changes occur as a result of the call.
					

> 🚧 Response taking too long? Response time may vary based on function complexity!
>
> When executing a smart contract function at a specific state, response time can increase depending on the complexity of the function.
> For faster responses, optimize the function being executed and minimize the execution result.

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
| id | integer | ✓ |  |
| jsonrpc | string | ✓ |  |
| method | string | ✓ |  |
| params | array | ✓ | Pass the following parameters as an array with the appropriate types. 1. `call object` - Contract address and call data. 2. `block identifier` - Block number, block hash, or block tag. 3. `state override set` - Optional state overrides for the call. |
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
  "method": "eth_call",
  "params": [
    {
      "to": "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48",
      "data": "0x70a08231000000000000000000000000d8dA6BF26964aF9D7eEd9e03E53415D37aA96045"
    },
    "latest"
  ]
}
```

## Response

### Example

```json
{
  "jsonrpc": "2.0",
  "id": "1",
  "result": "0x0000000000000000000000000000000000000000000000000000028ec55d5a09"
}
```
