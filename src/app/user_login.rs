use super::App;

impl App {
    pub async fn user_login(&self, phone: String, code: String) -> anyhow::Result<()> {
        let result = self.client.user_login(phone, code).await?;
        println!("{}", (result));
        Ok(())
    }
}
