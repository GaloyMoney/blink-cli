use crate::client::queries::query_me::WalletCurrency;
use crate::types::Wallet;

pub fn wallet_to_currency(wallet: &Wallet) -> WalletCurrency {
    match wallet {
        Wallet::Usd => WalletCurrency::USD,
        Wallet::Btc => WalletCurrency::BTC,
    }
}
