use anyhow::{Context, Result};

use crate::app::{file_manager, App};

impl App {
    pub async fn user_login(&self, phone: Option<String>, code: String, email: bool) -> Result<()> {
        if let Some(phone) = phone {
            let result = self
                .client
                .user_login_phone(phone.clone(), code.clone())
                .await
                .context("Failed to log in")?;

            file_manager::save_data(file_manager::TOKEN_FILE_NAME, &result)
                .context("Failed to save token")?;
        } else if email {
            let email_login_id =
                file_manager::get_data(file_manager::EMAIL_LOGIN_ID_FILE_NAME)?.unwrap();

            let result = self
                .client
                .user_login_email(email_login_id, code.clone())
                .await
                .context("Failed to log in")?;

            file_manager::save_data(file_manager::TOKEN_FILE_NAME, &result)
                .context("Failed to save token")?;

            println!("User logged in successfully!");
        }
        Ok(())
    }

    pub async fn user_logout(&self) -> Result<()> {
        file_manager::remove_data(file_manager::TOKEN_FILE_NAME).context("Failed to log out")?;
        println!("User logged out successfully!");
        Ok(())
    }
}