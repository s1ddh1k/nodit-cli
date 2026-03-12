# Get table item

**`POST /tables/{table_handle}/item`**

A specific table item can be queried using table_handle, key_type, value_type, and key value.
Aptos nodes prune account state history according to a configurable retention period. If the requested ledger version has already been pruned, the server returns a 410 response.

## Parameters

| Name | In | Type | Required | Description |
|------|----|------|----------|-------------|
| table_handle | path | string | ✓ | A pointer value indicating where the table is stored. |
| ledger_version | query | integer |  | The ledger version of the transaction to fetch account state from. If no value is provided, the latest version is used.  |

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
| key_type | string | ✓ | Type of the key |
| value_type | string | ✓ | Type of the value |
| key | string | ✓ | Key value of the table item. The format of the returned data varies depending on key_type.  |

### Example

```json
{
  "key_type": "0x1::type_info::TypeInfo",
  "value_type": "0x1::object::Object<0x1::fungible_asset::Metadata>",
  "key": "{\"account_address\":\"0x1\",\"module_name\":\"0x6170746f735f636f696e\",\"struct_name\":\"0x4170746f73436f696e\"}"
}
```

## Response

### Example

```json
{
  "inner": "0xa"
}
```
