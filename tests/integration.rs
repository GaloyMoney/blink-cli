use galoy_cli::GaloyClient;

use rust_decimal_macros::dec;

mod common;

#[test]
fn globals() {
    let galoy_cli = common::unauth_client();

    let query = galoy_cli.globals();

    assert!(query.is_ok());
    let r = query.unwrap();

    println!("{:?}", r);
    assert_eq!(r.nodes_ids.len(), 2)
}
#[test]
fn default_wallet_for_username() {
    let galoy_cli = common::unauth_client();

    let username = "test".to_string();

    let query_result = galoy_cli.default_wallet(username);

    assert!(query_result.is_ok(), "Query failed unexpectedly");

    // default wallet id of username "test" is "b5f11112-4fdf-41c4-91df-6e75bcc6f8fe".
    let wallet_id = query_result.unwrap();
    assert_eq!(wallet_id, "b5f11112-4fdf-41c4-91df-6e75bcc6f8fe");
}

#[test]
#[ignore]
fn login() {
    let galoy_cli = common::unauth_client();

    let phone = "+16505554321".to_string();
    let code = "321321".to_string();

    // Assuming backend has UserRequestAuthCode mutation
    galoy_cli
        .request_phone_code(phone.clone(), true)
        .expect("request should succeed");

    let result = galoy_cli
        .user_login(phone, code)
        .expect("request should succeed");
    assert_eq!(result[..3], "ory".to_string());
}

#[test]
#[ignore]
fn intraledger_send() {
    let galoy_cli = common::auth_client();

    let username = "userB".to_string();

    let amount = dec!(2);

    let memo = Some("test_integration".to_string());

    let result = galoy_cli.intraleger_send(username, amount, memo);

    assert!(result.is_ok())
}

#[test]
fn create_captcha_challenge() -> anyhow::Result<()> {
    let galoy_cli = common::unauth_client();
    let captcha = galoy_cli.create_captcha_challenge()?;

    assert!(captcha.failback_mode == false);
    assert!(captcha.new_captcha == true);
    assert_eq!(captcha.id.len(), 32);
    assert_eq!(captcha.challenge_code.len(), 32);

    Ok(())
}
