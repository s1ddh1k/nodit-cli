# optimism_rollupConfig

**`POST /`**

This method is used to query rollup configuration parameters. The call returns information including various configuration options for the rollup. This information may include values necessary for rollup operation such as genesis block information, batch size, sequencer window size, and channel timeout. This allows developers to understand the current configuration state of the rollup and optimize as needed.

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
| id | integer | ✓ |  |
| jsonrpc | string | ✓ |  |
| method | string | ✓ |  |

### Example

```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "method": "optimism_rollupConfig"
}
```

## Response

| Name | Type | Required | Description |
|------|------|----------|-------------|
| genesis | object |  | Rollup genesis configuration |
| block_time | integer |  | L2 block production interval (seconds) |
| max_sequencer_drift | integer |  | Maximum allowed sequencer drift (seconds) |
| seq_window_size | integer |  | Sequencer window size (number of blocks) |
| channel_timeout | integer |  | Channel timeout (number of blocks) |
| l1_chain_id | string |  | L1 chain ID (hex) |
| l2_chain_id | string |  | L2 chain ID (hex) |
| batch_inbox_address | string |  | L1 batch inbox contract address |
| deposit_contract_address | string |  | L1 deposit contract address |
| l1_system_config_address | string |  | L1 system config contract address |

### Example

```json
{
  "genesis": {
    "l1": {
      "hash": "0x48f520cf4ddaf34c8336e6e490632ea3cf1e5e93b0b2bc6e917557e31845371b",
      "number": 4071408
    },
    "l2": {
      "hash": "0x102de6ffb001480cc9b8b548fd05c34cd4f46ae4aa91759393db90ea0409887d",
      "number": 0
    },
    "l2_time": 1691802540,
    "system_config": {
      "batcherAddr": "0x8f23bb38f531600e5d8fddaaec41f13fab46e98c",
      "overhead": "0x00000000000000000000000000000000000000000000000000000000000000bc",
      "scalar": "0x00000000000000000000000000000000000000000000000000000000000a6fe0",
      "gasLimit": 30000000
    }
  },
  "block_time": 2,
  "max_sequencer_drift": 600,
  "seq_window_size": 3600,
  "channel_timeout": 300,
  "l1_chain_id": 11155111,
  "l2_chain_id": 11155420,
  "regolith_time": 0,
  "canyon_time": 1699981200,
  "batch_inbox_address": "0xff00000000000000000000000000000011155420",
  "deposit_contract_address": "0x16fc5058f25648194471939df75cf27a2fdc48bc",
  "l1_system_config_address": "0x034edd2a225f7f429a63e0f1d2084b9e0a93b538",
  "protocol_versions_address": "0x79add5713b383daa0a138d3c4780c7a1804a8090"
}
```
