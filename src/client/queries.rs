use graphql_client::GraphQLQuery;

type Username = String;
type WalletId = String;
type SatAmount = u64;
type Memo = String;
type Phone = String;
type SignedAmount = i64;
type Language = String;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/client/graphql/schema.graphql",
    query_path = "src/client/graphql/queries/default_wallet.graphql",
    response_derives = "Debug, Serialize"
)]
pub(super) struct QueryDefaultWallet;
pub use self::query_default_wallet::QueryDefaultWalletAccountDefaultWallet;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/client/graphql/schema.graphql",
    query_path = "src/client/graphql/queries/query_globals.graphql",
    response_derives = "Debug, Serialize"
)]
pub(super) struct QueryGlobals;
pub use self::query_globals::QueryGlobalsGlobals;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/client/graphql/schema.graphql",
    query_path = "src/client/graphql/queries/me.graphql",
    response_derives = "Debug, Serialize"
)]
pub(super) struct QueryMe;
pub use self::query_me::QueryMeMe;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/client/graphql/schema.graphql",
    query_path = "src/client/graphql/mutations/intraledger_send.graphql",
    response_derives = "Debug, Serialize"
)]
pub(super) struct IntraLedgerPaymentSend;
// pub use self::query_globals::QueryGlobalsGlobals;
