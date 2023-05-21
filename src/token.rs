use super::constants::{TOKEN_FILE_NAME, TOKEN_FOLDER_NAME};
use anyhow::Context;
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;

pub fn get_token_file_path() -> Result<PathBuf, anyhow::Error> {
    let home_dir = dirs::home_dir().context("failed to get home directory")?;
    let token_dir = home_dir.join(TOKEN_FOLDER_NAME);
    Ok(token_dir.join(TOKEN_FILE_NAME))
}

pub fn save_token(token_file: &PathBuf, token: &str) -> Result<(), anyhow::Error> {
    fs::create_dir_all(token_file.parent().unwrap()).with_context(|| {
        format!(
            "failed to create directory '{}'",
            token_file.parent().unwrap().display()
        )
    })?;

    let mut file = File::create(token_file)
        .with_context(|| format!("failed to create file '{}'", token_file.display()))?;

    file.write_all(token.as_bytes())
        .with_context(|| format!("failed to write to file '{}'", token_file.display()))?;

    println!("Token saved to {}", token_file.display());
    Ok(())
}
