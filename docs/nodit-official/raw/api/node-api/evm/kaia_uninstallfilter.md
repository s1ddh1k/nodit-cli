# kaia_uninstallFilter

**`POST /`**

Removes a filter by filter ID. Enter the filter ID created via kaia_newFilter. Returns `false` if the filter has already been removed or does not exist.

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
| id | integer | ✓ |  |
| jsonrpc | string | ✓ |  |
| method | string | ✓ |  |
| params | array | ✓ | Enter the following parameters as an array with the appropriate types. 1. `filter ID`: Enter the previously created filter ID. |

### Example

```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "method": "kaia_uninstallFilter",
  "params": [
    "0xaf35d60b70eb3b54018456a0d365ea49"
  ]
}
```
