use super::*;

pub fn unauth_client() -> galoy_client::GaloyClient {
    let api = "https://api.staging.galoy.io/graphql".to_string();
    GaloyClient::new(api, None)
}

pub fn auth_client() -> galoy_client::GaloyClient {
    let api = "https://api.staging.galoy.io/graphql".to_string();

    let galoy_client = unauth_client();

    let phone = "+16505554321".to_string();
    let code = "321321".to_string();

    let jwt = galoy_client
        .user_login(phone, code)
        .expect("request should succeed");

    GaloyClient::new(api, Some(jwt))
}
