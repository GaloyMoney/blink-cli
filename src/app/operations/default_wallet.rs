use crate::app::App;

impl App {
    pub async fn default_wallet(&self, username: String) -> anyhow::Result<()> {
        let result = self.client.default_wallet(username.clone()).await;
        match result {
            Ok(wallet_id) => {
                println!("Default wallet ID for {} is: {}", username, wallet_id);
            }
            Err(err) => {
                println!("Error occurred while fetching default wallet id: {}", err);
            }
        }
        Ok(())
    }
}
