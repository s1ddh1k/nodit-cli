# Get Daily Active Accounts Stats By Contract

**`POST /{chain}/{network}/stats/getDailyActiveAccountsStatsByContract`**

Retrieves the daily count of active accounts for a specific contract within the specified range.

> 📘 When is data reflected?
>
> In the current daily statistics API, a 'day' is based on UTC, and each day's data is aggregated from UTC 00:00:00 to before UTC 24:00:00 of that day. For daily statistics, the previous day's statistics may be delayed until 00:30:00 the next morning, so this should be considered when querying the latest data.


> 🚧 Check the network when using!
>
> This API is only supported on Ethereum Mainnet and TheBalance Mainnet. It cannot be used on other networks. Please verify the network when using.

## Parameters

| Name | In | Type | Required | Description |
|------|----|------|----------|-------------|
| chain | path | string | ✓ | Parameter to specify the target chain for the query. |
| network | path | string | ✓ | Parameter to specify the network of the target chain. Supported networks may vary depending on the chain. |

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
| contractAddress | string | ✓ | Parameter specifying the contract address to query. Can be entered as a 40-character hexadecimal string starting with 0x. |
| startDate | string | ✓ | Parameter specifying the start date for the query. Data can be queried for up to 100 days from start to end date. Supports YYYY-MM-DD format. |
| endDate | string | ✓ | Parameter specifying the end date for the query. Data can be queried for up to 100 days from start to end date. Supports YYYY-MM-DD format. |

## Response

| Name | Type | Required | Description |
|------|------|----------|-------------|
| count | integer |  | Field representing the total count of requested data. This field is only included in the response when true is set for the withCount parameter. |
| items | array | ✓ |  |
| items[].date | string | ✓ | Field representing the date. Provided in YYYY-MM-DD format. (e.g., 2021-01-01) |
| items[].count | integer | ✓ | Field representing the number of transactions per date. |
