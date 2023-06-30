use crate::client::{
    errors::{api_error::ApiError, ClientError},
    queries::{query_default_wallet, QueryDefaultWallet},
    GaloyClient,
};

use graphql_client::reqwest::post_graphql;

impl GaloyClient {
    pub async fn default_wallet(&self, username: String) -> Result<String, ClientError> {
        let variables = query_default_wallet::Variables {
            username: username.clone(),
        };

        let response_body =
            post_graphql::<QueryDefaultWallet, _>(&self.graphql_client, &self.api, variables)
                .await
                .map_err(|err| ApiError::IssueGettingResponse(anyhow::Error::new(err)))?;

        let response_data = response_body.data.ok_or(ApiError::IssueParsingResponse)?;

        let recipient_wallet_id = response_data.account_default_wallet.id;

        Ok(recipient_wallet_id)
    }
}
