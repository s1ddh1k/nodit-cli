# Delete Webhook

**`DELETE /{chain}/{network}/webhooks/{subscriptionId}`**

API for deleting a Webhook. Deleting a Webhook cancels the subscription, and you will no longer receive events.

## Parameters

| Name | In | Type | Required | Description |
|------|----|------|----------|-------------|
| chain | path | string | ✓ | Parameter to specify the target chain for the query. |
| network | path | string | ✓ | Parameter to specify the network of the target chain. Supported networks may vary depending on the chain. |
| subscriptionId | path | string | ✓ | Parameter that specifies the subscriptionId assigned to the Webhook to query. Use the subscriptionId returned when creating the Webhook to query, update, or delete Webhook information. |

## Response

| Name | Type | Required | Description |
|------|------|----------|-------------|
| result | boolean |  | Field representing the call result. Returns true on success, false on failure. |

### Example

```json
{
  "result": true
}
```
