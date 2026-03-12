# trace_filter

**`POST /`**

Filters and traces the execution of transactions matching the specified criteria. Users can set various conditions such as block number range, addresses used, and tokens used, and obtain trace information for transactions that match these criteria.


> 🚧 Experiencing slow response times? Response time may increase depending on the block range!
>
> Specifying a wide block range or including blocks with many events may cause delayed responses or timeouts.
>
> For faster responses, consider the following recommendations:
>
> - Set the fromBlock ~ toBlock range to 1000 blocks or less.
> - When using latest as blockTag, the internal block number may not be known, which can increase response time.
  In that case, query the latest block number with eth_blockNumber and specify it explicitly.

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
| id | integer | ✓ |  |
| jsonrpc | string | ✓ |  |
| method | string | ✓ |  |
| params | array | ✓ | Enter the following parameters as an array with the appropriate types.  1. `trace object`: Enter trace filter options as an Object. |
| params[].fromBlock | string |  | Block number indicates the sequential order of a block. Block numbers are expressed as hexadecimal strings. |
| params[].toBlock | string |  | Block number indicates the sequential order of a block. Block numbers are expressed as hexadecimal strings. |
| params[].fromAddress | array |  | Enter the from addresses to filter as an array. |
| params[].toAddress | array |  | Enter the to addresses to filter as an array. |
| params[].after | string |  | The offset size for traces. |
| params[].count | integer |  | The number of traces to display in the trace batch. |

### Example

```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "method": "trace_filter",
  "params": [
    {
      "fromBlock": "0x1253B3F",
      "toBlock": "0x1253B5F",
      "fromAddress": [
        "0xB287eaC48aB21c5FB1d3723830d60b4c797555B0"
      ]
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
  "result": [
    {
      "action": {
        "callType": "call",
        "from": "0xb287eac48ab21c5fb1d3723830d60b4c797555b0",
        "gas": "0x0",
        "input": "0x",
        "to": "0x6a78a5f8a7839a234dee4de29c26c5f6ebad5caa",
        "value": "0x521153de4fc000"
      },
      "blockHash": "0x7a1e8b72cbb07cae9c4b1d4fa32593ff2446c8a0a8e65ee9dadd6fb2f04862eb",
      "blockNumber": 19217231,
      "result": {
        "gasUsed": "0x0",
        "output": "0x"
      },
      "subtraces": 0,
      "traceAddress": [],
      "transactionHash": "0x3b60b4393fbc01b2ec6a0a24c0c39b42ee49c0dded76f1eb5af623004d2ddc6e",
      "transactionPosition": 62,
      "type": "call"
    }
  ]
}
```
