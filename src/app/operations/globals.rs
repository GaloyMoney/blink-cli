use crate::app::App;

impl App {
    pub async fn globals(&self) -> anyhow::Result<()> {
        match self.client.globals().await {
            Ok(globals) => {
                println!("{}", serde_json::to_string_pretty(&globals)?);
                Ok(())
            }
            Err(err) => {
                println!("Error occurred while fetching globals");
                Err(err.into())
            }
        }
    }
}
