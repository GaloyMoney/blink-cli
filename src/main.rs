#![cfg_attr(feature = "fail-on-warnings", deny(warnings))]
#![cfg_attr(feature = "fail-on-warnings", deny(clippy::all))]

use clap::{Parser, Subcommand};
use log::{self, info, LevelFilter};
use url::Url;

use galoy_cli::GaloyClient;

use anyhow::Context;

use rust_decimal::Decimal;

use std::fs::{self};
mod constants;
mod token;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(
        short,
        long,
        value_parser,
        env = "GALOY_API",
        default_value = "http://localhost:4002/graphql"
    )]
    api: String,

    #[clap(short, long, value_parser, default_value_t = false)]
    debug: bool,

    #[clap(short, long, value_parser)]
    token: Option<String>,

    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Get globals value from the instance
    Getinfo {},
    /// Request a code from a Phone number
    RequestPhoneCode {
        #[clap(value_parser)]
        phone: String,

        #[clap(long, action)]
        nocaptcha: bool,
    },
    /// get auth token of an account
    Login { phone: String, code: String },
    /// logout the current user by deleting token file
    Logout,
    /// Get WalletId for an account
    DefaultWallet {
        #[clap(value_parser)]
        username: String,
    },
    /// Execute Me query
    Me,
    /// Execute a Payment
    Pay {
        #[clap(short, long)]
        username: String,
        #[clap(short, long, value_parser)]
        wallet: Wallet,
        #[clap(short, long)]
        cents: Option<Decimal>,
        #[clap(short, long)]
        sats: Option<Decimal>,
        #[clap(short, long)]
        memo: Option<String>,
    },
    /// execute a batch payment
    Batch { filename: String, price: Decimal },
}

#[derive(Debug, Clone, clap::ValueEnum, PartialEq, Eq)]
enum Wallet {
    BTC,
    USD,
}

fn main() -> anyhow::Result<()> {
    log::set_max_level(LevelFilter::Warn);

    let cli = Cli::parse();
    if cli.debug {
        log::set_max_level(LevelFilter::Debug);
    }

    let api = cli.api;

    Url::parse(&api).context(format!("API: {api} is not valid"))?;

    let mut token: Option<String> = cli.token;
    let token_file = token::get_token_file_path()?;

    if token_file.exists() {
        token = Some(fs::read_to_string(&token_file).with_context(|| {
            format!("failed to read token from file '{}'", token_file.display())
        })?);
    }

    info!("using api: {api} and token: {:?}", &token);
    let galoy_cli = GaloyClient::new(api, token);

    match cli.command {
        Commands::Getinfo {} => {
            let result = galoy_cli.globals()?;
            println!("{:#?}", result);
        }
        Commands::DefaultWallet { username } => {
            let result = galoy_cli.default_wallet(username)?;
            println!("{:#?}", result);
        }
        Commands::Me => {
            let result = galoy_cli.me().context("can't get me")?;
            println!("{:#?}", result);
        }
        Commands::Pay {
            username,
            wallet,
            cents,
            sats,
            memo,
        } => {
            if (wallet == Wallet::BTC && sats.is_none())
                || (wallet == Wallet::USD && cents.is_none())
            {
                eprintln!("Appropriate amount (sats/cents) not provided");
                std::process::exit(1);
            }

            match wallet {
                Wallet::BTC => {
                    let sats = sats.expect("Can't unwrap sats");
                    let result = galoy_cli
                        .intraleger_send(username, sats, memo)
                        .context("issue sending intraledger")?;
                    println!("{:?}", result);
                }
                Wallet::USD => {
                    let cents = cents.expect("Can't unwrap cents");
                    let result = galoy_cli
                        .intraleger_usd_send(username, cents, memo)
                        .context("issue sending intraledger usd")?;
                    println!("{:?}", result);
                }
            }
        }
        Commands::RequestPhoneCode { phone, nocaptcha } => {
            galoy_cli
                .request_phone_code(phone, nocaptcha)
                .expect("issue getting code");
        }
        Commands::Login { phone, code } => {
            let result = galoy_cli
                .user_login(phone, code)
                .context("issue logging in")?;
            token::save_token(&token_file, &result)?;
        }
        Commands::Logout => {
            token::remove_token(&token_file).expect("Failed to remove token");
        }
        Commands::Batch { filename, price } => {
            let result = galoy_cli
                .batch(filename, price)
                .context("issue batching payment");
            println!("{:#?}", result);
        }
    };

    Ok(())
}
