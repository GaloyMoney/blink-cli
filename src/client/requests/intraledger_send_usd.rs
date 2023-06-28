use graphql_client::reqwest::post_graphql;
use rust_decimal::Decimal;

use crate::{
    client::{
        queries::{
            intra_ledger_usd_payment_send, query_me::WalletCurrency, IntraLedgerUsdPaymentSend,
            IntraLedgerUsdPaymentSendInput,
        },
        GaloyClient,
    },
    errors::{api_error::ApiError, payment_error::PaymentError, CliError},
};

impl GaloyClient {
    pub async fn intraleger_send_usd(
        &self,
        username: String,
        amount: Decimal,
        memo: Option<String>,
    ) -> Result<(), CliError> {
        let me = self.me().await?;
        let usd_wallet_id = me
            .default_account
            .wallets
            .iter()
            .find(|wallet| wallet.wallet_currency == WalletCurrency::USD)
            .map(|wallet| &wallet.id)
            .expect("Can't get USD wallet")
            .to_owned();

        let recipient_wallet_id = self.default_wallet(username).await?;
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
        .await
        .map_err(|_| ApiError::IssueGettingResponse)?;

        let response_data = response_body.data.ok_or(ApiError::IssueParsingResponse)?;

        if !response_data
            .intra_ledger_usd_payment_send
            .errors
            .is_empty()
        {
            return Err(CliError::PaymentError(PaymentError::PaymentError));
        } else {
            Ok(())
        }
    }
}
