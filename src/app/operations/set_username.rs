use anyhow::{Context, Result};

use crate::app::App;

impl App {
    pub async fn set_username(&self, username: String) -> Result<()> {
        self.client
            .set_username(username)
            .await
            .context("Failed to set username")?;

        println!("Username has been successfully set!");

        Ok(())
    }
}
