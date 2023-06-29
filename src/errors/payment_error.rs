use thiserror::Error;

#[derive(Error, Debug)]
pub enum PaymentError {
    #[error("Failed to get {0} wallet")]
    FailedToGetWallet(String),
}
