use anyhow::{bail, Context};
use graphql_client::reqwest::post_graphql_blocking as post_graphql;
use reqwest::blocking::Client;

use log::info;
use rust_decimal::Decimal;
use std::net::TcpListener;

pub mod queries;
pub use queries::*;

pub mod error;
pub use error::*;

pub mod batch;
pub use batch::Batch;

pub mod server;

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

    pub fn globals(&self) -> anyhow::Result<QueryGlobalsGlobals> {
        let variables = query_globals::Variables;

        let response_body =
            post_graphql::<QueryGlobals, _>(&self.graphql_client, &self.api, variables)
                .context("issue fetching response")?;

        let response_data = response_body.data.context("bad response from server")?;

        let result = response_data.globals.context("empty response")?;

        Ok(result)
    }

    pub fn default_wallet(&self, username: String) -> anyhow::Result<String> {
        let variables = query_default_wallet::Variables {
            username: username.clone(),
        };

        let response_body =
            post_graphql::<QueryDefaultWallet, _>(&self.graphql_client, &self.api, variables)
                .context("issue fetching response")?;

        let response_data = response_body
            .data
            .context(format!("Username {username} doesn't exist"))?;

        let recipient_wallet_id = response_data.account_default_wallet.id;

        Ok(recipient_wallet_id)
    }

    pub fn me(&self) -> anyhow::Result<QueryMeMe> {
        let variables = query_me::Variables;

        let response_body = post_graphql::<QueryMe, _>(&self.graphql_client, &self.api, variables)
            .context("issue getting response")?;

        let response_data = response_body.data.context("issue parsing response")?; // TODO: check the error given is correct

        let me = response_data.me.context("impossible to unwrap .me")?;
        let default_account = &me.id;
        let default_wallet = &me.default_account.default_wallet_id;
        info!(
            "default account {:?}, default walletId {:?}",
            default_account, default_wallet
        );

        Ok(me)
    }

    pub fn request_phone_code(&self, phone: String, nocaptcha: bool) -> std::io::Result<()> {
        match nocaptcha {
            true => {
                println!("Fetching Captcha Challenge...");

                let cc = self
                    .create_captcha_challenge()
                    .expect("Failed to get captcha");

                let listener = TcpListener::bind("127.0.0.1:0")?;
                let port = listener.local_addr().unwrap().port();
                println!(
                    "Visit http://127.0.0.1:{}/login and solve the Captcha",
                    port
                );

                let rt = tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()?;
                rt.block_on(server::run(listener, cc, phone, self.api.clone())?)
            }

            false => {
                let input = UserRequestAuthCodeInput { phone };

                let variables = user_request_auth_code::Variables { input };
                let response_body = post_graphql::<UserRequestAuthCode, _>(
                    &self.graphql_client,
                    &self.api,
                    variables,
                )
                .expect("issue fetching response");

                let response_data = response_body.data.expect("Query failed or is empty"); // TODO: understand when this can fail here
                let UserRequestAuthCodeUserRequestAuthCode { success, errors } =
                    response_data.user_request_auth_code;

                match success {
                    Some(true) => {}
                    _ if !errors.is_empty() => {
                        println!("{:?}", errors);
                        log::error!("request failed (graphql errors)");
                    }
                    Some(false) => {
                        log::error!("request failed (success is false)");
                    }
                    _ => {
                        log::error!("request failed (unknown)");
                    }
                }

                Ok(())
            }
        }
    }

    pub fn user_login(&self, phone: String, code: String) -> anyhow::Result<String> {
        let input = UserLoginInput { phone, code };

        let variables = user_login::Variables { input };

        let response_body =
            post_graphql::<UserLogin, _>(&self.graphql_client, &self.api, variables)
                .context("issue fetching response")?;

        let response_data = response_body.data.context("Query failed or is empty")?; // TODO: understand when this can fail here

        if let Some(auth_token) = response_data.user_login.auth_token {
            Ok(auth_token)
        } else if response_data.user_login.errors.is_empty() {
            bail!("request failed (unknown)")
        } else {
            println!("{:?}", response_data.user_login.errors);
            bail!("request failed (graphql errors)")
        }
    }

    pub fn intraleger_send(
        &self,
        username: String,
        amount: Decimal,
        memo: Option<String>,
    ) -> anyhow::Result<PaymentSendResult> {
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
            post_graphql::<IntraLedgerPaymentSend, _>(&self.graphql_client, &self.api, variables)
                .context("issue fetching response")?;

        let response_data = response_body.data.context("Query failed or is empty")?; // TODO: understand when this can fail here

        if !response_data.intra_ledger_payment_send.errors.is_empty() {
            bail!(format!(
                "payment error: {:?}",
                response_data.intra_ledger_payment_send.errors
            ))
        };

        match response_data.intra_ledger_payment_send.status {
            Some(status) => Ok(status),
            None => bail!("failed payment (empty response)"),
        }
    }

    // TODO: check if we can do self without &
    pub fn batch(self, filename: String, price: Decimal) -> anyhow::Result<()> {
        let mut batch = Batch::new(self, price);

        batch.add_csv(filename).context("can't load file")?;

        batch
            .populate_wallet_id()
            .context("cant get wallet id for all username")?;

        batch
            .populate_sats()
            .context("cant set sats all payments")?;

        println!("going to execute:");
        batch.show();

        batch.execute().context("can't make payment successfully")?;

        Ok(())
    }

    fn create_captcha_challenge(&self) -> Result<CaptchaChallenge, CliError> {
        let variables = captcha_create_challenge::Variables;
        let response =
            post_graphql::<CaptchaCreateChallenge, _>(&self.graphql_client, &self.api, variables)?;
        if response.errors.is_some() {
            if let Some(error) = response.errors {
                return Err(CliError::CaptchaTopLevelError(error));
            }
        }
        let response = response.data.ok_or_else(|| {
            CliError::CaptchaInnerError("Empty captcha response data".to_string())
        })?;
        let captcha_challenge_result = CaptchaChallenge::try_from(response)?;

        Ok(captcha_challenge_result)
    }
}
