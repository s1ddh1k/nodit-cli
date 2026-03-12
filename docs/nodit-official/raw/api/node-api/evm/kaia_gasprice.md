# kaia_gasPrice

**`POST /`**

Returns the expected gas price in wei for the network the current node is connected to.

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
  "method": "kaia_gasPrice"
}
```
