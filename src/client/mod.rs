use anyhow::Context;
use graphql_client::reqwest::post_graphql_blocking as post_graphql;
use reqwest::blocking::Client;

mod queries;

use queries::*;

pub struct GaloyClient {
    graphql_client: Client,
    api: String,
}

impl GaloyClient {
    pub fn new(api: String) -> Self {
        let graphql_client = Client::builder().build().expect("can't initialize client");

        Self {
            graphql_client,
            api,
        }
    }

    pub fn globals(&self) -> anyhow::Result<QueryGlobalsGlobals> {
        let variables = query_globals::Variables;

        let response_body =
            post_graphql::<QueryGlobals, _>(&self.graphql_client, &self.api, variables)
                .context("issue fetching response")?;

        let response_data = response_body.data.context("bad response from server")?;

        let result = response_data.globals.context("empty response")?;

        Ok(result)
    }

    pub fn default_wallet(
        &self,
        username: String,
    ) -> anyhow::Result<QueryDefaultWalletAccountDefaultWallet> {
        let variables = query_default_wallet::Variables { username };

        let response_body =
            post_graphql::<QueryDefaultWallet, _>(&self.graphql_client, &self.api, variables)
                .context("issue fetching response")?;

        let response_data = response_body.data.context("Username doesn't exist")?;

        Ok(response_data.account_default_wallet)
    }
}
