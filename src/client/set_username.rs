use graphql_client::reqwest::post_graphql;

use crate::errors::{api_error::ApiError, set_username_error::SetUsernameError, CliError};

use super::{
    queries::{user_update_username, UserUpdateUsername, UserUpdateUsernameInput},
    GaloyClient,
};

impl GaloyClient {
    pub async fn set_username(&self, username: String) -> Result<(), CliError> {
        let input = UserUpdateUsernameInput { username };

        let variables = user_update_username::Variables { input };

        let response_body =
            post_graphql::<UserUpdateUsername, _>(&self.graphql_client, &self.api, variables)
                .await
                .map_err(|_| ApiError::IssueGettingResponse)?;

        let response_data = response_body.data.ok_or(ApiError::IssueParsingResponse)?;

        if !response_data.user_update_username.errors.is_empty() {
            return Err(CliError::SetUsernameError(
                SetUsernameError::FailedToUpdateUsername,
            ));
        } else {
            Ok(())
        }
    }
}
