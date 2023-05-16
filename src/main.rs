#![cfg_attr(feature = "fail-on-warnings", deny(warnings))]
#![cfg_attr(feature = "fail-on-warnings", deny(clippy::all))]

use clap::{Parser, Subcommand};
use log::{self, info, LevelFilter};
use url::Url;

use galoy_cli::GaloyClient;

use anyhow::Context;

use jsonwebtoken::decode_header;

use rust_decimal::Decimal;

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

    #[clap(short, long, value_parser, env = "GALOY_JWT", hide_env_values = true)]
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

    let jwt = cli.jwt;

    if let Some(jwt) = &jwt {
        decode_header(jwt).context("jwt syntax issue")?;
    }

    info!("using api: {api} and jwt: {:?}", &jwt);
    let galoy_client = GaloyClient::new(api, jwt);

    match cli.command {
        Commands::Getinfo {} => {
            let result = galoy_client.globals()?;
            println!("{:#?}", result);
        }
        Commands::DefaultWallet { username } => {
            let result = galoy_client.default_wallet(username)?;
            println!("{:#?}", result);
        }
        Commands::Me => {
            let result = galoy_client.me().context("can't get me")?;
            println!("{:#?}", result);
        }
        Commands::SendIntraledger {
            username,
            amount,
            memo,
        } => {
            let result = galoy_client
                .intraleger_send(username, amount, memo)
                .context("issue sending intraledger")?;
            println!("{:#?}", result);
        }
        Commands::RequestPhoneCode { phone, nocaptcha } => {
            galoy_client
                .request_phone_code(phone, nocaptcha)
                .expect("issue getting code");
        }
        Commands::Login { phone, code } => {
            let result = galoy_client
                .user_login(phone, code)
                .context("issue logging in")?;
            println!("{:#?}", result);
        }
        Commands::Batch { filename, price } => {
            let result = galoy_client
                .batch(filename, price)
                .context("issue batching payment");
            println!("{:#?}", result);
        }
    };

    Ok(())
}
