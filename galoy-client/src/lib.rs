pub mod default_wallet;

mod globals;

use globals::QueryGlobalsGlobals;
use reqwest::blocking::Client;

use anyhow::Error;

struct ClientWrapper {
    pub client: Client,
    pub api_url: String,
}

fn create_client(api_url: String) -> ClientWrapper {
    let client = Client::builder().build().expect("error creating client");
    return ClientWrapper { client, api_url };
}

pub fn globals(api_url: String) -> Result<QueryGlobalsGlobals, Error> {
    let client = create_client(api_url);

    let globals = || globals::globals(client);

    return globals;
}
