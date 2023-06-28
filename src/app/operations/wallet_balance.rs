use anyhow::{Context, Result};

use crate::app::App;

impl App {
    pub async fn wallet_balance(
        &self,
        btc: bool,
        usd: bool,
        wallet_ids: Vec<String>,
    ) -> Result<()> {
        match self.client.fetch_balance(btc, usd, wallet_ids).await {
            Ok(balances) => {
                let balances_json =
                    serde_json::to_string_pretty(&balances).context("Can't serialize json")?;
                println!("{}", balances_json);
                Ok(())
            }
            Err(err) => {
                println!("Error occurred while fetching wallet balances");
                Err(err)
            }
        }
    }
}
