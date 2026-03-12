# suix_getCoinMetadata

**`POST /`**

Return metadata (e.g., symbol, decimals) for a coin. Note that if the coin's metadata was wrapped in the transaction that published its marker type, or the latest version of the metadata object is wrapped or deleted, it will not be found.

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
  "method": "suix_getCoinMetadata",
  "params": [
    "0x168da5bf1f48dafc111b0a488fa454aca95e0b5e::usdc::USDC"
  ]
}
```

## Response

| Name | Type | Required | Description |
|------|------|----------|-------------|
| id | integer |  |  |
| jsonrpc | string |  |  |
| result | object |  |  |
| result.decimals | integer | ✓ | Number of decimal places the coin uses. |
| result.description | string | ✓ | Description of the token |
| result.iconUrl | string,null |  | URL for the token logo |
| result.id | string |  | Hex string encoding. |
| result.name | string | ✓ | Name for the token |
| result.symbol | string | ✓ | Symbol for the token |

### Example

```json
{
  "jsonrpc": "2.0",
  "result": {
    "decimals": 9,
    "name": "Usdc",
    "symbol": "USDC",
    "description": "Stable coin.",
    "iconUrl": null,
    "id": "0x51ceab2edc89f74730e683ebee65578cb3bc9237ba6fca019438a9737cf156ae"
  },
  "id": 1
}
```
