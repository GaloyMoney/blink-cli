#![cfg_attr(feature = "fail-on-warnings", deny(warnings))]
#![cfg_attr(feature = "fail-on-warnings", deny(clippy::all))]

use clap::{Parser, Subcommand};
use log::{self, info, LevelFilter};
use url::Url;

use galoy_client::GaloyClient;

use anyhow::Context;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(short, long, value_parser, env = "GALOY_API")]
    api: Option<String>,

    #[clap(short, long, value_parser, default_value_t = false)]
    debug: bool,

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
}

fn main() -> anyhow::Result<()> {
    log::set_max_level(LevelFilter::Warn);

    let cli = Cli::parse();
    if cli.debug {
        log::set_max_level(LevelFilter::Debug);
    }

    let api = cli
        .api
        .unwrap_or_else(|| String::from("http://localhost:4002/graphql"));

    Url::parse(&api).context(format!("API: {api} is not valid"))?;

    info!("using api: {api}");

    let galoy_client = GaloyClient::new(api);

    match cli.command {
        Commands::Getinfo {} => {
            let result = galoy_client.globals()?;
            let serialized_str = serde_json::to_string(&result)?;
            println!("{}", serialized_str);
        }
        Commands::DefaultWallet { username } => {
            let result = galoy_client.default_wallet(username)?;
            let serialized_str = serde_json::to_string(&result)?;
            println!("{}", serialized_str);
        }
    };

    Ok(())
}
