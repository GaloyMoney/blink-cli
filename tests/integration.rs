use galoy_client::GaloyClient;

use rust_decimal_macros::dec;

#[test]
fn globals() {
    let api = "http://localhost:4002/graphql".to_string();
    // TODO: setup settings

    let galoy_client = GaloyClient::new(api, None);

    let query = galoy_client.globals();

    assert!(query.is_ok());
    let r = query.unwrap();

    println!("{:?}", r);
    assert_eq!(r.nodes_ids.len(), 2)
}

#[test]
fn default_wallet_for_username() {
    let api = "http://localhost:4002/graphql".to_string();
    // TODO: setup settings

    let username = "wrong username".to_string();

    let galoy_client = GaloyClient::new(api, None);
    let query = galoy_client.default_wallet(username);

    assert_eq!(query.is_err(), true);

    if let Err(value) = query {
        assert_eq!(value.to_string(), "Username doesn't exist");
    } else {
        panic!("should error")
    }

    let username = "userA".to_string();
    let query = galoy_client.default_wallet(username);

    assert_eq!(query.is_err(), false)
}

#[test]
fn login() {
    let api = "http://localhost:4002/graphql".to_string();

    let phone = "+16505554321".to_string();
    let code = "321321".to_string();

    let galoy_client = GaloyClient::new(api, None);

    let result = galoy_client
        .request_auth_code(phone.clone())
        .expect("request should succeed");
    assert!(result);

    let result = galoy_client
        .user_login(phone, code)
        .expect("request should succeed");
    assert_eq!(result[..2], "ey".to_string());
}

#[test]
fn intraledger_send() {
    let api = "http://localhost:4002/graphql".to_string();

    let phone = "+16505554321".to_string();
    let code = "321321".to_string();

    let galoy_client = GaloyClient::new(api.clone(), None);

    let jwt = galoy_client
        .user_login(phone, code)
        .expect("request should succeed");

    let username = "userB".to_string();

    let galoy_client = GaloyClient::new(api, Some(jwt));

    let amount = dec!(2);

    let result = galoy_client.intraleger_send(username, amount);

    assert!(result.is_ok())
}
