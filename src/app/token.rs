use anyhow::Context;
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;

pub const TOKEN_FILE_NAME: &str = "GALOY_TOKEN";
pub const TOKEN_FOLDER_NAME: &str = ".galoy-cli";

pub fn get_token_file_path() -> Result<PathBuf, anyhow::Error> {
    let home_dir = dirs::home_dir().context("failed to get home directory")?;
    let token_dir = home_dir.join(TOKEN_FOLDER_NAME);
    Ok(token_dir.join(TOKEN_FILE_NAME))
}

pub fn get_token() -> anyhow::Result<Option<String>> {
    let token_file = get_token_file_path()?;

    if token_file.exists() {
        let token = fs::read_to_string(&token_file).with_context(|| {
            format!("failed to read token from file '{}'", token_file.display())
        })?;
        Ok(Some(token))
    } else {
        Ok(None)
    }
}

pub fn save_token(token: &str) -> Result<(), anyhow::Error> {
    let token_file = get_token_file_path()?;
    let parent_dir = token_file.parent().unwrap().to_owned();
    fs::create_dir_all(&parent_dir)
        .with_context(|| format!("failed to create directory '{}'", parent_dir.display()))?;

    let mut file = File::create(&token_file)
        .with_context(|| format!("failed to create file '{}'", token_file.display()))?;

    file.write_all(token.as_bytes())
        .with_context(|| format!("failed to write to file '{}'", token_file.display()))?;

    Ok(())
}

pub fn remove_token() -> Result<(), anyhow::Error> {
    let token_file = get_token_file_path()?;
    if token_file.exists() {
        fs::remove_file(&token_file)
            .with_context(|| format!("failed to delete token file '{}'", token_file.display()))?;
        Ok(())
    } else {
        Err(anyhow::anyhow!("Token file does not exist"))
    }
}
