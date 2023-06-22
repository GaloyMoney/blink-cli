mod operations;
pub use operations::*;

mod token;

use crate::client::GaloyClient;

pub struct App {
    client: GaloyClient,
}

impl App {
    pub fn new(api: String) -> anyhow::Result<Self> {
        let token = token::get_token()?;
        let client = GaloyClient::new(api, token)?;
        Ok(Self { client })
    }
}
