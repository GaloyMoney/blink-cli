use super::{queries::query_me, GaloyClient};
use crate::client::queries::{QueryMe, QueryMeMe};
use crate::errors::api_error::ApiError;
use crate::errors::me_error::MeError;
use crate::errors::CliError;
use graphql_client::reqwest::post_graphql;

impl GaloyClient {
    pub async fn me(&self) -> Result<QueryMeMe, CliError> {
        let variables = query_me::Variables;

        let response_body = post_graphql::<QueryMe, _>(&self.graphql_client, &self.api, variables)
            .await
            .map_err(|_| ApiError::IssueGettingResponse)?;

        let response_data = response_body.data.ok_or(ApiError::IssueParsingResponse)?;
        let me = response_data.me.ok_or(MeError::FailedToUnwrapMe)?;
        Ok(me)
    }
}
