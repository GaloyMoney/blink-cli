use anyhow::Context;
use graphql_client::reqwest::post_graphql;

use super::{
    queries::{query_globals, QueryGlobals, QueryGlobalsGlobals},
    GaloyClient,
};

impl GaloyClient {
    pub async fn globals(&self) -> anyhow::Result<QueryGlobalsGlobals> {
        let variables = query_globals::Variables;

        let response_body =
            post_graphql::<QueryGlobals, _>(&self.graphql_client, &self.api, variables)
                .await
                .context("issue fetching response")?;

        let response_data = response_body.data.context("bad response from server")?;
        let result = response_data.globals.context("empty response")?;

        Ok(result)
    }
}
