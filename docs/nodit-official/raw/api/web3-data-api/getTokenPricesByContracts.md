# Get Token Prices by Contracts

**`POST /{chain}/{network}/token/getTokenPricesByContracts`**

Retrieves the on-chain market price of tokens issued by the specified ERC20 token contracts. Multiple contracts can be queried, up to a maximum of 100 contracts.


> 🚧 Getting a 429 error? Check your subscription plan!
> 429 errors may occur when the Throughput limit of your subscription plan is exceeded.
> For example, the Starter plan has a limit of 300 CU per second, so if you use the Starter plan and call an API that consumes 350 CU, you may get a 429 error.
> Check the CU consumption of the API you are using on the Compute Unit Costs page, and consider upgrading to a higher plan if you need more Throughput!
> 👉 [Go to Compute Unit Costs page](/guides/usage-measuringcu)


> 🚧 Things to know when querying token prices
>
> Token prices provided by this API are based on CoinMarketCap data. Price information is updated periodically, so it may differ from real-time prices. Additionally, querying tokens not registered on CoinMarketCap will return an empty array.

## Parameters

| Name | In | Type | Required | Description |
|------|----|------|----------|-------------|
| chain | path | string | ✓ | Parameter to specify the target chain for the query. |
| network | path | string | ✓ | Parameter to specify the network of the target chain. Supported networks may vary depending on the chain. |

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
| contractAddresses | array | ✓ | Parameter specifying an array of contract addresses to query. Contract addresses can be entered as 40-character hexadecimal strings starting with 0x. |
| currency | string |  | Parameter for selecting the currency unit for Token Price queries. Supports USD and KRW. Default is USD. |
