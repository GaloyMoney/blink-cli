use anyhow::{Context, Result};

use crate::app::App;

impl App {
    pub async fn default_wallet(&self, username: String) -> Result<()> {
        let result = self
            .client
            .default_wallet(username.clone())
            .await
            .context("Error occurred while fetching default wallet id")?;

        println!("Default wallet ID for {} is: {}", username, result);
        Ok(())
    }
}
