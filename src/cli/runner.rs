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
            onchain_address,
            wallet,
            cents,
            sats,
            memo,
            ln_payment_request,
        } => match (username, onchain_address, ln_payment_request) {
            (Some(username), None, None) => {
                app.intraledger_payment(username, wallet, cents, sats, memo)
                    .await?;
            }
            (None, Some(onchain_address), None) => {
                app.send_onchain(onchain_address, wallet, cents, sats, memo)
                    .await?;
            }
            (None, None, Some(ln_payment_request)) => {
                app.send_lightning(ln_payment_request, wallet, memo).await?;
            }
            _ => {}
        },
        Command::Receive { wallet, via } => {
            app.receive(wallet, via).await?;
        }
        Command::LnInvoice {
            wallet,
            amount,
            memo,
        } => {
            app.ln_invoice_create(wallet, amount, memo).await?;
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
