use graphql_client::reqwest::post_graphql;
use rust_decimal::Decimal;

use crate::client::{
    errors::{api_error::ApiError, ClientError},
    queries::{
        ln_invoice_create::{self, LnInvoiceCreateLnInvoiceCreate},
        ln_invoice_payment_send,
        ln_usd_invoice_create::{self, LnUsdInvoiceCreateLnUsdInvoiceCreate},
        LnInvoiceCreate, LnInvoicePaymentInput, LnInvoicePaymentSend, LnUsdInvoiceCreate,
    },
    GaloyClient,
};

impl GaloyClient {
    pub async fn lightning_invoice_create_btc(
        &self,
        receiving_wallet_id: String,
        amount: Decimal,
        memo: Option<String>,
    ) -> Result<LnInvoiceCreateLnInvoiceCreate, ClientError> {
        let input = ln_invoice_create::LnInvoiceCreateInput {
            wallet_id: receiving_wallet_id,
            amount,
            memo,
        };
        let variables = ln_invoice_create::Variables { input };

        let response_body =
            post_graphql::<LnInvoiceCreate, _>(&self.graphql_client, &self.api, variables)
                .await
                .map_err(|err| ApiError::IssueGettingResponse(anyhow::Error::new(err)))?;

        let response_data = response_body.data.ok_or(ApiError::IssueParsingResponse)?;

        let result = response_data.ln_invoice_create;

        Ok(result)
    }

    pub async fn lightning_invoice_create_usd(
        &self,
        receiving_wallet_id: String,
        amount: Decimal,
        memo: Option<String>,
    ) -> Result<LnUsdInvoiceCreateLnUsdInvoiceCreate, ClientError> {
        let input = ln_usd_invoice_create::LnUsdInvoiceCreateInput {
            wallet_id: receiving_wallet_id,
            amount,
            memo,
        };
        let variables = ln_usd_invoice_create::Variables { input };

        let response_body =
            post_graphql::<LnUsdInvoiceCreate, _>(&self.graphql_client, &self.api, variables)
                .await
                .map_err(|err| ApiError::IssueGettingResponse(anyhow::Error::new(err)))?;

        let response_data = response_body.data.ok_or(ApiError::IssueParsingResponse)?;

        let result = response_data.ln_usd_invoice_create;

        Ok(result)
    }

    pub async fn ln_payment_send(
        &self,
        sender_wallet_id: String,
        payment_request: String,
        memo: Option<String>,
    ) -> Result<(), ClientError> {
        let input = LnInvoicePaymentInput {
            wallet_id: sender_wallet_id,
            payment_request,
            memo,
        };

        let variables = ln_invoice_payment_send::Variables { input };

        let response_body =
            post_graphql::<LnInvoicePaymentSend, _>(&self.graphql_client, &self.api, variables)
                .await
                .map_err(|err| ApiError::IssueGettingResponse(anyhow::Error::new(err)))?;

        let response_data = response_body.data.ok_or(ApiError::IssueParsingResponse)?;

        if !response_data.ln_invoice_payment_send.errors.is_empty() {
            let error_string: String = response_data
                .ln_invoice_payment_send
                .errors
                .iter()
                .map(|error| format!("{:?}", error))
                .collect::<Vec<String>>()
                .join(", ");

            return Err(ClientError::ApiError(ApiError::RequestFailedWithError(
                error_string,
            )));
        } else {
            Ok(())
        }
    }
}
