use clap::{Parser, Subcommand};
use log::{self, info, LevelFilter};
use url::Url;

use galoy_client::GaloyClient;

use anyhow::{Context, Result};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(short, long, value_parser, env = "GALOYAPI")]
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

fn main() -> Result<()> {
    log::set_max_level(LevelFilter::Warn);

    let cli = Cli::parse();
    if cli.debug {
        log::set_max_level(LevelFilter::Debug);
    }

    let api = match cli.api {
        None => String::from("http://localhost:4002/graphql"),
        Some(value) => value,
    };

    Url::parse(&api).context(format!("API: {api} is not valid"))?;

    info!("using api: {api}");

    let galoy_client = GaloyClient::new(api);

    match cli.command {
        Commands::Getinfo {} => {
            let result = galoy_client.globals().unwrap();
            let serialized_str = serde_json::to_string(&result).unwrap();
            println!("{}", serialized_str);
        }
        Commands::DefaultWallet { username } => {
            let result = galoy_client.default_wallet(username).unwrap();
            let serialized_str = serde_json::to_string(&result).unwrap();
            println!("{}", serialized_str);
        }
    };

    return Ok(());
}
