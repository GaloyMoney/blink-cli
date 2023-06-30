use thiserror::Error;

use self::payment_error::PaymentError;

pub mod payment_error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error(transparent)]
    PaymentError(#[from] PaymentError),
}
