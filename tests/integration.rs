use galoy_client::GaloyClient;

use rust_decimal_macros::dec;

mod common;

#[test]
fn globals() {
    let galoy_client = common::unauth_client();

    let query = galoy_client.globals();

    assert!(query.is_ok());
    let r = query.unwrap();

    println!("{:?}", r);
    assert_eq!(r.nodes_ids.len(), 2)
}

#[test]
fn default_wallet_for_username() {
    let galoy_client = common::unauth_client();

    let username = "doesnotexit".to_string();

    let query = galoy_client.default_wallet(username.clone());

    assert_eq!(query.is_err(), true);

    if let Err(value) = query {
        assert!(value
            .to_string()
            .contains(&format!("Account does not exist for username {}", username)))
    } else {
        panic!("should error")
    }

    let username = "test".to_string();
    let query = galoy_client.default_wallet(username);
    assert_eq!(query.is_err(), false)
}

#[test]
fn login() -> anyhow::Result<()> {
    let galoy_client = common::unauth_client();

    let phone = "+16505554321".to_string();
    let code = "321321".to_string();

    // Assuming backend has UserRequestAuthCode mutation
    galoy_client
        .request_phone_code(phone.clone(), true)
        .expect("request should succeed");

    let result = galoy_client
        .user_login(phone, code)
        .expect("request should succeed");
    assert_eq!(result[..2], "ey".to_string());

    Ok(())
}

#[test]
fn intraledger_send() {
    let galoy_client = common::auth_client();

    let username = "userB".to_string();

    let amount = dec!(2);

    let memo = Some("test_integration".to_string());

    let result = galoy_client.intraleger_send(username, amount, memo);

    assert!(result.is_ok())
}

/// WIP test. To be updated as other login features are implemented
#[test]
fn alternative_captcha_login() -> anyhow::Result<()> {
    let galoy_client = common::unauth_client();
    let captcha = galoy_client.create_captcha_challenge()?;

    assert!(captcha.failback_mode == false);
    assert!(captcha.new_captcha == true);
    assert_eq!(captcha.id.len(), 32);
    assert_eq!(captcha.challenge_code.len(), 32);

    Ok(())
}
