# Get Webhook History

**`GET /{chain}/{network}/webhooks/history`**

API for querying Webhook call history. Use this API to check Webhook execution status and call results. Call history includes Webhook event success/failure status and detailed information for each call.

> 💡 History data retention period
> Only Webhook call history from the last 30 days is stored. Data older than 30 days is not retained, so you must query needed history within that period.

## Parameters

| Name | In | Type | Required | Description |
|------|----|------|----------|-------------|
| chain | path | string | ✓ | Parameter to specify the target chain for the query. |
| network | path | string | ✓ | Parameter to specify the network of the target chain. Supported networks may vary depending on the chain. |
| subscriptionId | query | string | ✓ | Parameter that specifies the subscriptionId assigned to the Webhook to query. Use the subscriptionId returned when creating the Webhook to query, update, or delete Webhook information. Enter an empty value to query all Webhook information. |
| page | query | string |  | Specifies the page to query. Default is 1. |
| rpp | query | string |  | Specifies the number of events to query per page. Value range is 1 to 100. Default is 10. |
| withEventMessage | query | boolean |  | Specifies whether to include event messages in the query. Default is false. |
| status | query | string |  | Filters events by webhook call success (SUCCESS) or failure (FAIL). |
| startAt | query | string |  | Specifies the start time for event occurrence in ISO 8601 format. |
| endAt | query | string |  | Specifies the end time for event occurrence in ISO 8601 format. |
| startSequenceNumber | query | string |  | Specifies the sequenceNumber to start the query from. |

## Response

| Name | Type | Required | Description |
|------|------|----------|-------------|
| total | number |  | Total number of events matching the criteria. |
| page | number |  | Current page number of the query. |
| rpp | number |  | Number of events per page returned based on the request. |
| items | array |  | Array containing event detail data. |
| items[].subscriptionId | string |  | Field representing the subscriptionId assigned to the Webhook. subscriptionId is a unique value that identifies the Webhook and is used to query, update, or delete the Webhook. |
| items[].description | string |  | Field representing the Webhook description. |
| items[].protocol | string |  | Field representing the chain subscribed to via this Webhook. (e.g., ETHEREUM, POLYGON, OPTIMISM, ...) |
| items[].network | string |  | Field representing the network subscribed to via this Webhook. (e.g., MAINNET, SEPOLIA, MUMBAI, ...) |
| items[].subscriptionType | string |  | Field for identifying the subscription type. |
| items[].eventType | string |  | Field representing the event type subscribed to by this Webhook. |
| items[].notification | object |  | Field representing the information for receiving notifications via Webhook. |
| items[].notification.webhookUrl | string | ✓ | Field representing the URL for receiving Webhook notifications. |
| items[].isActive | boolean |  | Field representing whether the Webhook is active. true when the Webhook is enabled, false when disabled. When disabled, you cannot receive notifications via the Webhook. |
| items[].updatedAt | string |  | Indicates when there was a change in the webhook delivery result (status), e.g., when a failed delivery was retried and changed to success. This field is returned in ISO 8601 format. |
| items[].createdAt | string |  | Indicates when the webhook event occurred and the log was created. This field is returned in ISO 8601 format. |
| items[].condition | object |  | Field representing the Webhook subscription conditions. Conditions may vary by event type. |

### Example

```json
{
  "total": 146779,
  "rpp": 2,
  "page": 1,
  "items": [
    {
      "id": "1596",
      "subscriptionId": "103524",
      "sequenceNumber": "20",
      "status": "FAIL",
      "updatedAt": "2024-12-13T02:06:09.000Z",
      "createdAt": "2024-12-13T02:06:07.000Z",
      "eventMessage": {
        "subscriptionId": "103524",
        "sequenceNumber": "20",
        "eventType": "EVENT",
        "network": "MAINNET",
        "protocol": "APTOS",
        "event": {
          "eventType": "0x1::block::NewBlockEvent",
          "eventAccountAddress": null,
          "payloadFunction": null,
          "messages": [
            [
              {
                "guid": {
                  "creation_number": "3",
                  "account_address": "0x0000000000000000000000000000000000000000000000000000000000000001"
                },
                "sequence_number": "264073031",
                "type": "0x1::block::NewBlockEvent",
                "data": {
                  "epoch": "9559",
                  "failed_proposer_indices": [],
                  "hash": "0x7d360d5e499d93eb87ca34fe3f8f18eab602cb8d365e402da750b7e6c5786c4d",
                  "height": "264073031",
                  "previous_block_votes_bitvec": "0x177fffef57fbbf7ff63c15cfef4679a1bb79d8",
                  "proposer": "0xa651c7c52d64a2014379902bbc92439d196499bcc36d94ff0395aa45837c66db",
                  "round": "13485",
                  "time_microseconds": "1734048334428800"
                }
              }
            ]
          ]
        }
      }
    },
    {
      "id": "1597",
      "subscriptionId": "103524",
      "sequenceNumber": "21",
      "status": "FAIL",
      "updatedAt": "2024-12-13T02:26:29.000Z",
      "createdAt": "2024-12-13T02:26:28.000Z",
      "eventMessage": {
        "subscriptionId": "103524",
        "sequenceNumber": "21",
        "eventType": "EVENT",
        "network": "MAINNET",
        "protocol": "APTOS",
        "event": {
          "eventType": "0x1::block::NewBlockEvent",
          "eventAccountAddress": null,
          "payloadFunction": null,
          "messages": [
            [
              {
                "guid": {
                  "creation_number": "3",
                  "account_address": "0x0000000000000000000000000000000000000000000000000000000000000001"
                },
                "sequence_number": "264073128",
                "type": "0x1::block::NewBlockEvent",
                "data": {
                  "epoch": "9559",
                  "failed_proposer_indices": [],
                  "hash": "0x53570fbecfb44c723c2db3bac968d467bd4d4a8185a383b094f7578c4a3f014",
                  "height": "264073128",
                  "previous_block_votes_bitvec": "0x7f048b4a700f7259fff3fafb9dc89141c2ddfc",
                  "proposer": "0x6d00d52f94579e51308ae83841b5133e3a7e93b51fcd8cf338d766e3f0331026",
                  "round": "13582",
                  "time_microseconds": "1734048354656343"
                }
              }
            ]
          ]
        }
      }
    }
  ]
}
```
