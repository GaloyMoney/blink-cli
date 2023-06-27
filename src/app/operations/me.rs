use crate::app::App;

impl App {
    pub async fn me(&self) -> anyhow::Result<()> {
        let result = self.client.me().await?;
        println!(
            "{}",
            serde_json::to_string_pretty(&result).expect("Can't serialize json")
        );
        Ok(())
    }
}
