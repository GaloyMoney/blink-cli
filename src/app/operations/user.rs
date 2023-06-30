use anyhow::Context;

use crate::app::App;

impl App {
    pub async fn me(&self) -> anyhow::Result<()> {
        let result = self
            .client
            .me()
            .await
            .context("Error occurred while fetching 'me' data")?;

        println!(
            "{}",
            serde_json::to_string_pretty(&result).context("Failed to serialize JSON")?
        );

        Ok(())
    }

    pub async fn set_username(&self, username: String) -> anyhow::Result<()> {
        self.client
            .set_username(username)
            .await
            .context("Failed to set username")?;

        println!("Username has been successfully set!");

        Ok(())
    }
}
