use anyhow::{bail, Context};
use graphql_client::reqwest::post_graphql_blocking as post_graphql;
use reqwest::blocking::Client;

use log::info;
use rust_decimal::Decimal;
use std::{collections::HashSet, net::TcpListener};

pub mod queries;
pub use queries::*;

pub mod error;
pub use error::*;

pub mod batch;
use crate::client::batch::*;

pub use self::query_me::WalletCurrency;

use crate::types::*;

pub mod server;

impl From<&WalletCurrency> for Wallet {
    fn from(currency: &WalletCurrency) -> Self {
        match currency {
            WalletCurrency::USD => Wallet::Usd,
            WalletCurrency::BTC => Wallet::Btc,
            _ => panic!("Unsupported currency"),
        }
    }
}

pub struct GaloyClient {
    graphql_client: Client,
    api: String,
}

impl GaloyClient {
    pub fn new(api: String, token: Option<String>) -> Self {
        let mut client_builder = Client::builder();

        if let Some(token) = token {
            client_builder = client_builder.default_headers(
                std::iter::once((
                    reqwest::header::AUTHORIZATION,
                    reqwest::header::HeaderValue::from_str(&format!("Bearer {}", token)).unwrap(),
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

    pub fn fetch_balance(
        &self,
        wallet_type: Option<Wallet>,
        wallet_ids: Vec<String>,
    ) -> anyhow::Result<Vec<WalletBalance>> {
        let me = self.me()?;
        let default_wallet_id = me.default_account.default_wallet_id;
        let wallets = &me.default_account.wallets;

        let wallet_ids_set: HashSet<_> = wallet_ids.into_iter().collect();

        let balances: Vec<_> = wallets
            .iter()
            .filter(|wallet_info| {
                wallet_ids_set.contains(&wallet_info.id)
                    || wallet_type.as_ref().map_or(wallet_ids_set.is_empty(), |w| {
                        *w == Wallet::from(&wallet_info.wallet_currency)
                    })
            })
            .map(|wallet_info| WalletBalance {
                currency: format!("{:?}", Wallet::from(&wallet_info.wallet_currency)),
                balance: wallet_info.balance,
                id: if wallet_info.wallet_currency == WalletCurrency::USD
                    || wallet_info.wallet_currency == WalletCurrency::BTC
                {
                    None
                } else {
                    Some(wallet_info.id.clone())
                },
                default: wallet_info.id == default_wallet_id,
            })
            .collect();

        if balances.is_empty() {
            Err(anyhow::anyhow!("No matching wallet found"))
        } else {
            Ok(balances)
        }
    }

    pub fn request_phone_code(&self, phone: String, nocaptcha: bool) -> std::io::Result<()> {
        match nocaptcha {
            false => {
                let listener = TcpListener::bind("127.0.0.1:0")?;
                let port = listener.local_addr().unwrap().port();
                println!(
                    "Visit http://127.0.0.1:{}/login and solve the Captcha",
                    port
                );

                let rt = tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()?;
                rt.block_on(server::run(listener, phone, self.api.clone())?)
            }

            true => {
                let input = UserRequestAuthCodeInput {
                    phone,
                    channel: Some(PhoneCodeChannelType::SMS),
                };

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
        let btc_wallet_id = me
            .default_account
            .wallets
            .iter()
            .find(|wallet| wallet.wallet_currency == WalletCurrency::BTC)
            .map(|wallet| &wallet.id)
            .expect("Can't get BTC wallet")
            .to_owned();

        let recipient_wallet_id = self.default_wallet(username)?;
        let input = IntraLedgerPaymentSendInput {
            amount,
            memo,
            recipient_wallet_id,
            wallet_id: btc_wallet_id,
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

    pub fn intraleger_usd_send(
        &self,
        username: String,
        amount: Decimal,
        memo: Option<String>,
    ) -> anyhow::Result<UsdPaymentSendResult> {
        let me = self.me()?;
        let usd_wallet_id = me
            .default_account
            .wallets
            .iter()
            .find(|wallet| wallet.wallet_currency == WalletCurrency::USD)
            .map(|wallet| &wallet.id)
            .expect("Can't get USD wallet")
            .to_owned();

        let recipient_wallet_id = self.default_wallet(username)?;
        let input = IntraLedgerUsdPaymentSendInput {
            amount,
            memo,
            recipient_wallet_id,
            wallet_id: usd_wallet_id,
        };

        let variables = intra_ledger_usd_payment_send::Variables { input };

        let response_body = post_graphql::<IntraLedgerUsdPaymentSend, _>(
            &self.graphql_client,
            &self.api,
            variables,
        )
        .context("issue fetching response")?;

        let response_data = response_body.data.context("Query failed or is empty")?; // TODO: understand when this can fail here

        if !response_data
            .intra_ledger_usd_payment_send
            .errors
            .is_empty()
        {
            bail!(format!(
                "payment error: {:?}",
                response_data.intra_ledger_usd_payment_send.errors
            ))
        };

        match response_data.intra_ledger_usd_payment_send.status {
            Some(status) => Ok(status),
            None => bail!("failed payment (empty response)"),
        }
    }

    pub fn set_username(&self, username: String) -> Result<(), &'static str> {
        let input = UserUpdateUsernameInput { username };

        let variables = user_update_username::Variables { input };

        let response_body =
            post_graphql::<UserUpdateUsername, _>(&self.graphql_client, &self.api, variables)
                .map_err(|_| "issue fetching response")?;

        let response_data = response_body.data.ok_or("Query failed or is empty")?;

        if !response_data.user_update_username.errors.is_empty() {
            return Err("Update username error");
        }

        Ok(())
    }

    pub fn batch_payment(self, file: String) -> anyhow::Result<String> {
        check_file_exists(&file)?;
        let (reader, wallet_type) = validate_csv(&self, &file)?;
        check_sufficient_balance(&reader, wallet_type.clone(), &self)?;
        execute_batch_payment(&reader, wallet_type, &self)?;
        Ok("Batch Payment Successful".to_string())
    }

    pub fn create_captcha_challenge(&self) -> Result<CaptchaChallenge, CliError> {
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
