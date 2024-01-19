use anyhow::Context;
use rust_decimal::Decimal;

use crate::{
    app::App,
    client::types::{ReceiveVia, Wallet},
};

impl App {
    pub async fn send_onchain(
        &self,
        onchain_address: String,
        wallet: Wallet,
        cents: Option<Decimal>,
        sats: Option<Decimal>,
        memo: Option<String>,
    ) -> anyhow::Result<()> {
        match (wallet, sats, cents) {
            (Wallet::Btc, Some(sats), _) => {
                let btc_wallet_id = self.get_user_btc_wallet_id().await?;
                self.client
                    .onchain_payment_send(btc_wallet_id, onchain_address.clone(), sats, memo)
                    .await
                    .context("Error occurred while executing BTC onchain payment")?;

                println!(
                    "Successfully sent {} sats to address: {}",
                    sats, onchain_address
                );
            }
            (Wallet::Usd, _, Some(cents)) => {
                let usd_wallet_id = self.get_user_usd_wallet_id().await?;
                self.client
                    .onchain_payment_send(usd_wallet_id, onchain_address.clone(), cents, memo)
                    .await
                    .context("Error occurred while executing USD onchain payment")?;

                println!(
                    "Successfully sent {} cents to address: {}",
                    cents, onchain_address
                );
            }
            _ => {}
        }

        Ok(())
    }

    pub async fn receive(&self, wallet: Wallet, via: ReceiveVia) -> anyhow::Result<()> {
        let receiving_wallet_id = match wallet {
            Wallet::Btc => self.get_user_btc_wallet_id().await?,
            Wallet::Usd => self.get_user_usd_wallet_id().await?,
        };

        match via {
            ReceiveVia::Onchain => {
                let data = self
                    .client
                    .onchain_address_current(receiving_wallet_id)
                    .await
                    .context("Error occurred while fetching 'onchain address' data")?;

                println!("{}", serde_json::to_string_pretty(&data)?);
            }
        }

        Ok(())
    }
}
