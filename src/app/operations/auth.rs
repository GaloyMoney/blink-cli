use anyhow::{Context, Result};

use crate::app::{errors::token_error::TokenError, token, App};

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
        if let Some(token) = token::get_token()? {
            self.client
                .user_logout(token)
                .await
                .context("Failed to log out")?;

            token::remove_token().context("Failed to delete token")?;

            println!("User logged out successfully!");
            Ok(())
        } else {
            Err(TokenError::TokenFileNotFound.into())
        }
    }
}
