use graphql_client::reqwest::post_graphql;

use crate::{
    client::{
        queries::{user_login, UserLogin, UserLoginInput},
        GaloyClient,
    },
    errors::{api_error::ApiError, CliError},
};

impl GaloyClient {
    pub async fn user_login(&self, phone: String, code: String) -> Result<String, CliError> {
        let input = UserLoginInput { phone, code };

        let variables = user_login::Variables { input };

        let response_body =
            post_graphql::<UserLogin, _>(&self.graphql_client, &self.api, variables)
                .await
                .map_err(|err| ApiError::IssueGettingResponse(anyhow::Error::new(err)))?;

        let response_data = response_body.data.ok_or(ApiError::IssueParsingResponse)?;

        if let Some(auth_token) = response_data.user_login.auth_token {
            Ok(auth_token)
        } else {
            let error_string: String = response_data
                .user_login
                .errors
                .iter()
                .map(|error| format!("{:?}", error))
                .collect::<Vec<String>>()
                .join(", ");

            return Err(CliError::ApiError(ApiError::RequestFailedWithError(
                error_string,
            )));
        }
    }
}
