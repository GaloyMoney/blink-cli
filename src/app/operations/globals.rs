use crate::app::App;

impl App {
    pub async fn globals(&self) -> anyhow::Result<()> {
        let globals = self.client.globals().await?;
        println!("{}", serde_json::to_string_pretty(&globals)?);
        Ok(())
    }
}
