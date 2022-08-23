use anyhow::Context;
use graphql_client::{reqwest::post_graphql_blocking as post_graphql, GraphQLQuery};
use reqwest::blocking::Client;

pub use self::query_globals::QueryGlobalsGlobals;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.graphql",
    query_path = "src/graphql/query_globals.graphql",
    response_derives = "Debug, Serialize"
)]
struct QueryGlobals;

pub fn run(client: &Client, api_url: &String) -> anyhow::Result<QueryGlobalsGlobals> {
    let variables = query_globals::Variables;

    let response_body = post_graphql::<QueryGlobals, _>(client, api_url, variables)
        .context("issue fetching response")?;

    let response_data = response_body.data.context("bad response from server")?;

    let result = response_data.globals.context("empty response")?;

    Ok(result)
}
