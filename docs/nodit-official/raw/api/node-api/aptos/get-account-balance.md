# Get account balance

**`GET /accounts/{address}/balance/{asset_type}`**

Returns the balance of coin/fungible asset (primary fungible asset store only) for the specified account, asset type, and ledger version of a specific transaction. If no ledger version is specified in the request, the latest ledger version is used.
Aptos nodes prune account state history according to a configurable retention period. If the requested ledger version has been pruned, the server responds with a 410 status code.

## Parameters

| Name | In | Type | Required | Description |
|------|----|------|----------|-------------|
| address | path | string | ✓ | The address of the account to retrieve. Account addresses without the hexadecimal prefix can also be queried.  |
| asset_type | path | string | ✓ | A parameter used to specify the type of asset to retrieve. The asset type must be provided in one of the following formats: - Coin: Struct-form asset ID (e.g., `0x1::aptos_coin::AptosCoin`) - Fungible Asset: Object address that owns the Token Metadata (e.g., `0xa`)  |
| ledger_version | query | integer |  | The ledger version of the transaction to fetch account state from. If no value is provided, the latest version is used.  |

## Response

### Example

```json
10478529012
```
