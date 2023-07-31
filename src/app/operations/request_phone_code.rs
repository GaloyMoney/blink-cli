use anyhow::Result;
use std::net::TcpListener;
use webbrowser;

use crate::app::{server::server::run, App};

const PORT: u16 = 42909;

impl App {
    pub async fn request_phone_code(&self, phone: String) -> Result<()> {
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
}
