use std::path::PathBuf;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum TokenError {
    #[error("Token file does not exist")]
    TokenFileNotFound,
    #[error("Failed to get home directory")]
    HomeDirectoryNotFound,
    #[error("Failed to create directory: {0}")]
    FailedToCreateDirectory(PathBuf),
    #[error("Failed to create token file: {0}")]
    FailedToCreateFile(PathBuf),
    #[error("Failed to write token to file: {0}")]
    FailedToWriteToken(PathBuf),
    #[error("Failed to read token: {0}")]
    FailedToReadToken(PathBuf),
    #[error("Failed to delete token file: {0}")]
    FailedToDeleteFile(PathBuf),
}

#[derive(Error, Debug)]
pub enum CaptchaError {
    #[error("Empty captcha create challenge")]
    EmptyCaptcha,
}
