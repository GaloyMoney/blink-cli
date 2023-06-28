use anyhow::Context;

use crate::app::App;

impl App {
    pub async fn wallet_balance(
        &self,
        btc: bool,
        usd: bool,
        wallet_ids: Vec<String>,
    ) -> anyhow::Result<()> {
        let balances = self.client.fetch_balance(btc, usd, wallet_ids).await?;
        let balances_json =
            serde_json::to_string_pretty(&balances).context("Can't serialize json")?;
        println!("{}", balances_json);

        Ok(())
    }
}
