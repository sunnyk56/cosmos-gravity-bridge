//! Command line argument definitions for Gravity bridge tools
//! See the clap documentation for how exactly this works, note that doc comments are displayed to the user

use clap::AppSettings;
use clap::Clap;
use clarity::PrivateKey as EthPrivateKey;
use deep_space::PrivateKey as CosmosPrivateKey;

/// Gravity Bridge tools (gbt) provides tools for interacting with the Althea Gravity bridge for Cosmos based blockchains.
#[derive(Clap)]
#[clap(version = "1.0", author = "Justin Kilpatrick <justin@althea.net>")]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Opts {
    /// Increase the logging verbosity
    #[clap(short, long)]
    verbose: bool,
    /// Decrease the logging verbosity
    #[clap(short, long)]
    quiet: bool,
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    Orchestrator(OrchestratorOpts),
    Relayer(RelayerOpts),
    Client(ClientOpts),
    Keys(KeyOpts),
}
/// The Gravity Bridge orchestrator is required for all validators of the Cosmos chain running
/// the Gravity Bridge module. It contains an Ethereum Signer, Oracle, and optional relayer
#[derive(Clap)]
#[clap(setting = AppSettings::ColoredHelp)]
struct OrchestratorOpts {}

/// The Gravity Bridge Relayer is an unpermissioned role that takes data from the Cosmos blockchain
/// packages it into Ethereum transactions and is paid to submit these transactions to the Ethereum blockchain
#[derive(Clap)]
#[clap(setting = AppSettings::ColoredHelp)]
struct RelayerOpts {}

/// The Gravity Bridge client contains helpful command line tools for interacting with the Gravity bridge
#[derive(Clap)]
#[clap(setting = AppSettings::ColoredHelp)]
struct ClientOpts {
    #[clap(subcommand)]
    subcmd: ClientSubcommand,
}

#[derive(Clap)]
enum ClientSubcommand {
    CosmosToEth(CosmosToEthOpts),
    EthToCosmos(EthToCosmosOpts),
    DeployErc20Representation(DeployErc20RepresentationOpts),
}

/// Send Cosmos tokens to Ethereum
#[derive(Clap)]
#[clap(setting = AppSettings::ColoredHelp)]
struct CosmosToEthOpts {}

/// Send an Ethereum ERC20 token to Cosmos
#[derive(Clap)]
#[clap(setting = AppSettings::ColoredHelp)]
struct EthToCosmosOpts {
    /// (Optional) The Ethereum private key to register, will be generated if not provided
    #[clap(short, long, parse(try_from_str))]
    ethereum_key: EthPrivateKey,
}

/// Deploy an ERC20 representation of a Cosmos asset on the Ethereum chain
/// this can only be run once for each time of Cosmos asset
#[derive(Clap)]
#[clap(setting = AppSettings::ColoredHelp)]
struct DeployErc20RepresentationOpts {}

/// Manage keys
#[derive(Clap)]
#[clap(setting = AppSettings::ColoredHelp)]
struct KeyOpts {
    #[clap(subcommand)]
    subcmd: KeysSubcommand,
}

#[derive(Clap)]
enum KeysSubcommand {
    SetOrchestratorAddress(SetOrchestratorAddress),
}

/// Register delegate keys for the Gravity Orchestrator.
/// this is a mandatory part of setting up a Gravity Orchestrator
/// If you would like sign using a ledger see `cosmos tx gravity set-orchestrator-address` instead
#[derive(Clap)]
#[clap(setting = AppSettings::ColoredHelp)]
struct SetOrchestratorAddress {
    /// The Cosmos private key of the validator
    #[clap(short, long, parse(try_from_str))]
    validator_phrase: CosmosPrivateKey,
    /// (Optional) The Ethereum private key to register, will be generated if not provided
    #[clap(short, long, parse(try_from_str))]
    ethereum_key: Option<EthPrivateKey>,
    /// (Optional) The phrase for the Cosmos key to register, will be generated if not provided.
    #[clap(short, long, parse(try_from_str))]
    cosmos_phrase: Option<CosmosPrivateKey>,
    ///The prefix for Addresses on this chain (eg 'cosmos')
    #[clap(short, long)]
    address_prefix: String,
    /// (Optional) The Cosmos RPC url, usually the validator. Default is localhost:9090
    #[clap(short, long)]
    cosmos_grpc: Option<String>,
    /// The Cosmos Denom in which to pay Cosmos chain fees
    #[clap(short, long)]
    fees: String,
}
