use anyhow::{Context, Result};

use rust_decimal::Decimal;

use crate::{app::App, client::types::Wallet};

impl App {
    pub async fn intraledger_payment(
        &self,
        username: String,
        wallet: Wallet,
        cents: Option<Decimal>,
        sats: Option<Decimal>,
        memo: Option<String>,
    ) -> Result<()> {
        match wallet {
            Wallet::Btc => {
                let sats = sats.expect("Can't unwrap sats");
                self.client
                    .intraleger_send_btc(username.clone(), sats, memo)
                    .await
                    .context("Error occurred while executing BTC intraledger payment")?;
                println!("Successfully sent {} sats to username: {}", sats, username);
            }
            Wallet::Usd => {
                let cents = cents.expect("Can't unwrap cents");
                self.client
                    .intraleger_send_usd(username.clone(), cents, memo)
                    .await
                    .context("Error occurred while sending USD intraledger payment")?;
                println!(
                    "Successfully sent {} cents to username: {}",
                    cents, username
                );
            }
        }

        Ok(())
    }
}
