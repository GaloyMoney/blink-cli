use anyhow::{anyhow, Result};
use graphql_client::{reqwest::post_graphql_blocking as post_graphql, GraphQLQuery};
use reqwest::blocking::Client;

use self::query_globals::QueryGlobalsGlobals;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.graphql",
    query_path = "src/graphql/query_globals.graphql",
    response_derives = "Debug, Serialize"
)]
struct QueryGlobals;

pub fn globals(api_url: String) -> anyhow::Result<QueryGlobalsGlobals> {
    let client = Client::builder().build()?;

    let variables = query_globals::Variables;

    let response_body = post_graphql::<QueryGlobals, _>(&client, api_url, variables)
        .expect("issue fetching response");

    let response_data = response_body.data.expect("bad response from server");

    let result = match response_data.globals {
        Some(value) => value,
        None => return Err(anyhow!("empty response")),
    };

    Ok(result)
}
