use thiserror::Error;

#[derive(Error, Debug)]
pub enum MeError {
    #[error("Failed to unwrap .me")]
    FailedToUnwrapMe,
}
