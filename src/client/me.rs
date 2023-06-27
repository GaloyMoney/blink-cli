use super::{queries::query_me, GaloyClient};
use crate::client::queries::{QueryMe, QueryMeMe};
use crate::errors::me_error::MeError;
use graphql_client::reqwest::post_graphql;

impl GaloyClient {
    pub async fn me(&self) -> Result<QueryMeMe, MeError> {
        let variables = query_me::Variables;

        let response_body = post_graphql::<QueryMe, _>(&self.graphql_client, &self.api, variables)
            .await
            .map_err(|_| MeError::IssueGettingResponse)?;

        let response_data = response_body.data.ok_or(MeError::IssueParsingResponse)?;
        let me = response_data.me.ok_or(MeError::FailedToUnwrapMe)?;
        Ok(me)
    }
}
