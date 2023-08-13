mod errors;
mod file_manager;
mod operations;
mod server;

use crate::client::GaloyClient;

pub struct App {
    client: GaloyClient,
}

impl App {
    pub fn new(api: String) -> anyhow::Result<Self> {
        let token = file_manager::get_data(file_manager::TOKEN_FILE_NAME)?;
        let client = GaloyClient::new(api, token)?;
        Ok(Self { client })
    }
}
