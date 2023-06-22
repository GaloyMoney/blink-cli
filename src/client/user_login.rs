use anyhow::{bail, Context};
use graphql_client::reqwest::post_graphql;

use super::{queries::user_login, GaloyClient};
use crate::client::queries::{UserLogin, UserLoginInput};

impl GaloyClient {
    pub async fn user_login(&self, phone: String, code: String) -> anyhow::Result<String> {
        let input = UserLoginInput { phone, code };

        let variables = user_login::Variables { input };

        let response_body =
            post_graphql::<UserLogin, _>(&self.graphql_client, &self.api, variables)
                .await
                .context("issue fetching response")?;

        let response_data = response_body.data.context("Query failed or is empty")?;

        if let Some(auth_token) = response_data.user_login.auth_token {
            Ok(auth_token)
        } else if response_data.user_login.errors.is_empty() {
            bail!("request failed (unknown)")
        } else {
            println!("{:?}", response_data.user_login.errors);
            bail!("request failed (graphql errors)")
        }
    }
}
