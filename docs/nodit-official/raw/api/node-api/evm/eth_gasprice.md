# eth_gasPrice

**`POST /`**

Returns the estimated gas price of the network the node is connected to in wei.

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
| id | integer | ✓ |  |
| jsonrpc | string | ✓ |  |
| method | string | ✓ |  |

### Example

```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "method": "eth_gasPrice"
}
```

## Response

### Example

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": "0x5a44ff4e3"
}
```
