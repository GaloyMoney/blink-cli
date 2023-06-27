use crate::app::App;

impl App {
    pub async fn request_phone_code(&self, phone: String, nocaptcha: bool) -> std::io::Result<()> {
        match self
            .client
            .request_phone_code(phone, nocaptcha, self.client.api.clone())
            .await
        {
            Ok(_) => {
                println!("Request succeeded");
                Ok(())
            }
            Err(err) => {
                println!("Request failed: {}", err);
                Err(err)
            }
        }
    }
}
