use graphql_client::reqwest::post_graphql;
use rust_decimal::Decimal;

use crate::errors::CliError;
use crate::{
    client::{
        queries::{
            intra_ledger_payment_send, query_me::WalletCurrency, IntraLedgerPaymentSend,
            IntraLedgerPaymentSendInput,
        },
        GaloyClient,
    },
    errors::api_error::ApiError,
};

impl GaloyClient {
    pub async fn intraleger_send_btc(
        &self,
        username: String,
        amount: Decimal,
        memo: Option<String>,
    ) -> Result<(), CliError> {
        let me = self.me().await?;
        let btc_wallet_id = me
            .default_account
            .wallets
            .iter()
            .find(|wallet| wallet.wallet_currency == WalletCurrency::BTC)
            .map(|wallet| &wallet.id)
            .expect("Can't get BTC wallet")
            .to_owned();

        let recipient_wallet_id = self.default_wallet(username).await?;
        let input = IntraLedgerPaymentSendInput {
            amount,
            memo,
            recipient_wallet_id,
            wallet_id: btc_wallet_id,
        };

        let variables = intra_ledger_payment_send::Variables { input };

        let response_body =
            post_graphql::<IntraLedgerPaymentSend, _>(&self.graphql_client, &self.api, variables)
                .await
                .map_err(|_| ApiError::IssueGettingResponse)?;

        let response_data = response_body.data.ok_or(ApiError::IssueParsingResponse)?;

        if !response_data.intra_ledger_payment_send.errors.is_empty() {
            let error_string: String = response_data
                .intra_ledger_payment_send
                .errors
                .iter()
                .map(|error| format!("{:?}", error))
                .collect::<Vec<String>>()
                .join(", ");

            return Err(CliError::ApiError(ApiError::RequestFailedWithError(
                error_string,
            )));
        } else {
            Ok(())
        }
    }
}
