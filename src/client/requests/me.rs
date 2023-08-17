use crate::client::{
    errors::{api_error::ApiError, me_error::MeError, ClientError},
    queries::{
        query_me, transactions, QueryMe, QueryMeMe, Transactions,
        TransactionsMeDefaultAccountTransactionsEdges,
    },
    GaloyClient,
};
use graphql_client::reqwest::post_graphql;

impl GaloyClient {
    pub async fn me(&self) -> Result<QueryMeMe, ClientError> {
        let variables = query_me::Variables;

        let response_body = post_graphql::<QueryMe, _>(&self.graphql_client, &self.api, variables)
            .await
            .map_err(|err| ApiError::IssueGettingResponse(anyhow::Error::new(err)))?;

        let response_data = response_body.data.ok_or(ApiError::IssueParsingResponse)?;
        let me = response_data.me.ok_or(MeError::FailedToUnwrapMe)?;
        Ok(me)
    }

    pub async fn list_transactions(
        &self,
        //TODO: Add pagination from BE

        // after: Option<String>,
        // before: Option<String>,
        // last: Option<i64>,
        // first: Option<i64>,
        // wallet_ids: Option<Vec<Option<String>>>,
    ) -> Result<Option<Vec<TransactionsMeDefaultAccountTransactionsEdges>>, ClientError> {
        let variables = transactions::Variables;

        let response_body =
            post_graphql::<Transactions, _>(&self.graphql_client, &self.api, variables)
                .await
                .map_err(|err| ApiError::IssueGettingResponse(anyhow::Error::new(err)))?;

        let response_data = response_body.data.ok_or(ApiError::IssueParsingResponse)?;
        let transactions = response_data
            .me
            .ok_or(MeError::FailedToUnwrapMe)?
            .default_account
            .transactions
            .ok_or(MeError::FailedToUnwrapTransactions)?
            .edges;
        Ok(transactions)
    }
}
