use thiserror::Error;

#[derive(Error, Debug)]
pub enum MeError {
    #[error("Issue getting response")]
    IssueGettingResponse,
    #[error("Issue parsing response")]
    IssueParsingResponse,
    #[error("Failed to unwrap .me")]
    FailedToUnwrapMe,
}
