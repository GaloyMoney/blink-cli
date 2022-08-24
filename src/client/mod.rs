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
    pub fn new(api: String, jwt: Option<String>) -> Self {
        let mut client_builder = Client::builder();

        if let Some(jwt) = jwt {
            client_builder = client_builder.default_headers(
                std::iter::once((
                    reqwest::header::AUTHORIZATION,
                    reqwest::header::HeaderValue::from_str(&format!("Bearer {}", jwt)).unwrap(),
                ))
                .collect(),
            )
        };

        let graphql_client = client_builder.build().expect("can't initialize client");

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

    pub fn me(&self) {
        // pub fn me(&self) -> anyhow::Result<QueryMeMe> {
        let variables = query_me::Variables;

        let response_body = post_graphql::<QueryMe, _>(&self.graphql_client, &self.api, variables)
            .expect("issue fetching response");

        let response_data = response_body.data.expect("Empty fields"); // TODO: check the error given is correct
                                                                       // let response_data = response_body.data.context("Empty fields")?; // TODO: check the error given is correct

        // response_data.me
        // Ok(response_data.me)
        if let Some(me) = response_data.me {

            // FIXME: why is `me` below typed as `unknown`?
            println!("{:?}", me);
        };
        // println!("{:?}", response_data.me);
    }

    // pub fn intradleger_send(
    //     &self,
    //     username: String,
    // ) -> anyhow::Result<QueryDefaultWalletAccountDefaultWallet> {
    //     let variables = query_default_wallet::Variables { username };

    //     let response_body =
    //         post_graphql::<QueryDefaultWallet, _>(&self.graphql_client, &self.api, variables)
    //             .context("issue fetching response")?;

    //     let response_data = response_body.data.context("Username doesn't exist")?;

    //     Ok(response_data.account_default_wallet)
    // }
}
