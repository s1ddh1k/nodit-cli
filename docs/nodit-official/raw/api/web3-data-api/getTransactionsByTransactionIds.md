# Get Transactions By Transaction IDs

**`POST /{chain}/{network}/blockchain/getTransactionsByTransactionIds`**

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
| transactionIds | array | ✓ | Enter the transaction IDs to query as an array. |
