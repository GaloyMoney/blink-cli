use graphql_client::reqwest::post_graphql;
use reqwest::Client;

use crate::client::{
    errors::{api_error::ApiError, ClientError},
    queries::{
        captcha_create_challenge, user_login, user_logout, CaptchaChallenge,
        CaptchaCreateChallenge, UserLogin, UserLoginInput, UserLogout, UserLogoutInput,
    },
    GaloyClient,
};

impl GaloyClient {
    pub async fn user_login(&self, phone: String, code: String) -> Result<String, ClientError> {
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

            return Err(ClientError::ApiError(ApiError::RequestFailedWithError(
                error_string,
            )));
        }
    }

    pub async fn user_logout(&self, auth_token: String) -> Result<(), ClientError> {
        let input = UserLogoutInput {
            device_token: auth_token,
        };

        let variables = user_logout::Variables { input };

        let response_body =
            post_graphql::<UserLogout, _>(&self.graphql_client, &self.api, variables)
                .await
                .map_err(|err| ApiError::IssueGettingResponse(anyhow::Error::new(err)))?;

        let response_data = response_body.data.ok_or(ApiError::IssueParsingResponse)?;
        if !response_data.user_logout.errors.is_empty() {
            let error_string: String = response_data
                .user_logout
                .errors
                .iter()
                .map(|error| format!("{:?}", error))
                .collect::<Vec<String>>()
                .join(", ");

            return Err(ClientError::ApiError(ApiError::RequestFailedWithError(
                error_string,
            )));
        } else {
            Ok(())
        }
    }

    pub async fn create_captcha_challenge(&self) -> Result<CaptchaChallenge, ClientError> {
        let client = Client::builder().build().expect("Can't build client");
        let variables = captcha_create_challenge::Variables;
        let response_body =
            post_graphql::<CaptchaCreateChallenge, _>(&client, &self.api.clone(), variables)
                .await
                .map_err(|err| ApiError::IssueGettingResponse(anyhow::Error::new(err)))?;
        let response_data = response_body.data.ok_or(ApiError::IssueParsingResponse)?;
        let captcha_challenge_result = CaptchaChallenge::try_from(response_data)
            .map_err(|err| ApiError::IssueGettingResponse(anyhow::Error::new(err)))?;
        Ok(captcha_challenge_result)
    }
}
