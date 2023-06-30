use graphql_client::reqwest::post_graphql;

use crate::client::{
    errors::{api_error::ApiError, ClientError},
    queries::{query_globals, QueryGlobals, QueryGlobalsGlobals},
    GaloyClient,
};

impl GaloyClient {
    pub async fn globals(&self) -> Result<QueryGlobalsGlobals, ClientError> {
        let variables = query_globals::Variables;

        let response_body =
            post_graphql::<QueryGlobals, _>(&self.graphql_client, &self.api, variables)
                .await
                .map_err(|err| ApiError::IssueGettingResponse(anyhow::Error::new(err)))?;

        let response_data = response_body.data.ok_or(ApiError::IssueParsingResponse)?;
        let result = response_data
            .globals
            .ok_or(ApiError::IssueParsingResponse)?;

        Ok(result)
    }
}
