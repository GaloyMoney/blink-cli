use std::collections::HashSet;

use anyhow::{Context, Result};

use crate::{
    app::App,
    client::types::{Wallet, WalletBalance},
};

impl App {
    pub async fn default_wallet(&self, username: String) -> Result<()> {
        let result = self
            .client
            .default_wallet(username.clone())
            .await
            .context("Error occurred while fetching default wallet id")?;

        println!("Default wallet ID for {} is: {}", username, result);
        Ok(())
    }

    pub async fn wallet_balance(
        &self,
        btc: bool,
        usd: bool,
        wallet_ids: Vec<String>,
    ) -> Result<()> {
        let me = self.client.me().await?;
        let default_wallet_id = me.default_account.default_wallet_id;
        let wallets = &me.default_account.wallets;

        let wallet_ids_set: HashSet<_> = wallet_ids.into_iter().collect();

        let wallet_type = match (btc, usd) {
            (true, true) | (false, false) => None,
            (true, false) => Some(Wallet::Btc),
            (false, true) => Some(Wallet::Usd),
        };

        let balances: Vec<_> = wallets
            .iter()
            .filter(|wallet_info| {
                wallet_ids_set.contains(&wallet_info.id)
                    || wallet_type.as_ref().map_or(wallet_ids_set.is_empty(), |w| {
                        *w == Wallet::from(&wallet_info.wallet_currency)
                    })
            })
            .map(|wallet_info| WalletBalance {
                currency: format!("{:?}", Wallet::from(&wallet_info.wallet_currency)),
                balance: wallet_info.balance,
                id: Some(wallet_info.id.clone()),
                default: wallet_info.id == default_wallet_id,
            })
            .collect();

        let balances_json =
            serde_json::to_string_pretty(&balances).context("Can't serialize json")?;

        println!("{}", balances_json);
        Ok(())
    }
}
