mod cli;
mod config;
mod output;

use anyhow::{anyhow, Context, Result};
use axum::{
    body::Bytes,
    extract::State,
    http::{HeaderMap as AxumHeaderMap, StatusCode},
    response::IntoResponse,
    routing::post,
    Router,
};
use clap::Parser;
use futures_util::{SinkExt, StreamExt};
use hmac::{Hmac, Mac};
use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue, ACCEPT, CONTENT_TYPE},
    Client, Method,
};
use serde_json::{json, Map, Value};
use sha2::Sha256;
use std::{net::SocketAddr, path::PathBuf, sync::Arc};
use tokio::{fs::OpenOptions, io::AsyncWriteExt, net::TcpListener};
use tokio_tungstenite::{connect_async, tungstenite::Message};
use url::Url;

use crate::cli::{
    AptosNodeCommand, Args, BitcoinNodeCommand, Command, DataAccountCommand, DataAssetCommand,
    DataBlockCommand, DataBodyArgs, DataCommand, DataEnsCommand, DataNativeCommand, DataNftCommand,
    DataRawArgs, DataStatsCommand, DataTokenCommand, DataTxCommand, EvmNodeCommand,
    GenericNodeCommand, HeaderArg, MultichainCommand, NetworkArgs, NodeCommand, NodeRawArgs,
    OutputFormat, SolanaNodeCommand, StreamCommand, WebhookCommand, WebhookServeArgs,
};
use crate::config::Config;
use crate::output::render_output;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let config = Config::from_env(&args)?;
    let client = Client::builder()
        .user_agent("nodit-cli/0.1.0")
        .build()
        .context("failed to build HTTP client")?;

    let output = match args.command {
        Command::Node(cmd) => handle_node(&client, &config, &args.output, cmd).await?,
        Command::Data(cmd) => handle_data(&client, &config, &args.output, cmd).await?,
        Command::Webhook(cmd) => handle_webhook(&client, &config, &args.output, cmd).await?,
        Command::Stream(cmd) => handle_stream(&config, cmd).await?,
    };

    render_output(&output, args.output)?;
    Ok(())
}

async fn handle_node(
    client: &Client,
    config: &Config,
    output: &OutputFormat,
    cmd: NodeCommand,
) -> Result<Value> {
    match cmd {
        NodeCommand::Evm(cmd) => handle_evm_node(client, config, output, cmd).await,
        NodeCommand::Aptos(cmd) => handle_aptos_node(client, config, output, cmd).await,
        NodeCommand::Solana(cmd) => handle_solana_node(client, config, output, cmd).await,
        NodeCommand::Bitcoin(cmd) => handle_bitcoin_node(client, config, output, cmd).await,
        NodeCommand::Dogecoin(cmd) => handle_generic_node(client, config, output, cmd).await,
        NodeCommand::Xrpl(cmd) => handle_generic_node(client, config, output, cmd).await,
    }
}

async fn handle_generic_node(
    client: &Client,
    config: &Config,
    output: &OutputFormat,
    cmd: GenericNodeCommand,
) -> Result<Value> {
    match cmd {
        GenericNodeCommand::Raw(cmd) => execute_node_rpc(client, config, output, cmd).await,
    }
}

async fn handle_solana_node(
    client: &Client,
    config: &Config,
    output: &OutputFormat,
    cmd: SolanaNodeCommand,
) -> Result<Value> {
    match cmd {
        SolanaNodeCommand::Raw(cmd) => execute_node_rpc(client, config, output, cmd).await,
        SolanaNodeCommand::Slot(args) => {
            execute_node_method(
                client,
                config,
                output,
                &args.target,
                &args.headers,
                "getSlot",
                Value::Array(Vec::new()),
            )
            .await
        }
        SolanaNodeCommand::LatestBlockhash(args) => {
            execute_node_method(
                client,
                config,
                output,
                &args.target,
                &args.headers,
                "getLatestBlockhash",
                Value::Array(Vec::new()),
            )
            .await
        }
        SolanaNodeCommand::BlockHeight(args) => {
            execute_node_method(
                client,
                config,
                output,
                &args.target,
                &args.headers,
                "getBlockHeight",
                Value::Array(Vec::new()),
            )
            .await
        }
        SolanaNodeCommand::EpochInfo(args) => {
            execute_node_method(
                client,
                config,
                output,
                &args.target,
                &args.headers,
                "getEpochInfo",
                Value::Array(Vec::new()),
            )
            .await
        }
        SolanaNodeCommand::Version(args) => {
            execute_node_method(
                client,
                config,
                output,
                &args.target,
                &args.headers,
                "getVersion",
                Value::Array(Vec::new()),
            )
            .await
        }
        SolanaNodeCommand::GenesisHash(args) => {
            execute_node_method(
                client,
                config,
                output,
                &args.target,
                &args.headers,
                "getGenesisHash",
                Value::Array(Vec::new()),
            )
            .await
        }
        SolanaNodeCommand::Health(args) => {
            execute_node_method(
                client,
                config,
                output,
                &args.target,
                &args.headers,
                "getHealth",
                Value::Array(Vec::new()),
            )
            .await
        }
        SolanaNodeCommand::Balance(args) => {
            let mut params = vec![Value::String(args.address)];
            if let Some(commitment) = args.commitment {
                params.push(json!({ "commitment": commitment }));
            }
            execute_node_method(
                client,
                config,
                output,
                &args.target,
                &args.headers,
                "getBalance",
                Value::Array(params),
            )
            .await
        }
        SolanaNodeCommand::AccountInfo(args) => {
            let mut params = vec![Value::String(args.address)];
            if let Some(config_json) = args.config_json {
                params.push(parse_json_required(&config_json)?);
            }
            execute_node_method(
                client,
                config,
                output,
                &args.target,
                &args.headers,
                "getAccountInfo",
                Value::Array(params),
            )
            .await
        }
        SolanaNodeCommand::TokenAccountBalance(args) => {
            let mut params = vec![Value::String(args.address)];
            if let Some(config_json) = args.config_json {
                params.push(parse_json_required(&config_json)?);
            }
            execute_node_method(
                client,
                config,
                output,
                &args.target,
                &args.headers,
                "getTokenAccountBalance",
                Value::Array(params),
            )
            .await
        }
        SolanaNodeCommand::MinimumBalanceForRentExemption(args) => {
            let mut params = vec![json!(args.data_length)];
            if let Some(commitment) = args.commitment {
                params.push(json!({ "commitment": commitment }));
            }
            execute_node_method(
                client,
                config,
                output,
                &args.target,
                &args.headers,
                "getMinimumBalanceForRentExemption",
                Value::Array(params),
            )
            .await
        }
        SolanaNodeCommand::TokenSupply(args) => {
            let mut params = vec![Value::String(args.mint_address)];
            if let Some(commitment) = args.commitment {
                params.push(json!({ "commitment": commitment }));
            }
            execute_node_method(
                client,
                config,
                output,
                &args.target,
                &args.headers,
                "getTokenSupply",
                Value::Array(params),
            )
            .await
        }
        SolanaNodeCommand::Block(args) => {
            let mut params = vec![json!(args.slot)];
            if let Some(config_json) = args.config_json {
                params.push(parse_json_required(&config_json)?);
            }
            execute_node_method(
                client,
                config,
                output,
                &args.target,
                &args.headers,
                "getBlock",
                Value::Array(params),
            )
            .await
        }
        SolanaNodeCommand::Blocks(args) => {
            let mut params = vec![json!(args.start_slot)];
            if let Some(end_slot) = args.end_slot {
                params.push(json!(end_slot));
            }
            if let Some(commitment) = args.commitment {
                params.push(json!({ "commitment": commitment }));
            }
            execute_node_method(
                client,
                config,
                output,
                &args.target,
                &args.headers,
                "getBlocks",
                Value::Array(params),
            )
            .await
        }
        SolanaNodeCommand::Transaction(args) => {
            let mut params = vec![Value::String(args.signature)];
            if let Some(config_json) = args.config_json {
                params.push(parse_json_required(&config_json)?);
            }
            execute_node_method(
                client,
                config,
                output,
                &args.target,
                &args.headers,
                "getTransaction",
                Value::Array(params),
            )
            .await
        }
        SolanaNodeCommand::SignaturesForAddress(args) => {
            let mut params = vec![Value::String(args.address)];
            if let Some(config_json) = args.config_json {
                params.push(parse_json_required(&config_json)?);
            }
            execute_node_method(
                client,
                config,
                output,
                &args.target,
                &args.headers,
                "getSignaturesForAddress",
                Value::Array(params),
            )
            .await
        }
        SolanaNodeCommand::SignatureStatuses(args) => {
            let mut params = vec![Value::Array(
                args.signature.into_iter().map(Value::String).collect(),
            )];
            if let Some(config_json) = args.config_json {
                params.push(parse_json_required(&config_json)?);
            }
            execute_node_method(
                client,
                config,
                output,
                &args.target,
                &args.headers,
                "getSignatureStatuses",
                Value::Array(params),
            )
            .await
        }
    }
}

async fn handle_bitcoin_node(
    client: &Client,
    config: &Config,
    output: &OutputFormat,
    cmd: BitcoinNodeCommand,
) -> Result<Value> {
    if config.rpc_url.is_none() {
        return Err(anyhow!(
            "bitcoin node is not available on the default Nodit Elastic Node endpoint for this CLI flow; use `nodit data ... --protocol bitcoin --network mainnet` or pass --rpc-url for a custom Bitcoin RPC endpoint"
        ));
    }

    match cmd {
        BitcoinNodeCommand::Raw(cmd) => execute_node_rpc(client, config, output, cmd).await,
        BitcoinNodeCommand::BlockCount(args) => {
            execute_node_method(
                client,
                config,
                output,
                &args.target,
                &args.headers,
                "getblockcount",
                Value::Array(Vec::new()),
            )
            .await
        }
        BitcoinNodeCommand::BlockHash(args) => {
            execute_node_method(
                client,
                config,
                output,
                &args.target,
                &args.headers,
                "getblockhash",
                json!([args.height]),
            )
            .await
        }
        BitcoinNodeCommand::Block(args) => {
            let verbosity = args.verbosity.unwrap_or(1);
            execute_node_method(
                client,
                config,
                output,
                &args.target,
                &args.headers,
                "getblock",
                json!([args.hash, verbosity]),
            )
            .await
        }
        BitcoinNodeCommand::Transaction(args) => {
            execute_node_method(
                client,
                config,
                output,
                &args.target,
                &args.headers,
                "getrawtransaction",
                json!([args.txid, args.verbose.unwrap_or(true)]),
            )
            .await
        }
    }
}

async fn handle_evm_node(
    client: &Client,
    config: &Config,
    output: &OutputFormat,
    cmd: EvmNodeCommand,
) -> Result<Value> {
    match cmd {
        EvmNodeCommand::Raw(cmd) => execute_node_rpc(client, config, output, cmd).await,
        EvmNodeCommand::Batch(cmd) => execute_node_batch(client, config, output, cmd).await,
        EvmNodeCommand::BlockNumber(args) => {
            execute_node_method(
                client,
                config,
                output,
                &args.target,
                &args.headers,
                "eth_blockNumber",
                Value::Array(Vec::new()),
            )
            .await
        }
        EvmNodeCommand::ChainId(args) => {
            execute_node_method(
                client,
                config,
                output,
                &args.target,
                &args.headers,
                "eth_chainId",
                Value::Array(Vec::new()),
            )
            .await
        }
        EvmNodeCommand::Balance(args) => {
            execute_node_method(
                client,
                config,
                output,
                &args.target,
                &args.headers,
                "eth_getBalance",
                json!([
                    args.account,
                    args.block_tag.unwrap_or_else(|| "latest".to_string())
                ]),
            )
            .await
        }
        EvmNodeCommand::TransactionCount(args) => {
            execute_node_method(
                client,
                config,
                output,
                &args.target,
                &args.headers,
                "eth_getTransactionCount",
                json!([
                    args.account,
                    args.block_tag.unwrap_or_else(|| "latest".to_string())
                ]),
            )
            .await
        }
        EvmNodeCommand::Code(args) => {
            execute_node_method(
                client,
                config,
                output,
                &args.target,
                &args.headers,
                "eth_getCode",
                json!([
                    args.account,
                    args.block_tag.unwrap_or_else(|| "latest".to_string())
                ]),
            )
            .await
        }
        EvmNodeCommand::Logs(args) => {
            let filter = build_node_logs_filter(args);
            execute_node_method(
                client,
                config,
                output,
                &filter.target,
                &filter.headers,
                "eth_getLogs",
                json!([filter.payload]),
            )
            .await
        }
        EvmNodeCommand::Transaction(args) => {
            execute_node_method(
                client,
                config,
                output,
                &args.target,
                &args.headers,
                "eth_getTransactionByHash",
                json!([args.hash]),
            )
            .await
        }
        EvmNodeCommand::TransactionReceipt(args) => {
            execute_node_method(
                client,
                config,
                output,
                &args.target,
                &args.headers,
                "eth_getTransactionReceipt",
                json!([args.hash]),
            )
            .await
        }
        EvmNodeCommand::GasPrice(args) => {
            execute_node_method(
                client,
                config,
                output,
                &args.target,
                &args.headers,
                "eth_gasPrice",
                Value::Array(Vec::new()),
            )
            .await
        }
        EvmNodeCommand::Call(args) => {
            let mut call = Map::new();
            call.insert("to".to_string(), Value::String(args.to));
            call.insert("data".to_string(), Value::String(args.data));
            if let Some(from) = args.from {
                call.insert("from".to_string(), Value::String(from));
            }
            if let Some(value) = args.value {
                call.insert("value".to_string(), Value::String(value));
            }
            execute_node_method(
                client,
                config,
                output,
                &args.target,
                &args.headers,
                "eth_call",
                json!([
                    Value::Object(call),
                    args.block_tag.unwrap_or_else(|| "latest".to_string())
                ]),
            )
            .await
        }
    }
}

async fn execute_node_rpc(
    client: &Client,
    config: &Config,
    output: &OutputFormat,
    cmd: NodeRawArgs,
) -> Result<Value> {
    let url = match cmd.url {
        Some(url) => url,
        None => build_rpc_url(config, &cmd.target)?,
    };
    let params = parse_optional_json_array(cmd.params.as_deref())?;
    let body = json!({
        "jsonrpc": "2.0",
        "id": cmd.id,
        "method": cmd.method,
        "params": params,
    });

    execute_json_request(
        client,
        Method::POST,
        &url,
        build_headers(&config.api_key, &cmd.headers)?,
        Some(body),
        output,
    )
    .await
}

async fn execute_node_batch(
    client: &Client,
    config: &Config,
    output: &OutputFormat,
    cmd: crate::cli::NodeBatchArgs,
) -> Result<Value> {
    let url = match cmd.url {
        Some(url) => url,
        None => build_rpc_url(config, &cmd.target)?,
    };
    let body = parse_json_required(&cmd.body)?;
    if !body.is_array() {
        return Err(anyhow!("node batch body must be a JSON array"));
    }

    execute_json_request(
        client,
        Method::POST,
        &url,
        build_headers(&config.api_key, &cmd.headers)?,
        Some(body),
        output,
    )
    .await
}

struct NodeLogsFilter {
    target: NetworkArgs,
    headers: Vec<HeaderArg>,
    payload: Value,
}

fn build_node_logs_filter(args: crate::cli::NodeLogsArgs) -> NodeLogsFilter {
    let mut filter = Map::new();
    if let Some(from_block) = args.from_block {
        filter.insert("fromBlock".to_string(), Value::String(from_block));
    }
    if let Some(to_block) = args.to_block {
        filter.insert("toBlock".to_string(), Value::String(to_block));
    }
    if let Some(address) = args.address {
        filter.insert("address".to_string(), Value::String(address));
    }
    if !args.topic.is_empty() {
        filter.insert(
            "topics".to_string(),
            Value::Array(args.topic.into_iter().map(Value::String).collect()),
        );
    }

    NodeLogsFilter {
        target: args.target,
        headers: args.headers,
        payload: Value::Object(filter),
    }
}

async fn execute_node_method(
    client: &Client,
    config: &Config,
    output: &OutputFormat,
    target: &NetworkArgs,
    headers: &[HeaderArg],
    method: &str,
    params: Value,
) -> Result<Value> {
    let url = build_rpc_url(config, target)?;
    let body = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": method,
        "params": params,
    });

    execute_json_request(
        client,
        Method::POST,
        &url,
        build_headers(&config.api_key, headers)?,
        Some(body),
        output,
    )
    .await
}

async fn handle_data(
    client: &Client,
    config: &Config,
    output: &OutputFormat,
    cmd: DataCommand,
) -> Result<Value> {
    match cmd {
        DataCommand::Native(cmd) => handle_data_native(client, config, output, cmd).await,
        DataCommand::Account(cmd) => handle_data_account(client, config, output, cmd).await,
        DataCommand::Tx(cmd) => handle_data_tx(client, config, output, cmd).await,
        DataCommand::Block(cmd) => handle_data_block(client, config, output, cmd).await,
        DataCommand::Token(cmd) => handle_data_token(client, config, output, cmd).await,
        DataCommand::Nft(cmd) => handle_data_nft(client, config, output, cmd).await,
        DataCommand::Ens(cmd) => handle_data_ens(client, config, output, cmd).await,
        DataCommand::Stats(cmd) => handle_data_stats(client, config, output, cmd).await,
        DataCommand::Asset(cmd) => handle_data_asset(client, config, output, cmd).await,
        DataCommand::Multichain(cmd) => handle_multichain(client, config, output, cmd).await,
        DataCommand::Raw(args) => handle_data_raw(client, config, output, args).await,
    }
}

async fn handle_data_native(
    client: &Client,
    config: &Config,
    output: &OutputFormat,
    cmd: DataNativeCommand,
) -> Result<Value> {
    match cmd {
        DataNativeCommand::Balance(args) => {
            let body = merge_json_objects(
                json!({ "accountAddress": args.account }),
                args.body.as_deref(),
            )?;
            execute_data_action(
                client,
                config,
                output,
                DataActionRequest {
                    target: &args.target,
                    category: "native",
                    action: "getNativeBalanceByAccount",
                    body,
                    headers: &args.headers,
                },
            )
            .await
        }
        DataNativeCommand::TokenBalance(args) => {
            let body = merge_json_objects(
                json!({ "accountAddress": args.account }),
                args.body.as_deref(),
            )?;
            execute_data_action(
                client,
                config,
                output,
                DataActionRequest {
                    target: &args.target,
                    category: "token",
                    action: "getNativeTokenBalanceByAccount",
                    body,
                    headers: &args.headers,
                },
            )
            .await
        }
        DataNativeCommand::TransfersByAccount(args) => {
            let body = merge_json_objects(
                json!({ "accountAddress": args.account }),
                args.body.as_deref(),
            )?;
            execute_data_action(
                client,
                config,
                output,
                DataActionRequest {
                    target: &args.target,
                    category: "native",
                    action: "getNativeTransfersByAccount",
                    body,
                    headers: &args.headers,
                },
            )
            .await
        }
        DataNativeCommand::TransfersWithinRange(args) => {
            let body = merge_json_objects(
                json!({ "fromBlock": args.from_block, "toBlock": args.to_block }),
                args.body.as_deref(),
            )?;
            execute_data_action(
                client,
                config,
                output,
                DataActionRequest {
                    target: &args.target,
                    category: "native",
                    action: "getNativeTransfersWithinRange",
                    body,
                    headers: &args.headers,
                },
            )
            .await
        }
        DataNativeCommand::Holders(args) => {
            let body = optional_json_object(args.body.as_deref())?;
            execute_data_action(
                client,
                config,
                output,
                DataActionRequest {
                    target: &args.target,
                    category: "native",
                    action: "getNativeHolders",
                    body,
                    headers: &args.headers,
                },
            )
            .await
        }
    }
}

async fn handle_data_account(
    client: &Client,
    config: &Config,
    output: &OutputFormat,
    cmd: DataAccountCommand,
) -> Result<Value> {
    match cmd {
        DataAccountCommand::TotalTransactionCount(args) => {
            let body = merge_json_objects(
                json!({ "accountAddress": args.account }),
                args.body.as_deref(),
            )?;
            execute_data_action(
                client,
                config,
                output,
                DataActionRequest {
                    target: &args.target,
                    category: "blockchain",
                    action: "getTotalTransactionCountByAccount",
                    body,
                    headers: &args.headers,
                },
            )
            .await
        }
        DataAccountCommand::InternalTransactions(args) => {
            let body = merge_json_objects(
                json!({ "accountAddress": args.account }),
                args.body.as_deref(),
            )?;
            execute_data_action(
                client,
                config,
                output,
                DataActionRequest {
                    target: &args.target,
                    category: "blockchain",
                    action: "getInternalTransactionsByAccount",
                    body,
                    headers: &args.headers,
                },
            )
            .await
        }
        DataAccountCommand::NextNonce(args) => {
            let body = merge_json_objects(
                json!({ "accountAddress": args.account }),
                args.body.as_deref(),
            )?;
            execute_data_action(
                client,
                config,
                output,
                DataActionRequest {
                    target: &args.target,
                    category: "blockchain",
                    action: "getNextNonceByAccount",
                    body,
                    headers: &args.headers,
                },
            )
            .await
        }
        DataAccountCommand::IsContract(args) => {
            let body = merge_json_objects(
                json!({ "accountAddress": args.account }),
                args.body.as_deref(),
            )?;
            execute_data_action(
                client,
                config,
                output,
                DataActionRequest {
                    target: &args.target,
                    category: "blockchain",
                    action: "isContract",
                    body,
                    headers: &args.headers,
                },
            )
            .await
        }
        DataAccountCommand::UnspentTransactionOutputs(args) => {
            let body = merge_json_objects(
                json!({ "accountAddress": args.account }),
                args.body.as_deref(),
            )?;
            execute_data_action(
                client,
                config,
                output,
                DataActionRequest {
                    target: &args.target,
                    category: "blockchain",
                    action: "getUnspentTransactionOutputsByAccount",
                    body,
                    headers: &args.headers,
                },
            )
            .await
        }
    }
}

async fn handle_data_tx(
    client: &Client,
    config: &Config,
    output: &OutputFormat,
    cmd: DataTxCommand,
) -> Result<Value> {
    match cmd {
        DataTxCommand::ByHash(args) => {
            let body = merge_json_objects(
                json!({ "transactionHash": args.hash }),
                args.body.as_deref(),
            )?;
            execute_data_action(
                client,
                config,
                output,
                DataActionRequest {
                    target: &args.target,
                    category: "blockchain",
                    action: "getTransactionByHash",
                    body,
                    headers: &args.headers,
                },
            )
            .await
        }
        DataTxCommand::ByTransactionId(args) => {
            let body = merge_json_objects(
                json!({ "transactionId": args.transaction_id }),
                args.body.as_deref(),
            )?;
            execute_data_action(
                client,
                config,
                output,
                DataActionRequest {
                    target: &args.target,
                    category: "blockchain",
                    action: "getTransactionByTransactionId",
                    body,
                    headers: &args.headers,
                },
            )
            .await
        }
        DataTxCommand::ByAccount(args) => {
            let body = merge_json_objects(
                json!({ "accountAddress": args.account }),
                args.body.as_deref(),
            )?;
            execute_data_action(
                client,
                config,
                output,
                DataActionRequest {
                    target: &args.target,
                    category: "blockchain",
                    action: "getTransactionsByAccount",
                    body,
                    headers: &args.headers,
                },
            )
            .await
        }
        DataTxCommand::InBlock(args) => {
            let body = merge_json_objects(
                json!({ "blockHashOrNumber": args.block }),
                args.body.as_deref(),
            )?;
            execute_data_action(
                client,
                config,
                output,
                DataActionRequest {
                    target: &args.target,
                    category: "blockchain",
                    action: "getTransactionsInBlock",
                    body,
                    headers: &args.headers,
                },
            )
            .await
        }
        DataTxCommand::ByHashes(args) => {
            let body = merge_json_objects(
                json!({ "transactionHashes": args.hash }),
                args.body.as_deref(),
            )?;
            execute_data_action(
                client,
                config,
                output,
                DataActionRequest {
                    target: &args.target,
                    category: "blockchain",
                    action: "getTransactionsByHashes",
                    body,
                    headers: &args.headers,
                },
            )
            .await
        }
        DataTxCommand::ByTransactionIds(args) => {
            let body = merge_json_objects(
                json!({ "transactionIds": args.transaction_id }),
                args.body.as_deref(),
            )?;
            execute_data_action(
                client,
                config,
                output,
                DataActionRequest {
                    target: &args.target,
                    category: "blockchain",
                    action: "getTransactionsByTransactionIds",
                    body,
                    headers: &args.headers,
                },
            )
            .await
        }
        DataTxCommand::InternalByTransactionHash(args) => {
            let body = merge_json_objects(
                json!({ "transactionHash": args.hash }),
                args.body.as_deref(),
            )?;
            execute_data_action(
                client,
                config,
                output,
                DataActionRequest {
                    target: &args.target,
                    category: "blockchain",
                    action: "getInternalTransactionsByTransactionHash",
                    body,
                    headers: &args.headers,
                },
            )
            .await
        }
        DataTxCommand::SearchEvents(args) => {
            execute_data_body_action(client, config, output, &args, "blockchain", "searchEvents")
                .await
        }
    }
}

async fn handle_data_token(
    client: &Client,
    config: &Config,
    output: &OutputFormat,
    cmd: DataTokenCommand,
) -> Result<Value> {
    match cmd {
        DataTokenCommand::OwnedByAccount(args) => {
            let body = merge_json_objects(
                json!({ "accountAddress": args.account }),
                args.body.as_deref(),
            )?;
            execute_data_action(
                client,
                config,
                output,
                DataActionRequest {
                    target: &args.target,
                    category: "token",
                    action: "getTokensOwnedByAccount",
                    body,
                    headers: &args.headers,
                },
            )
            .await
        }
        DataTokenCommand::Allowance(args) => {
            let body = merge_json_objects(
                json!({
                    "contractAddress": args.contract,
                    "ownerAddress": args.owner,
                    "spenderAddress": args.spender
                }),
                args.body.as_deref(),
            )?;
            execute_data_action(
                client,
                config,
                output,
                DataActionRequest {
                    target: &args.target,
                    category: "token",
                    action: "getTokenAllowance",
                    body,
                    headers: &args.headers,
                },
            )
            .await
        }
        DataTokenCommand::ContractMetadataByContracts(args) => {
            let body = merge_json_objects(
                json!({ "contractAddresses": args.contract }),
                args.body.as_deref(),
            )?;
            execute_data_action(
                client,
                config,
                output,
                DataActionRequest {
                    target: &args.target,
                    category: "token",
                    action: "getTokenContractMetadataByContracts",
                    body,
                    headers: &args.headers,
                },
            )
            .await
        }
        DataTokenCommand::HoldersByContract(args) => {
            let body = merge_json_objects(
                json!({ "contractAddress": args.contract }),
                args.body.as_deref(),
            )?;
            execute_data_action(
                client,
                config,
                output,
                DataActionRequest {
                    target: &args.target,
                    category: "token",
                    action: "getTokenHoldersByContract",
                    body,
                    headers: &args.headers,
                },
            )
            .await
        }
        DataTokenCommand::PricesByContracts(args) => {
            let body = merge_json_objects(
                json!({ "contractAddresses": args.contract }),
                args.body.as_deref(),
            )?;
            execute_data_action(
                client,
                config,
                output,
                DataActionRequest {
                    target: &args.target,
                    category: "token",
                    action: "getTokenPricesByContracts",
                    body,
                    headers: &args.headers,
                },
            )
            .await
        }
        DataTokenCommand::TransfersByContract(args) => {
            let body = merge_json_objects(
                json!({ "contractAddress": args.contract }),
                args.body.as_deref(),
            )?;
            execute_data_action(
                client,
                config,
                output,
                DataActionRequest {
                    target: &args.target,
                    category: "token",
                    action: "getTokenTransfersByContract",
                    body,
                    headers: &args.headers,
                },
            )
            .await
        }
        DataTokenCommand::TransfersWithinRange(args) => {
            let body = merge_json_objects(
                json!({ "fromBlock": args.from_block, "toBlock": args.to_block }),
                args.body.as_deref(),
            )?;
            execute_data_action(
                client,
                config,
                output,
                DataActionRequest {
                    target: &args.target,
                    category: "token",
                    action: "getTokenTransfersWithinRange",
                    body,
                    headers: &args.headers,
                },
            )
            .await
        }
        DataTokenCommand::TransfersByAccount(args) => {
            let body = merge_json_objects(
                json!({ "accountAddress": args.account }),
                args.body.as_deref(),
            )?;
            execute_data_action(
                client,
                config,
                output,
                DataActionRequest {
                    target: &args.target,
                    category: "token",
                    action: "getTokenTransfersByAccount",
                    body,
                    headers: &args.headers,
                },
            )
            .await
        }
        DataTokenCommand::SearchContractMetadata(args) => {
            let body =
                merge_json_objects(json!({ "keyword": args.keyword }), args.body.as_deref())?;
            execute_data_action(
                client,
                config,
                output,
                DataActionRequest {
                    target: &args.target,
                    category: "token",
                    action: "searchTokenContractMetadataByKeyword",
                    body,
                    headers: &args.headers,
                },
            )
            .await
        }
    }
}

async fn handle_data_nft(
    client: &Client,
    config: &Config,
    output: &OutputFormat,
    cmd: DataNftCommand,
) -> Result<Value> {
    match cmd {
        DataNftCommand::OwnedByAccount(args) => {
            let body = merge_json_objects(
                json!({ "accountAddress": args.account }),
                args.body.as_deref(),
            )?;
            execute_data_action(
                client,
                config,
                output,
                DataActionRequest {
                    target: &args.target,
                    category: "nft",
                    action: "getNftsOwnedByAccount",
                    body,
                    headers: &args.headers,
                },
            )
            .await
        }
        DataNftCommand::ContractsByAccount(args) => {
            let body = merge_json_objects(
                json!({ "accountAddress": args.account }),
                args.body.as_deref(),
            )?;
            execute_data_action(
                client,
                config,
                output,
                DataActionRequest {
                    target: &args.target,
                    category: "nft",
                    action: "getNftContractsByAccount",
                    body,
                    headers: &args.headers,
                },
            )
            .await
        }
        DataNftCommand::ContractMetadataByContracts(args) => {
            let body = merge_json_objects(
                json!({ "contractAddresses": args.contract }),
                args.body.as_deref(),
            )?;
            execute_data_action(
                client,
                config,
                output,
                DataActionRequest {
                    target: &args.target,
                    category: "nft",
                    action: "getNftContractMetadataByContracts",
                    body,
                    headers: &args.headers,
                },
            )
            .await
        }
        DataNftCommand::HoldersByContract(args) => {
            let body = merge_json_objects(
                json!({ "contractAddress": args.contract }),
                args.body.as_deref(),
            )?;
            execute_data_action(
                client,
                config,
                output,
                DataActionRequest {
                    target: &args.target,
                    category: "nft",
                    action: "getNftHoldersByContract",
                    body,
                    headers: &args.headers,
                },
            )
            .await
        }
        DataNftCommand::HoldersByTokenId(args) => {
            let body = merge_json_objects(
                json!({ "contractAddress": args.contract, "tokenId": args.token_id }),
                args.body.as_deref(),
            )?;
            execute_data_action(
                client,
                config,
                output,
                DataActionRequest {
                    target: &args.target,
                    category: "nft",
                    action: "getNftHoldersByTokenId",
                    body,
                    headers: &args.headers,
                },
            )
            .await
        }
        DataNftCommand::MetadataByContract(args) => {
            let body = merge_json_objects(
                json!({ "contractAddress": args.contract }),
                args.body.as_deref(),
            )?;
            execute_data_action(
                client,
                config,
                output,
                DataActionRequest {
                    target: &args.target,
                    category: "nft",
                    action: "getNftMetadataByContract",
                    body,
                    headers: &args.headers,
                },
            )
            .await
        }
        DataNftCommand::MetadataByTokenIds(args) => {
            let body = merge_json_objects(
                json!({ "contractAddress": args.contract, "tokenIds": args.token_id }),
                args.body.as_deref(),
            )?;
            execute_data_action(
                client,
                config,
                output,
                DataActionRequest {
                    target: &args.target,
                    category: "nft",
                    action: "getNftMetadataByTokenIds",
                    body,
                    headers: &args.headers,
                },
            )
            .await
        }
        DataNftCommand::TransfersByContract(args) => {
            let body = merge_json_objects(
                json!({ "contractAddress": args.contract }),
                args.body.as_deref(),
            )?;
            execute_data_action(
                client,
                config,
                output,
                DataActionRequest {
                    target: &args.target,
                    category: "nft",
                    action: "getNftTransfersByContract",
                    body,
                    headers: &args.headers,
                },
            )
            .await
        }
        DataNftCommand::TransfersByTokenId(args) => {
            let body = merge_json_objects(
                json!({ "contractAddress": args.contract, "tokenId": args.token_id }),
                args.body.as_deref(),
            )?;
            execute_data_action(
                client,
                config,
                output,
                DataActionRequest {
                    target: &args.target,
                    category: "nft",
                    action: "getNftTransfersByTokenId",
                    body,
                    headers: &args.headers,
                },
            )
            .await
        }
        DataNftCommand::TransfersWithinRange(args) => {
            let body = merge_json_objects(
                json!({ "fromBlock": args.from_block, "toBlock": args.to_block }),
                args.body.as_deref(),
            )?;
            execute_data_action(
                client,
                config,
                output,
                DataActionRequest {
                    target: &args.target,
                    category: "nft",
                    action: "getNftTransfersWithinRange",
                    body,
                    headers: &args.headers,
                },
            )
            .await
        }
        DataNftCommand::TransfersByAccount(args) => {
            let body = merge_json_objects(
                json!({ "accountAddress": args.account }),
                args.body.as_deref(),
            )?;
            execute_data_action(
                client,
                config,
                output,
                DataActionRequest {
                    target: &args.target,
                    category: "nft",
                    action: "getNftTransfersByAccount",
                    body,
                    headers: &args.headers,
                },
            )
            .await
        }
        DataNftCommand::SearchContractMetadata(args) => {
            let body =
                merge_json_objects(json!({ "keyword": args.keyword }), args.body.as_deref())?;
            execute_data_action(
                client,
                config,
                output,
                DataActionRequest {
                    target: &args.target,
                    category: "nft",
                    action: "searchNftContractMetadataByKeyword",
                    body,
                    headers: &args.headers,
                },
            )
            .await
        }
        DataNftCommand::SyncMetadata(args) => {
            let body = merge_json_objects(
                json!({ "contractAddress": args.contract, "tokenId": args.token_id }),
                args.body.as_deref(),
            )?;
            execute_data_action(
                client,
                config,
                output,
                DataActionRequest {
                    target: &args.target,
                    category: "nft",
                    action: "syncNftMetadata",
                    body,
                    headers: &args.headers,
                },
            )
            .await
        }
    }
}

async fn handle_data_ens(
    client: &Client,
    config: &Config,
    output: &OutputFormat,
    cmd: DataEnsCommand,
) -> Result<Value> {
    match cmd {
        DataEnsCommand::AddressByName(args) => {
            let body = merge_json_objects(json!({ "ensName": args.name }), args.body.as_deref())?;
            execute_data_action(
                client,
                config,
                output,
                DataActionRequest {
                    target: &args.target,
                    category: "ens",
                    action: "getAddressByEnsName",
                    body,
                    headers: &args.headers,
                },
            )
            .await
        }
        DataEnsCommand::NameByAddress(args) => {
            let body =
                merge_json_objects(json!({ "address": args.address }), args.body.as_deref())?;
            execute_data_action(
                client,
                config,
                output,
                DataActionRequest {
                    target: &args.target,
                    category: "ens",
                    action: "getEnsNameByAddress",
                    body,
                    headers: &args.headers,
                },
            )
            .await
        }
        DataEnsCommand::RecordByName(args) => {
            let body = merge_json_objects(json!({ "ensName": args.name }), args.body.as_deref())?;
            execute_data_action(
                client,
                config,
                output,
                DataActionRequest {
                    target: &args.target,
                    category: "ens",
                    action: "getEnsRecordByName",
                    body,
                    headers: &args.headers,
                },
            )
            .await
        }
        DataEnsCommand::RecordsByAccount(args) => {
            let mut base = Map::new();
            if let Some(owner_address) = args.owner_address {
                base.insert("ownerAddress".to_string(), Value::String(owner_address));
            }
            if let Some(resolved_address) = args.resolved_address {
                base.insert(
                    "resolvedAddress".to_string(),
                    Value::String(resolved_address),
                );
            }
            if base.is_empty() && args.body.is_none() {
                return Err(anyhow!(
                    "pass --owner-address, --resolved-address, or --body for ens records"
                ));
            }
            let body = merge_json_objects(Value::Object(base), args.body.as_deref())?;
            execute_data_action(
                client,
                config,
                output,
                DataActionRequest {
                    target: &args.target,
                    category: "ens",
                    action: "getEnsRecordsByAccount",
                    body,
                    headers: &args.headers,
                },
            )
            .await
        }
    }
}

async fn handle_data_stats(
    client: &Client,
    config: &Config,
    output: &OutputFormat,
    cmd: DataStatsCommand,
) -> Result<Value> {
    match cmd {
        DataStatsCommand::Account(args) => {
            let body =
                merge_json_objects(json!({ "address": args.account }), args.body.as_deref())?;
            execute_data_action(
                client,
                config,
                output,
                DataActionRequest {
                    target: &args.target,
                    category: "stats",
                    action: "getAccountStats",
                    body,
                    headers: &args.headers,
                },
            )
            .await
        }
        DataStatsCommand::DailyActiveAccounts(args) => {
            execute_data_body_action(
                client,
                config,
                output,
                &args,
                "stats",
                "getDailyActiveAccountsStats",
            )
            .await
        }
        DataStatsCommand::DailyActiveAccountsByContract(args) => {
            execute_data_body_action(
                client,
                config,
                output,
                &args,
                "stats",
                "getDailyActiveAccountsStatsByContract",
            )
            .await
        }
        DataStatsCommand::DailyTransactions(args) => {
            execute_data_body_action(
                client,
                config,
                output,
                &args,
                "stats",
                "getDailyTransactionsStats",
            )
            .await
        }
        DataStatsCommand::DailyTransactionsByContract(args) => {
            execute_data_body_action(
                client,
                config,
                output,
                &args,
                "stats",
                "getDailyTransactionsStatsByContract",
            )
            .await
        }
        DataStatsCommand::HourlyActiveAccounts(args) => {
            execute_data_body_action(
                client,
                config,
                output,
                &args,
                "stats",
                "getHourlyActiveAccountsStats",
            )
            .await
        }
        DataStatsCommand::HourlyActiveAccountsByContract(args) => {
            execute_data_body_action(
                client,
                config,
                output,
                &args,
                "stats",
                "getHourlyActiveAccountsStatsByContract",
            )
            .await
        }
        DataStatsCommand::HourlyTransactions(args) => {
            execute_data_body_action(
                client,
                config,
                output,
                &args,
                "stats",
                "getHourlyTransactionsStats",
            )
            .await
        }
        DataStatsCommand::HourlyTransactionsByContract(args) => {
            execute_data_body_action(
                client,
                config,
                output,
                &args,
                "stats",
                "getHourlyTransactionsStatsByContract",
            )
            .await
        }
    }
}

async fn handle_data_asset(
    client: &Client,
    config: &Config,
    output: &OutputFormat,
    cmd: DataAssetCommand,
) -> Result<Value> {
    match cmd {
        DataAssetCommand::MetadataByIssuer(args) => {
            let body = merge_json_objects(
                json!({ "issuerAddress": args.issuer_address }),
                args.body.as_deref(),
            )?;
            execute_data_action(
                client,
                config,
                output,
                DataActionRequest {
                    target: &args.target,
                    category: "asset",
                    action: "getAssetMetadataByIssuer",
                    body,
                    headers: &args.headers,
                },
            )
            .await
        }
        DataAssetCommand::MetadataByIds(args) => {
            let body =
                merge_json_objects(json!({ "assetIds": args.asset_id }), args.body.as_deref())?;
            execute_data_action(
                client,
                config,
                output,
                DataActionRequest {
                    target: &args.target,
                    category: "asset",
                    action: "getAssetMetadataByIds",
                    body,
                    headers: &args.headers,
                },
            )
            .await
        }
        DataAssetCommand::SearchMetadata(args) => {
            let body =
                merge_json_objects(json!({ "keyword": args.keyword }), args.body.as_deref())?;
            execute_data_action(
                client,
                config,
                output,
                DataActionRequest {
                    target: &args.target,
                    category: "asset",
                    action: "searchAssetMetadataByKeyword",
                    body,
                    headers: &args.headers,
                },
            )
            .await
        }
        DataAssetCommand::HoldersById(args) => {
            let body =
                merge_json_objects(json!({ "assetId": args.asset_id }), args.body.as_deref())?;
            execute_data_action(
                client,
                config,
                output,
                DataActionRequest {
                    target: &args.target,
                    category: "asset",
                    action: "getAssetHoldersById",
                    body,
                    headers: &args.headers,
                },
            )
            .await
        }
        DataAssetCommand::TransfersById(args) => {
            let body =
                merge_json_objects(json!({ "assetId": args.asset_id }), args.body.as_deref())?;
            execute_data_action(
                client,
                config,
                output,
                DataActionRequest {
                    target: &args.target,
                    category: "asset",
                    action: "getAssetTransfersById",
                    body,
                    headers: &args.headers,
                },
            )
            .await
        }
        DataAssetCommand::TransfersWithinRange(args) => {
            let body = merge_json_objects(
                json!({ "fromBlock": args.from_block, "toBlock": args.to_block }),
                args.body.as_deref(),
            )?;
            execute_data_action(
                client,
                config,
                output,
                DataActionRequest {
                    target: &args.target,
                    category: "asset",
                    action: "getAssetTransfersWithinRange",
                    body,
                    headers: &args.headers,
                },
            )
            .await
        }
        DataAssetCommand::TransfersByAccount(args) => {
            let body = merge_json_objects(
                json!({ "accountAddress": args.account }),
                args.body.as_deref(),
            )?;
            execute_data_action(
                client,
                config,
                output,
                DataActionRequest {
                    target: &args.target,
                    category: "asset",
                    action: "getAssetTransfersByAccount",
                    body,
                    headers: &args.headers,
                },
            )
            .await
        }
        DataAssetCommand::OwnedByAccount(args) => {
            let body = merge_json_objects(
                json!({ "accountAddress": args.account }),
                args.body.as_deref(),
            )?;
            execute_data_action(
                client,
                config,
                output,
                DataActionRequest {
                    target: &args.target,
                    category: "asset",
                    action: "getAssetsOwnedByAccount",
                    body,
                    headers: &args.headers,
                },
            )
            .await
        }
    }
}

async fn handle_data_block(
    client: &Client,
    config: &Config,
    output: &OutputFormat,
    cmd: DataBlockCommand,
) -> Result<Value> {
    match cmd {
        DataBlockCommand::ByNumber(args) => {
            let body =
                merge_json_objects(json!({ "blockNumber": args.number }), args.body.as_deref())?;
            execute_data_action(
                client,
                config,
                output,
                DataActionRequest {
                    target: &args.target,
                    category: "blockchain",
                    action: "getBlockByNumber",
                    body,
                    headers: &args.headers,
                },
            )
            .await
        }
        DataBlockCommand::ByHash(args) => {
            let body = merge_json_objects(json!({ "blockHash": args.hash }), args.body.as_deref())?;
            execute_data_action(
                client,
                config,
                output,
                DataActionRequest {
                    target: &args.target,
                    category: "blockchain",
                    action: "getBlockByHash",
                    body,
                    headers: &args.headers,
                },
            )
            .await
        }
        DataBlockCommand::ByHashOrNumber(args) => {
            let body = merge_json_objects(json!({ "block": args.block }), args.body.as_deref())?;
            execute_data_action(
                client,
                config,
                output,
                DataActionRequest {
                    target: &args.target,
                    category: "blockchain",
                    action: "getBlockByHashOrNumber",
                    body,
                    headers: &args.headers,
                },
            )
            .await
        }
        DataBlockCommand::WithinRange(args) => {
            let body = merge_json_objects(
                json!({ "fromBlock": args.from_block, "toBlock": args.to_block }),
                args.body.as_deref(),
            )?;
            execute_data_action(
                client,
                config,
                output,
                DataActionRequest {
                    target: &args.target,
                    category: "blockchain",
                    action: "getBlocksWithinRange",
                    body,
                    headers: &args.headers,
                },
            )
            .await
        }
        DataBlockCommand::GasPrice(args) => {
            let body = optional_json_object(args.body.as_deref())?;
            execute_data_action(
                client,
                config,
                output,
                DataActionRequest {
                    target: &args.target,
                    category: "blockchain",
                    action: "getGasPrice",
                    body,
                    headers: &args.headers,
                },
            )
            .await
        }
    }
}

async fn handle_multichain(
    client: &Client,
    config: &Config,
    output: &OutputFormat,
    cmd: MultichainCommand,
) -> Result<Value> {
    match cmd {
        MultichainCommand::LookupEntities(args) => {
            let url = join_url(&config.api_base_url, "/v1/multichain/lookupEntities")?;
            execute_json_request(
                client,
                Method::POST,
                &url,
                build_headers(&config.api_key, &args.headers)?,
                Some(parse_json_required(&args.body)?),
                output,
            )
            .await
        }
    }
}

async fn handle_data_raw(
    client: &Client,
    config: &Config,
    output: &OutputFormat,
    args: DataRawArgs,
) -> Result<Value> {
    let path = build_web3_path(&args.target, args.category.as_deref(), &args.action);
    let url = join_url(&config.api_base_url, &path)?;
    execute_json_request(
        client,
        args.method.into(),
        &url,
        build_headers(&config.api_key, &args.headers)?,
        maybe_parse_json(args.body.as_deref())?,
        output,
    )
    .await
}

async fn handle_webhook(
    client: &Client,
    config: &Config,
    output: &OutputFormat,
    cmd: WebhookCommand,
) -> Result<Value> {
    match cmd {
        WebhookCommand::List(args) => {
            let url = join_url(
                &config.api_base_url,
                &build_webhook_path(&args.target, None),
            )?;
            execute_json_request(
                client,
                Method::GET,
                &url,
                build_headers(&config.api_key, &args.headers)?,
                None,
                output,
            )
            .await
        }
        WebhookCommand::Get(args) => {
            let url = append_query(
                join_url(
                    &config.api_base_url,
                    &build_webhook_path(&args.target, None),
                )?,
                &[("subscriptionId", args.subscription_id)],
            )?;
            execute_json_request(
                client,
                Method::GET,
                &url,
                build_headers(&config.api_key, &args.headers)?,
                None,
                output,
            )
            .await
        }
        WebhookCommand::Create(args) => {
            let url = join_url(
                &config.api_base_url,
                &build_webhook_path(&args.target, None),
            )?;
            let body = build_webhook_create_body(&args)?;
            execute_json_request(
                client,
                Method::POST,
                &url,
                build_headers(&config.api_key, &args.headers)?,
                Some(body),
                output,
            )
            .await
        }
        WebhookCommand::Update(args) => {
            let url = join_url(
                &config.api_base_url,
                &build_webhook_path(&args.target, Some(&args.subscription_id)),
            )?;
            let body = build_webhook_update_body(&args)?;
            execute_json_request(
                client,
                Method::PATCH,
                &url,
                build_headers(&config.api_key, &args.headers)?,
                Some(body),
                output,
            )
            .await
        }
        WebhookCommand::Delete(args) => {
            let url = join_url(
                &config.api_base_url,
                &build_webhook_path(&args.target, Some(&args.subscription_id)),
            )?;
            execute_json_request(
                client,
                Method::DELETE,
                &url,
                build_headers(&config.api_key, &args.headers)?,
                None,
                output,
            )
            .await
        }
        WebhookCommand::History(args) => {
            let mut url = join_url(
                &config.api_base_url,
                &format!(
                    "/v1/{}/{}/webhooks/history",
                    args.target.protocol, args.target.network
                ),
            )?;
            if let Some(subscription_id) = args.subscription_id {
                url = append_query(url, &[("subscriptionId", subscription_id)])?;
            }
            execute_json_request(
                client,
                Method::GET,
                &url,
                build_headers(&config.api_key, &args.headers)?,
                maybe_parse_json(args.body.as_deref())?,
                output,
            )
            .await
        }
        WebhookCommand::Serve(args) => serve_webhook(args).await,
        WebhookCommand::Raw(args) => {
            let path = if args.path.starts_with('/') {
                args.path
            } else {
                format!(
                    "/v1/{}/{}/{}",
                    args.target.protocol, args.target.network, args.path
                )
            };
            let url = join_url(&config.api_base_url, &path)?;
            execute_json_request(
                client,
                args.method.into(),
                &url,
                build_headers(&config.api_key, &args.headers)?,
                maybe_parse_json(args.body.as_deref())?,
                output,
            )
            .await
        }
    }
}

async fn serve_webhook(args: WebhookServeArgs) -> Result<Value> {
    let path = normalize_route_path(&args.path);
    let state = Arc::new(WebhookServerState {
        output_file: args.output_file.map(PathBuf::from),
        print_body: args.print_body,
        signing_key: args.signing_key,
    });

    let app = Router::new()
        .route(&path, post(webhook_ingest))
        .with_state(state);

    let addr: SocketAddr = format!("{}:{}", args.host, args.port)
        .parse()
        .with_context(|| format!("invalid listen address: {}:{}", args.host, args.port))?;
    let listener = TcpListener::bind(addr)
        .await
        .with_context(|| format!("failed to bind webhook server on {addr}"))?;

    println!("Listening for Nodit webhooks on http://{addr}{path}");
    println!("Use a tunnel such as ngrok or cloudflared if Nodit must reach this machine from the internet.");

    axum::serve(listener, app)
        .await
        .context("webhook server exited unexpectedly")?;

    Ok(json!({
        "status": "stopped"
    }))
}

async fn handle_aptos_node(
    client: &Client,
    config: &Config,
    output: &OutputFormat,
    cmd: AptosNodeCommand,
) -> Result<Value> {
    match cmd {
        AptosNodeCommand::Raw(args) => {
            let path = if args.path.starts_with('/') {
                args.path
            } else {
                format!("/{}", args.path)
            };
            let url = join_url(&config.aptos_api_base_url, &path)?;
            execute_json_request(
                client,
                args.method.into(),
                &url,
                build_headers(&config.api_key, &args.headers)?,
                maybe_parse_json(args.body.as_deref())?,
                output,
            )
            .await
        }
        AptosNodeCommand::LedgerInfo(args) => {
            execute_json_request(
                client,
                Method::GET,
                &config.aptos_api_base_url,
                build_headers(&config.api_key, &args.headers)?,
                None,
                output,
            )
            .await
        }
        AptosNodeCommand::Account(args) => {
            let url = join_url(
                &config.aptos_api_base_url,
                &format!("/accounts/{}", args.address),
            )?;
            execute_json_request(
                client,
                Method::GET,
                &url,
                build_headers(&config.api_key, &args.headers)?,
                None,
                output,
            )
            .await
        }
        AptosNodeCommand::Resources(args) => {
            let url = join_url(
                &config.aptos_api_base_url,
                &format!("/accounts/{}/resources", args.address),
            )?;
            execute_json_request(
                client,
                Method::GET,
                &url,
                build_headers(&config.api_key, &args.headers)?,
                None,
                output,
            )
            .await
        }
        AptosNodeCommand::Module(args) => {
            let url = join_url(
                &config.aptos_api_base_url,
                &format!("/accounts/{}/module/{}", args.address, args.module_name),
            )?;
            execute_json_request(
                client,
                Method::GET,
                &url,
                build_headers(&config.api_key, &args.headers)?,
                None,
                output,
            )
            .await
        }
        AptosNodeCommand::TransactionByHash(args) => {
            let url = join_url(
                &config.aptos_api_base_url,
                &format!("/transactions/by_hash/{}", args.hash),
            )?;
            execute_json_request(
                client,
                Method::GET,
                &url,
                build_headers(&config.api_key, &args.headers)?,
                None,
                output,
            )
            .await
        }
    }
}

async fn handle_stream(config: &Config, cmd: StreamCommand) -> Result<Value> {
    let subscribe_message = build_stream_subscribe_message(&cmd)?;
    let url = cmd
        .url
        .clone()
        .unwrap_or_else(|| build_stream_url(config, &cmd.target));
    let _ = Url::parse(&url).with_context(|| format!("invalid websocket URL: {url}"))?;

    let (mut socket, _) = connect_async(url.as_str())
        .await
        .with_context(|| format!("failed to connect to stream endpoint: {url}"))?;

    if let Some(message) = subscribe_message {
        socket
            .send(Message::Text(message))
            .await
            .context("failed to send stream subscription message")?;
    }

    let mut received = Vec::new();
    for _ in 0..cmd.messages {
        let Some(next) = socket.next().await else {
            break;
        };
        let message = next.context("failed to read stream message")?;
        match message {
            Message::Text(text) => {
                let value =
                    serde_json::from_str::<Value>(&text).unwrap_or(Value::String(text.to_string()));
                received.push(value);
            }
            Message::Binary(bytes) => {
                received.push(json!({
                    "type": "binary",
                    "hex": bytes.iter().map(|b| format!("{b:02x}")).collect::<String>(),
                }));
            }
            Message::Ping(payload) => {
                socket
                    .send(Message::Pong(payload))
                    .await
                    .context("failed to answer ping")?;
            }
            Message::Pong(_) => {}
            Message::Close(frame) => {
                received.push(json!({
                    "type": "close",
                    "reason": frame.map(|f| f.reason.to_string()).unwrap_or_default(),
                }));
                break;
            }
            Message::Frame(_) => {}
        }
    }

    Ok(json!({
        "url": url,
        "message_count": received.len(),
        "messages": received,
    }))
}

fn build_stream_subscribe_message(cmd: &StreamCommand) -> Result<Option<String>> {
    if let Some(message) = &cmd.subscribe {
        return Ok(Some(message.clone()));
    }

    let Some(event_type) = &cmd.event_type else {
        return Ok(None);
    };

    let condition = build_webhook_condition(
        Some(event_type),
        cmd.condition_json.as_deref(),
        cmd.period,
        &cmd.addresses,
    )?;
    let payload = json!({
        "id": cmd.id.unwrap_or(1),
        "method": "subscribe",
        "params": [{
            "eventType": event_type,
            "protocol": cmd.target.protocol,
            "network": cmd.target.network,
            "condition": condition.unwrap_or(Value::Object(Map::new()))
        }]
    });

    Ok(Some(payload.to_string()))
}

async fn execute_data_action(
    client: &Client,
    config: &Config,
    output: &OutputFormat,
    request: DataActionRequest<'_>,
) -> Result<Value> {
    let path = build_web3_path(request.target, Some(request.category), request.action);
    let url = join_url(&config.api_base_url, &path)?;
    execute_json_request(
        client,
        Method::POST,
        &url,
        build_headers(&config.api_key, request.headers)?,
        Some(request.body),
        output,
    )
    .await
}

async fn execute_data_body_action(
    client: &Client,
    config: &Config,
    output: &OutputFormat,
    args: &DataBodyArgs,
    category: &str,
    action: &str,
) -> Result<Value> {
    execute_data_action(
        client,
        config,
        output,
        DataActionRequest {
            target: &args.target,
            category,
            action,
            body: parse_json_required(&args.body)?,
            headers: &args.headers,
        },
    )
    .await
}

struct DataActionRequest<'a> {
    target: &'a NetworkArgs,
    category: &'a str,
    action: &'a str,
    body: Value,
    headers: &'a [HeaderArg],
}

async fn execute_json_request(
    client: &Client,
    method: Method,
    url: &str,
    headers: HeaderMap,
    body: Option<Value>,
    output: &OutputFormat,
) -> Result<Value> {
    let mut request = client.request(method.clone(), url).headers(headers);
    if let Some(body) = body {
        request = request.json(&body);
    }

    let response = request
        .send()
        .await
        .with_context(|| format!("request failed: {method} {url}"))?;
    let status = response.status();
    let headers = response.headers().clone();
    let text = response
        .text()
        .await
        .context("failed to read response body")?;
    let parsed = serde_json::from_str::<Value>(&text).unwrap_or(Value::String(text));

    if !status.is_success() {
        return Err(anyhow!(json!({
            "status": status.as_u16(),
            "url": url,
            "body": parsed,
        })
        .to_string()));
    }

    if matches!(output, OutputFormat::Pretty | OutputFormat::Json) {
        Ok(json!({
            "status": status.as_u16(),
            "headers": header_map_to_json(&headers),
            "body": parsed,
        }))
    } else {
        Ok(parsed)
    }
}

fn build_headers(api_key: &Option<String>, custom_headers: &[HeaderArg]) -> Result<HeaderMap> {
    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    if let Some(api_key) = api_key {
        headers.insert(
            HeaderName::from_static("x-api-key"),
            HeaderValue::from_str(api_key).context("invalid NODIT_API_KEY value")?,
        );
    }

    for header in custom_headers {
        let name = HeaderName::from_bytes(header.name.as_bytes())
            .with_context(|| format!("invalid header name: {}", header.name))?;
        let value = HeaderValue::from_str(&header.value)
            .with_context(|| format!("invalid header value for {}", header.name))?;
        headers.insert(name, value);
    }

    Ok(headers)
}

fn maybe_parse_json(input: Option<&str>) -> Result<Option<Value>> {
    input.map(parse_json_required).transpose()
}

fn parse_json_required(input: &str) -> Result<Value> {
    serde_json::from_str::<Value>(input).with_context(|| "invalid JSON body".to_string())
}

fn parse_optional_json_array(input: Option<&str>) -> Result<Value> {
    match input {
        Some(input) => {
            let parsed = parse_json_required(input)?;
            if !parsed.is_array() {
                return Err(anyhow!("RPC params must be a JSON array"));
            }
            Ok(parsed)
        }
        None => Ok(Value::Array(Vec::new())),
    }
}

fn build_web3_path(target: &NetworkArgs, category: Option<&str>, action: &str) -> String {
    match category {
        Some(category) => format!(
            "/v1/{}/{}/{}/{}",
            target.protocol, target.network, category, action
        ),
        None => format!("/v1/{}/{}/{}", target.protocol, target.network, action),
    }
}

fn build_webhook_path(target: &NetworkArgs, subscription_id: Option<&str>) -> String {
    match subscription_id {
        Some(subscription_id) => format!(
            "/v1/{}/{}/webhooks/{}",
            target.protocol, target.network, subscription_id
        ),
        None => format!("/v1/{}/{}/webhooks", target.protocol, target.network),
    }
}

fn build_rpc_url(config: &Config, target: &NetworkArgs) -> Result<String> {
    match &config.rpc_url {
        Some(url) => Ok(url.clone()),
        None => Ok(format!(
            "https://{}-{}.nodit.io",
            target.protocol, target.network
        )),
    }
}

fn build_stream_url(config: &Config, target: &NetworkArgs) -> String {
    match &config.stream_url {
        Some(url) => url.clone(),
        None => format!(
            "{}/v1/{}/{}",
            config.stream_base_url.trim_end_matches('/'),
            target.protocol,
            target.network
        ),
    }
}

fn merge_json_objects(base: Value, extra: Option<&str>) -> Result<Value> {
    let Some(extra) = extra else {
        return Ok(base);
    };

    let extra = parse_json_required(extra)?;
    let Value::Object(mut merged) = base else {
        return Err(anyhow!("base JSON payload must be an object"));
    };
    let Value::Object(extra) = extra else {
        return Err(anyhow!("extra JSON payload must be an object"));
    };

    for (key, value) in extra {
        merged.insert(key, value);
    }

    Ok(Value::Object(merged))
}

fn optional_json_object(extra: Option<&str>) -> Result<Value> {
    match extra {
        Some(extra) => {
            let parsed = parse_json_required(extra)?;
            if !parsed.is_object() {
                return Err(anyhow!("JSON payload must be an object"));
            }
            Ok(parsed)
        }
        None => Ok(Value::Object(Map::new())),
    }
}

fn build_webhook_create_body(args: &crate::cli::WebhookCreateArgs) -> Result<Value> {
    if let Some(body) = &args.body {
        return parse_json_required(body);
    }

    let event_type = args
        .event_type
        .as_ref()
        .context("missing webhook event type; pass --event-type or --body")?;
    let webhook_url = args
        .webhook_url
        .as_ref()
        .context("missing webhook URL; pass --webhook-url or --body")?;

    let mut object = Map::new();
    object.insert("eventType".to_string(), Value::String(event_type.clone()));
    object.insert(
        "notification".to_string(),
        json!({
            "webhookUrl": webhook_url
        }),
    );

    if let Some(description) = &args.description {
        object.insert(
            "description".to_string(),
            Value::String(description.clone()),
        );
    }
    if let Some(is_instant) = args.is_instant {
        object.insert("isInstant".to_string(), Value::Bool(is_instant));
    }

    let condition = build_webhook_condition(
        Some(event_type),
        args.condition_json.as_deref(),
        args.period,
        &args.addresses,
    )?;
    if let Some(condition) = condition {
        object.insert("condition".to_string(), condition);
    }

    Ok(Value::Object(object))
}

fn build_webhook_update_body(args: &crate::cli::WebhookUpdateArgs) -> Result<Value> {
    if let Some(body) = &args.body {
        return parse_json_required(body);
    }

    let mut object = Map::new();

    if let Some(description) = &args.description {
        object.insert(
            "description".to_string(),
            Value::String(description.clone()),
        );
    }
    if let Some(webhook_url) = &args.webhook_url {
        object.insert(
            "notification".to_string(),
            json!({
                "webhookUrl": webhook_url
            }),
        );
    }
    if let Some(is_instant) = args.is_instant {
        object.insert("isInstant".to_string(), Value::Bool(is_instant));
    }
    if let Some(is_active) = args.is_active {
        object.insert("isActive".to_string(), Value::Bool(is_active));
    }

    let condition = build_webhook_condition(
        None,
        args.condition_json.as_deref(),
        args.period,
        &args.addresses,
    )?;
    if let Some(condition) = condition {
        object.insert("condition".to_string(), condition);
    }

    if object.is_empty() {
        return Err(anyhow!(
            "missing update fields; pass at least one typed option or use --body"
        ));
    }

    Ok(Value::Object(object))
}

fn build_webhook_condition(
    event_type: Option<&str>,
    condition_json: Option<&str>,
    period: Option<u64>,
    addresses: &[String],
) -> Result<Option<Value>> {
    let mut object = match condition_json {
        Some(condition_json) => {
            let parsed = parse_json_required(condition_json)?;
            let Value::Object(object) = parsed else {
                return Err(anyhow!("webhook condition must be a JSON object"));
            };
            object
        }
        None => Map::new(),
    };

    if let Some(period) = period {
        object.insert("period".to_string(), json!(period));
    }
    if !addresses.is_empty() {
        object.insert(
            "addresses".to_string(),
            Value::Array(addresses.iter().cloned().map(Value::String).collect()),
        );
    }

    if let Some(event_type) = event_type {
        match event_type {
            "BLOCK_PERIOD" => {
                if !object.contains_key("period") && condition_json.is_none() {
                    return Err(anyhow!(
                        "BLOCK_PERIOD requires --period or --condition-json"
                    ));
                }
            }
            "ADDRESS_ACTIVITY" | "MINED_TRANSACTION" => {
                if !object.contains_key("addresses") && condition_json.is_none() {
                    return Err(anyhow!(
                        "{event_type} requires --address or --condition-json"
                    ));
                }
            }
            _ => {}
        }
    }

    if object.is_empty() {
        Ok(None)
    } else {
        Ok(Some(Value::Object(object)))
    }
}

fn join_url(base: &str, path: &str) -> Result<String> {
    let base = base.trim_end_matches('/');
    if path.starts_with("http://") || path.starts_with("https://") {
        return Ok(path.to_string());
    }
    let path = path.trim_start_matches('/');
    Ok(format!("{base}/{path}"))
}

fn append_query(url: String, params: &[(&str, String)]) -> Result<String> {
    let mut url = Url::parse(&url).with_context(|| format!("invalid URL: {url}"))?;
    {
        let mut query = url.query_pairs_mut();
        for (key, value) in params {
            query.append_pair(key, value);
        }
    }
    Ok(url.to_string())
}

fn header_map_to_json(headers: &HeaderMap) -> Value {
    let mut object = Map::new();
    for (name, value) in headers {
        let key = name.as_str().to_string();
        let value = value.to_str().unwrap_or_default().to_string();
        object.insert(key, Value::String(value));
    }
    Value::Object(object)
}

#[derive(Clone)]
struct WebhookServerState {
    output_file: Option<PathBuf>,
    print_body: bool,
    signing_key: Option<String>,
}

async fn webhook_ingest(
    State(state): State<Arc<WebhookServerState>>,
    headers: AxumHeaderMap,
    body: Bytes,
) -> impl IntoResponse {
    match handle_webhook_ingest(state, headers, body).await {
        Ok(value) => (StatusCode::OK, axum::Json(value)).into_response(),
        Err((status, value)) => (status, axum::Json(value)).into_response(),
    }
}

async fn handle_webhook_ingest(
    state: Arc<WebhookServerState>,
    headers: AxumHeaderMap,
    body: Bytes,
) -> std::result::Result<Value, (StatusCode, Value)> {
    if let Err(err) = validate_secret(&state, &headers, body.as_ref()) {
        return Err((StatusCode::UNAUTHORIZED, json!({ "error": err })));
    }

    let body_text = String::from_utf8(body.to_vec()).map_err(|_| {
        (
            StatusCode::BAD_REQUEST,
            json!({ "error": "request body must be valid UTF-8" }),
        )
    })?;
    let body_json =
        serde_json::from_str::<Value>(&body_text).unwrap_or(Value::String(body_text.clone()));
    let event = json!({
        "headers": header_map_to_json_req(&headers),
        "body": body_json,
    });

    if state.print_body {
        println!(
            "{}",
            serde_json::to_string_pretty(&event).unwrap_or_else(|_| event.to_string())
        );
    }

    if let Some(path) = &state.output_file {
        if let Err(err) = append_jsonl(path, &event).await {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                json!({ "error": format!("failed to write webhook event: {err}") }),
            ));
        }
    }

    Ok(json!({
        "received": true
    }))
}

fn validate_secret(
    state: &WebhookServerState,
    headers: &AxumHeaderMap,
    body: &[u8],
) -> std::result::Result<(), String> {
    let Some(signing_key) = &state.signing_key else {
        return Ok(());
    };

    let header = headers
        .get("x-signature")
        .ok_or_else(|| "missing required header: x-signature".to_string())?;
    let actual_signature = header
        .to_str()
        .map_err(|_| "invalid header encoding: x-signature".to_string())?;
    let expected_signature = compute_signature(signing_key, body)?;

    if actual_signature != expected_signature {
        return Err("invalid header value for x-signature".to_string());
    }

    Ok(())
}

async fn append_jsonl(path: &PathBuf, value: &Value) -> Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .await
        .with_context(|| format!("failed to open output file: {}", path.display()))?;
    let mut line = serde_json::to_vec(value)?;
    line.push(b'\n');
    file.write_all(&line).await?;
    Ok(())
}

fn compute_signature(signing_key: &str, body: &[u8]) -> std::result::Result<String, String> {
    let mut mac = Hmac::<Sha256>::new_from_slice(signing_key.as_bytes())
        .map_err(|_| "invalid signing key".to_string())?;
    mac.update(body);
    Ok(hex::encode(mac.finalize().into_bytes()))
}

fn normalize_route_path(path: &str) -> String {
    let trimmed = path.trim();
    if trimmed.is_empty() || trimmed == "/" {
        "/".to_string()
    } else if trimmed.starts_with('/') {
        trimmed.to_string()
    } else {
        format!("/{trimmed}")
    }
}

fn header_map_to_json_req(headers: &AxumHeaderMap) -> Value {
    let mut object = Map::new();
    for (name, value) in headers {
        let key = name.as_str().to_string();
        let value = value.to_str().unwrap_or_default().to_string();
        object.insert(key, Value::String(value));
    }
    Value::Object(object)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::HeaderValue;

    #[test]
    fn build_web3_path_includes_category() {
        let target = NetworkArgs {
            protocol: "ethereum".to_string(),
            network: "mainnet".to_string(),
        };
        assert_eq!(
            build_web3_path(&target, Some("token"), "getTokensOwnedByAccount"),
            "/v1/ethereum/mainnet/token/getTokensOwnedByAccount"
        );
    }

    #[test]
    fn merge_json_objects_overrides_existing_keys() {
        let merged = merge_json_objects(
            json!({ "accountAddress": "0x1", "withCount": false }),
            Some(r#"{"withCount":true}"#),
        )
        .expect("merge should succeed");

        assert_eq!(merged["accountAddress"], "0x1");
        assert_eq!(merged["withCount"], true);
    }

    #[test]
    fn optional_json_object_defaults_to_empty_object() {
        let value = optional_json_object(None).expect("empty object");
        assert_eq!(value, Value::Object(Map::new()));
    }

    #[test]
    fn normalize_route_path_adds_leading_slash() {
        assert_eq!(normalize_route_path("webhooks/nodit"), "/webhooks/nodit");
        assert_eq!(normalize_route_path("/webhooks/nodit"), "/webhooks/nodit");
    }

    #[test]
    fn validate_secret_accepts_matching_header() {
        let state = WebhookServerState {
            output_file: None,
            print_body: false,
            signing_key: Some("test-signing-key".to_string()),
        };
        let mut headers = AxumHeaderMap::new();
        let signature =
            compute_signature("test-signing-key", br#"{"event":"transaction.confirmed"}"#)
                .expect("signature");
        headers.insert(
            "x-signature",
            HeaderValue::from_str(&signature).expect("header value"),
        );

        assert!(validate_secret(&state, &headers, br#"{"event":"transaction.confirmed"}"#).is_ok());
    }

    #[test]
    fn webhook_create_body_uses_typed_fields() {
        let args = crate::cli::WebhookCreateArgs {
            target: NetworkArgs {
                protocol: "ethereum".to_string(),
                network: "mainnet".to_string(),
            },
            body: None,
            event_type: Some("BLOCK_PERIOD".to_string()),
            description: Some("Webhook Test".to_string()),
            webhook_url: Some("https://example.com/webhook".to_string()),
            is_instant: Some(true),
            condition_json: None,
            period: Some(1),
            addresses: Vec::new(),
            headers: Vec::new(),
        };

        let body = build_webhook_create_body(&args).expect("typed body");
        assert_eq!(body["eventType"], "BLOCK_PERIOD");
        assert_eq!(
            body["notification"]["webhookUrl"],
            "https://example.com/webhook"
        );
        assert_eq!(body["condition"]["period"], 1);
        assert_eq!(body["isInstant"], true);
    }

    #[test]
    fn webhook_update_body_rejects_empty_update() {
        let args = crate::cli::WebhookUpdateArgs {
            target: NetworkArgs {
                protocol: "ethereum".to_string(),
                network: "mainnet".to_string(),
            },
            subscription_id: "1".to_string(),
            body: None,
            description: None,
            webhook_url: None,
            is_instant: None,
            is_active: None,
            condition_json: None,
            period: None,
            addresses: Vec::new(),
            headers: Vec::new(),
        };

        assert!(build_webhook_update_body(&args).is_err());
    }

    #[test]
    fn webhook_create_body_uses_address_conditions() {
        let args = crate::cli::WebhookCreateArgs {
            target: NetworkArgs {
                protocol: "ethereum".to_string(),
                network: "mainnet".to_string(),
            },
            body: None,
            event_type: Some("ADDRESS_ACTIVITY".to_string()),
            description: None,
            webhook_url: Some("https://example.com/webhook".to_string()),
            is_instant: Some(false),
            condition_json: None,
            period: None,
            addresses: vec!["0x123".to_string(), "0x456".to_string()],
            headers: Vec::new(),
        };

        let body = build_webhook_create_body(&args).expect("typed body");
        assert_eq!(body["condition"]["addresses"][0], "0x123");
        assert_eq!(body["condition"]["addresses"][1], "0x456");
    }

    #[test]
    fn stream_subscribe_message_uses_typed_fields() {
        let cmd = StreamCommand {
            target: NetworkArgs {
                protocol: "ethereum".to_string(),
                network: "mainnet".to_string(),
            },
            url: None,
            subscribe: None,
            event_type: Some("BLOCK_PERIOD".to_string()),
            condition_json: None,
            id: Some(7),
            period: Some(1),
            addresses: Vec::new(),
            messages: 1,
        };

        let message = build_stream_subscribe_message(&cmd)
            .expect("stream message")
            .expect("message present");
        let value: Value = serde_json::from_str(&message).expect("valid json");

        assert_eq!(value["id"], 7);
        assert_eq!(value["method"], "subscribe");
        assert_eq!(value["params"][0]["eventType"], "BLOCK_PERIOD");
        assert_eq!(value["params"][0]["protocol"], "ethereum");
        assert_eq!(value["params"][0]["network"], "mainnet");
        assert_eq!(value["params"][0]["condition"]["period"], 1);
    }

    #[test]
    fn stream_subscribe_message_uses_address_conditions() {
        let cmd = StreamCommand {
            target: NetworkArgs {
                protocol: "ethereum".to_string(),
                network: "mainnet".to_string(),
            },
            url: None,
            subscribe: None,
            event_type: Some("MINED_TRANSACTION".to_string()),
            condition_json: None,
            id: Some(9),
            period: None,
            addresses: vec!["0xabc".to_string()],
            messages: 1,
        };

        let message = build_stream_subscribe_message(&cmd)
            .expect("stream message")
            .expect("message present");
        let value: Value = serde_json::from_str(&message).expect("valid json");

        assert_eq!(value["params"][0]["condition"]["addresses"][0], "0xabc");
    }

    #[test]
    fn node_logs_filter_includes_topics() {
        let filter = build_node_logs_filter(crate::cli::NodeLogsArgs {
            target: NetworkArgs {
                protocol: "ethereum".to_string(),
                network: "mainnet".to_string(),
            },
            from_block: Some("0x1".to_string()),
            to_block: Some("latest".to_string()),
            address: Some("0xabc".to_string()),
            topic: vec!["0xtopic1".to_string(), "0xtopic2".to_string()],
            headers: Vec::new(),
        });

        assert_eq!(filter.payload["fromBlock"], "0x1");
        assert_eq!(filter.payload["address"], "0xabc");
        assert_eq!(filter.payload["topics"][1], "0xtopic2");
    }

    #[test]
    fn merge_json_objects_can_hold_lists() {
        let merged = merge_json_objects(
            json!({ "transactionHashes": ["0x1", "0x2"] }),
            Some(r#"{"withDecode":true}"#),
        )
        .expect("merge should succeed");

        assert_eq!(merged["transactionHashes"][0], "0x1");
        assert_eq!(merged["withDecode"], true);
    }
}
