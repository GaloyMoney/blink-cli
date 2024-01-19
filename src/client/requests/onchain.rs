use graphql_client::reqwest::post_graphql;
use on_chain_address_current::OnChainAddressCurrentOnChainAddressCurrent;
use rust_decimal::Decimal;

use crate::client::{
    errors::{api_error::ApiError, ClientError},
    queries::{
        on_chain_address_current, on_chain_payment_send, OnChainAddressCurrent, OnChainPaymentSend,
        OnChainPaymentSendInput, PayoutSpeed,
    },
    GaloyClient,
};

impl GaloyClient {
    pub async fn onchain_address_current(
        &self,
        receiving_wallet_id: String,
    ) -> Result<OnChainAddressCurrentOnChainAddressCurrent, ClientError> {
        let input = on_chain_address_current::OnChainAddressCurrentInput {
            wallet_id: receiving_wallet_id,
        };
        let variables = on_chain_address_current::Variables { input };

        let response_body =
            post_graphql::<OnChainAddressCurrent, _>(&self.graphql_client, &self.api, variables)
                .await
                .map_err(|err| ApiError::IssueGettingResponse(anyhow::Error::new(err)))?;

        let response_data = response_body.data.ok_or(ApiError::IssueParsingResponse)?;

        let result = response_data.on_chain_address_current;

        Ok(result)
    }

    pub async fn onchain_payment_send(
        &self,
        sender_wallet_id: String,
        onchain_address: String,
        amount: Decimal,
        memo: Option<String>,
    ) -> Result<(), ClientError> {
        let input = OnChainPaymentSendInput {
            wallet_id: sender_wallet_id,
            address: onchain_address,
            speed: PayoutSpeed::FAST,
            amount,
            memo,
        };

        let variables = on_chain_payment_send::Variables { input };

        let response_body =
            post_graphql::<OnChainPaymentSend, _>(&self.graphql_client, &self.api, variables)
                .await
                .map_err(|err| ApiError::IssueGettingResponse(anyhow::Error::new(err)))?;

        let response_data = response_body.data.ok_or(ApiError::IssueParsingResponse)?;

        if !response_data.on_chain_payment_send.errors.is_empty() {
            let error_string: String = response_data
                .on_chain_payment_send
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
