use anyhow::Context;

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
    ) -> anyhow::Result<()> {
        let recipient_wallet_id = self.client.default_wallet(username.clone()).await?;

        match (wallet, sats, cents) {
            (Wallet::Btc, Some(sats), _) => {
                let btc_wallet_id = self.get_user_btc_wallet_id().await?;

                self.client
                    .intraleger_send_btc(btc_wallet_id, recipient_wallet_id, sats, memo)
                    .await
                    .context("Error occurred while executing BTC intraledger payment")?;

                println!("Successfully sent {} sats to username: {}", sats, username);
            }
            (Wallet::Usd, _, Some(cents)) => {
                let usd_wallet_id = self.get_user_usd_wallet_id().await?;

                self.client
                    .intraleger_send_usd(usd_wallet_id, recipient_wallet_id, cents, memo)
                    .await
                    .context("Error occurred while sending USD intraledger payment")?;

                println!(
                    "Successfully sent {} cents to username: {}",
                    cents, username
                );
            }
            _ => {}
        }
        Ok(())
    }
}
