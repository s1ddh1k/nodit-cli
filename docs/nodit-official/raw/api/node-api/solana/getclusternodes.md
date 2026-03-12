# getClusterNodes

**`POST /`**

Returns information about all the nodes participating in the cluster

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
| jsonrpc | string | ✓ |  |
| id | integer | ✓ |  |
| method | string | ✓ |  |

### Example

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "getClusterNodes"
}
```

## Response

| Name | Type | Required | Description |
|------|------|----------|-------------|
| id | integer |  | The request ID |
| jsonrpc | string |  | The JSON-RPC version |
| result | array |  |  |
| result[].featureSet | integer |  | The unique identifier of the node's feature set |
| result[].gossip | string |  | Gossip network address for the node |
| result[].pubkey | string |  | Node public key, as base-58 encoded string |
| result[].rpc | string |  | JSON RPC network address for the node, or null if the JSON RPC service is not enabled |
| result[].shredVersion | integer |  | The shred version the node has been configured to use |
| result[].tpu | string |  | TPU network address for the node |
| result[].version | string |  | The software version of the node, or null if the version information is not available |
