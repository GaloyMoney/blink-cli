use crate::app::errors::CaptchaError;

use self::captcha_create_challenge::ResponseData;
use graphql_client::GraphQLQuery;

type Phone = String;
type AuthToken = String;
type OneTimeAuthCode = String;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/client/gql/schema.gql",
    query_path = "src/client/gql/queries/globals.gql",
    response_derives = "Debug, Serialize"
)]
pub(super) struct QueryGlobals;
pub use self::query_globals::QueryGlobalsGlobals;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/client/gql/schema.gql",
    query_path = "src/client/gql/mutations/user_login.gql",
    response_derives = "Debug, Serialize"
)]
pub(super) struct UserLogin;
pub use self::user_login::UserLoginInput;
pub use self::user_login::UserLoginUserLogin;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/client/gql/schema.gql",
    query_path = "src/client/gql/mutations/request_auth_code.gql",
    response_derives = "Debug, Serialize"
)]
pub(super) struct UserRequestAuthCode;
pub use self::user_request_auth_code::PhoneCodeChannelType;
pub use self::user_request_auth_code::UserRequestAuthCodeInput;
pub use self::user_request_auth_code::UserRequestAuthCodeUserRequestAuthCode;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/client/gql/schema.gql",
    query_path = "src/client/gql/mutations/captcha_create_challenge.gql",
    response_derives = "Debug, Serialize"
)]
pub(super) struct CaptchaCreateChallenge;
pub use self::captcha_create_challenge::*;

pub struct CaptchaChallenge {
    pub id: String,
    pub challenge_code: String,
    pub new_captcha: bool,
    pub failback_mode: bool,
}

impl TryFrom<ResponseData> for CaptchaChallenge {
    type Error = CaptchaError;

    fn try_from(response: ResponseData) -> Result<Self, Self::Error> {
        let result = response.captcha_create_challenge.result;
        let challenge = result.ok_or_else(|| CaptchaError::EmptyCaptcha)?;

        let (id, challenge_code, new_captcha, failback_mode) = (
            challenge.id,
            challenge.challenge_code,
            challenge.new_captcha,
            challenge.failback_mode,
        );
        Ok(CaptchaChallenge {
            id,
            challenge_code,
            new_captcha,
            failback_mode,
        })
    }
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/client/gql/schema.gql",
    query_path = "src/client/gql/mutations/captcha_request_auth_code.gql",
    response_derives = "Debug, Serialize"
)]
pub(super) struct CaptchaRequestAuthCode;
pub use self::captcha_request_auth_code::CaptchaRequestAuthCodeCaptchaRequestAuthCode;
pub use self::captcha_request_auth_code::CaptchaRequestAuthCodeInput;
