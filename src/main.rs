use anyhow::Result;
use clap::{CommandFactory, Parser, Subcommand};
use colored::Colorize;

use crate::{provider::ProviderArgs, subcommands::*};

mod account;
mod account_factory;
mod address_book;
mod casm;
mod chain_id;
mod compiler;
mod decode;
mod fee;
mod network;
mod path;
mod provider;
mod signer;
mod subcommands;
mod utils;
mod verbosity;

const VERSION_STRING: &str = concat!(env!("CARGO_PKG_VERSION"), " (", env!("VERGEN_GIT_SHA"), ")");
const VERSION_STRING_VERBOSE: &str = concat!(
    env!("CARGO_PKG_VERSION"),
    " (",
    env!("VERGEN_GIT_SHA"),
    ")\n",
    "JSON-RPC version: 0.4.0"
);

#[derive(Debug, Parser)]
#[clap(author, about)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Subcommands>,
    #[clap(long = "version", short = 'V', help = "Print version info and exit")]
    version: bool,
    #[clap(
        long = "verbose",
        short = 'v',
        help = "Use verbose output (currently only applied to version)"
    )]
    verbose: bool,
}

#[derive(Debug, Subcommand)]
enum Subcommands {
    //
    // Local utilities
    //
    #[clap(about = "Calculate selector from name")]
    Selector(Selector),
    #[clap(about = "Calculate class hash from any contract artifacts (Sierra, casm, legacy)")]
    ClassHash(ClassHash),
    #[clap(about = "Encode string into felt with the Cairo short string representation")]
    ToCairoString(ToCairoString),
    #[clap(about = "Decode string from felt with the Cairo short string representation")]
    ParseCairoString(ParseCairoString),
    #[clap(about = "Print the montgomery representation of a field element")]
    Mont(Mont),
    //
    // JSON-RPC query client
    //
    #[clap(about = "Call contract functions without sending transactions")]
    Call(Call),
    #[clap(alias = "tx", about = "Get Starknet transaction by hash")]
    Transaction(Transaction),
    #[clap(alias = "bn", about = "Get latest block number")]
    BlockNumber(BlockNumber),
    #[clap(about = "Get latest block hash")]
    BlockHash(BlockHash),
    #[clap(about = "Get Starknet block")]
    Block(Block),
    #[clap(about = "Get Starknet block timestamp only")]
    BlockTime(BlockTime),
    #[clap(about = "Get state update from a certain block")]
    StateUpdate(StateUpdate),
    #[clap(alias = "receipt", about = "Get transaction receipt by hash")]
    TransactionReceipt(TransactionReceipt),
    #[clap(about = "Get Starknet network ID")]
    ChainId(ChainId),
    #[clap(about = "Get native gas token (currently ETH) balance")]
    Balance(Balance),
    #[clap(about = "Get nonce for a certain contract")]
    Nonce(Nonce),
    #[clap(about = "Get storage value for a slot at a contract")]
    Storage(Storage),
    #[clap(about = "Get contract class hash deployed at a certain address")]
    ClassHashAt(ClassHashAt),
    #[clap(about = "Get contract class by hash")]
    ClassByHash(ClassByHash),
    #[clap(about = "Get contract class deployed at a certain address")]
    ClassAt(ClassAt),
    #[clap(about = "Get node syncing status")]
    Syncing(Syncing),
    //
    // Signer management
    //
    #[clap(about = "Signer management commands")]
    Signer(Signer),
    //
    // Account management
    //
    #[clap(about = "Account management commands")]
    Account(Account),
    //
    // Sending out transactions
    //
    #[clap(about = "Send an invoke transaction from an account contract")]
    Invoke(Invoke),
    #[clap(about = "Declare a contract class")]
    Declare(Declare),
    #[clap(about = "Deploy contract via the Universal Deployer Contract")]
    Deploy(Deploy),
    //
    // Misc
    //
    #[clap(about = "Generate shell completions script")]
    Completions(Completions),
    //
    // Experimental
    //
    #[clap(
        about = "Experimental commands for fun and profit",
        long_about = "Experimental new commands that are shipped with no stability guarantee. \
            They might break or be removed anytime."
    )]
    Lab(Lab),
}

#[tokio::main]
async fn main() {
    if let Err(err) = run_command(Cli::parse()).await {
        eprintln!("{}", format!("Error: {err}").red());
        std::process::exit(1);
    }
}

async fn run_command(cli: Cli) -> Result<()> {
    match (cli.version, cli.command) {
        (false, None) => Ok(Cli::command().print_help()?),
        (true, _) => {
            println!(
                "{}",
                if cli.verbose {
                    VERSION_STRING_VERBOSE
                } else {
                    VERSION_STRING
                }
            );

            Ok(())
        }
        (false, Some(command)) => match command {
            Subcommands::Selector(cmd) => cmd.run(),
            Subcommands::ClassHash(cmd) => cmd.run(),
            Subcommands::ToCairoString(cmd) => cmd.run(),
            Subcommands::ParseCairoString(cmd) => cmd.run(),
            Subcommands::Mont(cmd) => cmd.run(),
            Subcommands::Call(cmd) => cmd.run().await,
            Subcommands::Transaction(cmd) => cmd.run().await,
            Subcommands::BlockNumber(cmd) => cmd.run().await,
            Subcommands::BlockHash(cmd) => cmd.run().await,
            Subcommands::Block(cmd) => cmd.run().await,
            Subcommands::BlockTime(cmd) => cmd.run().await,
            Subcommands::StateUpdate(cmd) => cmd.run().await,
            Subcommands::TransactionReceipt(cmd) => cmd.run().await,
            Subcommands::ChainId(cmd) => cmd.run().await,
            Subcommands::Balance(cmd) => cmd.run().await,
            Subcommands::Nonce(cmd) => cmd.run().await,
            Subcommands::Storage(cmd) => cmd.run().await,
            Subcommands::ClassHashAt(cmd) => cmd.run().await,
            Subcommands::ClassByHash(cmd) => cmd.run().await,
            Subcommands::ClassAt(cmd) => cmd.run().await,
            Subcommands::Syncing(cmd) => cmd.run().await,
            Subcommands::Signer(cmd) => cmd.run(),
            Subcommands::Account(cmd) => cmd.run().await,
            Subcommands::Invoke(cmd) => cmd.run().await,
            Subcommands::Declare(cmd) => cmd.run().await,
            Subcommands::Deploy(cmd) => cmd.run().await,
            Subcommands::Completions(cmd) => cmd.run(),
            Subcommands::Lab(cmd) => cmd.run(),
        },
    }
}
