use clap::Parser;

use crate::app::App;
use crate::cli::commands::{Cli, Command};

pub async fn run() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let app = App::new(cli.api)?;

    match cli.command {
        Command::Globals => {
            app.globals().await?;
        }
        Command::Login { phone, code } => {
            app.user_login(phone, code).await?;
        }
        Command::Logout => {
            app.user_logout().await?;
        }
        Command::Me => {
            app.me().await?;
        }
    }

    Ok(())
}
