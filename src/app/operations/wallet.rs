use std::collections::HashSet;

use anyhow::{Context, Result};

use crate::{
    app::{errors::payment_error::PaymentError, App},
    client::{
        queries::query_me::{QueryMeMeDefaultAccountWallets, WalletCurrency},
        types::{Wallet, WalletBalance},
    },
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

    pub async fn set_default_wallet(
        &self,
        wallet: Option<Wallet>,
        wallet_id: Option<String>,
    ) -> Result<()> {
        let me = self.client.me().await?;
        let wallets = me.default_account.wallets;

        let wallet_id = if let Some(wallet_id) = wallet_id {
            wallet_id
        } else {
            match wallet {
                Some(Wallet::Btc) => self.get_user_btc_wallet_id(wallets)?,
                Some(Wallet::Usd) => self.get_user_usd_wallet_id(wallets)?,
                None => {
                    return Err(anyhow::anyhow!(
                        "Either 'wallet' or 'wallet_id' must be provided."
                    ))
                }
            }
        };

        self.client
            .update_default_wallet(wallet_id.clone())
            .await
            .context("Failed to update default wallet")?;

        println!(
            "Default wallet ID has been successfully set to {}",
            wallet_id
        );

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

    pub fn get_user_btc_wallet_id(
        &self,
        wallets: Vec<QueryMeMeDefaultAccountWallets>,
    ) -> Result<String> {
        let btc_wallet_id = wallets
            .iter()
            .find(|wallet| wallet.wallet_currency == WalletCurrency::BTC)
            .map(|wallet| &wallet.id)
            .ok_or_else(|| PaymentError::FailedToGetWallet("BTC".to_string()))
            .map(|id| id.to_owned())?;
        Ok(btc_wallet_id)
    }

    pub fn get_user_usd_wallet_id(
        &self,
        wallets: Vec<QueryMeMeDefaultAccountWallets>,
    ) -> Result<String> {
        let usd_wallet_id = wallets
            .iter()
            .find(|wallet| wallet.wallet_currency == WalletCurrency::USD)
            .map(|wallet| &wallet.id)
            .ok_or_else(|| PaymentError::FailedToGetWallet("USD".to_string()))
            .map(|id| id.to_owned())?;
        Ok(usd_wallet_id)
    }
}
