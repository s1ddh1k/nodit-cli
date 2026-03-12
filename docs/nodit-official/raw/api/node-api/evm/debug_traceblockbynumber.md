# debug_traceBlockByNumber

**`POST /`**

A debugging tool that allows you to trace chain state changes caused by transactions and the history of actual calls for a specific block, by configuring a tracer and using the information it provides.


> 🚧 Getting a 429 error? Check your subscription plan!
> 429 errors can occur when you exceed the Throughput limit of your subscription plan.
> For example, the Starter plan has a limit of 300 CU per second, so calling an API that consumes 350 CU while on the Starter plan may result in a 429 error.
> Check the CU consumption of the APIs you use on the Compute Unit Costs page, and consider upgrading to a higher plan if you need more Throughput!
> 👉 [Go to Compute Unit Costs page](/guides/usage-measuringcu)

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
| id | integer | ✓ |  |
| jsonrpc | string | ✓ |  |
| method | string | ✓ |  |
| params | array | ✓ | Enter the following parameters as an array with the appropriate types.  1. `block number or block tag`: Enter the block number in hexadecimal string format. You can also enter block tags such as "earliest", "latest", or "pending".	 2. `traceOption`: Object for trace option configuration. |

### Example

```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "method": "debug_traceBlockByNumber",
  "params": [
    "latest",
    {
      "tracer": "callTracer",
      "tracerConfig": {
        "onlyTopCall": true
      }
    }
  ]
}
```

## Response

### Example

```json
{
  "jsonrpc": "2.0",
  "id": "1",
  "result": [
    {
      "result": {
        "from": "0x39fa8c5f2793459d6622857e7d9fbb4bd91766d3",
        "gas": "0x1a6d4",
        "gasUsed": "0x723c",
        "to": "0xc083e9947cf02b8ffc7d3090ae9aea72df98fd47",
        "input": "0x",
        "output": "0x0000000000000000000000000000000000000000000000000000000000000000",
        "calls": [
          {
            "from": "0xc083e9947cf02b8ffc7d3090ae9aea72df98fd47",
            "gas": "0x18c56",
            "gasUsed": "0x5a4",
            "to": "0x273930d21e01ee25e4c219b63259d214872220a2",
            "input": "0x",
            "value": "0x56bc75e2d63100000",
            "type": "CALLCODE"
          }
        ],
        "value": "0x56bc75e2d63100000",
        "type": "CALL"
      }
    },
    {
      "result": {
        "from": "0x32be343b94f860124dc4fee278fdcbd38c102d88",
        "gas": "0x7148",
        "gasUsed": "0x5208",
        "to": "0xdf190dc7190dfba737d7777a163445b7fff16133",
        "input": "0x",
        "value": "0x6113a84987be800",
        "type": "CALL"
      }
    }
  ]
}
```
