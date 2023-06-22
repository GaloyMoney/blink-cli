use crate::app::{token, App};

impl App {
    pub async fn user_logout(&self) -> anyhow::Result<()> {
        match token::remove_token() {
            Ok(()) => println!("User logged out successfully!"),
            Err(err) => eprintln!("Failed to log out: {}", err),
        }
        Ok(())
    }
}
