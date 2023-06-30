use thiserror::Error;

use self::{payment_error::PaymentError, token_error::TokenError};

pub mod payment_error;
pub mod token_error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error(transparent)]
    PaymentError(#[from] PaymentError),
    #[error(transparent)]
    TokenError(#[from] TokenError),
}
