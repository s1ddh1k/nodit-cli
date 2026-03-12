# bor_getCurrentProposer

**`POST /`**

Returns the address of the validator (or proposer) that will propose the current block. This method is used to identify the node responsible for producing the next block on the network. In PoA networks, block production rights rotate or are assigned according to a specific algorithm, and this method allows you to confirm the address of the node currently responsible for block production.

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
| id | integer | ✓ |  |
| jsonrpc | string | ✓ |  |
| method | string | ✓ |  |

### Example

```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "method": "bor_getCurrentProposer"
}
```

## Response

### Example

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": "0xf0245f6251bef9447a08766b9da2b07b28ad80b0"
}
```
