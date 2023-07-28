use graphql_client::reqwest::post_graphql;
use on_chain_address_current::OnChainAddressCurrentOnChainAddressCurrent;

use crate::client::{
    errors::{api_error::ApiError, ClientError},
    queries::{on_chain_address_current, OnChainAddressCurrent},
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
}
