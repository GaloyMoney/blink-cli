use csv::StringRecord;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PaymentError {
    #[error("Failed to get {0} wallet")]
    FailedToGetWallet(String),
    #[error("File does not exist: {0}")]
    FileNotFound(String),
    #[error("CSV format not correct, requires: username, (cents or sats), memo(optional)")]
    IncorrectCSVFormat,
    #[error("Failed to read CSV file.")]
    FailedToReadCSV,
    #[error("Failed to get headers of CSV file.")]
    FailedToGetHeaders,
    #[error("Failed to collect records from CSV file.")]
    FailedToGetRecords,
    #[error("No username found for the record {:?}", 0)]
    NoUsernameFound(StringRecord),
    #[error("Username {0} does not exist")]
    UsernameDoesNotExist(String),
    #[error("Insufficient balance in the sender's wallet")]
    InsufficientBalance,
}
