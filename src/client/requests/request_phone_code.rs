use crate::client::{
    errors::{api_error::ApiError, ClientError},
    queries::{
        user_request_auth_code, PhoneCodeChannelType, UserRequestAuthCode,
        UserRequestAuthCodeInput, UserRequestAuthCodeUserRequestAuthCode,
    },
    GaloyClient,
};
use graphql_client::reqwest::post_graphql;

impl GaloyClient {
    pub async fn request_phone_code(
        &self,
        phone: String,
    ) -> Result<UserRequestAuthCodeUserRequestAuthCode, ClientError> {
        let input = UserRequestAuthCodeInput {
            phone,
            channel: Some(PhoneCodeChannelType::SMS),
        };

        let variables = user_request_auth_code::Variables { input };
        let response_body =
            post_graphql::<UserRequestAuthCode, _>(&self.graphql_client, &self.api, variables)
                .await
                .map_err(|err| ApiError::IssueGettingResponse(anyhow::Error::new(err)))?;
        let response_data = response_body.data.ok_or(ApiError::IssueParsingResponse)?;

        let user_request_auth_code = response_data.user_request_auth_code;

        if !user_request_auth_code.errors.is_empty() {
            let error_string: String = user_request_auth_code
                .errors
                .iter()
                .map(|error| format!("{:?}", error))
                .collect::<Vec<String>>()
                .join(", ");

            return Err(ClientError::ApiError(ApiError::RequestFailedWithError(
                error_string,
            )));
        }
        Ok(user_request_auth_code)
    }
}
