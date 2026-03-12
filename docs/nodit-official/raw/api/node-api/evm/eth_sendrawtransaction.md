# eth_sendRawTransaction

**`POST /`**

Broadcasts a signed transaction to the network. On success, returns the transaction hash. Transaction signing must be done client-side with the private key. The node only validates the transaction.

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
| id | integer | ✓ |  |
| jsonrpc | string | ✓ |  |
| method | string | ✓ |  |
| params | array | ✓ | Pass the following parameters as an array with the appropriate types. 1. `signed transaction`: The RLP-encoded signed transaction as a hex string. |

### Example

```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "method": "eth_sendRawTransaction",
  "params": [
    "0x02f8688080808080943f39cfbaff46cb736a603269d14a7e9adf5158b488016345785d8a000080c001a005599173ee4483fa38044e8d7bf592b58a9ab598f7d4a510702d193c60af15a0a00f2d39e8202dc9d7d66a51fc67fcf1d893b20e080c6acf2b25610f5e926cfa21"
  ]
}
```

## Response

### Example

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": "0x0100000000000000ee32c7a8d24aac1f"
}
```
