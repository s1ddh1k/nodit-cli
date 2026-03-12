# Submit batch transactions

**`POST /transactions/batch`**

API for submitting multiple transactions in a batch. The response can have three outcomes:
1. If all transactions succeed, a 202 code is returned
2. If some transactions succeed, a 206 code is returned with the failed transactions
3. If all transactions fail, a 206 code is returned with the failed transactions

There are two ways to submit transactions.
1. **Submitting in JSON format:**
  - First encode the transaction in BCS. If your language has a BCS-supporting library, use it.
  - If you cannot encode BCS directly, you can use the Encode Submission API. When using this API, you must only send requests to a trusted node, as the node could tamper with the request content.
  - Sign the encoded transaction.
  - Submit the signed transaction with "application/json" Content-Type.

2. **Submitting in BCS format:**
  - Submit the BCS-encoded signed transaction with "application/x.aptos.signed_transaction+bcs" Content-Type.

## Parameters

| Name | In | Type | Required | Description |
|------|----|------|----------|-------------|
| accept | header | string |  | The content type that the client can receive. The API works correctly only when application/x-bcs is specified.  |
