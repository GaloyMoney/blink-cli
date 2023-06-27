use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;

use crate::errors::token_error::TokenError;

pub const TOKEN_FILE_NAME: &str = "GALOY_TOKEN";
pub const TOKEN_FOLDER_NAME: &str = ".galoy-cli";

pub fn get_token_file_path() -> Result<PathBuf, TokenError> {
    let home_dir = dirs::home_dir().ok_or(TokenError::HomeDirectoryNotFound)?;
    let token_dir = home_dir.join(TOKEN_FOLDER_NAME);
    Ok(token_dir.join(TOKEN_FILE_NAME))
}

pub fn get_token() -> anyhow::Result<Option<String>> {
    let token_file = get_token_file_path()?;

    if token_file.exists() {
        let token = fs::read_to_string(&token_file)
            .map_err(|_| TokenError::FailedToReadToken(token_file.clone()))?;
        Ok(Some(token))
    } else {
        Ok(None)
    }
}

pub fn save_token(token: &str) -> Result<(), TokenError> {
    let token_file = get_token_file_path()?;
    let parent_dir = token_file.parent().unwrap().to_owned();
    fs::create_dir_all(&parent_dir)
        .map_err(|_| TokenError::FailedToCreateDirectory(parent_dir.clone()))?;

    let mut file = File::create(&token_file)
        .map_err(|_| TokenError::FailedToCreateFile(token_file.clone()))?;

    file.write_all(token.as_bytes())
        .map_err(|_| TokenError::FailedToWriteToken(token_file.clone()))?;

    Ok(())
}

pub fn remove_token() -> Result<(), TokenError> {
    let token_file = get_token_file_path()?;
    if token_file.exists() {
        fs::remove_file(&token_file)
            .map_err(|_| TokenError::FailedToDeleteFile(token_file.clone()))?;
        Ok(())
    } else {
        Err(TokenError::TokenFileNotFound)
    }
}
