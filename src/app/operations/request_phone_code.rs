use crate::app::{server::server::run, App};
use anyhow::{Context, Result};
use std::net::TcpListener;

impl App {
    pub async fn request_phone_code(&self, phone: String, nocaptcha: bool) -> Result<()> {
        if !nocaptcha {
            let listener = TcpListener::bind("127.0.0.1:0")?;
            let port = listener.local_addr().unwrap().port();
            println!(
                "Visit http://127.0.0.1:{}/login and solve the Captcha",
                port
            );
            let api_clone = self.client.api.clone();
            tokio::task::spawn_blocking(move || {
                let rt = tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()?;
                rt.block_on(run(listener, phone, api_clone))
            })
            .await?
        } else {
            let result = self
                .client
                .request_phone_code(phone)
                .await
                .context("Failed to request phone code")?;
            println!("{:?}", result);
            Ok(())
        }
    }
}
