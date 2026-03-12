# suix_getCoins

**`POST /`**

Return all Coin<`coin_type`> objects owned by an address.

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
  "method": "suix_getCoins",
  "params": [
    "0x6d907beaa3a49db57bdfdb3557e6d405cbf01c293a53e01457d65e92b5d8dd68",
    "0x2::sui::SUI",
    "0xee6b5173afedb35330f60397c2cbb48196ba41921246c304be7b490cee0904eb",
    3
  ]
}
```

## Response

| Name | Type | Required | Description |
|------|------|----------|-------------|
| id | integer |  |  |
| jsonrpc | string |  |  |
| result | object |  | `next_cursor` points to the last item in the page; Reading with `next_cursor` will start from the next item after `next_cursor` if `next_cursor` is `Some`, otherwise it will start from the first item. |
| result.data | array | ✓ |  |
| result.data[].balance | string | ✓ |  |
| result.data[].coinObjectId | string | ✓ | Hex string encoding. |
| result.data[].coinType | string | ✓ |  |
| result.data[].digest | string | ✓ |  |
| result.data[].previousTransaction | string | ✓ |  |
| result.data[].version | integer | ✓ |  |
| result.hasNextPage | boolean | ✓ |  |
| result.nextCursor | string,null |  |  |

### Example

```json
{
  "jsonrpc": "2.0",
  "result": {
    "data": [
      {
        "coinType": "0x2::sui::SUI",
        "coinObjectId": "0xd62ca040aba24f862a763851c54908cd2a0ee7d709c11b93d4a2083747b76856",
        "version": "103626",
        "digest": "C9fdokK19BpDCgUgWsJv3cfd4LDyk7WGYBeGhFHbEL2Z",
        "balance": "200000000",
        "previousTransaction": "tw5DzJTfdxTn4f3rekFrhN7dQTUezBgsEhycDobTBLb"
      },
      {
        "coinType": "0x2::sui::SUI",
        "coinObjectId": "0xf44d295a385dc3544d211411b865e8bc4f01f49186970c7cf61e1cc829cc0be7",
        "version": "103626",
        "digest": "5qZkmtN5J5uGHURtiy9BtBhnXATPR2Wa6BJBDLrMzCaf",
        "balance": "200000000",
        "previousTransaction": "AfgFe7ZfjJ5dWV6VAy2LbtvBFhcABkvdvwEjLrRcFqtr"
      },
      {
        "coinType": "0x2::sui::SUI",
        "coinObjectId": "0x42ef9314ccc792dd4401a88e69c66b4c5e43f21e9e57f4abe3c702649d3a7dd0",
        "version": "103626",
        "digest": "FLE2nB2Wio3oUyTx6HyzkrMsWiZxDg9Kk8s7ivvuoBbD",
        "balance": "200000000",
        "previousTransaction": "9er6jxigfuQEKsn9gtPV2oW1zGQRcFtKNijHVe88GUJD"
      }
    ],
    "nextCursor": "abcd",
    "hasNextPage": true
  },
  "id": 1
}
```
