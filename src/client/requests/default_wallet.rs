use crate::client::{
    errors::{api_error::ApiError, ClientError},
    queries::{
        account_update_default_wallet_id, query_default_wallet, AccountUpdateDefaultWalletId,
        AccountUpdateDefaultWalletIdInput, QueryDefaultWallet,
    },
    GaloyClient,
};

use graphql_client::reqwest::post_graphql;

impl GaloyClient {
    pub async fn default_wallet(&self, username: String) -> Result<String, ClientError> {
        let variables = query_default_wallet::Variables {
            username: username.clone(),
        };

        let response_body =
            post_graphql::<QueryDefaultWallet, _>(&self.graphql_client, &self.api, variables)
                .await
                .map_err(|err| ApiError::IssueGettingResponse(anyhow::Error::new(err)))?;

        let response_data = response_body.data.ok_or(ApiError::IssueParsingResponse)?;

        let recipient_wallet_id = response_data.account_default_wallet.id;

        Ok(recipient_wallet_id)
    }

    pub async fn update_default_wallet(&self, wallet_id: String) -> Result<(), ClientError> {
        let input = AccountUpdateDefaultWalletIdInput { wallet_id };

        let variables = account_update_default_wallet_id::Variables { input };

        let response_body = post_graphql::<AccountUpdateDefaultWalletId, _>(
            &self.graphql_client,
            &self.api,
            variables,
        )
        .await
        .map_err(|err| ApiError::IssueGettingResponse(anyhow::Error::new(err)))?;

        let response_data = response_body.data.ok_or(ApiError::IssueParsingResponse)?;

        if !response_data
            .account_update_default_wallet_id
            .errors
            .is_empty()
        {
            let error_string: String = response_data
                .account_update_default_wallet_id
                .errors
                .iter()
                .map(|error| format!("{:?}", error))
                .collect::<Vec<String>>()
                .join(", ");

            return Err(ClientError::ApiError(ApiError::RequestFailedWithError(
                error_string,
            )));
        }
        Ok(())
    }
}
