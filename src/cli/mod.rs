use clap::{Parser, Subcommand};

use crate::app::App;

#[derive(Parser)]
#[clap(
    version,
    author = "Galoy",
    about = "Galoy CLI",
    long_about = "CLI client to interact with Galoy's APIs"
)]
struct Cli {
    #[clap(
        long,
        env = "GALOY_API",
        default_value = "http://localhost:4002/graphql"
    )]
    api: String,

    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Get global values from the instance
    Globals,
}

pub async fn run() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let app = App::new(cli.api)?;

    match cli.command {
        Command::Globals => {
            app.globals().await?;
        }
    }

    Ok(())
}
