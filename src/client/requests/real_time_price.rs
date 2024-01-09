use graphql_client::{reqwest::post_graphql, Response};

use crate::client::{
    errors::{api_error::ApiError, ClientError},
    queries::{realtime_price, RealtimePrice, RealtimePriceRealtimePrice},
    GaloyClient,
};

impl GaloyClient {
    pub async fn realtime_price(
        &self,
        currency: String,
    ) -> Result<RealtimePriceRealtimePrice, ClientError> {
        let currency = currency;

        let variables = realtime_price::Variables {
            currency: Some(currency),
        };

        let response_body: Response<realtime_price::ResponseData> =
            post_graphql::<RealtimePrice, _>(&self.graphql_client, &self.api, variables)
                .await
                .map_err(|err| ApiError::IssueGettingResponse(anyhow::Error::new(err)))?;

        let response_data = response_body.data.ok_or(ApiError::IssueParsingResponse)?;

        let result = response_data.realtime_price;

        Ok(result)
    }

    pub async fn realtime_price_usd(&self) -> Result<RealtimePriceRealtimePrice, ClientError> {
        self.realtime_price("USD".to_string()).await
    }
}
