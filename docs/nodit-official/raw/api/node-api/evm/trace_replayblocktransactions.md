# trace_replayBlockTransactions

**`POST /`**

Traces the execution of all transactions included in a specified block. This method provides detailed trace information for all transactions executed in that block and is used to analyze the transaction processing at the block level.

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
| id | integer | ✓ |  |
| jsonrpc | string | ✓ |  |
| method | string | ✓ |  |
| params | array | ✓ | Enter the following parameters as an array with the appropriate types.  1. `block number or block tag`: Enter the block number in hexadecimal string format. You can also enter block tags such as "earliest", "latest", or "pending". |

### Example

```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "method": "trace_replayBlockTransactions",
  "params": [
    "latest",
    [
      "trace"
    ]
  ]
}
```

## Response

### Example

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": [
    {
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
      "vmTrace": null,
      "transactionHash": "0x63c4a9145bb1b89cbd43ec178b91ee94ca3508e247be025ba162dad625b78759"
    }
  ]
}
```
