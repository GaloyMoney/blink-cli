#![cfg_attr(feature = "fail-on-warnings", deny(warnings))]
#![cfg_attr(feature = "fail-on-warnings", deny(clippy::all))]

use default_wallet::QueryDefaultWalletAccountDefaultWallet;
use globals::QueryGlobalsGlobals;
use reqwest::blocking::Client;

mod default_wallet;
mod globals;

pub struct GaloyClient {
    graphql_client: Client,
    api: String,
}

impl GaloyClient {
    pub fn new(api: String) -> GaloyClient {
        let graphql_client = Client::builder().build().expect("can't initialize client");

        GaloyClient {
            graphql_client,
            api,
        }
    }

    pub fn globals(&self) -> Result<QueryGlobalsGlobals, anyhow::Error> {
        globals::run(&self.graphql_client, &self.api)
    }

    pub fn default_wallet(
        &self,
        username: String,
    ) -> Result<QueryDefaultWalletAccountDefaultWallet, anyhow::Error> {
        default_wallet::run(&self.graphql_client, &self.api, username)
    }
}
