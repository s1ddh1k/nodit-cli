# suix_subscribeEvent

**`POST /`**

Subscribe to a stream of Sui event

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
| id | integer | ✓ |  |
| jsonrpc | string | ✓ |  |
| method | string | ✓ |  |
| params | array | ✓ |  |
| params[].All | array | ✓ |  |

## Response

| Name | Type | Required | Description |
|------|------|----------|-------------|
| jsonrpc | string |  |  |
| id | integer |  |  |
| result | object |  |  |
| result.bcs | string | ✓ | Base64 encoding |
| result.bcsEncoding | string | ✓ |  |
