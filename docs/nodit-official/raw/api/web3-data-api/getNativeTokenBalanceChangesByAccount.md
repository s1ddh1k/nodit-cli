# Get Native Token Balance Changes By Account

**`POST /{chain}/{network}/native/getNativeTokenBalanceChangesByAccount`**

Retrieves the native token balance change history for a specific account.

## Parameters

| Name | In | Type | Required | Description |
|------|----|------|----------|-------------|
| chain | path | string | ✓ | Parameter to specify the target chain for the query. |
| network | path | string | ✓ | Parameter to specify the network of the target chain. Supported networks may vary depending on the chain. |

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
| accountAddress | string | ✓ | Parameter specifying the account address for balance query. It is a Base58-encoded string of 25-35 characters, always starting with "r". |
| fromLedger | string |  | Parameter specifying the starting ledger for the query. You can enter one of the following formats: - **ledger index**: Enter ledger number as a decimal string - **ledger hash**: 64-character hexadecimal string (excluding 0x prefix) - **ledger tag**: Enter "earliest" or "latest" (oldest or most recent ledger)  Notes: - If provided without toLedger, results are queried from this ledger to the latest ledger. - fromLedger value cannot be greater than toLedger value. - If fromLedger and toLedger have the same value, only that ledger is queried. - "latest" for fromLedger is only allowed when toLedger is "latest". |
| toLedger | string |  | Parameter specifying the ending ledger for the query. You can enter one of the following formats: - **ledger index**: Enter ledger number as a decimal string - **ledger hash**: 64-character hexadecimal string (excluding 0x prefix) - **ledger tag**: Enter "earliest" or "latest" (oldest or most recent ledger)  Notes: - If provided without fromLedger, results are queried from genesis ledger to this ledger. - toLedger value cannot be less than fromLedger value. - If fromLedger and toLedger have the same value, only that ledger is queried. - "earliest" for toLedger is only allowed when fromLedger is "earliest". |
| fromDate | string |  | Parameter specifying the start date for the query. Date must follow ISO 8601 format (YYYY-MM-DDThh:mm:ss{time zone}) including seconds.  Notes: - If provided without toDate, results are queried from this date to the latest ledger. - fromDate value must be equal to or earlier than toDate value. - If fromDate and toDate are the same, only ledgers from that date are queried. - This field cannot be used together with fromLedger and toLedger. |
| toDate | string |  | Parameter specifying the end date for the query. Date must follow ISO 8601 format (YYYY-MM-DDThh:mm:ss{time zone}) including seconds.  Notes: - If provided without fromDate, results are queried from genesis ledger creation to this date. - toDate value must be equal to or later than fromDate value. - If fromDate and toDate are the same, only ledgers from that date are queried. - This field cannot be used together with fromLedger and toLedger. |
| page | integer |  | The page parameter specifies the data page to query. It accepts values up to 100; for pages exceeding 100, use the cursor pagination method.  The page and cursor parameters cannot be used together. If both page and cursor are empty, cursor pagination is used. |
| rpp | integer |  | rpp stands for results per page and specifies the page size. You can specify a number greater than 0 and up to 1000. |
| cursor | string |  | The cursor parameter supports pagination and data navigation between pages. Provide the cursor value from the previous page in the next request to load the next page of data.  The page and cursor parameters cannot be used together. If both page and cursor are empty, cursor pagination is used. |
| withCount | boolean |  | Parameter specifying whether to include the count field in the response. The count field indicates the total number of requested data items. When set to true, the count field is included and response speed may be slower. |

## Response

| Name | Type | Required | Description |
|------|------|----------|-------------|
| page | integer |  | Field representing the page number specified in the page parameter. This field is only included in the response when a value greater than 0 is specified for the page parameter. |
| rpp | integer |  | Field representing the number of results per page specified in the rpp parameter. |
| cursor | string |  | A field for cursor pagination. This value must be provided in the next request to load the next page of data. |
| count | integer |  | Field representing the total count of requested data. This field is only included in the response when true is set for the withCount parameter. |
| items | array | ✓ |  |
| items[].ledgerIndex | integer | ✓ | Integer value indicating the creation order of the ledger, used to identify its position (sequence number) on the network. Required field in the Balance Changes query API. |
| items[].ledgerTimestamp | integer | ✓ | The ledger's close time converted to a Unix timestamp. The close time recorded in the ledger header is expressed in seconds since the Ripple Epoch (January 1, 2000 00:00 UTC), which differs from the Unix Epoch (January 1, 1970 00:00 UTC) by 946,684,800 seconds. Thus ledgerTimestamp is the recorded close time plus 946,684,800 seconds. Required field in the Transfer query API. |
| items[].transactionIndex | integer | ✓ | Index indicating the processing order of the transaction within the ledger. Used to determine the order of transactions in the ledger. Required field in the Balance Changes query API. |
| items[].transactionHash | string | ✓ | A 256-bit (64-digit hexadecimal) cryptographic hash that uniquely identifies the transaction. Used for transaction data integrity and identification. Required field in the Balance Changes query API. |
| items[].transactionType | string | ✓ | String indicating the transaction type (Payment, OfferCreate, EscrowCreate, etc.). Required field in the Transfer query API. |
| items[].affectedNodesIndex | integer | ✓ | Index of the node within the AffectedNodes array. |
| items[].account | string | ✓ | Account address whose balance was changed by the transaction. The account address is a Base58-encoded string 25-35 characters in length, starting with 'r'. |
| items[].currency | string | ✓ | Indicates the currency code of the asset. Typically uses ISO 4217 standard codes (e.g. "USD", "EUR"), but 160-bit custom codes may also be used. Custom codes are expressed as 40-character hexadecimal strings and are used for proprietary assets defined by specific projects or platforms (e.g. "SOLO"). For XRP, the fixed string "XRP" is used in the currency field and is returned in the same structure as IOU assets. |
| items[].issuer | string | ✓ | The address of the account that issued the asset. Even with the same currency code, asset credibility and risk may vary by issuer. Thus, the XRP Ledger uniquely identifies assets by the combination of currency code and issuer. Each asset's trustworthiness largely depends on the issuer's reputation. This field is a Base58-encoded string 25-35 characters in length, always starting with 'r'. XRP is a native asset with no issuer; in that case, issuer is shown as an empty string (""). |
| items[].previousBalance | string | ✓ | Amount of the asset before the change. For XRP, expressed in XRP units where 1 XRP equals 1,000,000 drops. |
| items[].finalBalance | string | ✓ | Amount of the asset after the change. For XRP, expressed in XRP units where 1 XRP equals 1,000,000 drops. |
| items[].balanceChange | string |  | Amount changed by the transaction, including whether it increased or decreased. For XRP, expressed in XRP units where 1 XRP equals 1,000,000 drops. |
