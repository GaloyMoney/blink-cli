use thiserror::Error;

#[derive(Error, Debug)]
pub enum PaymentError {
    #[error("For BTC wallet, use --sats to specify amount")]
    AmountNotSpecifiedBTC,
    #[error("For USD wallet, use --cents to specify amount")]
    AmountNotSpecifiedUSD,
}
