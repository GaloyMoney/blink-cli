use graphql_client::GraphQLQuery;

type Phone = String;
type AuthToken = String;
type OneTimeAuthCode = String;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/client/gql/schema.gql",
    query_path = "src/client/gql/queries/globals.gql",
    response_derives = "Debug, Serialize"
)]
pub(super) struct QueryGlobals;
pub use self::query_globals::QueryGlobalsGlobals;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/client/gql/schema.gql",
    query_path = "src/client/gql/mutations/user_login.gql",
    response_derives = "Debug, Serialize"
)]
pub(super) struct UserLogin;
pub use self::user_login::UserLoginInput;
pub use self::user_login::UserLoginUserLogin;
