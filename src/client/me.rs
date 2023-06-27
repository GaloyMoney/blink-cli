use anyhow::Context;
use graphql_client::reqwest::post_graphql;

use super::{queries::query_me, GaloyClient};
use crate::client::queries::{QueryMe, QueryMeMe};

impl GaloyClient {
    pub async fn me(&self) -> anyhow::Result<QueryMeMe> {
        let variables = query_me::Variables;

        let response_body = post_graphql::<QueryMe, _>(&self.graphql_client, &self.api, variables)
            .await
            .context("issue getting response")?;

        let response_data = response_body.data.context("issue parsing response")?;
        let me = response_data.me.context("impossible to unwrap .me")?;
        Ok(me)
    }
}
