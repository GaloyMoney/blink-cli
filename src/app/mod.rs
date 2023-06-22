mod commands;
pub use commands::*;

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
