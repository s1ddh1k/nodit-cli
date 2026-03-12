# Get raw table item

**`POST /tables/{table_handle}/raw_item`**

A specific table item can be identified using table_handle and key.
Aptos nodes prune account state history according to a configurable retention period. If the requested ledger version has already been pruned, the server returns a 410 response.

## Parameters

| Name | In | Type | Required | Description |
|------|----|------|----------|-------------|
| accept | header | string |  | The content type that the client can receive. The API works correctly only when application/x-bcs is specified.  |
| table_handle | path | string | ✓ | A pointer value indicating where the table is stored. |
| ledger_version | query | integer |  | The ledger version of the transaction to fetch account state from. If no value is provided, the latest version is used.  |

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
| key | string | ✓ | Key hex string value of the table item |

## Response

### Example

```json
""
```
