# sui_getLatestCheckpointSequenceNumber

**`POST /`**

Return the sequence number of the latest checkpoint that has been executed

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
  "method": "sui_getLatestCheckpointSequenceNumber",
  "params": []
}
```

## Response

| Name | Type | Required | Description |
|------|------|----------|-------------|
| jsonrpc | string |  |  |
| id | integer |  |  |
| result | string |  |  |

### Example

```json
{
  "jsonrpc": "2.0",
  "result": "507021",
  "id": 1
}
```
