use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;

use super::errors::token_error::TokenError;
use super::errors::AppError;

pub const TOKEN_FILE_NAME: &str = "GALOY_TOKEN";
pub const EMAIL_LOGIN_ID_FILE_NAME: &str = "EMAIL_LOGIN_ID";

pub const GALOY_CLI_FOLDER_NAME: &str = ".galoy-cli";

fn get_file_path(file_name: &str) -> Result<PathBuf, AppError> {
    let home_dir = dirs::home_dir().ok_or(TokenError::HomeDirectoryNotFound)?;
    let file_dir = home_dir.join(GALOY_CLI_FOLDER_NAME);
    Ok(file_dir.join(file_name))
}

pub fn get_data(file_type: &str) -> Result<Option<String>, AppError> {
    let file_path = get_file_path(file_type)?;

    if file_path.exists() {
        let data = fs::read_to_string(&file_path)
            .map_err(|_| TokenError::FailedToReadToken(file_path.clone()))?;
        return Ok(Some(data));
    }
    Ok(None)
}

pub fn save_data(file_type: &str, data: &str) -> Result<(), AppError> {
    let file_path = get_file_path(file_type)?;
    let parent_dir = file_path.parent().unwrap().to_owned();
    fs::create_dir_all(&parent_dir)
        .map_err(|_| TokenError::FailedToCreateDirectory(parent_dir.clone()))?;

    let mut file =
        File::create(&file_path).map_err(|_| TokenError::FailedToCreateFile(file_path.clone()))?;

    file.write_all(data.as_bytes())
        .map_err(|_| TokenError::FailedToWriteToken(file_path.clone()))?;

    Ok(())
}

pub fn remove_data(file_type: &str) -> Result<(), AppError> {
    let file_path = get_file_path(file_type)?;
    if file_path.exists() {
        fs::remove_file(&file_path)
            .map_err(|_| TokenError::FailedToDeleteFile(file_path.clone()))?;
        return Ok(());
    }
    Err(AppError::TokenError(TokenError::TokenFileNotFound))
}
