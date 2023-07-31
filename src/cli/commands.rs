use clap::{Parser, Subcommand};

use crate::client::types::{ReceiveVia, Wallet};
use rust_decimal::Decimal;

#[derive(Parser)]
#[clap(
    version,
    author = "Galoy",
    about = "Galoy CLI",
    long_about = "CLI client to interact with Galoy's APIs"
)]
pub struct Cli {
    #[clap(
        long,
        env = "GALOY_API",
        default_value = "http://localhost:4002/graphql"
    )]
    pub api: String,

    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Get global values from the instance
    Globals,
    /// Get auth token of an account
    Login { phone: String, code: String },
    /// Logout the current user by removing the auth token
    Logout,
    /// Execute Me query
    Me,
    /// Get WalletId for an account
    DefaultWallet {
        #[clap(value_parser)]
        username: String,
    },
    // Update the default wallet of an account
    SetDefaultWallet {
        #[clap(short, long, value_parser, conflicts_with("wallet_id"))]
        wallet: Option<Wallet>,
        #[clap(long)]
        wallet_id: Option<String>,
    },
    /// Set a username for a new account
    SetUsername {
        #[clap(short, long)]
        username: String,
    },
    /// Fetch the balance of a wallet
    Balance {
        #[clap(long)]
        btc: bool,
        #[clap(long)]
        usd: bool,
        #[clap(long, use_value_delimiter = true)]
        wallet_ids: Vec<String>,
    },
    /// Execute a Payment
    Pay {
        #[clap(short, long)]
        username: String,
        #[clap(short, long, value_parser)]
        wallet: Wallet,
        #[clap(short, long, required_if_eq("wallet", "usd"))]
        cents: Option<Decimal>,
        #[clap(short, long, required_if_eq("wallet", "btc"))]
        sats: Option<Decimal>,
        #[clap(short, long)]
        memo: Option<String>,
    },
    /// Receive a Payment
    Receive {
        #[clap(short, long, value_parser)]
        wallet: Wallet,
        #[clap(short, long, value_parser)]
        via: ReceiveVia,
    },
    /// execute a batch payment
    Batch {
        #[clap(short, long = "csv")]
        file: String,
        #[clap(action, long)]
        skip_confirmation: bool,
    },
    /// Request a code from a Phone number
    RequestPhoneCode {
        #[clap(value_parser)]
        phone: String,
    },
}
