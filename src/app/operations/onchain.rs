use anyhow::Context;

use crate::{
    app::App,
    client::types::{ReceiveVia, Wallet},
};

impl App {
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
