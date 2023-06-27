use graphql_client::GraphQLQuery;

use rust_decimal::Decimal;

type Phone = String;
type AuthToken = String;
type OneTimeAuthCode = String;
type Username = String;
type WalletId = String;
type SignedAmount = Decimal;

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

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/client/gql/schema.gql",
    query_path = "src/client/gql/queries/me.gql",
    response_derives = "Debug, Serialize, PartialEq"
)]
pub struct QueryMe;
pub use self::query_me::QueryMeMe;
