# Get Transactions In Ledger

**`POST /{chain}/{network}/blockchain/getTransactionsInLedger`**

Retrieves the list of transactions included in a specific Ledger.

## Parameters

| Name | In | Type | Required | Description |
|------|----|------|----------|-------------|
| chain | path | string | ✓ | Parameter to specify the target chain for the query. |
| network | path | string | ✓ | Parameter to specify the network of the target chain. Supported networks may vary depending on the chain. |

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
| ledger | string | ✓ | Parameter specifying the ledger to query. You can enter one of the following formats: - ledger index: Ledger number as a decimal string - ledger hash: 64-character hexadecimal string (excluding 0x prefix) - ledger tag: "earliest" or "latest" (oldest or most recent ledger)  |
| page | integer |  | The page parameter specifies the data page to query. It accepts values up to 100; for pages exceeding 100, use the cursor pagination method.  The page and cursor parameters cannot be used together. If both page and cursor are empty, cursor pagination is used. |
| rpp | integer |  | rpp stands for results per page and specifies the page size. You can specify a number greater than 0 and up to 1000. |
| cursor | string |  | The cursor parameter supports pagination and data navigation between pages. Provide the cursor value from the previous page in the next request to load the next page of data.  The page and cursor parameters cannot be used together. If both page and cursor are empty, cursor pagination is used. |
| withCount | boolean |  | Parameter specifying whether to include the count field in the response. The count field indicates the total number of requested data items. When set to true, the count field is included and response speed may be slower. |
| withBalanceChanges | boolean |  | Optional parameter determining whether to include the balanceChanges field in the response. balanceChanges contains balance change details for native token (XRP) and IOU tokens. Response speed may be slower when set to true. |
| withTokenTransfers | boolean |  | Optional parameter determining whether to include the tokenTransfers field in the response. tokenTransfers contains IOU token transfer history. Response speed may be slower when set to true. |

## Response

| Name | Type | Required | Description |
|------|------|----------|-------------|
| page | integer |  | Field representing the page number specified in the page parameter. This field is only included in the response when a value greater than 0 is specified for the page parameter. |
| rpp | integer |  | Field representing the number of results per page specified in the rpp parameter. |
| cursor | string |  | A field for cursor pagination. This value must be provided in the next request to load the next page of data. |
| count | integer |  | Field representing the total count of requested data. This field is only included in the response when true is set for the withCount parameter. |
| items | array | ✓ |  |
| items[].ledgerIndex | integer | ✓ | Sequence number of the ledger that contains the transaction. Used to identify which ledger the transaction was recorded in. |
| items[].ledgerTimestamp | integer | ✓ | Returns the close time of the ledger containing this transaction as a Unix timestamp. In the XRP Ledger, close time is recorded in seconds since the Ripple Epoch (January 1, 2000 00:00 UTC). Standard Unix timestamps use the Unix Epoch (January 1, 1970 00:00 UTC); the difference between them is 946,684,800 seconds. Thus ledgerTimestamp is the recorded close time plus 946,684,800 seconds. |
| items[].transactionIndex | integer | ✓ | Index indicating the processing order of the transaction within the ledger. Used to determine the order of transactions in the ledger. |
| items[].transactionHash | string | ✓ | A 256-bit (64-digit hexadecimal) cryptographic hash that uniquely identifies the transaction. Used for transaction data integrity and identification. |
| items[].transactionType | string | ✓ | String indicating the transaction type. Distinguishes various types such as Payment, OfferCreate, EscrowCreate, and is classified by processing method. |
| items[].transactionResult | string | ✓ | String indicating the transaction processing result, displayed as a result code such as 'tesSUCCESS'. Used to verify whether the transaction succeeded. |
| items[].account | string | ✓ | Account address that initiated the transaction. The account address is a Base58-encoded string 25-35 characters in length, starting with 'r'. |
| items[].sequence | integer | ✓ | Sequential number of transactions issued by the account, used to prevent duplicate submissions and replay attacks. |
| items[].lastLedgerSequence | integer |  | Sequence number of the last ledger in which the transaction is valid. The transaction will not be processed in ledgers after this value. |
| items[].ticketSequence | integer |  | For ticket-based transactions, the ticket number. Only present for transactions that use tickets. |
| items[].signingPubKey | string | ✓ | Public key used for the transaction signature, typically expressed as a 33-byte (66-digit hexadecimal) compressed public key. |
| items[].txnSignature | string |  | Transaction signature value generated with signingPubKey. Used for transaction verification and authentication. |
| items[].fee | string | ✓ | Transaction processing fee, expressed as an XRP amount (in XRP units) as a string. In XRP units where 1 XRP equals 1,000,000 drops. |
| items[].flags | string |  | Flag values applied to the transaction. Each bit controls a specific option or behavior; the default is often 0. |
| items[].accountTxnId | string |  | 256-bit hash identifying another transaction. If set, this transaction is valid only when it matches the sending account's most recent transaction ID. Used to prevent duplicate transaction submission or to verify that account state has not changed since a previous transaction. |
| items[].sourceTag | integer |  | 32-bit unsigned integer that the sender may specify when sending a transaction. Primarily used to identify the sending user from shared addresses (e.g. exchange or hosted wallet). Provides additional information so the receiver can determine "who sent it". |
| items[].signers | array |  | For multi-signed transactions, an array of objects containing each signer's information. Each object includes signer account, signing public key, signature value, etc. |
| items[].signers[].account | string | ✓ | Account address that signed the transaction. The account address is a Base58-encoded string 25-35 characters in length, starting with 'r'. |
| items[].signers[].txnSignature | string | ✓ | Transaction signature value generated by the signing account, used for transaction validity verification. This signature is DER-encoded; length varies but is typically 70-80 hexadecimal characters. |
| items[].signers[].signingPubKey | string | ✓ | Public key used for the transaction signature, typically a 33-byte compressed public key expressed as a hexadecimal string (about 66 characters). |
| items[].memos | array |  | Array of objects containing memo data attached to the transaction. Each memo object provides additional information such as memo content and format. |
| items[].memos[].memoData | string |  | Arbitrary hexadecimal string containing the memo content. Can be freely specified. |
| items[].memos[].memoFormat | string |  | Hexadecimal string describing the memo encoding. Typically used to specify MIME type. |
| items[].memos[].memoType | string |  | Hexadecimal string describing the memo type. Typically represents a unique relation such as a URL defined in RFC 5988. |
| items[].transactionCategory | string | ✓ | Enumeration string indicating the transaction category. Possible values include AMM, Check, Escrow, Offer, Payment, Payment Channel; each category is defined by the corresponding transaction type (e.g. Payment, OfferCreate). |
| items[].transactionDetails | object |  | Object containing details related to the transaction. May include additional data and fields required for transaction processing. |
| items[].balanceChanges | object |  |  |
| items[].balanceChanges.affectedNodesIndex | integer | ✓ | Index of the node within the AffectedNodes array. |
| items[].balanceChanges.account | string | ✓ | Account address whose balance was changed by the transaction. The account address is a Base58-encoded string 25-35 characters in length, starting with 'r'. |
| items[].balanceChanges.currency | string | ✓ | Indicates the currency code of the asset. Typically uses ISO 4217 standard codes (e.g. "USD", "EUR"), but 160-bit custom codes may also be used. Custom codes are expressed as 40-character hexadecimal strings and are used for proprietary assets defined by specific projects or platforms (e.g. "SOLO"). For XRP, the fixed string "XRP" is used in the currency field and is returned in the same structure as IOU assets. |
| items[].balanceChanges.issuer | string | ✓ | The address of the account that issued the asset. Even with the same currency code, asset credibility and risk may vary by issuer. Thus, the XRP Ledger uniquely identifies assets by the combination of currency code and issuer. Each asset's trustworthiness largely depends on the issuer's reputation. This field is a Base58-encoded string 25-35 characters in length, always starting with 'r'. XRP is a native asset with no issuer; in that case, issuer is shown as an empty string (""). |
| items[].balanceChanges.previousBalance | string | ✓ | Amount of the asset before the change. For XRP, expressed in XRP units where 1 XRP equals 1,000,000 drops. |
| items[].balanceChanges.finalBalance | string | ✓ | Amount of the asset after the change. For XRP, expressed in XRP units where 1 XRP equals 1,000,000 drops. |
| items[].balanceChanges.balanceChange | string |  | Amount changed by the transaction, including whether it increased or decreased. For XRP, expressed in XRP units where 1 XRP equals 1,000,000 drops. |
| items[].tokenTransfers | object |  |  |
| items[].tokenTransfers.ledgerIndex | integer |  | Integer value indicating the creation order of the ledger, used to identify its position (sequence number) on the network. Required field in the Transfer query API. |
| items[].tokenTransfers.ledgerTimestamp | integer |  | The ledger's close time converted to a Unix timestamp. The close time recorded in the ledger header is expressed in seconds since the Ripple Epoch (January 1, 2000 00:00 UTC), which differs from the Unix Epoch (January 1, 1970 00:00 UTC) by 946,684,800 seconds. Thus ledgerTimestamp is the recorded close time plus 946,684,800 seconds. Required field in the Transfer query API. |
| items[].tokenTransfers.transactionIndex | integer |  | Index indicating the processing order of the transaction within the ledger. Required field in the Transfer query API. |
| items[].tokenTransfers.transactionHash | string |  | A 256-bit (64-digit hexadecimal) cryptographic hash that uniquely identifies the transaction. Used for transaction data integrity and identification. Required field in the Transfer query API. |
| items[].tokenTransfers.transactionType | string |  | String indicating the transaction type. Distinguishes various types such as Payment, OfferCreate, EscrowCreate, and is classified by processing method. Required field in the Transfer query API. |
| items[].tokenTransfers.affectedNodesIndex | integer | ✓ | Index of the node within the AffectedNodes array. |
| items[].tokenTransfers.from | string | ✓ | Account address that initiated the transaction. The account address is a Base58-encoded string 25-35 characters in length, starting with 'r'. |
| items[].tokenTransfers.to | string | ✓ | Recipient account address of the transaction. The account address is a Base58-encoded string 25-35 characters in length, starting with 'r'. |
| items[].tokenTransfers.currency | string | ✓ | Indicates the currency code of the asset. Typically uses ISO 4217 standard codes (e.g. "USD", "EUR"), but 160-bit custom codes may also be used. Custom codes are expressed as 40-character hexadecimal strings and are used for proprietary assets defined by specific projects or platforms (e.g. "SOLO"). For XRP, the fixed string "XRP" is used in the currency field and is returned in the same structure as IOU assets. |
| items[].tokenTransfers.issuer | string | ✓ | The address of the account that issued the asset. Even with the same currency code, asset credibility and risk may vary by issuer. Thus, the XRP Ledger uniquely identifies assets by the combination of currency code and issuer. Each asset's trustworthiness largely depends on the issuer's reputation. This field is a Base58-encoded string 25-35 characters in length, always starting with 'r'. XRP is a native asset with no issuer; in that case, issuer is shown as an empty string (""). |
| items[].tokenTransfers.value | string | ✓ | Returns the asset amount as a string. The amount format varies by currency. For \"XRP\", the value is in standard XRP units where 1 XRP equals 1,000,000 drops. IOU assets are expressed based on the precision defined by the issuer. |
| items[].balanceOutAccounts | array |  | List of account addresses whose final net balance change is less than zero (decreased) after summing all sends and receives in the transaction. Reflects both XRP and IOU token balance changes. |
| items[].balanceInAccounts | array |  | List of account addresses whose final net balance change is greater than zero (increased) after summing all sends and receives in the transaction. Reflects both XRP and IOU token balance changes. |
| items[].balanceChangedTokens | array |  | List of token identifiers with non-zero final net balance change when summing the entire transaction. Each item follows the 'currency-issuer' format. |
