use graphql_client::GraphQLQuery;

pub use self::query_default_wallet::QueryDefaultWalletAccountDefaultWallet;

type Username = String;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/client/graphql/schema.graphql",
    query_path = "src/client/graphql/default_wallet.graphql",
    response_derives = "Debug, Serialize"
)]
pub(super) struct QueryDefaultWallet;

pub use self::query_globals::QueryGlobalsGlobals;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/client/graphql/schema.graphql",
    query_path = "src/client/graphql/query_globals.graphql",
    response_derives = "Debug, Serialize"
)]
pub(super) struct QueryGlobals;
