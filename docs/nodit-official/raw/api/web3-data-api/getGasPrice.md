# Get Gas Price

**`POST /{chain}/{network}/blockchain/getGasPrice`**

Retrieves the current Gas Price.

## Parameters

| Name | In | Type | Required | Description |
|------|----|------|----------|-------------|
| chain | path | string | ✓ | Parameter to specify the target chain for the query. |
| network | path | string | ✓ | Parameter to specify the network of the target chain. Supported networks may vary depending on the chain. |

## Response

| Name | Type | Required | Description |
|------|------|----------|-------------|
| high | string | ✓ | Indicates the highest price of the current Gas Price. Provided as a decimal string. |
| average | string | ✓ | Indicates the average price of the current Gas Price. Provided as a decimal string. |
| low | string | ✓ | Indicates the lowest price of the current Gas Price. Provided as a decimal string. |
| latestBlock | string | ✓ | Indicates the latest block. Provided as a decimal string. |
| baseFee | string | ✓ | Indicates the base fee of the latest block. Provided as a decimal string. |
