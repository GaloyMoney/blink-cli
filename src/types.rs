#[derive(Debug, Clone, clap::ValueEnum, PartialEq, Eq)]
pub enum Wallet {
    Btc,
    Usd,
}
