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
}
