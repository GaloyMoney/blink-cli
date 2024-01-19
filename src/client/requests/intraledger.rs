use graphql_client::reqwest::post_graphql;
use intra_ledger_payment_send::IntraLedgerPaymentSendInput;
use rust_decimal::Decimal;

use crate::client::{
    errors::{api_error::ApiError, ClientError},
    queries::{
        intra_ledger_payment_send, intra_ledger_usd_payment_send, IntraLedgerPaymentSend,
        IntraLedgerUsdPaymentSend, IntraLedgerUsdPaymentSendInput,
    },
    GaloyClient,
};

impl GaloyClient {
    pub async fn intraleger_send_btc(
        &self,
        sender_wallet_id: String,
        recipient_wallet_id: String,
        amount: Decimal,
        memo: Option<String>,
    ) -> Result<(), ClientError> {
        let input = IntraLedgerPaymentSendInput {
            amount,
            memo,
            recipient_wallet_id,
            wallet_id: sender_wallet_id,
        };

        let variables = intra_ledger_payment_send::Variables { input };

        let response_body =
            post_graphql::<IntraLedgerPaymentSend, _>(&self.graphql_client, &self.api, variables)
                .await
                .map_err(|err| ApiError::IssueGettingResponse(anyhow::Error::new(err)))?;

        let response_data = response_body.data.ok_or(ApiError::IssueParsingResponse)?;

        if !response_data.intra_ledger_payment_send.errors.is_empty() {
            let error_string: String = response_data
                .intra_ledger_payment_send
                .errors
                .iter()
                .map(|error| format!("{:?}", error))
                .collect::<Vec<String>>()
                .join(", ");

            Err(ClientError::ApiError(ApiError::RequestFailedWithError(
                error_string,
            )))
        } else {
            Ok(())
        }
    }

    pub async fn intraleger_send_usd(
        &self,
        sender_wallet_id: String,
        recipient_wallet_id: String,
        amount: Decimal,
        memo: Option<String>,
    ) -> Result<(), ClientError> {
        let input = IntraLedgerUsdPaymentSendInput {
            amount,
            memo,
            recipient_wallet_id,
            wallet_id: sender_wallet_id,
        };

        let variables = intra_ledger_usd_payment_send::Variables { input };

        let response_body = post_graphql::<IntraLedgerUsdPaymentSend, _>(
            &self.graphql_client,
            &self.api,
            variables,
        )
        .await
        .map_err(|err| ApiError::IssueGettingResponse(anyhow::Error::new(err)))?;

        let response_data = response_body.data.ok_or(ApiError::IssueParsingResponse)?;

        if !response_data
            .intra_ledger_usd_payment_send
            .errors
            .is_empty()
        {
            let error_string: String = response_data
                .intra_ledger_usd_payment_send
                .errors
                .iter()
                .map(|error| format!("{:?}", error))
                .collect::<Vec<String>>()
                .join(", ");

            Err(ClientError::ApiError(ApiError::RequestFailedWithError(
                error_string,
            )))
        } else {
            Ok(())
        }
    }
}
