use graphql_client::{reqwest::post_graphql_blocking as post_graphql, GraphQLQuery};
use reqwest::blocking::Client;

use self::query_default_wallet::QueryDefaultWalletAccountDefaultWallet;

use anyhow::anyhow;

type Username = String;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.graphql",
    query_path = "src/graphql/default_wallet.graphql",
    response_derives = "Debug, Serialize"
)]
struct QueryDefaultWallet;

pub fn default_wallet(
    api_url: String,
    // TODO pass graphql client instead
    username: String,
) -> anyhow::Result<QueryDefaultWalletAccountDefaultWallet> {
    let client = Client::builder().build()?;

    let variables = query_default_wallet::Variables { username };

    let response_body = post_graphql::<QueryDefaultWallet, _>(&client, api_url, variables)
        .expect("issue fetching response");

    let response_data = match response_body.data {
        Some(value) => value,
        None => {
            return Err(anyhow!("Username doesn't exist".to_string()));
            // equivalent to bail!("Username doesn't exist".to_string()));
        }
    };

    Ok(response_data.account_default_wallet)
}
