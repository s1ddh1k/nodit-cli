# sui_verifyZkLoginSignature

**`POST /`**

Verify a zklogin signature for the given bytes, intent scope and author.

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
| id | integer | ✓ |  |
| jsonrpc | string | ✓ |  |
| method | string | ✓ |  |
| params | array | ✓ |  |

## Response

| Name | Type | Required | Description |
|------|------|----------|-------------|
| jsonrpc | string |  |  |
| id | integer |  |  |
| result | object |  |  |
| result.errors | array | ✓ | The errors field captures any verification error |
| result.success | boolean | ✓ | The boolean result of the verification. If true, errors should be empty. |
