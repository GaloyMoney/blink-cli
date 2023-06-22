mod unauth;
pub use unauth::*;
mod user_login;
pub use user_login::*;

use crate::client::GaloyClient;

pub struct App {
    client: GaloyClient,
}

impl App {
    pub fn new(api: String) -> anyhow::Result<Self> {
        let client = GaloyClient::new(api, None)?;
        Ok(Self { client })
    }
}
