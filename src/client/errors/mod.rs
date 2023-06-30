use thiserror::Error;

use self::{api_error::ApiError, me_error::MeError};

pub mod api_error;
pub mod me_error;

#[derive(Error, Debug)]
pub enum ClientError {
    #[error(transparent)]
    MeError(#[from] MeError),
    #[error(transparent)]
    ApiError(#[from] ApiError),
}
