use clap::{Parser, Subcommand};

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
}
