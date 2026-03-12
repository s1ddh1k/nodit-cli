# Get Hourly Transactions Stats

**`POST /{chain}/{network}/stats/getHourlyTransactionsStats`**

Retrieves the hourly transaction count within the specified range.

> 📘 When is data reflected?
>
> In the current hourly statistics API, time is based on UTC, and each response item provides statistics for the period from the date +1 hour. For hourly statistics, the most recent hour's statistics may be delayed by up to 40 minutes, so this should be considered when querying the latest data.


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
| startDateTime | string | ✓ | Parameter specifying the start date and time for the query. Data can be queried for up to 2400 hours from start to end datetime. Supports YYYY-MM-DD-HH format. |
| endDateTime | string | ✓ | Parameter specifying the end date and time for the query. Data can be queried for up to 2400 hours from start to end datetime. Supports YYYY-MM-DD-HH format. |

## Response

| Name | Type | Required | Description |
|------|------|----------|-------------|
| count | integer |  | Field representing the total count of requested data. This field is only included in the response when true is set for the withCount parameter. |
| items | array | ✓ |  |
| items[].date | string | ✓ | Field representing the date and time. Provided in YYYY-MM-DD-HH format. (e.g., 2021-01-01-00) |
| items[].count | integer | ✓ | Field representing the number of transactions per hour. |
