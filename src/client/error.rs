use thiserror::Error;

#[derive(Debug, Error)]
pub enum CliError {
    #[error("CaptchaInnerError - Error: {0}")]
    CaptchaInnerError(String),
    #[error("CaptchaTopLevelError - Error: {0:?}")]
    CaptchaTopLevelError(Vec<graphql_client::Error>),
    #[error("Reqwest: {0}")]
    Reqwest(#[from] reqwest::Error),
}
