use crate::app::App;

impl App {
    pub async fn set_username(&self, username: String) -> anyhow::Result<()> {
        let result = self.client.set_username(username).await;
        match result {
            Ok(()) => println!("Username has been successfully set!"),
            Err(err) => println!("Error occurred while setting username: {}", err),
        }
        Ok(())
    }
}
