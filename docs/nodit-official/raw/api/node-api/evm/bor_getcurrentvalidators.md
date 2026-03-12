# bor_getCurrentValidators

**`POST /`**

Returns the list of currently active validators on the network. This method is used to identify the current validators of the network and provides important information for understanding the network's governance and security status. The validator list may change according to the network's consensus protocol and governance mechanism, and this method allows you to query the currently active validators.

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
  "method": "bor_getCurrentValidators"
}
```

## Response

### Example

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": [
    {
      "ID": 0,
      "signer": "0x00856730088a5c3191bd26eb482e45229555ce57",
      "power": 1,
      "accum": -26
    }
  ]
}
```
