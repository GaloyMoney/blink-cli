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

    pub async fn list_transactions(
        &self,
        // after: Option<String>,
        // before: Option<String>,
        last: Option<i64>,
        first: Option<i64>,
        // wallet_ids: Option<Vec<Option<String>>>,
        all: bool,
    ) -> anyhow::Result<()> {
        let result = self
            .client
            .list_transactions()
            .await
            .context("Error occurred while fetching transactions")?;

        if let Some(transactions) = result {
            let selected_transactions: Vec<_> = if all {
                transactions.iter().collect()
            } else if let Some(first) = first {
                let total_transactions = transactions.len();
                transactions
                    .iter()
                    .skip(total_transactions.saturating_sub(first.try_into().unwrap()))
                    .collect()
            } else if let Some(last) = last {
                transactions.iter().take(last.try_into().unwrap()).collect()
            } else {
                transactions.iter().collect()
            };

            println!(
                "{}",
                serde_json::to_string_pretty(&selected_transactions)
                    .context("Failed to serialize JSON")?
            );
        }
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
