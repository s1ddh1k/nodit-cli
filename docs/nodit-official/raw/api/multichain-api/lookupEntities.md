# Lookup entities

**`POST /lookupEntities`**

Determines whether the input string corresponds to an entity (Account or Transaction) and returns the result.

> 💡 For Account entities, only active accounts are included in the results.
> When an entity is identified as an Account, only active accounts meeting one or more of the following criteria are included:
>  1.	Have sent or received transactions
>  2.	Have a history of sending or receiving on-chain assets such as Tokens or NFTs
>  3.	Have an on-chain asset balance (e.g., Tokens, NFTs)
>
> Entity details (e.g., account holdings, transaction history) can be queried via the Web3 Data API using the identifiers (e.g., account address, transaction hash) determined by this API.

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
| input | string | ✓ | Parameter that specifies the string to query  |
| chains | array | ✓ | Parameter that specifies the chains and networks to include in the results, in {chain}-{network} format.  |

## Response

| Name | Type | Required | Description |
|------|------|----------|-------------|
| input | string | ✓ | Returns the string that was entered for the query. |
| items | array | ✓ |  |
| items[].chain | string | ✓ | String representing the target chain for the query. The actual response returns in {chain}-{network} format.  |
| items[].type | string | ✓ | Type of the target chain being queried |
| items[].normalizedInput | string | ✓ | Value normalized to the format of the actual target chain. For example, - For Aptos, account addresses are normalized to zero-padded 64-digit hex. (e.g., "0x1" -> "0x0000000000000000000000000000000000000000000000000000000000000001") - For Tron, account addresses are normalized to base58 strings. (e.g., "0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045" -> "TVjpchRyV9wdpj6kmwqVsBDWY1J8PaFtnb")  |
