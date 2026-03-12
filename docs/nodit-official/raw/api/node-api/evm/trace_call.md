# trace_call

**`POST /`**

Simulates a specified transaction call and returns the result. This method does not actually send a transaction to the blockchain and provides detailed execution information including gas consumption, execution results, and logs. This is useful when you want to preview how a specific function call will be executed.

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
| id | integer | ✓ |  |
| jsonrpc | string | ✓ |  |
| method | string | ✓ |  |
| params | array | ✓ | Enter the following parameters as an array with the appropriate types.  1. `call object` is an object containing the following fields. 2. `trace type`: Select one or more trace types to use and enter as an array. You can use "vmTrace", "trace", or "stateDiff" values. See the API documentation for details on each option. 3. `block number or block tag`: Enter the block number in hexadecimal string format. You can also enter block tags such as "earliest", "latest", or "pending". |
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
  "method": "trace_call",
  "params": [
    {
      "from": "0xc90d3Ac75D1D36dF0b0a229E73D8409FB7F3c4ab",
      "to": "0xd3CdA913deB6f67967B99D67aCDFa1712C293601",
      "value": "0x186a0"
    },
    [
      "trace"
    ],
    "latest"
  ]
}
```

## Response

| Name | Type | Required | Description |
|------|------|----------|-------------|
| action | object |  | Information about the traced action |
| action.from | string |  | Caller address |
| action.callType | string |  | Call type (call, delegatecall, staticcall, etc.) |
| action.gas | string |  | Gas limit set for the call (hex) |
| action.input | string |  | Call input data (hex) |
| action.to | string |  | Callee address |
| action.value | string |  | Amount of ETH transferred (hex, in wei) |
| blockHash | string |  | Hash of the block containing the transaction |
| blockNumber | integer |  | Block number containing the transaction |
| result | object |  | Call result |
| result.gasUsed | string |  | Actual gas used (hex) |
| result.output | string |  | Call return data (hex) |
| subtraces | integer |  | Number of sub-traces |
| traceAddress | array |  | Index array representing the trace path |
| transactionHash | string |  | Hash of the traced transaction |
| transactionPosition | integer |  | Position of the transaction within the block |
| type | string |  | Trace type (call, create, reward, etc.) |

### Example

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "output": "0x",
    "stateDiff": null,
    "trace": [
      {
        "action": {
          "from": "0xc90d3ac75d1d36df0b0a229e73d8409fb7f3c4ab",
          "callType": "call",
          "gas": "0x2fa9e78",
          "input": "0x",
          "to": "0xd3cda913deb6f67967b99d67acdfa1712c293601",
          "value": "0x186a0"
        },
        "result": {
          "gasUsed": "0x0",
          "output": "0x"
        },
        "subtraces": 0,
        "traceAddress": [],
        "type": "call"
      }
    ],
    "vmTrace": null
  }
}
```
