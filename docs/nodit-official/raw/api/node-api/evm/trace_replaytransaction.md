# trace_replayTransaction

**`POST /`**

Re-executes a specified transaction and traces all events that occur during the process. Since the transaction execution is performed locally, it is not reflected on the actual blockchain. This method is used when you want to reproduce and analyze the execution of a past transaction.

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
| id | integer | ✓ |  |
| jsonrpc | string | ✓ |  |
| method | string | ✓ |  |
| params | array | ✓ | Enter the following parameters as an array with the appropriate types.  1. `transaction hash`: Enter the transaction hash to query as a string. 2. `trace type`: Select one or more trace types to use and enter as an array. You can use "vmTrace", "trace", or "stateDiff" values. See the API documentation for details on each option.  |

### Example

```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "method": "trace_replayTransaction",
  "params": [
    "0x17104ac9d3312d8c136b7f44d4b8b47852618065ebfa534bd2d3b5ef218ca1f3",
    [
      "trace"
    ]
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
          "from": "0xab40228a9d8659103de421fd77dbe192cf0e948f",
          "callType": "call",
          "gas": "0x0",
          "input": "0x008257ab785e1e1ee0e780fc7c2d04f99700000000017578dadae1cff0c367815b575353fa9d9b7c012b2cd7f4bb67ad927af0f169fa83df0ff8d6789cd7f823b9b639a0e3de024feb8fd5d991bb2fa76cf3bf9865b7e2b1a66bd9d6e4ec2f45c60f56c8245a077e6c493dcd3def00c4c04bb7a33efe52b4edf62de8dfae5fc7c07848c820c0edd4e5f0928551ef5a371e9724dac0055003779e14b1c9e57c6a60f0abf1834a67c2b18c4507a59e17ff8bd53edab0b8238349b539a0e3fe82f4e2147ef63ddaab3f15794e66bf99bef5ceeca5f5a91aeb4f852e573b536ea99d01327011d440991fa6850ef55ba5e77feee26a2ffebf6fdde97cff77d3d2437556bf774af7aa5b44b4814ba0069e0f93f9cec9d11a6750f4ec8decc3558efe31820b7982d6ef1499c2e5b938ab969b680397410ddc56e630e58d97fcc990af6b0d25d6b85a2c7b7443dd3cb372e93a2919d11bed96b2441bb8026ae06f9ed3afe71d64e148575e609fb07bf3b4486e4513b6ea26a384b29def99cd784e136de0aa0380000000ffffbfa413b601",
          "to": "0xff00000000000000000000000000000000001432",
          "value": "0x0"
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
