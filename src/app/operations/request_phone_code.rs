use crate::app::{server::server::run, App};
use anyhow::Result;
use std::net::TcpListener;

impl App {
    pub async fn request_phone_code(&self, phone: String) -> Result<()> {
        let listener = TcpListener::bind("127.0.0.1:0")?;
        let port = listener.local_addr().unwrap().port();
        println!(
            "Visit http://127.0.0.1:{}/login and solve the Captcha",
            port
        );
        let api_clone = self.client.api.clone();
        let captcha_challenge = self.client.create_captcha_challenge().await?;
        tokio::task::spawn_blocking(move || {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()?;
            rt.block_on(run(listener, phone, api_clone, captcha_challenge))
        })
        .await?
    }
}
