use graphql_client::reqwest::post_graphql;
use reqwest::{Client, StatusCode};
use serde_json::{json, Value};

use crate::client::{
    errors::{api_error::ApiError, ClientError},
    queries::{
        captcha_create_challenge, user_login, CaptchaChallenge, CaptchaCreateChallenge, UserLogin,
        UserLoginInput,
    },
    GaloyClient,
};

pub struct LoginResponse {
    pub auth_token: String,
    pub totp_required: bool,
}

impl GaloyClient {
    pub async fn user_login_phone(
        &self,
        phone: String,
        code: String,
    ) -> Result<LoginResponse, ClientError> {
        let input = UserLoginInput { phone, code };

        let variables = user_login::Variables { input };

        let response_body =
            post_graphql::<UserLogin, _>(&self.graphql_client, &self.api, variables)
                .await
                .map_err(|err| ApiError::IssueGettingResponse(anyhow::Error::new(err)))?;

        let response_data = response_body.data.ok_or(ApiError::IssueParsingResponse)?;

        let login_result = response_data.user_login;

        if let (Some(auth_token), Some(totp_required)) =
            (login_result.auth_token, login_result.totp_required)
        {
            Ok(LoginResponse {
                auth_token,
                totp_required,
            })
        } else {
            let error_string: String = login_result
                .errors
                .iter()
                .map(|error| format!("{:?}", error))
                .collect::<Vec<String>>()
                .join(", ");

            Err(ClientError::ApiError(ApiError::RequestFailedWithError(
                error_string,
            )))
        }
    }

    pub async fn user_login_email(
        &self,
        email_login_id: String,
        code: String,
    ) -> Result<LoginResponse, ClientError> {
        let endpoint = self.api.trim_end_matches("/graphql");
        let url = format!("{}/auth/email/login", endpoint);
        let request_body = json!({ "code": code, "emailLoginId": email_login_id });

        let response = reqwest::Client::new()
            .post(&url)
            .json(&request_body)
            .send()
            .await
            .map_err(|err| ApiError::IssueGettingResponse(anyhow::Error::new(err)))?;

        let response_json: Value = response
            .json()
            .await
            .map_err(|_| ApiError::IssueParsingResponse)?;

        let auth_token = response_json["result"]["authToken"]
            .as_str()
            .ok_or(ApiError::IssueParsingResponse)?
            .to_string();

        let totp_required = response_json["result"]["totpRequired"]
            .as_bool()
            .ok_or(ApiError::IssueParsingResponse)?;

        Ok(LoginResponse {
            auth_token,
            totp_required,
        })
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

    pub async fn request_email_code(&self, email: String) -> Result<String, ClientError> {
        let endpoint = self.api.trim_end_matches("/graphql");
        let url = format!("{}/auth/email/code", endpoint);
        let request_body = json!({ "email": email });

        let response = reqwest::Client::new()
            .post(&url)
            .json(&request_body)
            .send()
            .await
            .map_err(|err| ApiError::IssueGettingResponse(anyhow::Error::new(err)))?;

        let response_json = response
            .json::<serde_json::Value>()
            .await
            .map_err(|_| ApiError::IssueParsingResponse)?;

        let email_login_id = response_json
            .get("result")
            .and_then(|r| r.as_str())
            .ok_or(ApiError::IssueParsingResponse)?;
        Ok(email_login_id.to_string())
    }

    pub async fn validate_totp_code(
        &self,
        auth_token: String,
        totp_code: String,
    ) -> Result<bool, ClientError> {
        let endpoint = self.api.trim_end_matches("/graphql");
        let url = format!("{}/auth/totp/validate", endpoint);
        let request_body = json!({ "totpCode": totp_code, "authToken": auth_token });

        let response = Client::new().post(&url).json(&request_body).send().await;

        match response {
            Ok(resp) => match resp.status() {
                StatusCode::OK => Ok(true),
                StatusCode::UNPROCESSABLE_ENTITY => Ok(false),
                StatusCode::INTERNAL_SERVER_ERROR => {
                    let error_details = resp
                        .text()
                        .await
                        .unwrap_or_else(|_| "Unknown error".to_string());
                    if error_details.contains("Request failed with status code 400") {
                        return Ok(false);
                    }
                    Err(ClientError::ApiError(ApiError::RequestFailedWithError(
                        error_details,
                    )))
                }
                _ => {
                    let status = resp.status();
                    Err(ClientError::ApiError(ApiError::RequestFailedWithError(
                        format!("Unexpected status code: {}", status),
                    )))
                }
            },
            Err(e) => {
                eprintln!("Network or other error: {}", e);
                Err(ClientError::ApiError(ApiError::IssueGettingResponse(
                    anyhow::Error::new(e),
                )))
            }
        }
    }
}
