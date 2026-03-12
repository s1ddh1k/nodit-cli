# suix_queryTransactionBlocks

**`POST /`**

Return list of transactions for a specified query criteria.

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
| id | integer | ✓ |  |
| jsonrpc | string | ✓ |  |
| method | string | ✓ |  |
| params | array | ✓ |  |
| params[].filter | object |  | CURRENTLY NOT SUPPORTED. Query by checkpoint. |
| params[].filter.Checkpoint | string | ✓ |  |
| params[].options | object |  |  |
| params[].options.showBalanceChanges | boolean |  | Whether to show balance_changes. Default to be False |
| params[].options.showEffects | boolean |  | Whether to show transaction effects. Default to be False |
| params[].options.showEvents | boolean |  | Whether to show transaction events. Default to be False |
| params[].options.showInput | boolean |  | Whether to show transaction input data. Default to be False |
| params[].options.showObjectChanges | boolean |  | Whether to show object_changes. Default to be False |
| params[].options.showRawEffects | boolean |  | Whether to show raw transaction effects. Default to be False |
| params[].options.showRawInput | boolean |  | Whether to show bcs-encoded transaction input data |

### Example

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "suix_queryTransactionBlocks",
  "params": [
    {
      "filter": {
        "InputObject": "0x93633829fcba6d6e0ccb13d3dbfe7614b81ea76b255e5d435032cd8595f37eb8"
      },
      "options": null
    },
    "HxidAfFfyr4kXSiWeVq1J6Tk526YUVDoSUY5PSnS4tEJ",
    100,
    false
  ]
}
```

## Response

| Name | Type | Required | Description |
|------|------|----------|-------------|
| jsonrpc | string |  |  |
| id | integer |  |  |
| result | object |  | `next_cursor` points to the last item in the page; Reading with `next_cursor` will start from the next item after `next_cursor` if `next_cursor` is `Some`, otherwise it will start from the first item. |
| result.data | array | ✓ |  |
| result.data[].balanceChanges | array,null |  |  |
| result.data[].checkpoint | string |  |  |
| result.data[].confirmedLocalExecution | boolean,null |  |  |
| result.data[].digest | string | ✓ |  |
| result.data[].effects | object |  | The response from processing a transaction or a certified transaction |
| result.data[].errors | array |  |  |
| result.data[].events | array,null |  |  |
| result.data[].objectChanges | array,null |  |  |
| result.data[].rawEffects | array |  |  |
| result.data[].rawTransaction | object |  | BCS encoded [SenderSignedData] that includes input object references returns empty array if `show_raw_transaction` is false |
| result.data[].timestampMs | string |  |  |
| result.data[].transaction | object |  |  |
| result.hasNextPage | boolean | ✓ |  |
| result.nextCursor | string |  |  |

### Example

```json
{
  "jsonrpc": "2.0",
  "result": {
    "data": [
      {
        "digest": "GUPcK4cmRmgsTFr52ab9f6fnzNVg3Lz6hF2aXFcsRzaD"
      },
      {
        "digest": "B2iV1SVbBjgTKfbJKPQrvTT6F3kNdekFuBwY9tQcAxV2"
      },
      {
        "digest": "8QrPa4x9iNG5r2zQfmeH8pJoVjjtq9AGzp8rp2fxi8Sk"
      },
      {
        "digest": "3nek86HEjXZ7K3EtrAcBG4wMrCS21gqr8BqwwC6M6P7F"
      }
    ],
    "nextCursor": "3nek86HEjXZ7K3EtrAcBG4wMrCS21gqr8BqwwC6M6P7F",
    "hasNextPage": false
  },
  "id": 1
}
```
