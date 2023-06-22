use crate::app::{save_token, App};

impl App {
    pub async fn user_login(&self, phone: String, code: String) -> anyhow::Result<()> {
        let result = self.client.user_login(phone, code).await?;
        match save_token(&result) {
            Ok(()) => println!("User logged in successfully!"),
            Err(err) => eprintln!("Failed to log in: {}", err),
        }
        Ok(())
    }
}
