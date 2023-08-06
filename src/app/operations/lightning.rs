use anyhow::Context;
use rust_decimal::Decimal;

use crate::{app::App, client::types::Wallet};

impl App {
    pub async fn ln_invoice_create(
        &self,
        wallet: Wallet,
        amount: Decimal,
        memo: Option<String>,
    ) -> anyhow::Result<()> {
        let receiving_wallet_id = match wallet {
            Wallet::Btc => self.get_user_btc_wallet_id().await?,
            Wallet::Usd => self.get_user_usd_wallet_id().await?,
        };

        match wallet {
            Wallet::Btc => {
                let data = self
                    .client
                    .lightning_invoice_create_btc(receiving_wallet_id, amount, memo)
                    .await
                    .context("Error occurred while creating BTC lightning invoice.")?;

                println!("{}", serde_json::to_string_pretty(&data.invoice)?);
            }
            Wallet::Usd => {
                let data = self
                    .client
                    .lightning_invoice_create_usd(receiving_wallet_id, amount, memo)
                    .await
                    .context("Error occurred while creating USD lightning invoice.")?;

                println!("{}", serde_json::to_string_pretty(&data.invoice)?);
            }
        }

        Ok(())
    }
}
