# debug_traceTransaction

**`POST /`**

A debugging method that replays an already-processed transaction at the node level and allows you to inspect detailed information for each step of the transaction execution. It returns various information such as the call stack, gas usage, state changes, and log events. This allows you to trace and debug contract function call flows, variable changes, and event emissions.

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
| id | integer | ✓ |  |
| jsonrpc | string | ✓ |  |
| method | string | ✓ |  |
| params | array | ✓ | Enter the following parameters as an array with the appropriate types.  1. `transaction hash`: Enter the transaction hash to query as a string. 2. `trace option`: Object for trace option configuration. |

### Example

```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "method": "debug_traceTransaction",
  "params": [
    "0xda148d856aef6d77d0b76c90ef1091ffe77afe9ee9b1c6cc23f28f042f198bd8",
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
    "from": "0xb64a30399f7f6b0c154c2e7af0a3ec7b0a5b131a",
    "gas": "0x565f",
    "gasUsed": "0x565f",
    "to": "0x94750381be1aba0504c666ee1db118f68f0780d4",
    "input": "0x",
    "value": "0x5d81a504883156",
    "type": "CALL"
  }
}
```
