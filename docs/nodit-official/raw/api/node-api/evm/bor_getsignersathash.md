# bor_getSignersAtHash

**`POST /`**

Returns the list of validators who signed a specific block hash. This method is used to query the addresses of signers (validators) for a given block hash and allows you to identify the nodes that participated in block validation at a specific point in the network.

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
| id | integer | ✓ |  |
| jsonrpc | string | ✓ |  |
| method | string | ✓ |  |
| params | array | ✓ | Enter the following parameters as an array with the appropriate types.  1. `block hash`: Enter the block hash to query in 64-digit hexadecimal string format. |

### Example

```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "method": "bor_getSignersAtHash",
  "params": [
    "0xa70c0bff4de8a59f521920deb8b6f3a4885845f2f418409f5fc8daade7717505"
  ]
}
```

## Response

### Example

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": [
    "0x00856730088a5c3191bd26eb482e45229555ce57",
    "0x02f70172f7f490653665c9bfac0666147c8af1f5",
    "0x048cfedf907c4c9ddd11ff882380906e78e84bbe",
    "0x06abe41e26db44ad94fe61db2ce56023347bcf0c",
    "0x127685d6dd6683085da4b6a041efcef1681e5c9c",
    "0x1b0840519a581f3779d0a10b77593d6d3894a76a",
    "0x1efecb61a2f80aa34d3b9218b564a64d05946290",
    "0x2c74ca71679cf1299936d6104d825c965448907b",
    "0x5fe93ddf4490a02257bef079f2498650c97c44de",
    "0x60e274b09f701107a4b3226fcc1376ebda3cdd92",
    "0x67b94473d81d0cd00849d563c94d0432ac988b49",
    "0x69f5c4d08f6bc8cd29fe5f004d46fb566270868d",
    "0x794e44d1334a56fea7f4df12633b88820d0c5888",
    "0x7c7379531b2aee82e4ca06d4175d13b9cbeafd49",
    "0x959c65b72147faf3450d8b50a0de57e72ffc5e0d",
    "0x9ead03f7136fc6b4bdb0780b00a1c14ae5a8b6d0",
    "0xa8b52f02108aa5f4b675bdcc973760022d7c6020",
    "0xaa6ac02fddaaf6f120f5bb98ce30809d19cd5d1b",
    "0xb2dd091ea6e591d62f565d7a18ce2a7640add227",
    "0xb95d435df3f8b2a8d8b9c2b7c8766c9ae6ed8cc9",
    "0xb9ede6f94d192073d8eaf85f8db677133d483249",
    "0xc35649ae99be820c7b200a0add09b96d7032d232",
    "0xc6869257205e20c2a43cb31345db534aecb49f6e",
    "0xe63727cb2b3a8d6e3a2d1df4990f441938b67a34",
    "0xec20607aa654d823dd01beb8780a44863c57ed07",
    "0xeedba2484aaf940f37cd3cd21a5d7c4a7dafbfc0",
    "0xef46d5fe753c988606e6f703260d816af53b03eb"
  ]
}
```
