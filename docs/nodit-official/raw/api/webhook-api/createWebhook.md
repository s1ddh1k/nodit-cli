# Create Webhook

**`POST /{chain}/{network}/webhooks`**

API for creating a Webhook. Create a Webhook by providing subscription details and Webhook URL. Once created, events will be sent to the Webhook URL. On creation, the API returns the Webhook's Subscription ID, which you can use to query, update, and delete the Webhook.

## Parameters

| Name | In | Type | Required | Description |
|------|----|------|----------|-------------|
| chain | path | string | ✓ | Parameter to specify the target chain for the query. |
| network | path | string | ✓ | Parameter to specify the network of the target chain. Supported networks may vary depending on the chain. |

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
| eventType | object | ✓ |  |
| description | string |  | Parameter that specifies the description of the event. |
| notification | object | ✓ | Parameter that specifies the information required to receive notifications when events occur. |
| notification.webhookUrl | string |  | Parameter that specifies the webhook URL to receive notifications when subscribed events occur. Must be a publicly accessible URL. |
| isInstant | boolean |  | Parameter that specifies whether to enable the Instant Webhook option. Defaults to false when not specified. - true: Enables the Instant Webhook option. You can receive Webhook messages immediately when the event is detected, regardless of block confirmation status. - false: Disables the Instant Webhook option. Messages are sent only after the block containing the event transaction has been confirmed. |
| condition | object | ✓ |  |

### Example

```json
{
  "eventType": "0x1::coin::CoinDeposit",
  "description": "Your webhook description",
  "notification": {
    "webhookUrl": "https://your-webhook-url.com"
  },
  "isInstant": false,
  "condition": {
    "type": "BLOCK_PERIOD",
    "block": 1
  }
}
```
