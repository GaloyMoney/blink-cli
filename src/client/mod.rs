use graphql_client::reqwest::post_graphql_blocking as post_graphql;
use log::info;
use reqwest::blocking::Client;

pub mod queries;
pub use queries::*;
pub mod batch;
use batch::Batch;
pub mod convert;
pub use convert::*;

use rust_decimal::Decimal;

use crate::{message_only_error, GaloyCliError};

pub struct GaloyClient {
    graphql_client: Client,
    api: String,
}

impl GaloyClient {
    pub fn new(api: String, jwt: Option<String>) -> Self {
        let mut client_builder = Client::builder();

        if let Some(jwt) = jwt {
            client_builder = client_builder.default_headers(
                std::iter::once((
                    reqwest::header::AUTHORIZATION,
                    reqwest::header::HeaderValue::from_str(&format!("Bearer {}", jwt)).unwrap(),
                ))
                .collect(),
            )
        };

        let graphql_client = client_builder.build().expect("can't initialize client");

        Self {
            graphql_client,
            api,
        }
    }

    pub fn globals(&self) -> Result<QueryGlobalsGlobals, GaloyCliError> {
        let variables = query_globals::Variables;

        let response_body =
            post_graphql::<QueryGlobals, _>(&self.graphql_client, &self.api, variables)?;

        if response_body.errors.is_some() {
            if let Some(errors) = response_body.errors {
                return Err(GaloyCliError::GraphQl(errors));
            }
        }

        let response_data = response_body.data.ok_or_else(|| {
            GaloyCliError::GraphQl(message_only_error(
                "Empty `globals` response data".to_string(),
            ))
        })?;

        let result = Globals::try_from(response_data)?;

        Ok(result)
    }

    pub fn default_wallet(&self, username: String) -> Result<String, GaloyCliError> {
        let variables = query_default_wallet::Variables {
            username: username.clone(),
        };

        let response_body =
            post_graphql::<QueryDefaultWallet, _>(&self.graphql_client, &self.api, variables)?;

        if response_body.errors.is_some() {
            if let Some(errors) = response_body.errors {
                return Err(GaloyCliError::GraphQl(errors));
            }
        }

        let response_data = response_body.data.ok_or_else(|| {
            GaloyCliError::GraphQl(message_only_error(format!(
                "Empty response data. Username {} does not exist",
                username
            )))
        })?;

        let recipient_wallet_id = DefaultWallet::from(response_data).id;

        Ok(recipient_wallet_id)
    }

    pub fn me(&self) -> Result<QueryMeMe, GaloyCliError> {
        let variables = query_me::Variables;

        let response_body = post_graphql::<QueryMe, _>(&self.graphql_client, &self.api, variables)?;

        if response_body.errors.is_some() {
            if let Some(error) = response_body.errors {
                return Err(GaloyCliError::GraphQl(error));
            }
        }

        let response_data = response_body.data.ok_or_else(|| {
            GaloyCliError::GraphQl(message_only_error(
                "Empty `me` in response data.".to_string(),
            ))
        })?;

        let me = Me::try_from(response_data)?;
        let default_account = &me.id;
        let default_wallet = &me.default_account.default_wallet_id;
        info!(
            "default account {:?}, default walletId {:?}",
            default_account, default_wallet
        );

        Ok(me)
    }

    pub fn request_auth_code(&self, phone: String) -> Result<bool, GaloyCliError> {
        let input = UserRequestAuthCodeInput { phone };

        let variables = user_request_auth_code::Variables { input };

        let response_body =
            post_graphql::<UserRequestAuthCode, _>(&self.graphql_client, &self.api, variables)?;
        if response_body.errors.is_some() {
            if let Some(errors) = response_body.errors {
                return Err(GaloyCliError::GraphQl(errors));
            }
        }

        let response_data = response_body.data.ok_or_else(|| {
            GaloyCliError::GraphQl(message_only_error(
                "Empty `userRequestAuthCode` in response data".to_string(),
            ))
        })?;

        let auth_code_status = AuthCodeStatus::try_from(response_data)?;

        Ok(auth_code_status)
    }

    pub fn user_login(&self, phone: String, code: String) -> Result<String, GaloyCliError> {
        let input = UserLoginInput { phone, code };

        let variables = user_login::Variables { input };

        let response_body =
            post_graphql::<UserLogin, _>(&self.graphql_client, &self.api, variables)?;

        if response_body.errors.is_some() {
            if let Some(errors) = response_body.errors {
                return Err(GaloyCliError::GraphQl(errors));
            }
        }

        let response_data = response_body.data.ok_or_else(|| {
            GaloyCliError::GraphQl(message_only_error(
                "Empty `UserLogin` in response data".to_string(),
            ))
        })?;

        let auth_token = UserLoginAuthCode::try_from(response_data)?;

        Ok(auth_token.auth_token)
    }

    pub fn intraleger_send(
        &self,
        username: String,
        amount: Decimal,
        memo: Option<String>,
    ) -> Result<PaymentSendResult, GaloyCliError> {
        let me = self.me()?;
        let wallet_id = me.default_account.default_wallet_id;

        let recipient_wallet_id = self.default_wallet(username)?;

        let input = IntraLedgerPaymentSendInput {
            amount,
            memo,
            recipient_wallet_id,
            wallet_id,
        };

        let variables = intra_ledger_payment_send::Variables { input };

        let response_body =
            post_graphql::<IntraLedgerPaymentSend, _>(&self.graphql_client, &self.api, variables)?;

        if response_body.errors.is_some() {
            if let Some(errors) = response_body.errors {
                return Err(GaloyCliError::GraphQl(errors));
            }
        }

        let response_data = response_body.data.ok_or_else(|| {
            GaloyCliError::GraphQl(message_only_error(
                "Empty `intraLedgerPaymentSend` in response data".to_string(),
            ))
        })?;

        let status = PaymentSendResult::try_from(response_data)?;
        Ok(status)
    }

    // TODO: check if we can do self without &
    pub fn batch(self, filename: String, price: Decimal) -> Result<(), GaloyCliError> {
        let mut batch = Batch::new(self, price)?;

        batch.add_csv(filename)?;

        batch.populate_wallet_id()?;

        batch.populate_sats();

        println!("going to execute:");
        batch.show();

        batch.execute()?;

        Ok(())
    }
}
