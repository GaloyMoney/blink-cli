use thiserror::Error;

use self::{
    api_error::ApiError, me_error::MeError, payment_error::PaymentError,
    set_username_error::SetUsernameError, token_error::TokenError,
};

pub mod api_error;
pub mod me_error;
pub mod payment_error;
pub mod set_username_error;
pub mod token_error;

#[derive(Error, Debug)]
pub enum CliError {
    #[error(transparent)]
    MeError(#[from] MeError),
    #[error(transparent)]
    ApiError(#[from] ApiError),
    #[error(transparent)]
    TokenError(#[from] TokenError),
    #[error(transparent)]
    SetUsernameError(#[from] SetUsernameError),
    #[error(transparent)]
    PaymentError(#[from] PaymentError),
}
