use std::fmt::Display;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum GaloyCliError {
    #[error("FileIO: {0}")]
    FileIO(#[from] std::io::Error),
    #[error("CSVReader: Error reading CSV - {0}")]
    CSVReader(#[from] csv::Error),
    #[error("Reqwest: Error fetching response - {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("GraphQl: Error with graphql response - {0:?}")]
    GraphQl(Vec<graphql_client::Error>),
    #[error("BatchCreation: {kind:#?}: {message:?}")]
    Batching { kind: BatchError, message: String },
    #[error("UrlParsing: {0:?}")]
    Url(#[from] url::ParseError),
    #[error("JWT: {0:?}")]
    Jwt(#[from] jsonwebtoken::errors::Error),
    #[error("Authorization Code Status: {0}")]
    Authorization(bool),
}

#[derive(Debug, Error)]
pub enum BatchError {
    SelfPayment,
    DivisionByZero,
    NoBalance,
    InsufficientBalance,
    Empty,
}
impl Display for BatchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BatchError::SelfPayment => writeln!(f, "BatchError::SelfPayment"),
            BatchError::DivisionByZero => writeln!(f, "BatchError::DivisionByZero"),
            BatchError::NoBalance => writeln!(f, "BatchError::NoBalance"),
            BatchError::InsufficientBalance => writeln!(f, "BatchError::InsufficientBalance"),
            BatchError::Empty => writeln!(f, "BatchError::Empty"),
        }
    }
}

pub fn message_only_error(message: String) -> Vec<graphql_client::Error> {
    vec![graphql_client::Error {
        message,
        locations: None,
        path: None,
        extensions: None,
    }]
}
