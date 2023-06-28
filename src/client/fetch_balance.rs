use super::types::{Wallet, WalletBalance};
use std::collections::HashSet;

use super::GaloyClient;

impl GaloyClient {
    pub async fn fetch_balance(
        &self,
        btc: bool,
        usd: bool,
        wallet_ids: Vec<String>,
    ) -> anyhow::Result<Vec<WalletBalance>> {
        let me = self.me().await?;
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

        if balances.is_empty() {
            Err(anyhow::anyhow!("No matching wallet found"))
        } else {
            Ok(balances)
        }
    }
}
