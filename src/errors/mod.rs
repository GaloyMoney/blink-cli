use thiserror::Error;

use self::{api_error::ApiError, me_error::MeError, token_error::TokenError};

pub mod api_error;
pub mod me_error;
pub mod token_error;

#[derive(Error, Debug)]
pub enum CliError {
    #[error(transparent)]
    MeError(#[from] MeError),
    #[error(transparent)]
    ApiError(#[from] ApiError),
    #[error(transparent)]
    TokenError(#[from] TokenError),
}
