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
        Command::Transactions {
            after,
            before,
            last,
            first,
            wallet_ids,
        } => {
            app.list_transactions(after, before, last, first, wallet_ids)
                .await?;
        }
        Command::DefaultWallet { username } => {
            app.default_wallet(username).await?;
        }
        Command::SetDefaultWallet { wallet, wallet_id } => {
            app.set_default_wallet(wallet, wallet_id).await?;
        }
        Command::Balance {
            btc,
            usd,
            wallet_ids,
        } => {
            app.wallet_balance(btc, usd, wallet_ids).await?;
        }
        Command::SetUsername { username } => {
            app.set_username(username).await?;
        }
        Command::Pay {
            username,
            wallet,
            cents,
            sats,
            memo,
        } => {
            app.intraledger_payment(username, wallet, cents, sats, memo)
                .await?;
        }
        Command::Receive { wallet, via } => {
            app.receive(wallet, via).await?;
        }
        Command::Batch {
            file,
            skip_confirmation,
        } => {
            app.batch_payment(file, skip_confirmation).await?;
        }
        Command::RequestPhoneCode { phone } => {
            app.request_phone_code(phone).await?;
        }
    }

    Ok(())
}
