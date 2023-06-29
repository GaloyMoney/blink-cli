use crate::app::App;

impl App {
    pub async fn me(&self) -> anyhow::Result<()> {
        match self.client.me().await {
            Ok(result) => {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&result).expect("Can't serialize json")
                );
                Ok(())
            }
            Err(err) => {
                eprintln!("Error occurred while fetching 'me' data");
                Err(err.into())
            }
        }
    }
}
