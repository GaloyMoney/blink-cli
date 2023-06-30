use anyhow::Error;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Issue getting response: {0}")]
    IssueGettingResponse(#[source] Error),
    #[error("Issue parsing response")]
    IssueParsingResponse,
    #[error("Request Failed with Error: {0}")]
    RequestFailedWithError(String),
}
