use anyhow::{Context, Result};

use crate::app::{token, App};

impl App {
    pub async fn user_login(&self, phone: String, code: String) -> Result<()> {
        let result = self
            .client
            .user_login(phone, code)
            .await
            .context("Failed to log in")?;

        token::save_token(&result).context("Failed to save token")?;

        println!("User logged in successfully!");
        Ok(())
    }

    pub async fn user_logout(&self) -> Result<()> {
        token::remove_token().context("Failed to log out")?;

        println!("User logged out successfully!");
        Ok(())
    }
}
