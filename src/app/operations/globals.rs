use anyhow::{Context, Result};

use crate::app::App;

impl App {
    pub async fn globals(&self) -> Result<()> {
        let globals = self
            .client
            .globals()
            .await
            .context("Error occurred while fetching globals")?;

        println!("{}", serde_json::to_string_pretty(&globals)?);
        Ok(())
    }
}
