# suix_getAllBalances

**`POST /`**

Return the total coin balance for all coin type, owned by the address owner. test

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
  "method": "suix_getAllBalances",
  "params": [
    "0x94f1a597b4e8f709a396f7f6b1482bdcd65a673d111e49286c527fab7c2d0961"
  ]
}
```

## Response

| Name | Type | Required | Description |
|------|------|----------|-------------|
| id | integer |  |  |
| jsonrpc | string |  |  |
| result | array |  |  |
| result[].coinObjectCount | integer | ✓ |  |
| result[].coinType | string | ✓ |  |
| result[].lockedBalance | object | ✓ |  |
| result[].totalBalance | string | ✓ |  |

### Example

```json
{
  "jsonrpc": "2.0",
  "result": [
    {
      "coinType": "0x2::sui::SUI",
      "coinObjectCount": 15,
      "totalBalance": "3000000000",
      "lockedBalance": {}
    }
  ],
  "id": 1
}
```
