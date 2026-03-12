# net_version

**`POST /`**

Returns the network ID of the connected Ethereum network. The network ID is used to distinguish between different Ethereum networks (e.g., mainnet, testnet). For example, Ethereum mainnet has network ID 1, Ropsten testnet has 3, and Rinkeby testnet has 4. This method is primarily used to identify the network and verify that an application is connected to the correct network.

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
  "method": "net_version"
}
```

## Response

### Example

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": "1"
}
```
