# Update Webhook

**`PATCH /{chain}/{network}/webhooks/{subscriptionId}`**

You can modify the subscription condition of a Webhook, or activate or deactivate a Webhook.

## Parameters

| Name | In | Type | Required | Description |
|------|----|------|----------|-------------|
| chain | path | string | ✓ | Parameter to specify the target chain for the query. |
| network | path | string | ✓ | Parameter to specify the network of the target chain. Supported networks may vary depending on the chain. |
| subscriptionId | path | string | ✓ | Parameter to specify the subscriptionId assigned to the Webhook you want to query. You can use the subscriptionId returned when creating a Webhook to query, modify, and delete Webhook information. |

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
| notification | object |  | Parameter specifying the information required to receive notifications when an event occurs. |
| notification.webhookUrl | string |  | Parameter specifying the webhook URL to receive notifications when the subscribed event occurs. A publicly accessible URL must be specified. |
| description | string |  | Parameter specifying the description of the event. |
| isActive | boolean |  | Parameter specifying whether the Webhook is active. When set to true, the Webhook is activated; when set to false, it is deactivated. A deactivated Webhook cannot receive notifications but is not deleted. |
| condition | object |  |  |

### Example

```json
{
  "notification": {
    "webhookUrl": "https://your-webhook-url.com"
  },
  "description": "Your webhook description",
  "isActive": false,
  "condition": {
    "type": "BLOCK_PERIOD",
    "block": 2
  }
}
```

## Response

| Name | Type | Required | Description |
|------|------|----------|-------------|
| result | boolean |  | Field indicating the result of the call. Returns true on success and false on failure. |

### Example

```json
{
  "result": true
}
```
