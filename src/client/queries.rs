use graphql_client::GraphQLQuery;

use self::captcha_create_challenge::ResponseData;
use rust_decimal::Decimal;

type Phone = String;
type AuthToken = String;
type OneTimeAuthCode = String;
type Username = String;
type WalletId = String;
type SignedAmount = Decimal;
type SatAmount = Decimal;
type CentAmount = Decimal;
type Memo = String;
type OnChainAddress = String;
type DisplayCurrency = String;
type Timestamp = i128;
type SafeInt = i128;

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
    query_path = "src/client/gql/mutations/user_logout.gql",
    response_derives = "Debug, Serialize"
)]
pub(super) struct UserLogout;
pub use self::user_logout::UserLogoutInput;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/client/gql/schema.gql",
    query_path = "src/client/gql/mutations/captcha_create_challenge.gql",
    response_derives = "Debug, Serialize"
)]
pub struct CaptchaCreateChallenge;
pub use self::captcha_create_challenge::*;

pub struct CaptchaChallenge {
    pub id: String,
    pub challenge_code: String,
    pub new_captcha: bool,
    pub failback_mode: bool,
}

impl TryFrom<ResponseData> for CaptchaChallenge {
    type Error = CaptchaError;

    fn try_from(response: ResponseData) -> Result<Self, Self::Error> {
        let result = response.captcha_create_challenge.result;
        let challenge = result.ok_or(CaptchaError::EmptyCaptcha)?;

        let (id, challenge_code, new_captcha, failback_mode) = (
            challenge.id,
            challenge.challenge_code,
            challenge.new_captcha,
            challenge.failback_mode,
        );
        Ok(CaptchaChallenge {
            id,
            challenge_code,
            new_captcha,
            failback_mode,
        })
    }
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/client/gql/schema.gql",
    query_path = "src/client/gql/mutations/captcha_request_auth_code.gql",
    response_derives = "Debug, Serialize"
)]
pub struct CaptchaRequestAuthCode;
pub use self::captcha_request_auth_code::CaptchaRequestAuthCodeCaptchaRequestAuthCode;
pub use self::captcha_request_auth_code::CaptchaRequestAuthCodeInput;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/client/gql/schema.gql",
    query_path = "src/client/gql/queries/me.gql",
    response_derives = "Debug, Serialize, PartialEq"
)]
pub struct QueryMe;
pub use self::query_me::QueryMeMe;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/client/gql/schema.gql",
    query_path = "src/client/gql/mutations/user_update_username.gql",
    response_derives = "Debug, Serialize"
)]
pub(super) struct UserUpdateUsername;
pub use self::user_update_username::UserUpdateUsernameInput;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/client/gql/schema.gql",
    query_path = "src/client/gql/mutations/account_update_default_wallet_id.gql",
    response_derives = "Debug, Serialize"
)]
pub(super) struct AccountUpdateDefaultWalletId;
pub use self::account_update_default_wallet_id::AccountUpdateDefaultWalletIdInput;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/client/gql/schema.gql",
    query_path = "src/client/gql/queries/default_wallet.gql",
    response_derives = "Debug, Serialize"
)]
pub(super) struct QueryDefaultWallet;
pub use self::query_default_wallet::QueryDefaultWalletAccountDefaultWallet;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/client/gql/schema.gql",
    query_path = "src/client/gql/mutations/intraledger_send.gql",
    response_derives = "Debug, Serialize"
)]
pub(super) struct IntraLedgerPaymentSend;
pub use self::intra_ledger_payment_send::IntraLedgerPaymentSendInput;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/client/gql/schema.gql",
    query_path = "src/client/gql/mutations/intraledger_usd_send.gql",
    response_derives = "Debug, Serialize"
)]
pub(super) struct IntraLedgerUsdPaymentSend;
pub use self::intra_ledger_usd_payment_send::IntraLedgerUsdPaymentSendInput;

use super::errors::captcha_error::CaptchaError;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/client/gql/schema.gql",
    query_path = "src/client/gql/mutations/onchain_address_current.gql",
    response_derives = "Debug, Serialize"
)]
pub(super) struct OnChainAddressCurrent;
pub use self::on_chain_address_current::OnChainAddressCurrentInput;
pub use self::on_chain_address_current::OnChainAddressCurrentOnChainAddressCurrent;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/client/gql/schema.gql",
    query_path = "src/client/gql/mutations/onchain_payment_send.gql",
    response_derives = "Debug, Serialize"
)]
pub(super) struct OnChainPaymentSend;
pub use self::on_chain_payment_send::OnChainPaymentSendInput;
pub use self::on_chain_payment_send::OnChainPaymentSendOnChainPaymentSend;
pub use self::on_chain_payment_send::PayoutSpeed;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/client/gql/schema.gql",
    query_path = "src/client/gql/queries/real_time_price.gql",
    response_derives = "Debug, Serialize"
)]
pub(super) struct RealtimePrice;
pub use self::realtime_price::RealtimePriceRealtimePrice;
pub use self::realtime_price::RealtimePriceRealtimePriceBtcSatPrice;
pub use self::realtime_price::RealtimePriceRealtimePriceUsdCentPrice;
