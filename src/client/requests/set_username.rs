use graphql_client::reqwest::post_graphql;

use crate::client::{
    errors::{api_error::ApiError, ClientError},
    queries::{user_update_username, UserUpdateUsername, UserUpdateUsernameInput},
    GaloyClient,
};

impl GaloyClient {
    pub async fn set_username(&self, username: String) -> Result<(), ClientError> {
        let input = UserUpdateUsernameInput { username };

        let variables = user_update_username::Variables { input };

        let response_body =
            post_graphql::<UserUpdateUsername, _>(&self.graphql_client, &self.api, variables)
                .await
                .map_err(|err| ApiError::IssueGettingResponse(anyhow::Error::new(err)))?;

        let response_data = response_body.data.ok_or(ApiError::IssueParsingResponse)?;

        if !response_data.user_update_username.errors.is_empty() {
            let error_string: String = response_data
                .user_update_username
                .errors
                .iter()
                .map(|error| format!("{:?}", error))
                .collect::<Vec<String>>()
                .join(", ");

            Err(ClientError::ApiError(ApiError::RequestFailedWithError(
                error_string,
            )))
        } else {
            Ok(())
        }
    }
}
