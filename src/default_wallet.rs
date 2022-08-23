use graphql_client::{reqwest::post_graphql_blocking as post_graphql, GraphQLQuery};
use reqwest::blocking::Client;

pub use self::query_default_wallet::QueryDefaultWalletAccountDefaultWallet;

use anyhow::{anyhow, Context};

type Username = String;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.graphql",
    query_path = "src/graphql/default_wallet.graphql",
    response_derives = "Debug, Serialize"
)]
struct QueryDefaultWallet;

pub fn run(
    client: &Client,
    api_url: &String,
    username: String,
) -> anyhow::Result<QueryDefaultWalletAccountDefaultWallet> {
    let variables = query_default_wallet::Variables { username };

    let response_body = post_graphql::<QueryDefaultWallet, _>(&client, api_url, variables)
        .context("issue fetching response")?;

    let response_data = match response_body.data {
        Some(value) => value,
        None => {
            return Err(anyhow!("Username doesn't exist".to_string()));
            // equivalent to bail!("Username doesn't exist".to_string()));
        }
    };

    Ok(response_data.account_default_wallet)
}
