use crate::app::{remove_token, App};

impl App {
    pub async fn user_logout(&self) -> anyhow::Result<()> {
        match remove_token() {
            Ok(()) => println!("User logged out successfully!"),
            Err(err) => eprintln!("Failed to log out: {}", err),
        }
        Ok(())
    }
}
