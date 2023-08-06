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

    pub async fn send_lightning(
        &self,
        ln_payment_request: String,
        wallet: Wallet,
        memo: Option<String>,
    ) -> anyhow::Result<()> {
        let sender_wallet_id = match wallet {
            Wallet::Btc => self.get_user_btc_wallet_id().await?,
            Wallet::Usd => self.get_user_usd_wallet_id().await?,
        };

        self.client
            .ln_payment_send(sender_wallet_id, ln_payment_request, memo)
            .await
            .context("Error occurred while sending payment to lightning invoice")?;

        println!("Successfully sent payment to the given lightning invoice",);

        Ok(())
    }
}
