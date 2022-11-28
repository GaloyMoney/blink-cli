use super::*;

/// Helper config struct for loading phone number and auth code from env var
#[derive(Debug)]
pub struct GaloyConfig {
    pub phone: String,
    pub code: String,
}

pub fn unauth_client() -> galoy_client::GaloyClient {
    let api = "http://localhost:4002/graphql".to_string();
    GaloyClient::new(api, None)
}

pub fn auth_client(config: GaloyConfig) -> galoy_client::GaloyClient {
    let api = "http://localhost:4002/graphql".to_string();

    let galoy_client = unauth_client();

    let jwt = galoy_client
        .user_login(config.phone, config.code)
        .expect("request should succeed");

    GaloyClient::new(api, Some(jwt))
}
