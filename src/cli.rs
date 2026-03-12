use clap::{Args as ClapArgs, Parser, Subcommand, ValueEnum};

#[derive(Parser, Debug)]
#[command(
    name = "nodit",
    version,
    about = "CLI for Nodit APIs and streams",
    next_line_help = true,
    after_help = "Machine-friendly usage:\n  nodit --json ...\n  nodit --field result ...\n  nodit --field body ...\n\nWhen --output is omitted, the CLI defaults to pretty on a TTY and json when stdout is piped.\nAll responses use {\"ok\":...,\"data\"|\"error\":...} envelopes."
)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,

    #[arg(long, alias = "format", value_enum)]
    pub output: Option<OutputFormat>,

    #[arg(long, global = true)]
    pub json: bool,

    #[arg(long, global = true)]
    pub field: Option<String>,

    #[arg(long, global = true)]
    pub api_key: Option<String>,

    #[arg(long, global = true)]
    pub api_base_url: Option<String>,

    #[arg(long, global = true)]
    pub rpc_url: Option<String>,

    #[arg(long, global = true)]
    pub stream_url: Option<String>,

    #[arg(long, global = true)]
    pub aptos_api_base_url: Option<String>,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    #[command(subcommand)]
    Node(NodeCommand),
    #[command(subcommand)]
    Data(DataCommand),
    #[command(subcommand)]
    Webhook(WebhookCommand),
    Stream(StreamCommand),
}

#[derive(Subcommand, Debug)]
pub enum NodeCommand {
    #[command(subcommand)]
    Evm(EvmNodeCommand),
    #[command(subcommand)]
    Aptos(AptosNodeCommand),
    #[command(subcommand)]
    Solana(SolanaNodeCommand),
    #[command(subcommand)]
    Bitcoin(BitcoinNodeCommand),
    #[command(subcommand)]
    Dogecoin(GenericNodeCommand),
    #[command(subcommand)]
    Xrpl(GenericNodeCommand),
}

#[derive(Subcommand, Debug)]
pub enum EvmNodeCommand {
    Raw(NodeRawArgs),
    Batch(NodeBatchArgs),
    BlockNumber(NodeTargetArgs),
    ChainId(NodeTargetArgs),
    Balance(NodeBalanceArgs),
    TransactionCount(NodeAccountArgs),
    Code(NodeAccountArgs),
    Logs(NodeLogsArgs),
    Transaction(NodeHashArgs),
    TransactionReceipt(NodeHashArgs),
    GasPrice(NodeTargetArgs),
    Call(NodeCallArgs),
}

#[derive(ClapArgs, Debug)]
pub struct NodeRawArgs {
    #[command(flatten)]
    pub target: NetworkArgs,

    #[arg(long)]
    pub method: String,

    #[arg(long)]
    pub params: Option<String>,

    #[arg(long, default_value_t = 1)]
    pub id: u64,

    #[arg(long)]
    pub url: Option<String>,

    #[arg(long = "header", value_parser = parse_header)]
    pub headers: Vec<HeaderArg>,
}

#[derive(ClapArgs, Debug)]
pub struct NodeTargetArgs {
    #[command(flatten)]
    pub target: NetworkArgs,

    #[arg(long = "header", value_parser = parse_header)]
    pub headers: Vec<HeaderArg>,
}

#[derive(ClapArgs, Debug)]
pub struct NodeBalanceArgs {
    #[command(flatten)]
    pub target: NetworkArgs,

    #[arg(long)]
    pub account: String,

    #[arg(long)]
    pub block_tag: Option<String>,

    #[arg(long = "header", value_parser = parse_header)]
    pub headers: Vec<HeaderArg>,
}

#[derive(ClapArgs, Debug)]
pub struct NodeAccountArgs {
    #[command(flatten)]
    pub target: NetworkArgs,

    #[arg(long)]
    pub account: String,

    #[arg(long)]
    pub block_tag: Option<String>,

    #[arg(long = "header", value_parser = parse_header)]
    pub headers: Vec<HeaderArg>,
}

#[derive(ClapArgs, Debug)]
pub struct NodeHashArgs {
    #[command(flatten)]
    pub target: NetworkArgs,

    #[arg(long)]
    pub hash: String,

    #[arg(long = "header", value_parser = parse_header)]
    pub headers: Vec<HeaderArg>,
}

#[derive(ClapArgs, Debug)]
pub struct NodeCallArgs {
    #[command(flatten)]
    pub target: NetworkArgs,

    #[arg(long)]
    pub to: String,

    #[arg(long)]
    pub data: String,

    #[arg(long)]
    pub from: Option<String>,

    #[arg(long)]
    pub value: Option<String>,

    #[arg(long)]
    pub block_tag: Option<String>,

    #[arg(long = "header", value_parser = parse_header)]
    pub headers: Vec<HeaderArg>,
}

#[derive(ClapArgs, Debug)]
pub struct NodeLogsArgs {
    #[command(flatten)]
    pub target: NetworkArgs,

    #[arg(long)]
    pub from_block: Option<String>,

    #[arg(long)]
    pub to_block: Option<String>,

    #[arg(long)]
    pub address: Option<String>,

    #[arg(long, num_args = 1.., value_delimiter = ',')]
    pub topic: Vec<String>,

    #[arg(long = "header", value_parser = parse_header)]
    pub headers: Vec<HeaderArg>,
}

#[derive(Subcommand, Debug)]
pub enum GenericNodeCommand {
    Raw(NodeRawArgs),
}

#[derive(Subcommand, Debug)]
pub enum SolanaNodeCommand {
    Raw(NodeRawArgs),
    Slot(NodeTargetArgs),
    LatestBlockhash(NodeTargetArgs),
    BlockHeight(NodeTargetArgs),
    EpochInfo(NodeTargetArgs),
    Version(NodeTargetArgs),
    GenesisHash(NodeTargetArgs),
    Health(NodeTargetArgs),
    Balance(SolanaBalanceArgs),
    AccountInfo(SolanaAddressArgs),
    TokenAccountBalance(SolanaAddressArgs),
    MinimumBalanceForRentExemption(SolanaRentArgs),
    TokenSupply(SolanaMintArgs),
    Block(SolanaBlockArgs),
    Blocks(SolanaBlocksArgs),
    Transaction(SolanaTransactionArgs),
    SignaturesForAddress(SolanaAddressArgs),
    SignatureStatuses(SolanaSignaturesArgs),
}

#[derive(Subcommand, Debug)]
pub enum BitcoinNodeCommand {
    Raw(NodeRawArgs),
    BlockCount(NodeTargetArgs),
    BlockHash(BitcoinBlockHashArgs),
    Block(BitcoinBlockArgs),
    Transaction(BitcoinTransactionArgs),
}

#[derive(ClapArgs, Debug)]
pub struct NodeBatchArgs {
    #[command(flatten)]
    pub target: NetworkArgs,

    #[arg(long)]
    pub body: String,

    #[arg(long)]
    pub url: Option<String>,

    #[arg(long = "header", value_parser = parse_header)]
    pub headers: Vec<HeaderArg>,
}

#[derive(ClapArgs, Debug)]
pub struct SolanaBalanceArgs {
    #[command(flatten)]
    pub target: NetworkArgs,

    #[arg(long)]
    pub address: String,

    #[arg(long)]
    pub commitment: Option<String>,

    #[arg(long = "header", value_parser = parse_header)]
    pub headers: Vec<HeaderArg>,
}

#[derive(ClapArgs, Debug)]
pub struct SolanaBlockArgs {
    #[command(flatten)]
    pub target: NetworkArgs,

    #[arg(long)]
    pub slot: u64,

    #[arg(long)]
    pub config_json: Option<String>,

    #[arg(long = "header", value_parser = parse_header)]
    pub headers: Vec<HeaderArg>,
}

#[derive(ClapArgs, Debug)]
pub struct SolanaTransactionArgs {
    #[command(flatten)]
    pub target: NetworkArgs,

    #[arg(long)]
    pub signature: String,

    #[arg(long)]
    pub config_json: Option<String>,

    #[arg(long = "header", value_parser = parse_header)]
    pub headers: Vec<HeaderArg>,
}

#[derive(ClapArgs, Debug)]
pub struct SolanaAddressArgs {
    #[command(flatten)]
    pub target: NetworkArgs,

    #[arg(long)]
    pub address: String,

    #[arg(long)]
    pub config_json: Option<String>,

    #[arg(long = "header", value_parser = parse_header)]
    pub headers: Vec<HeaderArg>,
}

#[derive(ClapArgs, Debug)]
pub struct SolanaBlocksArgs {
    #[command(flatten)]
    pub target: NetworkArgs,

    #[arg(long)]
    pub start_slot: u64,

    #[arg(long)]
    pub end_slot: Option<u64>,

    #[arg(long)]
    pub commitment: Option<String>,

    #[arg(long = "header", value_parser = parse_header)]
    pub headers: Vec<HeaderArg>,
}

#[derive(ClapArgs, Debug)]
pub struct SolanaSignaturesArgs {
    #[command(flatten)]
    pub target: NetworkArgs,

    #[arg(long = "signature", num_args = 1.., value_delimiter = ',')]
    pub signature: Vec<String>,

    #[arg(long)]
    pub config_json: Option<String>,

    #[arg(long = "header", value_parser = parse_header)]
    pub headers: Vec<HeaderArg>,
}

#[derive(ClapArgs, Debug)]
pub struct SolanaRentArgs {
    #[command(flatten)]
    pub target: NetworkArgs,

    #[arg(long)]
    pub data_length: u64,

    #[arg(long)]
    pub commitment: Option<String>,

    #[arg(long = "header", value_parser = parse_header)]
    pub headers: Vec<HeaderArg>,
}

#[derive(ClapArgs, Debug)]
pub struct SolanaMintArgs {
    #[command(flatten)]
    pub target: NetworkArgs,

    #[arg(long)]
    pub mint_address: String,

    #[arg(long)]
    pub commitment: Option<String>,

    #[arg(long = "header", value_parser = parse_header)]
    pub headers: Vec<HeaderArg>,
}

#[derive(ClapArgs, Debug)]
pub struct BitcoinBlockHashArgs {
    #[command(flatten)]
    pub target: NetworkArgs,

    #[arg(long)]
    pub height: u64,

    #[arg(long = "header", value_parser = parse_header)]
    pub headers: Vec<HeaderArg>,
}

#[derive(ClapArgs, Debug)]
pub struct BitcoinBlockArgs {
    #[command(flatten)]
    pub target: NetworkArgs,

    #[arg(long)]
    pub hash: String,

    #[arg(long)]
    pub verbosity: Option<u8>,

    #[arg(long = "header", value_parser = parse_header)]
    pub headers: Vec<HeaderArg>,
}

#[derive(ClapArgs, Debug)]
pub struct BitcoinTransactionArgs {
    #[command(flatten)]
    pub target: NetworkArgs,

    #[arg(long)]
    pub txid: String,

    #[arg(long)]
    pub verbose: Option<bool>,

    #[arg(long = "header", value_parser = parse_header)]
    pub headers: Vec<HeaderArg>,
}

#[derive(Subcommand, Debug)]
pub enum DataCommand {
    #[command(subcommand)]
    Native(DataNativeCommand),
    #[command(subcommand)]
    Account(DataAccountCommand),
    #[command(subcommand)]
    Tx(DataTxCommand),
    #[command(subcommand)]
    Block(DataBlockCommand),
    #[command(subcommand)]
    Token(DataTokenCommand),
    #[command(subcommand)]
    Nft(DataNftCommand),
    #[command(subcommand)]
    Ens(DataEnsCommand),
    #[command(subcommand)]
    Stats(DataStatsCommand),
    #[command(subcommand)]
    Asset(DataAssetCommand),
    #[command(subcommand)]
    Multichain(MultichainCommand),
    Raw(DataRawArgs),
}

#[derive(Subcommand, Debug)]
pub enum DataNativeCommand {
    Balance(DataAccountArgs),
    TokenBalance(DataAccountArgs),
    BalanceChangesByAccount(DataAccountArgs),
    TransfersByAccount(DataAccountArgs),
    TransfersWithinRange(DataRangeArgs),
    Holders(DataSimpleArgs),
}

#[derive(Subcommand, Debug)]
pub enum DataAccountCommand {
    TotalTransactionCount(DataAccountArgs),
    InternalTransactions(DataAccountArgs),
    NextNonce(DataAccountArgs),
    IsContract(DataAccountArgs),
    UnspentTransactionOutputs(DataAccountArgs),
}

#[derive(Subcommand, Debug)]
pub enum DataTxCommand {
    ByHash(DataTransactionArgs),
    ByTransactionId(DataTransactionIdArgs),
    ByHashes(DataHashesArgs),
    ByTransactionIds(DataTransactionIdsArgs),
    ByAccount(DataAccountArgs),
    InBlock(DataBlockRefArgs),
    InLedger(DataLedgerRefArgs),
    InternalByTransactionHash(DataTransactionArgs),
    SearchEvents(DataBodyArgs),
}

#[derive(Subcommand, Debug)]
pub enum DataBlockCommand {
    ByNumber(DataBlockNumberArgs),
    ByHash(DataBlockHashArgs),
    ByHashOrNumber(DataBlockRefArgs),
    LedgerByHashOrIndex(DataLedgerRefArgs),
    WithinRange(DataRangeArgs),
    LedgersWithinRange(DataLedgerRangeArgs),
    GasPrice(DataSimpleArgs),
}

#[derive(Subcommand, Debug)]
pub enum DataTokenCommand {
    OwnedByAccount(DataAccountArgs),
    Allowance(DataTokenAllowanceArgs),
    ContractMetadataByContracts(DataContractsArgs),
    BalanceChangesByAccount(DataAccountArgs),
    TransfersByCurrencyAndIssuer(DataTokenCurrencyIssuerArgs),
    HoldersByContract(DataContractArgs),
    PricesByContracts(DataContractsArgs),
    TransfersByContract(DataContractArgs),
    TransfersWithinRange(DataRangeArgs),
    TransfersByAccount(DataAccountArgs),
    SearchContractMetadata(DataKeywordArgs),
}

#[derive(Subcommand, Debug)]
pub enum DataNftCommand {
    OwnedByAccount(DataAccountArgs),
    ContractsByAccount(DataAccountArgs),
    ContractMetadataByContracts(DataContractsArgs),
    HoldersByContract(DataContractArgs),
    HoldersByTokenId(DataContractTokenIdArgs),
    MetadataByContract(DataContractArgs),
    MetadataByTokenIds(DataContractTokenIdsArgs),
    TransfersByContract(DataContractArgs),
    TransfersByTokenId(DataContractTokenIdArgs),
    TransfersWithinRange(DataRangeArgs),
    TransfersByAccount(DataAccountArgs),
    SearchContractMetadata(DataKeywordArgs),
    SyncMetadata(DataContractTokenIdArgs),
}

#[derive(Subcommand, Debug)]
pub enum DataEnsCommand {
    AddressByName(DataNameArgs),
    NameByAddress(DataAddressArgs),
    RecordByName(DataNameArgs),
    RecordsByAccount(DataEnsRecordsArgs),
}

#[derive(Subcommand, Debug)]
pub enum DataStatsCommand {
    Account(DataAccountArgs),
    DailyActiveAccounts(DataBodyArgs),
    DailyActiveAccountsByContract(DataBodyArgs),
    DailyTransactions(DataBodyArgs),
    DailyTransactionsByContract(DataBodyArgs),
    HourlyActiveAccounts(DataBodyArgs),
    HourlyActiveAccountsByContract(DataBodyArgs),
    HourlyTransactions(DataBodyArgs),
    HourlyTransactionsByContract(DataBodyArgs),
}

#[derive(Subcommand, Debug)]
pub enum DataAssetCommand {
    MetadataByIssuer(DataIssuerArgs),
    MetadataByIds(DataAssetIdsArgs),
    SearchMetadata(DataKeywordArgs),
    HoldersById(DataAssetIdArgs),
    TransfersById(DataAssetIdArgs),
    TransfersWithinRange(DataRangeArgs),
    TransfersByAccount(DataAccountArgs),
    OwnedByAccount(DataAccountArgs),
}

#[derive(Subcommand, Debug)]
pub enum MultichainCommand {
    LookupEntities(MultichainLookupArgs),
}

#[derive(Subcommand, Debug)]
pub enum WebhookCommand {
    List(WebhookListArgs),
    Get(WebhookGetArgs),
    Create(WebhookCreateArgs),
    Update(WebhookUpdateArgs),
    Delete(WebhookDeleteArgs),
    History(WebhookHistoryArgs),
    Serve(WebhookServeArgs),
    Raw(WebhookRawArgs),
}

#[derive(Subcommand, Debug)]
pub enum AptosNodeCommand {
    Raw(AptosRawArgs),
    LedgerInfo(AptosNodeBaseArgs),
    Account(AptosAccountArgs),
    Resources(AptosAccountArgs),
    Module(AptosModuleArgs),
    TransactionByHash(AptosTransactionArgs),
}

#[derive(ClapArgs, Debug, Clone)]
pub struct NetworkArgs {
    #[arg(long, default_value = "ethereum")]
    pub protocol: String,

    #[arg(long, default_value = "mainnet")]
    pub network: String,
}

#[derive(ClapArgs, Debug)]
pub struct DataRawArgs {
    #[command(flatten)]
    pub target: NetworkArgs,

    #[arg(long, value_enum)]
    pub method: HttpMethod,

    #[arg(long)]
    pub category: Option<String>,

    #[arg(long)]
    pub action: String,

    #[arg(long)]
    pub body: Option<String>,

    #[arg(long = "header", value_parser = parse_header)]
    pub headers: Vec<HeaderArg>,
}

#[derive(ClapArgs, Debug)]
pub struct DataAccountArgs {
    #[command(flatten)]
    pub target: NetworkArgs,

    #[arg(long)]
    pub account: String,

    #[arg(long)]
    pub body: Option<String>,

    #[arg(long = "header", value_parser = parse_header)]
    pub headers: Vec<HeaderArg>,
}

#[derive(ClapArgs, Debug)]
pub struct DataTransactionArgs {
    #[command(flatten)]
    pub target: NetworkArgs,

    #[arg(long)]
    pub hash: String,

    #[arg(long)]
    pub body: Option<String>,

    #[arg(long = "header", value_parser = parse_header)]
    pub headers: Vec<HeaderArg>,
}

#[derive(ClapArgs, Debug)]
pub struct DataTransactionIdArgs {
    #[command(flatten)]
    pub target: NetworkArgs,

    #[arg(long)]
    pub transaction_id: String,

    #[arg(long)]
    pub body: Option<String>,

    #[arg(long = "header", value_parser = parse_header)]
    pub headers: Vec<HeaderArg>,
}

#[derive(ClapArgs, Debug)]
pub struct DataTransactionIdsArgs {
    #[command(flatten)]
    pub target: NetworkArgs,

    #[arg(long = "transaction-id", num_args = 1.., value_delimiter = ',')]
    pub transaction_id: Vec<String>,

    #[arg(long)]
    pub body: Option<String>,

    #[arg(long = "header", value_parser = parse_header)]
    pub headers: Vec<HeaderArg>,
}

#[derive(ClapArgs, Debug)]
pub struct DataBlockNumberArgs {
    #[command(flatten)]
    pub target: NetworkArgs,

    #[arg(long)]
    pub number: String,

    #[arg(long)]
    pub body: Option<String>,

    #[arg(long = "header", value_parser = parse_header)]
    pub headers: Vec<HeaderArg>,
}

#[derive(ClapArgs, Debug)]
pub struct DataBlockHashArgs {
    #[command(flatten)]
    pub target: NetworkArgs,

    #[arg(long)]
    pub hash: String,

    #[arg(long)]
    pub body: Option<String>,

    #[arg(long = "header", value_parser = parse_header)]
    pub headers: Vec<HeaderArg>,
}

#[derive(ClapArgs, Debug)]
pub struct DataBlockRefArgs {
    #[command(flatten)]
    pub target: NetworkArgs,

    #[arg(long)]
    pub block: String,

    #[arg(long)]
    pub body: Option<String>,

    #[arg(long = "header", value_parser = parse_header)]
    pub headers: Vec<HeaderArg>,
}

#[derive(ClapArgs, Debug)]
pub struct DataLedgerRefArgs {
    #[command(flatten)]
    pub target: NetworkArgs,

    #[arg(long)]
    pub ledger: String,

    #[arg(long)]
    pub body: Option<String>,

    #[arg(long = "header", value_parser = parse_header)]
    pub headers: Vec<HeaderArg>,
}

#[derive(ClapArgs, Debug)]
pub struct DataLedgerRangeArgs {
    #[command(flatten)]
    pub target: NetworkArgs,

    #[arg(long)]
    pub from_ledger: String,

    #[arg(long)]
    pub to_ledger: String,

    #[arg(long)]
    pub body: Option<String>,

    #[arg(long = "header", value_parser = parse_header)]
    pub headers: Vec<HeaderArg>,
}

#[derive(ClapArgs, Debug)]
pub struct DataSimpleArgs {
    #[command(flatten)]
    pub target: NetworkArgs,

    #[arg(long)]
    pub body: Option<String>,

    #[arg(long = "header", value_parser = parse_header)]
    pub headers: Vec<HeaderArg>,
}

#[derive(ClapArgs, Debug)]
pub struct DataBodyArgs {
    #[command(flatten)]
    pub target: NetworkArgs,

    #[arg(long)]
    pub body: String,

    #[arg(long = "header", value_parser = parse_header)]
    pub headers: Vec<HeaderArg>,
}

#[derive(ClapArgs, Debug)]
pub struct DataContractArgs {
    #[command(flatten)]
    pub target: NetworkArgs,

    #[arg(long)]
    pub contract: String,

    #[arg(long)]
    pub body: Option<String>,

    #[arg(long = "header", value_parser = parse_header)]
    pub headers: Vec<HeaderArg>,
}

#[derive(ClapArgs, Debug)]
pub struct DataContractsArgs {
    #[command(flatten)]
    pub target: NetworkArgs,

    #[arg(long, num_args = 1.., value_delimiter = ',')]
    pub contract: Vec<String>,

    #[arg(long)]
    pub body: Option<String>,

    #[arg(long = "header", value_parser = parse_header)]
    pub headers: Vec<HeaderArg>,
}

#[derive(ClapArgs, Debug)]
pub struct DataRangeArgs {
    #[command(flatten)]
    pub target: NetworkArgs,

    #[arg(long)]
    pub from_block: String,

    #[arg(long)]
    pub to_block: String,

    #[arg(long)]
    pub body: Option<String>,

    #[arg(long = "header", value_parser = parse_header)]
    pub headers: Vec<HeaderArg>,
}

#[derive(ClapArgs, Debug)]
pub struct DataKeywordArgs {
    #[command(flatten)]
    pub target: NetworkArgs,

    #[arg(long)]
    pub keyword: String,

    #[arg(long)]
    pub body: Option<String>,

    #[arg(long = "header", value_parser = parse_header)]
    pub headers: Vec<HeaderArg>,
}

#[derive(ClapArgs, Debug)]
pub struct DataHashesArgs {
    #[command(flatten)]
    pub target: NetworkArgs,

    #[arg(long, num_args = 1.., value_delimiter = ',')]
    pub hash: Vec<String>,

    #[arg(long)]
    pub body: Option<String>,

    #[arg(long = "header", value_parser = parse_header)]
    pub headers: Vec<HeaderArg>,
}

#[derive(ClapArgs, Debug)]
pub struct DataTokenAllowanceArgs {
    #[command(flatten)]
    pub target: NetworkArgs,

    #[arg(long)]
    pub contract: String,

    #[arg(long)]
    pub owner: String,

    #[arg(long)]
    pub spender: String,

    #[arg(long)]
    pub body: Option<String>,

    #[arg(long = "header", value_parser = parse_header)]
    pub headers: Vec<HeaderArg>,
}

#[derive(ClapArgs, Debug)]
pub struct DataTokenCurrencyIssuerArgs {
    #[command(flatten)]
    pub target: NetworkArgs,

    #[arg(long)]
    pub currency: String,

    #[arg(long)]
    pub issuer_address: String,

    #[arg(long)]
    pub body: Option<String>,

    #[arg(long = "header", value_parser = parse_header)]
    pub headers: Vec<HeaderArg>,
}

#[derive(ClapArgs, Debug)]
pub struct DataContractTokenIdArgs {
    #[command(flatten)]
    pub target: NetworkArgs,

    #[arg(long)]
    pub contract: String,

    #[arg(long)]
    pub token_id: String,

    #[arg(long)]
    pub body: Option<String>,

    #[arg(long = "header", value_parser = parse_header)]
    pub headers: Vec<HeaderArg>,
}

#[derive(ClapArgs, Debug)]
pub struct DataNameArgs {
    #[command(flatten)]
    pub target: NetworkArgs,

    #[arg(long)]
    pub name: String,

    #[arg(long)]
    pub body: Option<String>,

    #[arg(long = "header", value_parser = parse_header)]
    pub headers: Vec<HeaderArg>,
}

#[derive(ClapArgs, Debug)]
pub struct DataAddressArgs {
    #[command(flatten)]
    pub target: NetworkArgs,

    #[arg(long)]
    pub address: String,

    #[arg(long)]
    pub body: Option<String>,

    #[arg(long = "header", value_parser = parse_header)]
    pub headers: Vec<HeaderArg>,
}

#[derive(ClapArgs, Debug)]
pub struct DataEnsRecordsArgs {
    #[command(flatten)]
    pub target: NetworkArgs,

    #[arg(long)]
    pub owner_address: Option<String>,

    #[arg(long)]
    pub resolved_address: Option<String>,

    #[arg(long)]
    pub body: Option<String>,

    #[arg(long = "header", value_parser = parse_header)]
    pub headers: Vec<HeaderArg>,
}

#[derive(ClapArgs, Debug)]
pub struct DataIssuerArgs {
    #[command(flatten)]
    pub target: NetworkArgs,

    #[arg(long)]
    pub issuer_address: String,

    #[arg(long)]
    pub body: Option<String>,

    #[arg(long = "header", value_parser = parse_header)]
    pub headers: Vec<HeaderArg>,
}

#[derive(ClapArgs, Debug)]
pub struct DataAssetIdArgs {
    #[command(flatten)]
    pub target: NetworkArgs,

    #[arg(long = "asset-id")]
    pub asset_id: String,

    #[arg(long)]
    pub body: Option<String>,

    #[arg(long = "header", value_parser = parse_header)]
    pub headers: Vec<HeaderArg>,
}

#[derive(ClapArgs, Debug)]
pub struct DataAssetIdsArgs {
    #[command(flatten)]
    pub target: NetworkArgs,

    #[arg(long = "asset-id", num_args = 1.., value_delimiter = ',')]
    pub asset_id: Vec<String>,

    #[arg(long)]
    pub body: Option<String>,

    #[arg(long = "header", value_parser = parse_header)]
    pub headers: Vec<HeaderArg>,
}

#[derive(ClapArgs, Debug)]
pub struct DataContractTokenIdsArgs {
    #[command(flatten)]
    pub target: NetworkArgs,

    #[arg(long)]
    pub contract: String,

    #[arg(long, num_args = 1.., value_delimiter = ',')]
    pub token_id: Vec<String>,

    #[arg(long)]
    pub body: Option<String>,

    #[arg(long = "header", value_parser = parse_header)]
    pub headers: Vec<HeaderArg>,
}

#[derive(ClapArgs, Debug)]
pub struct MultichainLookupArgs {
    #[arg(long)]
    pub body: String,

    #[arg(long = "header", value_parser = parse_header)]
    pub headers: Vec<HeaderArg>,
}

#[derive(ClapArgs, Debug)]
pub struct WebhookListArgs {
    #[command(flatten)]
    pub target: NetworkArgs,

    #[arg(long = "header", value_parser = parse_header)]
    pub headers: Vec<HeaderArg>,
}

#[derive(ClapArgs, Debug)]
pub struct WebhookGetArgs {
    #[command(flatten)]
    pub target: NetworkArgs,

    #[arg(long)]
    pub subscription_id: String,

    #[arg(long = "header", value_parser = parse_header)]
    pub headers: Vec<HeaderArg>,
}

#[derive(ClapArgs, Debug)]
pub struct WebhookCreateArgs {
    #[command(flatten)]
    pub target: NetworkArgs,

    #[arg(long)]
    pub body: Option<String>,

    #[arg(long)]
    pub event_type: Option<String>,

    #[arg(long)]
    pub description: Option<String>,

    #[arg(long)]
    pub webhook_url: Option<String>,

    #[arg(long)]
    pub is_instant: Option<bool>,

    #[arg(long)]
    pub condition_json: Option<String>,

    #[arg(long)]
    pub period: Option<u64>,

    #[arg(long = "address", num_args = 1.., value_delimiter = ',')]
    pub addresses: Vec<String>,

    #[arg(long = "header", value_parser = parse_header)]
    pub headers: Vec<HeaderArg>,
}

#[derive(ClapArgs, Debug)]
pub struct WebhookUpdateArgs {
    #[command(flatten)]
    pub target: NetworkArgs,

    #[arg(long)]
    pub subscription_id: String,

    #[arg(long)]
    pub body: Option<String>,

    #[arg(long)]
    pub description: Option<String>,

    #[arg(long)]
    pub webhook_url: Option<String>,

    #[arg(long)]
    pub is_instant: Option<bool>,

    #[arg(long)]
    pub is_active: Option<bool>,

    #[arg(long)]
    pub condition_json: Option<String>,

    #[arg(long)]
    pub period: Option<u64>,

    #[arg(long = "address", num_args = 1.., value_delimiter = ',')]
    pub addresses: Vec<String>,

    #[arg(long = "header", value_parser = parse_header)]
    pub headers: Vec<HeaderArg>,
}

#[derive(ClapArgs, Debug)]
pub struct WebhookDeleteArgs {
    #[command(flatten)]
    pub target: NetworkArgs,

    #[arg(long)]
    pub subscription_id: String,

    #[arg(long = "header", value_parser = parse_header)]
    pub headers: Vec<HeaderArg>,
}

#[derive(ClapArgs, Debug)]
pub struct WebhookHistoryArgs {
    #[command(flatten)]
    pub target: NetworkArgs,

    #[arg(long)]
    pub subscription_id: Option<String>,

    #[arg(long)]
    pub body: Option<String>,

    #[arg(long = "header", value_parser = parse_header)]
    pub headers: Vec<HeaderArg>,
}

#[derive(ClapArgs, Debug)]
pub struct WebhookRawArgs {
    #[command(flatten)]
    pub target: NetworkArgs,

    #[arg(long, value_enum)]
    pub method: HttpMethod,

    #[arg(long)]
    pub path: String,

    #[arg(long)]
    pub body: Option<String>,

    #[arg(long = "header", value_parser = parse_header)]
    pub headers: Vec<HeaderArg>,
}

#[derive(ClapArgs, Debug)]
pub struct WebhookServeArgs {
    #[arg(long, default_value = "127.0.0.1")]
    pub host: String,

    #[arg(long, default_value_t = 3000)]
    pub port: u16,

    #[arg(long, default_value = "/webhooks/nodit")]
    pub path: String,

    #[arg(long)]
    pub output_file: Option<String>,

    #[arg(long, default_value_t = true)]
    pub print_body: bool,

    #[arg(long)]
    pub signing_key: Option<String>,
}

#[derive(ClapArgs, Debug)]
pub struct AptosNodeBaseArgs {
    #[arg(long = "header", value_parser = parse_header)]
    pub headers: Vec<HeaderArg>,
}

#[derive(ClapArgs, Debug)]
pub struct AptosRawArgs {
    #[arg(long, value_enum)]
    pub method: HttpMethod,

    #[arg(long)]
    pub path: String,

    #[arg(long)]
    pub body: Option<String>,

    #[arg(long = "header", value_parser = parse_header)]
    pub headers: Vec<HeaderArg>,
}

#[derive(ClapArgs, Debug)]
pub struct AptosAccountArgs {
    #[arg(long)]
    pub address: String,

    #[arg(long = "header", value_parser = parse_header)]
    pub headers: Vec<HeaderArg>,
}

#[derive(ClapArgs, Debug)]
pub struct AptosModuleArgs {
    #[arg(long)]
    pub address: String,

    #[arg(long)]
    pub module_name: String,

    #[arg(long = "header", value_parser = parse_header)]
    pub headers: Vec<HeaderArg>,
}

#[derive(ClapArgs, Debug)]
pub struct AptosTransactionArgs {
    #[arg(long)]
    pub hash: String,

    #[arg(long = "header", value_parser = parse_header)]
    pub headers: Vec<HeaderArg>,
}

#[derive(ClapArgs, Debug)]
pub struct StreamCommand {
    #[command(flatten)]
    pub target: NetworkArgs,

    #[arg(long)]
    pub url: Option<String>,

    #[arg(long)]
    pub subscribe: Option<String>,

    #[arg(long)]
    pub event_type: Option<String>,

    #[arg(long)]
    pub condition_json: Option<String>,

    #[arg(long)]
    pub id: Option<u64>,

    #[arg(long)]
    pub period: Option<u64>,

    #[arg(long = "address", num_args = 1.., value_delimiter = ',')]
    pub addresses: Vec<String>,

    #[arg(long, default_value_t = 1)]
    pub messages: usize,
}

#[derive(Clone, Debug)]
pub struct HeaderArg {
    pub name: String,
    pub value: String,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, ValueEnum)]
pub enum OutputFormat {
    Pretty,
    Json,
    Raw,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, ValueEnum)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Patch,
    Delete,
}

impl From<HttpMethod> for reqwest::Method {
    fn from(value: HttpMethod) -> Self {
        match value {
            HttpMethod::Get => reqwest::Method::GET,
            HttpMethod::Post => reqwest::Method::POST,
            HttpMethod::Put => reqwest::Method::PUT,
            HttpMethod::Patch => reqwest::Method::PATCH,
            HttpMethod::Delete => reqwest::Method::DELETE,
        }
    }
}

fn parse_header(input: &str) -> Result<HeaderArg, String> {
    let Some((name, value)) = input.split_once('=') else {
        return Err("header must use NAME=VALUE format".to_string());
    };

    Ok(HeaderArg {
        name: name.to_string(),
        value: value.to_string(),
    })
}
