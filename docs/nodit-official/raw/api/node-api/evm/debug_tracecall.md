# debug_traceCall

**`POST /`**

Runs eth_call in debugging mode and provides trace functionality. You can trace all stack changes that occur when executing a specific call based on the current block's state.

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
| id | integer | ✓ |  |
| jsonrpc | string | ✓ |  |
| method | string | ✓ |  |
| params | array | ✓ | Enter the following parameters as an array with the appropriate types.  1. `call object` - Enter the contract address to call and the data of the function to execute. 2. `block identifier` - Block identifier for the target block: block number, block hash, or block tag. 3. `trace option`: Object for trace option configuration. |
| params[].from | string |  | Enter the transaction's from address as a string. |
| params[].to | string |  | Enter the transaction's to address as a string. |
| params[].gas | string |  | Enter the gas required to process the transaction as a hex-format string. For smart contract calls, no gas is consumed, so you can enter 0x0. |
| params[].gasPrice | string |  | Enter the desired cost per gas as a hex-format string. |
| params[].value | string |  | The value of the transaction. |
| params[].data | string |  | The method signature hash of the transaction to execute. Can be referenced from the ABI. |

### Example

```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "method": "debug_traceCall",
  "params": [
    {
      "from": "0xc90d3Ac75D1D36dF0b0a229E73D8409FB7F3c4ab",
      "to": "0xd3CdA913deB6f67967B99D67aCDFa1712C293601",
      "value": "0x186a0"
    },
    "finalized",
    {
      "tracer": "callTracer"
    }
  ]
}
```

## Response

| Name | Type | Required | Description |
|------|------|----------|-------------|
| from | string |  | Caller address |
| gas | string |  | Gas limit set for the call (hex) |
| gasUsed | string |  | Actual gas used (hex) |
| to | string |  | Callee address |
| input | string |  | Call input data (hex) |
| output | string |  | Call return data (hex) |
| value | string |  | Amount of ETH transferred (hex, in wei) |
| type | string |  | Call type (CALL, CREATE, STATICCALL, etc.) |
| calls | array |  | List of internal sub-calls |
| error | string |  | Error message if execution failed |
| revertReason | string |  | Revert reason if the transaction was reverted |

### Example

```json
{
  "jsonrpc": "2.0",
  "id": "1",
  "result": {
    "from": "0x0000000000000000000000000000000000000000",
    "gas": "0x2faf080",
    "gasUsed": "0x5d83",
    "to": "0xa3e0dfbf8dbd86e039f7cdb37682a776d66dae4b",
    "input": "0x70a08231000000000000000000000000a03167de1a56160e4647d77d81e9139af55b63d4",
    "output": "0x000000000000000000000000000000000000000000000000000003c18cd105e2",
    "value": "0x0",
    "type": "CALL"
  }
}
```
