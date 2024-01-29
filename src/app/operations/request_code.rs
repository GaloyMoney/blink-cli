use anyhow::Context;
use std::net::TcpListener;
use webbrowser;

use crate::app::{file_manager, server::run, App};

const PORT: u16 = 42909;

impl App {
    pub async fn request_phone_code(&self, phone: String) -> anyhow::Result<()> {
        let listener = TcpListener::bind(format!("127.0.0.1:{}", PORT))?;

        let url = format!("http://127.0.0.1:{}/login", PORT);
        webbrowser::open(&url)?;

        println!("Visit {} and solve the Captcha", url);

        let api = self.client.api.clone();
        let captcha_challenge = self.client.create_captcha_challenge().await?;
        tokio::task::spawn_blocking(move || {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()?;
            rt.block_on(run(listener, phone, api, captcha_challenge))
        })
        .await?
    }

    pub async fn request_email_code(&self, email: String) -> anyhow::Result<()> {
        let result = self
            .client
            .request_email_code(email)
            .await
            .context("Failed to request email code")?;

        let _ = file_manager::save_data(file_manager::EMAIL_LOGIN_ID_FILE_NAME, &result);
        println!("Code successfully sent to the email!");

        Ok(())
    }
}
