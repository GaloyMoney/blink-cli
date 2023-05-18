#![cfg_attr(feature = "fail-on-warnings", deny(warnings))]
#![cfg_attr(feature = "fail-on-warnings", deny(clippy::all))]

use clap::{Parser, Subcommand};
use log::{self, info, LevelFilter};
use url::Url;

use galoy_cli::GaloyClient;

use anyhow::Context;

use rust_decimal::Decimal;

use dirs;
use std::fs::{self, File};
use std::io::Write;

#[derive(Parser, Debug)]
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

    #[clap(short, long, value_parser, default_value = "None")]
    jwt: Option<String>,

    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Get globals value from the instance
    Getinfo {},
    /// Get WalletId for an account
    DefaultWallet {
        #[clap(value_parser)]
        username: String,
    },
    /// Execute Me query
    Me,
    /// Do an intraledger transaction
    SendIntraledger {
        #[clap(value_parser)]
        username: String,
        amount: Decimal,
        memo: Option<String>,
    },
    /// Request a code from a Phone number
    RequestPhoneCode {
        #[clap(value_parser)]
        phone: String,

        #[clap(long, action)]
        nocaptcha: bool,
    },
    /// get JWT of an account
    Login { phone: String, code: String },
    /// execute a batch payment
    Batch { filename: String, price: Decimal },
}

fn main() -> anyhow::Result<()> {
    log::set_max_level(LevelFilter::Warn);

    let cli = Cli::parse();
    if cli.debug {
        log::set_max_level(LevelFilter::Debug);
    }

    let api = cli.api;

    Url::parse(&api).context(format!("API: {api} is not valid"))?;

    let mut jwt = cli.jwt;
    let home_dir = dirs::home_dir().expect("failed to get home directory");
    let token_file = home_dir.join(".galoy-cli/GALOY_JWT");

    if token_file.exists() {
        jwt = Some(fs::read_to_string(&token_file)
            .with_context(|| format!("failed to read token from file '{}'", token_file.display()))?);
    }

    info!("using api: {api} and jwt: {:?}", &jwt);
    let galoy_cli = GaloyClient::new(api, jwt);

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
        Commands::SendIntraledger {
            username,
            amount,
            memo,
        } => {
            let result = galoy_cli
                .intraleger_send(username, amount, memo)
                .context("issue sending intraledger")?;
            println!("{:#?}", result);
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

            let home_dir = dirs::home_dir().expect("failed to get home directory");
            let galoy_cli_dir = home_dir.join(".galoy-cli");
            
            fs::create_dir_all(&galoy_cli_dir)
                .with_context(|| format!("failed to create directory '{}'", galoy_cli_dir.display()))?;
            
            let token_file = galoy_cli_dir.join("GALOY_JWT");
            
            let mut file = File::create(&token_file)
                .with_context(|| format!("failed to create file '{}'", token_file.display()))?;
            
            file.write_all(result.as_bytes())
                .with_context(|| format!("failed to write to file '{}'", token_file.display()))?;
            
            println!("Token saved to {}", token_file.display());
                

            println!("{:#?}", result);
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
