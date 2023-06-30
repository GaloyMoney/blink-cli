use crate::client::{
    errors::{api_error::ApiError, me_error::MeError, ClientError},
    queries::{query_me, QueryMe, QueryMeMe},
    GaloyClient,
};
use graphql_client::reqwest::post_graphql;

impl GaloyClient {
    pub async fn me(&self) -> Result<QueryMeMe, ClientError> {
        let variables = query_me::Variables;

        let response_body = post_graphql::<QueryMe, _>(&self.graphql_client, &self.api, variables)
            .await
            .map_err(|err| ApiError::IssueGettingResponse(anyhow::Error::new(err)))?;

        let response_data = response_body.data.ok_or(ApiError::IssueParsingResponse)?;
        let me = response_data.me.ok_or(MeError::FailedToUnwrapMe)?;
        Ok(me)
    }
}
