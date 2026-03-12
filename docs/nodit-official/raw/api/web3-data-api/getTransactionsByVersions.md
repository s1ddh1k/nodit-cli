# Get Transactions By Versions

**`POST /{chain}/{network}/blockchain/getTransactionsByVersions`**

Retrieves information for multiple transactions. Up to 1,000 transactions can be queried.


> 🚧 Getting a 429 error? Check your subscription plan!
> A 429 error may occur when the throughput limit of your subscribed plan is exceeded.
> For example, the Starter plan has a limit of 300 CU per second, so calling an API that consumes 350 CU while on the Starter plan may result in a 429 error.
> Check the CU consumption of the API you are using on the Compute Unit Costs page, and consider upgrading to a higher plan if you need more throughput!
> 👉 [Go to Compute Unit Costs page](/guides/usage-measuringcu)

## Parameters

| Name | In | Type | Required | Description |
|------|----|------|----------|-------------|
| chain | path | string | ✓ | Parameter to specify the target chain for the query. |
| network | path | string | ✓ | Parameter to specify the network of the target chain. Supported networks may vary depending on the chain. |

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
| transactionVersions | array | ✓ | Enter the transaction versions to query as an array. |
| withBalanceChanges | boolean |  | Parameter specifying whether to include the balanceChanges field in the response.  - balanceChanges is a field containing balance changes for Native token (APT) and Tokens. - Response speed may be slower when true is set for this parameter. |
