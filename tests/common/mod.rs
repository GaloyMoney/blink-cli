use galoy_client::GaloyClient;

pub fn unauth_client() -> galoy_client::GaloyClient {
    let api = "http://localhost:4002/graphql".to_string();
    GaloyClient::new(api, None)
}

pub fn auth_client() -> galoy_client::GaloyClient {
    let api = "http://localhost:4002/graphql".to_string();

    let galoy_client = common::unauth_client();

    let phone = "+16505554321".to_string();
    let code = "321321".to_string();

    let jwt = galoy_client
        .user_login(phone, code)
        .expect("request should succeed");

    GaloyClient::new(api, Some(jwt))
}
