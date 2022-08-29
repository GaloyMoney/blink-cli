use anyhow::{bail, Context};
use graphql_client::reqwest::post_graphql_blocking as post_graphql;
use log::info;
use reqwest::blocking::Client;

mod queries;

use queries::*;

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

    pub fn default_wallet(
        &self,
        username: String,
    ) -> anyhow::Result<QueryDefaultWalletAccountDefaultWallet> {
        let variables = query_default_wallet::Variables { username };

        let response_body =
            post_graphql::<QueryDefaultWallet, _>(&self.graphql_client, &self.api, variables)
                .context("issue fetching response")?;

        let response_data = response_body.data.context("Username doesn't exist")?;

        Ok(response_data.account_default_wallet)
    }

    pub fn me(&self) -> anyhow::Result<QueryMeMe> {
        let variables = query_me::Variables;

        let response_body = post_graphql::<QueryMe, _>(&self.graphql_client, &self.api, variables)
            .context("issue fetching response")?;

        let response_data = response_body.data.context("Empty fields")?; // TODO: check the error given is correct

        let me = response_data.me.context("impossible to unwrap .me")?;
        let default_account = &me.id;
        let default_wallet = &me.default_account.default_wallet_id;
        info!(
            "default account {:?}, default walletId {:?}",
            default_account, default_wallet
        );

        Ok(me)
    }

    pub fn request_auth_code(&self, phone: String) -> anyhow::Result<bool> {
        let input = UserRequestAuthCodeInput { phone };

        let variables = user_request_auth_code::Variables { input };

        let response_body =
            post_graphql::<UserRequestAuthCode, _>(&self.graphql_client, &self.api, variables)
                .context("issue fetching response")?;

        let response_data = response_body.data.context("Query failed or is empty")?; // TODO: understand when this can fail here

        if let Some(success) = response_data.user_request_auth_code.success {
            if success {
                Ok(true)
            } else {
                bail!("request failed (success is false)")
            }
        } else if response_data.user_request_auth_code.success.is_none() {
            println!("{:?}", response_data.user_request_auth_code.errors);
            bail!("request failed (graphql errors)")
        } else {
            bail!("request failed (unknown)")
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
        amount: u64,
    ) -> anyhow::Result<PaymentSendResult> {
        let me = self.me()?;
        let wallet_id = me.default_account.default_wallet_id;

        let query = self.default_wallet(username);
        let recipient_wallet_id = query.expect("result should be received").id;

        let input = IntraLedgerPaymentSendInput {
            amount,
            memo: None,
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
}
