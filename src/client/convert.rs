use super::{queries::query_me::WalletCurrency, types::Wallet};

impl From<&WalletCurrency> for Wallet {
    fn from(currency: &WalletCurrency) -> Self {
        match currency {
            WalletCurrency::USD => Wallet::Usd,
            WalletCurrency::BTC => Wallet::Btc,
            _ => panic!("Unsupported currency"),
        }
    }
}
