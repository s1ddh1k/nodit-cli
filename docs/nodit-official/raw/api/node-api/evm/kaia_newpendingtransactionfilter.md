# kaia_newPendingTransactionFilter

**`POST /`**

Creates a filter to receive notifications when new pending transactions occur and returns the filter ID. The filter ID is used with kaia_getFilterChanges to query filter results.

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
  "method": "kaia_newPendingTransactionFilter"
}
```
