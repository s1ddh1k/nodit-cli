---

## How is Nodit's Usage Measured?

Nodit's service usage is measured by Compute Unit (CU) and Throughput (Compute Unit per Second) metrics, and there are service usage limits according to the plans provided by Nodit. If users exceed the service usage criteria, Nodit usage may be restricted, so it is recommended to subscribe to a plan that suits your purpose and usage.

## Compute Unit(CU)

Compute Unit means the unit of computing resources used when users use Nodit's services. Through CU, users can accurately calculate the amount and cost of computing resources consumed in a project, and based on this, can flexibly design the pricing system.

### CU Provision by Plan

Nodit applies daily CU limit or monthly CU limit policies depending on the plan. The free plan, Starter plan, applies **daily CU limit**, and all paid plans have **monthly CU usage limits**. The amount of CU granted for each plan is shown in the table below. CUs are automatically recharged at 0:00 (UTC) every day or on the 1st of every month at 0:00 (UTC), and unused CUs are not carried over.

| **Spec**        | **Starter** | **Starter Plus** | **Developer** | **Business**  | **Enterprise** |
| --------------- | ----------- | :--------------- | ------------- | ------------- | -------------- |
| **Daily CUs**   | 1,000,000   | -                | -             | -             | Custom         |
| **Monthly CUs** | -           | 100,000,000      | 450,000,000   | 1,600,000,000 | Custom         |

### Auto-scaling

Nodit supports the Auto-scaling function so that users can **automatically use CUs in excess** when all CUs are exhausted for smooth service use. Click the link below to learn how to enable/disable Auto-scaling.

* [Go to Auto-scaling enable/disable method](/guides/auto-scaling)

## Throughput

Throughput means the amount of work that a user's project can process per second, and there is a difference in the amount of work that a project can process depending on the plan the user is subscribing to. The amount of work is measured in CU usage per second (Compute Unit per Second, hereinafter CUPS), and requests sent exceeding the Throughput limit set for the project may be restricted in processing. You can check the Throughput criteria for each Nodit plan through the table below.

**Throughput Criteria by Nodit Plan**

| **Spec**              | **Starter** | **Starter Plus** | **Developer** | **Business** | **Enterprise** |
| --------------------- | ----------- | :--------------- | ------------- | ------------ | -------------- |
| **Throughput (CU/s)** | 300         | 400              | 600           | 3,000        | Custom         |

## Compute Unit(CU) Costs

Compute Unit Per Second (CUPS) is an indicator representing the amount of calculations that can be performed per second, and is used to measure the amount of computing resources required for using Nodit services.

1 CUPS represents the amount of calculation work that can be processed in 1 second, and through this indicator, you can measure the performance required by the system to efficiently manage resources needed for business or projects and optimize costs.

### Web3 Data APIs

| **Category**         | **Method**                                    | **CU** |
| -------------------- | --------------------------------------------- | ------ |
| **NFT API**          | Get NFT Contract Metadata by Contracts        | 150    |
|                      | Get NFT Contracts by Account                  | 150    |
|                      | Get NFT Holders by Contract                   | 150    |
|                      | Get NFT Holders by Token ID                   | 80     |
|                      | Get NFT Metadata by Contract                  | 150    |
|                      | Get NFT Metadata by Token IDs                 | 150    |
|                      | Get NFT Transfers by Account                  | 150    |
|                      | Get NFT Transfers by Contract                 | 150    |
|                      | Get NFT Transfers by TokenId                  | 150    |
|                      | Get NFT Transfers Within Range                | 150    |
|                      | Get NFTs Owned by Account                     | 150    |
|                      | Search NFT Contract Metadata by Keyword       | 500    |
|                      | Sync Nft Metadata                             | 80     |
| **Token API**        | Get Native Balance by Account                 | 30     |
|                      | Get Native Token Balance by Account           | 150    |
|                      | Get Native Token Transfer by Account          | 150    |
|                      | Get Token Allowance                           | 30     |
|                      | Get Token Contract Metadata by Contracts      | 150    |
|                      | Get Token Holders by Contract                 | 150    |
|                      | Get Token Prices by Contracts                 | 350    |
|                      | Get Token Transfers by Account                | 150    |
|                      | Get Token Transfers by Contract               | 150    |
|                      | Get Token Transfers Within Range              | 150    |
|                      | Get Tokens Owned by Account                   | 150    |
|                      | Search Token Contract Metadata by Keyword     | 500    |
| **Blockchain API**   | Get Block by Hash or Number                   | 30     |
|                      | Get Total Transaction Count By Account        | 150    |
|                      | Get Blocks Within Range                       | 150    |
|                      | Get Gas Price                                 | 80     |
|                      | Get Internal Transactions by Account          | 500    |
|                      | Get Internal Transactions by Transaction Hash | 150    |
|                      | Get Next Nonce by Account                     | 30     |
|                      | Get Transaction by Hash                       | 80     |
|                      | Get Transaction By Transaction ID             | 80     |
|                      | Get Transactions by Account                   | 150    |
|                      | Get Transactions by Hashes                    | 500    |
|                      | Get Transactions By Transaction IDs           | 500    |
|                      | Get Transactions In Block                     | 150    |
|                      | Get Unspent Transaction Outputs By Account    | 150    |
|                      | Is Contract                                   | 30     |
|                      | Search Events                                 | 350    |
| **Statistics API**   | Get Account Stat                              | 150    |
|                      | Get Daily Active Accounts Stats               | 150    |
|                      | Get Daily Active Accounts Stats By Contract   | 150    |
|                      | Get Daily Transactions Stats                  | 150    |
|                      | Get Daily Transactions Stats By Contract      | 150    |
|                      | Get Hourly Active Accounts Stats              | 150    |
|                      | Get Hourly Active Accounts Stats By Contract  | 150    |
|                      | Get Hourly Transactions Stats                 | 150    |
|                      | Get Hourly Transactions Stats By Contract     | 150    |
| **ENS API**          | Get Address By ENS Name                       | 80     |
|                      | Get ENS Name By Address                       | 80     |
|                      | Get ENS Record By Name                        | 150    |
|                      | Get ENS Records By Account                    | 150    |
| **Asset(TRC10) API** | Get Asset(TRC10) Metadata by Issuer           | 150    |
|                      | Get Asset(TRC10) Metadata by IDs              | 150    |
|                      | Search Asset(TRC10) Metadata by Keyword       | 500    |
|                      | Get Asset(TRC10) Holders by ID                | 150    |
|                      | Get Asset(TRC10) Transfers by ID              | 150    |
|                      | Get Asset(TRC10) Transfers within Range       | 150    |
|                      | Get Asset(TRC10) Transfers by Account         | 150    |
|                      | Get Assets(TRC10) Owned by Account            | 150    |



### Multichain API

| **Method**      | **CU** |
| --------------- | ------ |
| Lookup Entities | 1000   |



### Webhook and Websocket APIs

When subscribing to Webhook and WebSocket, CU Cost is determined based on bandwidth, that is, the amount of data delivered.

| **Bandwidth** | **CU** |
| ------------- | ------ |
| 1 byte        | 0.03   |

### EVM Node API

#### eth namespace

| **Method**                              | **CU** | **Ethereum** | **Polygon** | **Arbitrum** | **Optimism** | **Base** | **Kaia** | **The Balance** |
| --------------------------------------- | ------ | ------------ | ----------- | ------------ | ------------ | -------- | -------- | --------------- |
| eth_blockNumber                         | 10     | ✅            | ✅           | ✅            | ✅            | ✅        | ✅        | ✅               |
| eth_call                                | 23     | ✅            | ✅           | ✅            | ✅            | ✅        | ✅        | ✅               |
| eth_chainId                             | 0      | ✅            | ✅           | ✅            | ✅            | ✅        | ✅        | ✅               |
| eth_createAccessList                    | 22     | ✅            | ✅           | ✅            | ✅            | ✅        | ✅        | ✅               |
| eth_estimateGas                         | 67     | ✅            | ✅           | ✅            | ✅            | ✅        | ✅        | ✅               |
| eth_feeHistory                          | 10     | ✅            | ✅           | ✅            | ✅            | ✅        | ✅        | ✅               |
| eth_gasPrice                            | 10     | ✅            | ✅           | ✅            | ✅            | ✅        | ✅        | ✅               |
| eth_getBalance                          | 26     | ✅            | ✅           | ✅            | ✅            | ✅        | ✅        | ✅               |
| eth_getBlockByHash                      | 32     | ✅            | ✅           | ✅            | ✅            | ✅        | ✅        | ✅               |
| eth_getBlockByNumber                    | 31     | ✅            | ✅           | ✅            | ✅            | ✅        | ✅        | ✅               |
| eth_getBlockReceipts                    | 28     | ✅            | ❌           | ✅            | ✅            | ✅        | ✅        | ❌               |
| eth_getBlockTransactionCountByHash      | 14     | ✅            | ✅           | ✅            | ✅            | ✅        | ✅        | ✅               |
| eth_getBlockTransactionCountByNumber    | 13     | ✅            | ✅           | ✅            | ✅            | ✅        | ✅        | ✅               |
| eth_getCode                             | 30     | ✅            | ✅           | ✅            | ✅            | ✅        | ✅        | ✅               |
| eth_getFilterChanges                    | 18     | ✅            | ✅           | ✅            | ✅            | ✅        | ✅        | ✅               |
| eth_getFilterLogs                       | 69     | ✅            | ✅           | ✅            | ✅            | ✅        | ✅        | ✅               |
| eth_getLogs                             | 66     | ✅            | ✅           | ✅            | ✅            | ✅        | ✅        | ✅               |
| eth_getProof                            | 23     | ✅            | ✅           | ✅            | ✅            | ✅        | ✅        | ✅               |
| eth_getStorageAt                        | 26     | ✅            | ✅           | ✅            | ✅            | ✅        | ✅        | ✅               |
| eth_getTransactionByBlockHashAndIndex   | 15     | ✅            | ✅           | ✅            | ✅            | ✅        | ✅        | ✅               |
| eth_getTransactionByBlockNumberAndIndex | 13     | ✅            | ✅           | ✅            | ✅            | ✅        | ✅        | ✅               |
| eth_getTransactionByHash                | 15     | ✅            | ✅           | ✅            | ✅            | ✅        | ✅        | ✅               |
| eth_getTransactionCount                 | 21     | ✅            | ✅           | ✅            | ✅            | ✅        | ✅        | ✅               |
| eth_getTransactionReceipt               | 16     | ✅            | ✅           | ✅            | ✅            | ✅        | ✅        | ✅               |
| eth_getUncleByBlockHashAndIndex         | 19     | ✅            | ✅           | ✅            | ✅            | ✅        | ✅        | ✅               |
| eth_getUncleByBlockNumberAndIndex       | 13     | ✅            | ✅           | ✅            | ✅            | ✅        | ✅        | ✅               |
| eth_getUncleCountByBlockHash            | 16     | ✅            | ✅           | ✅            | ✅            | ✅        | ✅        | ✅               |
| eth_getUncleCountByBlockNumber          | 13     | ✅            | ✅           | ✅            | ✅            | ✅        | ✅        | ✅               |
| eth_maxPriorityFeePerGas                | 23     | ✅            | ✅           | ✅            | ✅            | ✅        | ✅        | ✅               |
| eth_newBlockFilter                      | 18     | ✅            | ✅           | ✅            | ✅            | ✅        | ✅        | ✅               |
| eth_newFilter                           | 18     | ✅            | ✅           | ✅            | ✅            | ✅        | ✅        | ✅               |
| eth_newPendingTransactionFilter         | 18     | ✅            | ✅           | ✅            | ✅            | ✅        | ✅        | ✅               |
| eth_sendRawTransaction                  | 170    | ✅            | ✅           | ✅            | ✅            | ✅        | ✅        | ✅               |
| eth_subscribe                           | 10     | ✅            | ✅           | ✅            | ✅            | ✅        | ✅        | ✅               |
| eth_uninstallFilter                     | 10     | ✅            | ✅           | ✅            | ✅            | ✅        | ✅        | ✅               |
| eth_unsubscribe                         | 10     | ✅            | ✅           | ✅            | ✅            | ✅        | ✅        | ✅               |

#### net namespace

| **Method**    | **CU** | **Ethereum** | **Polygon** | **Optimism** | **Arbitrum** | **Base** | **Kaia** | **The Balance** |
| ------------- | ------ | ------------ | ----------- | ------------ | ------------ | -------- | -------- | --------------- |
| net_listening | 0      | ✅            | ✅           | ✅            | ❌            | ✅        | ✅        | ✅               |
| net_version   | 0      | ✅            | ✅           | ✅            | ✅            | ✅        | ✅        | ✅               |

#### web3 namespace

| **Method**         | **CU** | **Ethereum** | **Polygon** | **Optimism** | **Arbitrum** | **Base** | **Kaia** | **The Balance** |
| ------------------ | ------ | ------------ | ----------- | ------------ | ------------ | -------- | -------- | --------------- |
| web3_clientVersion | 10     | ✅            | ✅           | ✅            | ✅            | ✅        | ✅        | ✅               |
| web3_sha3          | 10     | ✅            | ✅           | ✅            | ✅            | ✅        | ✅        | ✅               |

#### trace namespace

| **Method**                    | **CU** | **Ethereum** |
| ----------------------------- | ------ | ------------ |
| trace_block                   | 113    | ✅            |
| trace_call                    | 73     | ✅            |
| trace_filter                  | 73     | ✅            |
| trace_get                     | 68     | ✅            |
| trace_replayBlockTransactions | 119    | ✅            |
| trace_replayTransaction       | 70     | ✅            |
| trace_transaction             | 73     | ✅            |

#### debug namespace

| **Method**               | **CU** | **Ethereum** | **Polygon** | **Optimism** | **Arbitrum** | **Base** | **Kaia** | **The Balance** |
| ------------------------ | ------ | ------------ | ----------- | ------------ | ------------ | -------- | -------- | --------------- |
| debug_traceBlockByHash   | 530    | ✅            | ✅           | ✅            | ✅            | ✅        | ✅        | ✅               |
| debug_traceBlockByNumber | 530    | ✅            | ✅           | ✅            | ✅            | ✅        | ✅        | ✅               |
| debug_traceCall          | 206    | ✅            | ✅           | ✅            | ✅            | ✅        | ✅        | ✅               |
| debug_traceTransaction   | 206    | ✅            | ✅           | ✅            | ✅            | ✅        | ✅        | ✅               |

#### bor namespace

| **Method**               | **CU** | **Polygon** |
| ------------------------ | ------ | ----------- |
| bor_getAuthor            | 10     | ✅           |
| bor_getCurrentProposer   | 10     | ✅           |
| bor_getCurrentValidators | 10     | ✅           |
| bor_getSignersAtHash     | 10     | ✅           |

#### optimism namespace

| **Method**             | **CU** | **Optimism** | **Base** |
| ---------------------- | ------ | ------------ | -------- |
| optimism_outputAtBlock | 10     | ✅            | ✅        |
| optimism_rollupConfig  | 10     | ✅            | ✅        |

#### kaia namespace

| **Method**                               | **CU** | **Kaia** |
| ---------------------------------------- | ------ | -------- |
| kaia_blockNumber                         | 10     | ✅        |
| kaia_call                                | 23     | ✅        |
| kaia_chainId                             | 0      | ✅        |
| kaia_createAccessList                    | 22     | ✅        |
| kaia_estimateGas                         | 67     | ✅        |
| kaia_feeHistory                          | 10     | ✅        |
| kaia_gasPrice                            | 10     | ✅        |
| kaia_getBalance                          | 26     | ✅        |
| kaia_getBlockByHash                      | 32     | ✅        |
| kaia_getBlockByNumber                    | 31     | ✅        |
| kaia_getBlockReceipts                    | 28     | ✅        |
| kaia_getBlockTransactionCountByHash      | 14     | ✅        |
| kaia_getBlockTransactionCountByNumber    | 13     | ✅        |
| kaia_getCode                             | 30     | ✅        |
| kaia_getFilterChanges                    | 18     | ✅        |
| kaia_getFilterLogs                       | 69     | ✅        |
| kaia_getLogs                             | 66     | ✅        |
| kaia_getProof                            | 23     | ✅        |
| kaia_getStorageAt                        | 26     | ✅        |
| kaia_getTransactionByBlockHashAndIndex   | 15     | ✅        |
| kaia_getTransactionByBlockNumberAndIndex | 13     | ✅        |
| kaia_getTransactionByHash                | 15     | ✅        |
| kaia_getTransactionCount                 | 21     | ✅        |
| kaia_getTransactionReceipt               | 16     | ✅        |
| kaia_maxPriorityFeePerGas                | 23     | ✅        |
| kaia_newBlockFilter                      | 18     | ✅        |
| kaia_newFilter                           | 18     | ✅        |
| kaia_newPendingTransactionFilter         | 18     | ✅        |
| kaia_sendRawTransaction                  | 170    | ✅        |
| kaia_subscribe                           | 10     | ✅        |
| kaia_uninstallFilter                     | 10     | ✅        |
| kaia_unsubscribe                         | 10     | ✅        |

#### klay namespace

| **Method**                               | **CU** | **kaia** |
| ---------------------------------------- | ------ | -------- |
| klay_blockNumber                         | 10     | ✅        |
| klay_call                                | 23     | ✅        |
| klay_chainId                             | 0      | ✅        |
| klay_createAccessList                    | 22     | ✅        |
| klay_estimateGas                         | 67     | ✅        |
| klay_feeHistory                          | 10     | ✅        |
| klay_gasPrice                            | 10     | ✅        |
| klay_getBalance                          | 26     | ✅        |
| klay_getBlockByHash                      | 32     | ✅        |
| klay_getBlockByNumber                    | 31     | ✅        |
| klay_getBlockReceipts                    | 28     | ✅        |
| klay_getBlockTransactionCountByHash      | 14     | ✅        |
| klay_getBlockTransactionCountByNumber    | 13     | ✅        |
| klay_getCode                             | 30     | ✅        |
| klay_getFilterChanges                    | 18     | ✅        |
| klay_getFilterLogs                       | 69     | ✅        |
| klay_getLogs                             | 66     | ✅        |
| klay_getProof                            | 23     | ✅        |
| klay_getStorageAt                        | 26     | ✅        |
| klay_getTransactionByBlockHashAndIndex   | 15     | ✅        |
| klay_getTransactionByBlockNumberAndIndex | 13     | ✅        |
| klay_getTransactionByHash                | 15     | ✅        |
| klay_getTransactionCount                 | 21     | ✅        |
| klay_getTransactionReceipt               | 16     | ✅        |
| klay_maxPriorityFeePerGas                | 23     | ✅        |
| klay_newBlockFilter                      | 18     | ✅        |
| klay_newFilter                           | 18     | ✅        |
| klay_newPendingTransactionFilter         | 18     | ✅        |
| klay_sendRawTransaction                  | 170    | ✅        |
| klay_subscribe                           | 10     | ✅        |
| klay_uninstallFilter                     | 10     | ✅        |
| klay_unsubscribe                         | 10     | ✅        |

### Aptos APIs

#### Aptos Node API

| **Category**     | **Method**                        | **CU** |
| ---------------- | --------------------------------- | ------ |
| **Account**      | Get account                       | 13     |
|                  | Get account resources             | 13     |
|                  | Get account modules               | 13     |
|                  | Get account resource              | 13     |
|                  | Get account module                | 13     |
|                  | Get account balance               | 13     |
| **Blocks**       | Get blocks by height              | 26     |
|                  | Get blocks by version             | 26     |
| **Events**       | Get events by creation number     | 66     |
|                  | Get events by event handle        | 66     |
| **General**      | Get ledger info                   | 13     |
| **Tables**       | Get table item                    | 13     |
|                  | Get raw table item                | 13     |
| **Transactions** | Get transactions                  | 66     |
|                  | Submit transaction                | 13     |
|                  | Get transaction by hash           | 13     |
|                  | Get transaction by version        | 13     |
|                  | Get account transactions          | 66     |
|                  | Get account transaction summaries | 26     |
|                  | Submit batch transactions         | 26     |
|                  | Simulate transaction              | 26     |
|                  | Encode submission                 | 13     |
|                  | Estimate gas price                | 13     |
|                  | Wait For Transaction              | 13     |
| **View**         | Execute view function of a module | 13     |

#### Aptos Indexer API

When using Aptos Indexer API, CU Cost is determined based on the amount of data queried.

| **Bandwidth** | **CU** |
| ------------- | ------ |
| 1 byte        | 0.03   |