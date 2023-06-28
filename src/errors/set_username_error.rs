use thiserror::Error;

#[derive(Error, Debug)]
pub enum SetUsernameError {
    #[error("Failed to update username")]
    FailedToUpdateUsername,
}
