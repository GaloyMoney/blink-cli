use galoy_client::GaloyClient;

#[test]
fn test_globals() {
    let api = "http://localhost:4002/graphql".to_string();
    // TODO: setup settings

    let galoy_client = GaloyClient::new(api);

    let query = galoy_client.globals();

    assert!(query.is_ok());
    let r = query.unwrap();

    println!("{:?}", r);
    assert_eq!(r.nodes_ids.len(), 2)
}

#[test]
fn test_default_wallet() {
    let api = "http://localhost:4002/graphql".to_string();
    // TODO: setup settings

    let username = "wrong username".to_string();

    let galoy_client = GaloyClient::new(api);
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
