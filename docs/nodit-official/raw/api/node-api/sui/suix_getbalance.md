# suix_getBalance

**`POST /`**

Return the total coin balance for one coin type, owned by the address owner.

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
| id | integer | ✓ |  |
| jsonrpc | string | ✓ |  |
| method | string | ✓ |  |
| params | array | ✓ |  |

### Example

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "suix_getBalance",
  "params": [
    "0xa6f5a7953a75dc632e696cabe60560522a017bf2fb0bd930d1ec22c06f1ee4e4",
    "0x168da5bf1f48dafc111b0a488fa454aca95e0b5e::usdc::USDC"
  ]
}
```

## Response

| Name | Type | Required | Description |
|------|------|----------|-------------|
| jsonrpc | string |  |  |
| result | object |  |  |
| result.coinObjectCount | integer | ✓ |  |
| result.coinType | string | ✓ |  |
| result.lockedBalance | object | ✓ |  |
| result.totalBalance | string | ✓ |  |

### Example

```json
{
  "jsonrpc": "2.0",
  "result": {
    "coinType": "0x168da5bf1f48dafc111b0a488fa454aca95e0b5e::usdc::USDC",
    "coinObjectCount": 15,
    "totalBalance": "15",
    "lockedBalance": {}
  },
  "id": 1
}
```
