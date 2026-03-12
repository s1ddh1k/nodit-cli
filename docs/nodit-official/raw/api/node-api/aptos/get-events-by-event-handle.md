# Get events by event handle

**`GET /accounts/{address}/events/{event_handle}/{field_name}`**

Event types are globally identified by account address and a monotonically increasing creation_number (a number assigned per event type for each account). This API returns the list of events corresponding to that event type.

> ⚠️ Notice regarding some API calls
> 
> As the latest version of the Aptos node client excludes the Legacy Indexer, Indexer-related errors may occur when making some API calls. 
> We are currently reviewing options to restore this functionality or provide alternative APIs, and will provide updates as related measures are completed.
> We apologize for any inconvenience.

## Parameters

| Name | In | Type | Required | Description |
|------|----|------|----------|-------------|
| address | path | string | ✓ | The address of the account to retrieve. Account addresses without the hexadecimal prefix can also be queried.  |
| event_handle | path | string | ✓ | The name of the event handle struct for querying a specific event handle.  |
| field_name | path | string | ✓ | The name of the field within the event_handle that specifies the event to query.  |
| limit | query | integer |  | The maximum number of account modules to fetch. If no value is provided, the default page size is used.  |
| start | query | string |  | A cursor that specifies the starting position for pagination.  This cursor cannot be arbitrarily generated on the client side. You must first call this endpoint without specifying this query parameter, then use the cursor returned in the X-Aptos-Cursor header of the response.  |

## Response

### Example

```json
[
  {
    "version": "221901708",
    "guid": {
      "creation_number": "0",
      "account_address": "0xacca9ab2d7e46a4ecc3171ab7d86306a6189311142c4ece077bd50327c34e5f6"
    },
    "sequence_number": "0",
    "type": "0x1::account::CoinRegisterEvent",
    "data": {
      "type_info": {
        "account_address": "0x1",
        "module_name": "0x6170746f735f636f696e",
        "struct_name": "0x4170746f73436f696e"
      }
    }
  },
  {
    "version": "223495356",
    "guid": {
      "creation_number": "0",
      "account_address": "0xacca9ab2d7e46a4ecc3171ab7d86306a6189311142c4ece077bd50327c34e5f6"
    },
    "sequence_number": "1",
    "type": "0x1::account::CoinRegisterEvent",
    "data": {
      "type_info": {
        "account_address": "0x4def3d3dee27308886f0a3611dd161ce34f977a9a5de4e80b237225923492a2a",
        "module_name": "0x636f696e",
        "struct_name": "0x54"
      }
    }
  }
]
```
