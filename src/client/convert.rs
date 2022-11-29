use crate::{
    message_only_error,
    queries::{
        intra_ledger_payment_send, query_default_wallet, query_globals, query_me, user_login,
        user_request_auth_code,
    },
    DefaultWallet, GaloyCliError, Me, PaymentSendResult, QueryGlobalsGlobals,
};

impl From<query_default_wallet::ResponseData> for DefaultWallet {
    fn from(response: query_default_wallet::ResponseData) -> Self {
        response.account_default_wallet
    }
}

impl TryFrom<query_me::ResponseData> for Me {
    type Error = GaloyCliError;

    fn try_from(response: query_me::ResponseData) -> Result<Self, Self::Error> {
        let me = response.me.ok_or_else(|| {
            GaloyCliError::GraphQl(message_only_error(
                "Empty `me` in response data".to_string(),
            ))
        })?;

        Ok(me)
    }
}

impl TryFrom<intra_ledger_payment_send::ResponseData> for PaymentSendResult {
    type Error = GaloyCliError;

    fn try_from(response: intra_ledger_payment_send::ResponseData) -> Result<Self, Self::Error> {
        let payment_send = response.intra_ledger_payment_send;

        if !payment_send.errors.is_empty() {
            let errors = payment_send.errors;
            let mut top_errors = Vec::new();
            for error in errors {
                top_errors.push(graphql_client::Error::from(error))
            }
            return Err(GaloyCliError::GraphQl(top_errors));
        }

        let status = payment_send.status.ok_or_else(|| {
            GaloyCliError::GraphQl(message_only_error("Empty payment status".to_string()))
        })?;

        Ok(status)
    }
}

pub type IntraLedgerPaymentSendErrors =
    intra_ledger_payment_send::IntraLedgerPaymentSendIntraLedgerPaymentSendErrors;
impl From<IntraLedgerPaymentSendErrors> for graphql_client::Error {
    fn from(error: IntraLedgerPaymentSendErrors) -> Self {
        let message = error.message;

        Self {
            message,
            locations: None,
            path: None,
            extensions: None,
        }
    }
}

pub type AuthCodeStatus = bool;
impl TryFrom<user_request_auth_code::ResponseData> for AuthCodeStatus {
    type Error = GaloyCliError;

    fn try_from(response: user_request_auth_code::ResponseData) -> Result<Self, Self::Error> {
        let auth_code = response.user_request_auth_code;

        let (errors, success) = (auth_code.errors, auth_code.success);

        if !errors.is_empty() {
            let mut container = Vec::new();

            for error in errors {
                container.push(graphql_client::Error {
                    message: error.message,
                    locations: None,
                    path: None,
                    extensions: None,
                })
            }

            return Err(GaloyCliError::GraphQl(container));
        }

        let success = success.ok_or_else(|| {
            GaloyCliError::GraphQl(message_only_error(
                "Empty `success` status in response data".to_string(),
            ))
        })?;

        Ok(success)
    }
}

pub type Globals = QueryGlobalsGlobals;
impl TryFrom<query_globals::ResponseData> for Globals {
    type Error = GaloyCliError;

    fn try_from(response: query_globals::ResponseData) -> Result<Self, Self::Error> {
        let globals = response.globals.ok_or_else(|| {
            GaloyCliError::GraphQl(message_only_error("Empty fields in `globals`".to_string()))
        })?;

        Ok(globals)
    }
}

pub struct UserLoginAuthCode {
    pub auth_token: String,
}
impl TryFrom<user_login::ResponseData> for UserLoginAuthCode {
    type Error = GaloyCliError;

    fn try_from(response: user_login::ResponseData) -> Result<Self, Self::Error> {
        let user_login = response.user_login;

        let (errors, auth_token) = (user_login.errors, user_login.auth_token);

        if !errors.is_empty() {
            let mut container = Vec::new();

            for error in errors {
                container.push(graphql_client::Error {
                    message: error.message,
                    locations: None,
                    path: None,
                    extensions: None,
                })
            }

            return Err(GaloyCliError::GraphQl(container));
        }

        let auth_token = auth_token.ok_or_else(|| {
            GaloyCliError::GraphQl(message_only_error(
                "Empty `auth token` in response data".to_string(),
            ))
        })?;

        Ok(Self { auth_token })
    }
}
