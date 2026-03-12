# Estimate gas price

**`GET /estimate_gas_price`**

Estimates and returns the unit price of gas required to execute a transaction.

## Response

| Name | Type | Required | Description |
|------|------|----------|-------------|
| deprioritized_gas_estimate | integer |  | Gas unit price estimate for lower priority |
| gas_estimate | integer | ✓ | Current gas unit price estimate |
| prioritized_gas_estimate | integer |  | Gas unit price estimate for higher priority |
