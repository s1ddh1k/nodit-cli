# suix_getTotalSupply

**`POST /`**

Return total supply for a coin

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
  "method": "suix_getTotalSupply",
  "params": [
    "0xe5c651321915b06c81838c2e370109b554a448a78d3a56220f798398dde66eab::acoin::ACOIN"
  ]
}
```

## Response

| Name | Type | Required | Description |
|------|------|----------|-------------|
| id | integer |  |  |
| jsonrpc | string |  |  |
| result | object |  |  |
| result.value | string | ✓ |  |

### Example

```json
{
  "jsonrpc": "2.0",
  "result": {
    "value": "12023692"
  },
  "id": 1
}
```
