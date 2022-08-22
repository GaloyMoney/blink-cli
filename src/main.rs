use clap::{Parser, Subcommand};
use log::{self, info, LevelFilter};
use url::Url;

use galoy_client::default_wallet::default_wallet;
use galoy_client::globals::globals;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(short, long, value_parser)]
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

fn main() {
    log::set_max_level(LevelFilter::Warn);

    let cli = Cli::parse();
    if cli.debug {
        log::set_max_level(LevelFilter::Debug);
    }

    let api_env = std::env::var("SERVER").unwrap_or(String::from("http://localhost:4002/graphql"));

    let api = match cli.api {
        None => api_env,
        Some(value) => value,
    };

    Url::parse(&api).expect(&format!("Api url \"{api}\" is not valid"));

    info!("using api: {api}");

    match cli.command {
        Commands::Getinfo {} => {
            let result = globals(&api).unwrap();
            let serialized_str = serde_json::to_string(&result).unwrap();
            println!("{}", serialized_str);
        }
        Commands::DefaultWallet { username } => {
            let result = default_wallet(&api, &username).unwrap();
            let serialized_str = serde_json::to_string(&result).unwrap();
            println!("{}", serialized_str);
        }
    };
}
