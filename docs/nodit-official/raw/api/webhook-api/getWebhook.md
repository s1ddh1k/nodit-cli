# Get Webhook

**`GET /{chain}/{network}/webhooks`**

API for querying Webhook information by Subscription ID.

## Parameters

| Name | In | Type | Required | Description |
|------|----|------|----------|-------------|
| chain | path | string | ✓ | Parameter to specify the target chain for the query. |
| network | path | string | ✓ | Parameter to specify the network of the target chain. Supported networks may vary depending on the chain. |
| subscriptionId | query | string |  | Parameter that specifies the subscriptionId assigned to the Webhook to query. Use the subscriptionId returned when creating the Webhook to query, update, or delete Webhook information. Enter an empty value to query all Webhook information. |

## Response

| Name | Type | Required | Description |
|------|------|----------|-------------|
| total | number | ✓ | Total number of events matching the criteria. |
| page | number | ✓ | Current page number of the query. |
| rpp | number | ✓ | Number of events per page returned based on the request. |
| items | array | ✓ | Array containing event detail data. |
| items[].subscriptionId | string | ✓ | Field representing the subscriptionId assigned to the Webhook. subscriptionId is a unique value that identifies the Webhook and is used to query, update, or delete the Webhook. |
| items[].description | string | ✓ | Field representing the Webhook description. |
| items[].protocol | string | ✓ | Field representing the chain subscribed to via this Webhook. (e.g., ETHEREUM, POLYGON, OPTIMISM, ...) |
| items[].network | string | ✓ | Field representing the network subscribed to via this Webhook. (e.g., MAINNET, SEPOLIA, MUMBAI, ...) |
| items[].subscriptionType | string | ✓ | Field for identifying the subscription type. |
| items[].eventType | string | ✓ | Field representing the event type subscribed to by this Webhook. |
| items[].notification | object | ✓ | Field representing the information for receiving notifications via Webhook. |
| items[].notification.webhookUrl | string | ✓ | Field representing the URL for receiving Webhook notifications. |
| items[].isInstant | boolean |  | Field that specifies whether the Instant Webhook option is enabled. - true: When enabled, you can receive Webhook messages immediately when the monitored event is detected, regardless of block confirmation status. - false: When disabled, messages are sent only after the block containing the event transaction has been confirmed. |
| items[].isActive | boolean | ✓ | Field representing whether the Webhook is active. true when the Webhook is enabled, false when disabled. When disabled, you cannot receive notifications via the Webhook. |
| items[].updatedAt | string | ✓ | Indicates when there was a change in the webhook delivery result (status), e.g., when a failed delivery was retried and changed to success. This field is returned in ISO 8601 format. |
| items[].createdAt | string | ✓ | Indicates when the webhook event occurred and the log was created. This field is returned in ISO 8601 format. |
| items[].condition | object | ✓ | Field representing the Webhook subscription conditions. Conditions may vary by event type. |

### Example

```json
{
  "total": 1,
  "rpp": 10,
  "page": 1,
  "items": [
    {
      "subscriptionId": "31",
      "description": "test",
      "protocol": "ETHEREUM",
      "network": "MAINNET",
      "subscriptionType": "WEBHOOK",
      "eventType": "BLOCK_PERIOD",
      "notification": {
        "webhookUrl": "https://webhook.mock.server/blockperiod"
      },
      "isActive": true,
      "isInstant": true,
      "updatedAt": "2023-04-21T09:40:49.678Z",
      "createdAt": "2023-04-21T09:40:49.678Z",
      "condition": {
        "period": 1
      }
    }
  ]
}
```
