use rust_decimal::Decimal;
use serde::Serialize;

#[derive(Debug, Clone, clap::ValueEnum, PartialEq, Eq)]
pub enum Wallet {
    Btc,
    Usd,
}

#[derive(Debug, Clone, clap::ValueEnum, PartialEq, Eq)]
pub enum AmountCurrency {
    SATS,
    USD,
}

#[derive(Debug, Serialize)]
pub struct WalletBalance {
    pub currency: String,
    pub balance: Decimal,
    pub id: Option<String>,
    pub default: bool,
}

#[derive(Debug, Clone, clap::ValueEnum, PartialEq, Eq)]
pub enum ReceiveVia {
    Onchain,
}
